# The comparer of the CONCACAF qualifying's next stage only runs the original and checked that the stage stays or moves
# on by one; at stage 2 -> 3 the original builds the three semi-final groups (stages 3, 4 and 5,
# WcConcacafCup::create_semi_final_groups) and leaves the current stage at the last one, 5 (the copy does the same).
# Long real game of batch 341, 2004-170: "stage 2 -> 5". Run from a tree root (argument, or the current folder), with no
# mutacoes.py running on that tree.
import os, sys
raiz = sys.argv[1] if len(sys.argv) > 1 else '.'
p = os.path.join(raiz, 'rust/src/motor/comp/worldcup/wc_concacaf_cup.rs')
s = open(p, encoding='utf-8').read()
a = "    SCOREBOARD.record(after == before || after == before + 1, || format!(\"wc_concacaf_cup_advance_stage({:#x}): stage {} -> {}\", this, before, after));\n"
b = ("    // to stage 3 the original builds the three semi-final groups and leaves the current stage at the last, 5\n"
     "    let ok = after == before || after == before + 1 || (before == 2 && after == 5);\n"
     "    SCOREBOARD.record(ok, || format!(\"wc_concacaf_cup_advance_stage({:#x}): stage {} -> {}\", this, before, after));\n")
assert s.count(a) == 1
s = s.replace(a, b)
a = ("/// Comparer for `wc_concacaf_cup_advance_stage`: it builds stages, so only the original runs; the stage stays or moves\n"
     "/// on by one.\n")
b = ("/// Comparer for `wc_concacaf_cup_advance_stage`: it builds stages, so only the original runs; the stage stays or moves\n"
     "/// on by one, or goes from 2 to 5 (the semi-final groups are built at stages 3 to 5 and the last one stays current).\n")
assert s.count(a) == 1
s = s.replace(a, b)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
