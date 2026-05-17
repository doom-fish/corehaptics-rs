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
use crate::HapticEngine;
use doom_fish_utils::completion::{
    error_from_cstr, AsyncCompletion, AsyncCompletionFuture,
};
use std::ffi::c_void;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

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
    inner: AsyncCompletionFuture<()>,
}

impl Future for NotifyPlayersFinishedFuture {
    type Output = crate::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(CoreHapticsError::InvalidArgument))
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
    if error.is_null() {
        unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
}

extern "C" fn engine_stop_callback(
    _result: *const c_void,
    error: *const i8,
    ctx: *mut c_void,
) {
    if error.is_null() {
        unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
}

extern "C" fn notify_players_finished_callback(
    _result: *const c_void,
    error: *const i8,
    ctx: *mut c_void,
) {
    if error.is_null() {
        unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
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
    /// Returns an error if the operation fails.
    pub fn notify_when_players_finished(
        engine: &HapticEngine,
    ) -> NotifyPlayersFinishedFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::chrs_engine_notify_when_players_finished_async(
                engine.as_raw(),
                notify_players_finished_callback,
                ctx,
            );
        }
        NotifyPlayersFinishedFuture { inner: future }
    }
}
