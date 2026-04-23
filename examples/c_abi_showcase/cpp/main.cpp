#include <aion/aion.h>
#include <cstdio>
#include <cstdlib>
#include <cstring>

static void check(int32_t code) {
    if (code != AION_OK) {
        const char *e = aion_last_error();
        std::fprintf(stderr, "ERR code=%d msg=%s\n", static_cast<int>(code), e ? e : "");
        std::exit(1);
    }
}

int main(int argc, char **argv) {
    const char *out = argc > 1 ? argv[1] : "showcase_capsule_cpp.aionai";
    AionCapsule cap{};
    cap.model = const_cast<char *>("demo");
    cap.prompt = const_cast<char *>("hello");
    cap.seed = 8u;
    check(aion_capsule_save(&cap, out));
    std::printf("capsule_save ok\n");

    AionRunResult rr{};
    check(aion_replay_capsule(out, &rr));
    std::printf("replay ok exit=%d\n", static_cast<int>(rr.exit_code));
    aion_free_run_result(&rr);

    AionDriftReport dr{};
    check(aion_drift_between_capsules(out, out, &dr));
    std::printf("drift self changed=%u\n", static_cast<unsigned>(dr.changed));
    if (dr.fields_json) {
        aion_free_string(dr.fields_json);
    }
    return 0;
}
