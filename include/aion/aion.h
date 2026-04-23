#ifndef AION_H
#define AION_H

#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>

#ifdef _WIN32
#  ifdef AION_BUILD_DLL
#    define AION_EXPORT __declspec(dllexport)
#  else
#    define AION_EXPORT __declspec(dllimport)
#  endif
#else
#  define AION_EXPORT __attribute__((visibility("default")))
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define AION_OK 0
#define AION_ERR_GENERIC 1
#define AION_ERR_CAPSULE_NOT_FOUND 2
#define AION_ERR_CAPSULE_CORRUPT 3
#define AION_ERR_INVALID_POLICY 4
#define AION_ERR_DETERMINISM_FAILURE 5
#define AION_ERR_INTEGRITY_FAILURE 6
#define AION_ERR_EVIDENCE_INVALID 7
#define AION_ERR_IO 8
#define AION_ERR_OUT_OF_MEMORY 9
#define AION_ERR_UNSUPPORTED_VERSION 10

typedef struct {
    char* stdout_data;
    size_t stdout_len;
    char* stderr_data;
    size_t stderr_len;
    int32_t exit_code;
    uint64_t duration_ms;
    char* capsule_id;
} AionRunResult;

typedef struct {
    char* model;
    char* prompt;
    uint64_t seed;
    char* determinism_profile_json;
    char* token_trace_json;
    char* events_json;
    char* graph_json;
    char* why_report_json;
    char* drift_report_json;
    char* evidence_path;
} AionCapsule;

typedef struct {
    uint8_t tokens_equal;
    uint8_t trace_equal;
    uint8_t events_equal;
    uint8_t graph_equal;
    uint8_t why_equal;
    uint8_t drift_equal;
    uint8_t capsule_equal;
    uint8_t evidence_equal;
    char** differences;
    size_t differences_count;
} AionReplayComparison;

typedef struct {
    uint8_t changed;
    char* fields_json;
} AionDriftReport;

typedef struct {
    uint8_t policy_ok;
    uint8_t determinism_ok;
    uint8_t integrity_ok;
    char* violations_json;
} AionGovernanceReport;

AION_EXPORT int32_t aion_run(const char* cmd, const char** args, size_t args_len, const char** env, size_t env_len, AionRunResult* out_result);
AION_EXPORT int32_t aion_capsule_save(const AionCapsule* capsule, const char* path);
AION_EXPORT int32_t aion_capsule_load(const char* path, AionCapsule* out_capsule);
AION_EXPORT int32_t aion_replay_capsule(const char* capsule_path, AionRunResult* out_result);
AION_EXPORT int32_t aion_compare_capsules(const char* left_path, const char* right_path, AionReplayComparison* out_comparison);
AION_EXPORT int32_t aion_drift_between_capsules(const char* a_path, const char* b_path, AionDriftReport* out_report);
AION_EXPORT int32_t aion_why_explain_capsule(const char* capsule_path, char** out_why_json);
AION_EXPORT int32_t aion_graph_causal(const char* capsule_path, char** out_graph_json);
AION_EXPORT int32_t aion_validate_capsule(const char* capsule_path, const char* policy_path, AionGovernanceReport* out_report);
AION_EXPORT int32_t aion_evidence_verify(const char* evidence_path, uint8_t* out_valid);
AION_EXPORT int32_t aion_evidence_generate_keypair(uint8_t* out_private_key, size_t* out_private_len, uint8_t* out_public_key, size_t* out_public_len);
AION_EXPORT int32_t aion_evidence_sign(const uint8_t* evidence_data, size_t evidence_len, const uint8_t* private_key, size_t private_key_len, uint8_t* out_signature, size_t* out_signature_len);
AION_EXPORT int32_t aion_evidence_verify_with_key(const uint8_t* evidence_data, size_t evidence_len, const uint8_t* signature, size_t signature_len, const uint8_t* public_key, size_t public_key_len, uint8_t* out_valid);
AION_EXPORT int32_t aion_telemetry_set_enabled(uint8_t enabled);
AION_EXPORT int32_t aion_telemetry_get_enabled(uint8_t* out_enabled);
AION_EXPORT int32_t aion_setup(const char* config_path);
AION_EXPORT int32_t aion_doctor(char** out_diagnostic_json);
void aion_free_string(char* s);
void aion_free_run_result(AionRunResult* res);
const char* aion_last_error(void);

static inline void aion_string_free(char* s) { aion_free_string(s); }

#ifdef __cplusplus
}
#endif

#endif
