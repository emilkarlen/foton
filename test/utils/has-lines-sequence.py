import sys

tokens = sys.argv[1:]

if not tokens:
    sys.exit(0)

prev = None

for line in sys.stdin.readlines():
    if tokens[0] in line:
        prev = tokens[0]
        prev_match = line
        del tokens[0]
        if not tokens:
            break

if tokens:
    hdr = "No line containing"
    if prev is None:
        print("{}: \"{}\"".format(hdr, tokens[0]), file=sys.stderr)
    else:
        print("{}: \"{}\"\nAfter match of \"{}\" on line:\n{}".format(hdr, tokens[0], prev, prev_match),
               file=sys.stderr)
    sys.exit(1)
