#pragma once

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

void *chrs_object_retain(void *obj);
void chrs_object_release(void *obj);
void chrs_string_free(char *ptr);

intptr_t chrs_error_code(void *error);
char *chrs_error_domain(void *error);
char *chrs_error_description(void *error);

void *chrs_capabilities_for_hardware(void);
bool chrs_capability_supports_haptics(void *capability);
bool chrs_capability_supports_audio(void *capability);
bool chrs_capability_event_parameter_attributes(
    void *capability,
    const char *parameterID,
    const char *eventType,
    float *outMin,
    float *outMax,
    float *outDefault,
    void **errorOut
);
bool chrs_capability_dynamic_parameter_attributes(
    void *capability,
    const char *parameterID,
    float *outMin,
    float *outMax,
    float *outDefault,
    void **errorOut
);

void *chrs_engine_create(void **errorOut);
bool chrs_engine_start(void *engine, void **errorOut);
bool chrs_engine_stop(void *engine, void **errorOut);
double chrs_engine_current_time(void *engine);
bool chrs_engine_plays_haptics_only(void *engine);
void chrs_engine_set_plays_haptics_only(void *engine, bool enabled);
bool chrs_engine_plays_audio_only(void *engine);
void chrs_engine_set_plays_audio_only(void *engine, bool enabled);
bool chrs_engine_is_muted_for_audio(void *engine);
void chrs_engine_set_muted_for_audio(void *engine, bool enabled);
bool chrs_engine_is_muted_for_haptics(void *engine);
void chrs_engine_set_muted_for_haptics(void *engine, bool enabled);
bool chrs_engine_auto_shutdown_enabled(void *engine);
void chrs_engine_set_auto_shutdown_enabled(void *engine, bool enabled);
void *chrs_engine_create_player(void *engine, void *pattern, void **errorOut);
void *chrs_engine_create_advanced_player(void *engine, void *pattern, void **errorOut);
bool chrs_engine_register_audio_resource(
    void *engine,
    const char *path,
    const char *optionsJSON,
    uint64_t *outResourceID,
    void **errorOut
);
bool chrs_engine_unregister_audio_resource(void *engine, uint64_t resourceID, void **errorOut);
bool chrs_engine_play_pattern_from_url(void *engine, const char *path, void **errorOut);
bool chrs_engine_play_pattern_from_data(
    void *engine,
    const uint8_t *bytes,
    uintptr_t length,
    void **errorOut
);
void chrs_engine_set_stopped_handler(
    void *engine,
    void (*callback)(void *context, int32_t reason),
    void *context,
    void (*dropContext)(void *context)
);
void chrs_engine_clear_stopped_handler(void *engine);
void chrs_engine_set_reset_handler(
    void *engine,
    void (*callback)(void *context),
    void *context,
    void (*dropContext)(void *context)
);
void chrs_engine_clear_reset_handler(void *engine);
void chrs_engine_notify_when_players_finished(
    void *engine,
    int32_t (*callback)(void *context, void *error),
    void *context,
    void (*dropContext)(void *context)
);

void *chrs_pattern_create(const char *patternJSON, void **errorOut);
void *chrs_pattern_create_from_dictionary_json(const char *patternJSON, void **errorOut);
void *chrs_pattern_create_from_ahap_file(const char *path, void **errorOut);
char *chrs_pattern_export_dictionary_json(void *pattern, void **errorOut);
double chrs_pattern_duration(void *pattern);

bool chrs_player_start(void *player, double time, void **errorOut);
bool chrs_player_stop(void *player, double time, void **errorOut);
bool chrs_player_cancel(void *player, void **errorOut);
bool chrs_player_send_parameters(void *player, const char *parametersJSON, double time, void **errorOut);
bool chrs_player_schedule_parameter_curve(
    void *player,
    const char *parameterCurveJSON,
    double time,
    void **errorOut
);
bool chrs_player_is_muted(void *player);
void chrs_player_set_muted(void *player, bool muted);

bool chrs_advanced_player_pause(void *player, double time, void **errorOut);
bool chrs_advanced_player_resume(void *player, double time, void **errorOut);
bool chrs_advanced_player_seek_to_offset(void *player, double offset, void **errorOut);
bool chrs_advanced_player_loop_enabled(void *player);
void chrs_advanced_player_set_loop_enabled(void *player, bool enabled);
double chrs_advanced_player_loop_end(void *player);
void chrs_advanced_player_set_loop_end(void *player, double loopEnd);
float chrs_advanced_player_playback_rate(void *player);
void chrs_advanced_player_set_playback_rate(void *player, float playbackRate);
void chrs_advanced_player_set_completion_handler(
    void *player,
    void (*callback)(void *context, void *error),
    void *context,
    void (*dropContext)(void *context)
);
void chrs_advanced_player_clear_completion_handler(void *player);

#ifdef __cplusplus
}
#endif
