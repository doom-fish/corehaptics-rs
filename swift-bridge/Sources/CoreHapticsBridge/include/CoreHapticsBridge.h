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

void *chrs_engine_create(void **errorOut);
bool chrs_engine_start(void *engine, void **errorOut);
bool chrs_engine_stop(void *engine, void **errorOut);
double chrs_engine_current_time(void *engine);
bool chrs_engine_auto_shutdown_enabled(void *engine);
void chrs_engine_set_auto_shutdown_enabled(void *engine, bool enabled);
void *chrs_engine_create_player(void *engine, void *pattern, void **errorOut);

void *chrs_pattern_create(const char *patternJSON, void **errorOut);
double chrs_pattern_duration(void *pattern);

bool chrs_player_start(void *player, double time, void **errorOut);
bool chrs_player_stop(void *player, double time, void **errorOut);
bool chrs_player_cancel(void *player, void **errorOut);

#ifdef __cplusplus
}
#endif
