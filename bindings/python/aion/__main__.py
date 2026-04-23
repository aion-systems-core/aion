"""``python -m aion`` — minimal pilot CLI (no colors)."""

from __future__ import annotations

import argparse
import json
import sys

from .api import Pilot


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(prog="aion", add_help=True)
    sub = p.add_subparsers(dest="cmd", required=True)

    s = sub.add_parser("execute-ai", help="save deterministic capsule")
    s.add_argument("--model", required=True)
    s.add_argument("--prompt", required=True)
    s.add_argument("--seed", type=int, required=True)
    s.add_argument("--out", required=True)

    s = sub.add_parser("replay", help="replay capsule path")
    s.add_argument("capsule")

    s = sub.add_parser("drift", help="drift between two capsule paths")
    s.add_argument("left")
    s.add_argument("right")

    s = sub.add_parser("evidence", help="verify evidence JSON path")
    s.add_argument("path")

    args = p.parse_args(argv)
    pilot = Pilot()
    if args.cmd == "execute-ai":
        pilot.execute_ai(model=args.model, prompt=args.prompt, seed=args.seed, out_path=args.out)
        sys.stdout.write("ok\n")
        return 0
    if args.cmd == "replay":
        r = pilot.replay(args.capsule)
        sys.stdout.write(json.dumps({"stdout": r.stdout, "exit_code": r.exit_code}, sort_keys=True))
        sys.stdout.write("\n")
        return 0
    if args.cmd == "drift":
        d = pilot.drift(args.left, args.right)
        sys.stdout.write(json.dumps({"changed": d.changed, "fields": d.fields}, sort_keys=True))
        sys.stdout.write("\n")
        return 0 if not d.changed else 2
    if args.cmd == "evidence":
        e = pilot.evidence(args.path)
        sys.stdout.write(json.dumps({"valid": e.valid}, sort_keys=True))
        sys.stdout.write("\n")
        return 0 if e.valid else 1
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
