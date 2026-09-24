//! Async API for `CoreHaptics`
//!
//! This module provides async versions of `CoreHaptics` operations when the `async` feature is enabled.
//! The async API is **executor-agnostic** and works with any async runtime (Tokio, async-std, smol, etc.).
//!
//! ## Available Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`AsyncHapticEngine`] | Async engine operations |
//! | [`EngineFuture`] | Future for engine start/stop operations |
//! | [`NotifyPlayersFinishedFuture`] | Future for waiting on players to finish |
//!
//! ## Runtime Agnostic Design
//!
//! This async API uses only `std` types and works with **any** async runtime:
//! - Uses callback-based Swift FFI for true async operations
//! - Uses `std::sync::{Arc, Mutex}` for synchronization
//! - Uses `std::task::{Poll, Waker}` for async primitives
//! - Uses `std::future::Future` trait
//!
//! ## Examples
//!
//! ### Start an Engine Asynchronously
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use corehaptics::prelude::*;
//! use corehaptics::async_api::AsyncHapticEngine;
//!
//! # async fn example() -> corehaptics::Result<()> {
//! let capability = DeviceCapability::current()?;
//! if !capability.supports_haptics() {
//!     return Ok(());
//! }
//! let engine = HapticEngine::new()?;
//! AsyncHapticEngine::start(&engine).await?;
//! println!("Engine started!");
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ### Stop an Engine Asynchronously
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use corehaptics::prelude::*;
//! use corehaptics::async_api::AsyncHapticEngine;
//!
//! # async fn example() -> corehaptics::Result<()> {
//! let capability = DeviceCapability::current()?;
//! if !capability.supports_haptics() {
//!     return Ok(());
//! }
//! let engine = HapticEngine::new()?;
//! AsyncHapticEngine::start(&engine).await?;
//! // ... play patterns ...
//! AsyncHapticEngine::stop(&engine).await?;
//! println!("Engine stopped!");
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ### Wait for Players to Finish
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use corehaptics::prelude::*;
//! use corehaptics::async_api::AsyncHapticEngine;
//!
//! # async fn example() -> corehaptics::Result<()> {
//! let capability = DeviceCapability::current()?;
//! if !capability.supports_haptics() {
//!     return Ok(());
//! }
//! let engine = HapticEngine::new()?;
//! AsyncHapticEngine::start(&engine).await?;
//! // ... play patterns ...
//! AsyncHapticEngine::notify_when_players_finished(&engine).await?;
//! println!("All players finished!");
//! # Ok(())
//! # }
//! # }
//! ```

use crate::error::CoreHapticsError;
use crate::{EngineFinishedAction, HapticEngine};
use doom_fish_utils::completion::{
    error_from_cstr, AsyncCompletion, AsyncCompletionFuture,
};
use doom_fish_utils::panic_safe::catch_user_panic;
use std::ffi::c_void;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, PoisonError};
use std::task::{Context, Poll, Waker};

// ============================================================================
// EngineFuture - Wraps AsyncCompletionFuture for engine operations
// ============================================================================

/// A future that completes when an engine operation (start/stop) finishes.
pub struct EngineFuture {
    inner: AsyncCompletionFuture<()>,
}

impl Future for EngineFuture {
    type Output = crate::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(CoreHapticsError::InvalidArgument))
    }
}

// ============================================================================
// NotifyPlayersFinishedFuture - Wraps AsyncCompletionFuture for players finished
// ============================================================================

/// A future that completes when all players have finished playing.
pub struct NotifyPlayersFinishedFuture {
    state: Arc<Mutex<PlayersFinishedState>>,
}

struct PlayersFinishedState {
    done: bool,
    result: Option<crate::Result<()>>,
    waker: Option<Waker>,
}

struct PlayersFinishedSignal(Arc<Mutex<PlayersFinishedState>>);

impl PlayersFinishedSignal {
    fn finish(&self, result: crate::Result<()>) {
        let waker = {
            let mut state = self.0.lock().unwrap_or_else(PoisonError::into_inner);
            if state.done {
                return;
            }
            state.done = true;
            state.result = Some(result);
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

impl Drop for PlayersFinishedSignal {
    fn drop(&mut self) {
        self.finish(Err(CoreHapticsError::OperationFailed(
            "CHHapticEngine.notifyWhenPlayersFinished released the handler before calling it: a later registration replaced it or the engine was released",
        )));
    }
}

fn players_finished_channel() -> (PlayersFinishedSignal, NotifyPlayersFinishedFuture) {
    let state = Arc::new(Mutex::new(PlayersFinishedState {
        done: false,
        result: None,
        waker: None,
    }));
    (
        PlayersFinishedSignal(Arc::clone(&state)),
        NotifyPlayersFinishedFuture { state },
    )
}

impl Future for NotifyPlayersFinishedFuture {
    type Output = crate::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(result) = state.result.take() {
            return Poll::Ready(result);
        }
        state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

// ============================================================================
// Callbacks for async operations
// ============================================================================

extern "C" fn engine_start_callback(
    _result: *const c_void,
    error: *const i8,
    ctx: *mut c_void,
) {
    // SAFETY: This is called from Swift FFI and must not panic across the boundary.
    // We wrap the entire callback in catch_user_panic to prevent UB.
    catch_user_panic("engine_start_callback", || {
        if error.is_null() {
            // SAFETY: ctx is a valid pointer from AsyncCompletion::create().
            unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
        } else {
            // SAFETY: error is a valid C string pointer from the Swift bridge.
            let msg = unsafe { error_from_cstr(error) };
            // SAFETY: ctx is a valid pointer from AsyncCompletion::create().
            unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
        }
    });
}

extern "C" fn engine_stop_callback(
    _result: *const c_void,
    error: *const i8,
    ctx: *mut c_void,
) {
    // SAFETY: This is called from Swift FFI and must not panic across the boundary.
    // We wrap the entire callback in catch_user_panic to prevent UB.
    catch_user_panic("engine_stop_callback", || {
        if error.is_null() {
            // SAFETY: ctx is a valid pointer from AsyncCompletion::create().
            unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
        } else {
            // SAFETY: error is a valid C string pointer from the Swift bridge.
            let msg = unsafe { error_from_cstr(error) };
            // SAFETY: ctx is a valid pointer from AsyncCompletion::create().
            unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
        }
    });
}

// ============================================================================
// AsyncHapticEngine - Main async API
// ============================================================================

/// Async operations for `HapticEngine`.
pub struct AsyncHapticEngine;

impl AsyncHapticEngine {
    /// Start the engine asynchronously.
    ///
    /// # Errors
    ///
    /// Returns an error if the engine is already running or if the operation fails.
    pub fn start(engine: &HapticEngine) -> EngineFuture {
        let (future, ctx) = AsyncCompletion::create();
        // SAFETY: engine.as_raw() is a valid engine pointer, engine_start_callback is a valid
        // callback, and ctx is a valid completion context from AsyncCompletion::create().
        unsafe {
            crate::ffi::chrs_engine_start_async(
                engine.as_raw(),
                engine_start_callback,
                ctx,
            );
        }
        EngineFuture { inner: future }
    }

    /// Stop the engine asynchronously.
    ///
    /// # Errors
    ///
    /// Returns an error if the engine is not running or if the operation fails.
    pub fn stop(engine: &HapticEngine) -> EngineFuture {
        let (future, ctx) = AsyncCompletion::create();
        // SAFETY: engine.as_raw() is a valid engine pointer, engine_stop_callback is a valid
        // callback, and ctx is a valid completion context from AsyncCompletion::create().
        unsafe {
            crate::ffi::chrs_engine_stop_async(
                engine.as_raw(),
                engine_stop_callback,
                ctx,
            );
        }
        EngineFuture { inner: future }
    }

    /// Wait for all pattern players to finish playing asynchronously.
    ///
    /// This is useful when you need to know when all patterns have completed
    /// playing before performing cleanup or shutdown operations.
    ///
    /// # Errors
    ///
    /// Returns the error `CoreHaptics` reports, or [`CoreHapticsError::OperationFailed`] when a
    /// later players-finished registration replaces this one or the engine is released first.
    #[must_use]
    pub fn notify_when_players_finished(
        engine: &HapticEngine,
    ) -> NotifyPlayersFinishedFuture {
        let (signal, future) = players_finished_channel();
        engine.notify_when_players_finished(move |error| {
            signal.finish(error.map_or(Ok(()), Err));
            EngineFinishedAction::LeaveEngineRunning
        });
        future
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Wake;

    struct CountingWaker(AtomicUsize);

    impl Wake for CountingWaker {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn a_handler_released_before_it_runs_resolves_the_future_with_an_error() {
        let (signal, mut future) = players_finished_channel();
        let counter = Arc::new(CountingWaker(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&counter));
        let mut cx = Context::from_waker(&waker);
        assert!(Pin::new(&mut future).poll(&mut cx).is_pending());
        drop(signal);
        assert_eq!(counter.0.load(Ordering::SeqCst), 1);
        assert!(matches!(
            Pin::new(&mut future).poll(&mut cx),
            Poll::Ready(Err(CoreHapticsError::OperationFailed(_)))
        ));
    }

    #[test]
    fn a_handler_that_ran_keeps_its_result_when_it_is_released() {
        let (signal, future) = players_finished_channel();
        signal.finish(Ok(()));
        signal.finish(Err(CoreHapticsError::OperationFailed("late")));
        drop(signal);
        assert!(pollster::block_on(future).is_ok());
    }

    #[test]
    fn handler_errors_reach_the_future() {
        let (signal, future) = players_finished_channel();
        signal.finish(Err(CoreHapticsError::InvalidArgument(
            "engine stopped".into(),
        )));
        drop(signal);
        assert!(matches!(
            pollster::block_on(future),
            Err(CoreHapticsError::InvalidArgument(message)) if message == "engine stopped"
        ));
    }
}
