"""
[PEG_TRACE] Attempting to match rule expression_operator at 1:2 (pos 1)
[PEG_TRACE] Attempting to match rule expression at 1:2 (pos 1)
[PEG_TRACE] Attempting to match rule expression_grouped at 1:2 (pos 1)
[PEG_TRACE] Attempting to match rule lparen at 1:2 (pos 1)
"""
import sys
name = sys.argv[1]
print "Opening {}".format(name)

prefix = "[PEG_TRACE] "
lpad = 0
indent = 1

for line in open(name):

    if not line.startswith(prefix):
        pass

    line = line[len(prefix):].strip()

    print (" " * lpad) + line

    if line.startswith("Attempting to match"):
        lpad += indent
    elif line.startswith("Failed to match") or line.startswith("Matched"):
        lpad -= indent
