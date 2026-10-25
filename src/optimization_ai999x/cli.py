"""optimization-ai999x "<a question in words>" [--json] | --formulas | --version"""
import argparse


def main(argv=None) -> int:
    import optimization_ai999x as M
    ap = argparse.ArgumentParser(prog="optimization-ai999x", description="Offline optimization answers with steps and verification.")
    ap.add_argument("question", nargs="?")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--formulas", action="store_true")
    ap.add_argument("--version", action="store_true")
    a = ap.parse_args(argv)
    if a.version:
        print(M.__version__, "pack", M.pack().version)
        return 0
    if a.formulas:
        for f in M.formulas():
            print(f"{f['key']:28s} {f['equation']}")
        return 0
    if not a.question:
        ap.print_help()
        return 2
    ans = M.ask(a.question)
    print(ans.to_json(indent=1) if a.json else ans)
    return 0 if ans.solved else 1


if __name__ == "__main__":
    raise SystemExit(main())
