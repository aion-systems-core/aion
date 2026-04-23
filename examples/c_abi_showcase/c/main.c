#include <aion/aion.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void die(const char *msg) {
    fprintf(stderr, "ERR %s\n", msg);
    exit(1);
}

static void check(int32_t code) {
    if (code != AION_OK) {
        const char *e = aion_last_error();
        fprintf(stderr, "ERR code=%d msg=%s\n", (int)code, e ? e : "");
        exit(1);
    }
}

int main(int argc, char **argv) {
    const char *out = argc > 1 ? argv[1] : "showcase_capsule.aionai";
    const char *evpath = argc > 2 ? argv[2] : "showcase_evidence.json";

    static char m[] = "demo";
    static char pr[] = "hello";
    AionCapsule cap;
    memset(&cap, 0, sizeof(cap));
    cap.model = m;
    cap.prompt = pr;
    cap.seed = 7u;
    check(aion_capsule_save(&cap, out));
    printf("capsule_save ok path=%s\n", out);

    AionRunResult rr;
    memset(&rr, 0, sizeof(rr));
    check(aion_replay_capsule(out, &rr));
    printf("replay ok exit=%d\n", (int)rr.exit_code);
    aion_free_run_result(&rr);

    uint8_t ev_ok = 0;
    int32_t evc = aion_evidence_verify(evpath, &ev_ok);
    if (evc != AION_OK) {
        printf("evidence_verify skipped or failed code=%d (provide valid JSON at %s)\n", (int)evc, evpath);
    } else {
        printf("evidence_verify ok valid=%u\n", (unsigned)ev_ok);
    }

    return 0;
}
