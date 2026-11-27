# comp_util_get_stage_name passes the empty text (0xad9c64) to langlib_sprintf as the argument of the stage text, as
# the original does for its 40 texts with a "<%s - COMMENT ...>" part (id 1106 "Rel. Pl<%s - COMMENT - Relegation
# playoff>" and the others): the port passed none, the %s read a stack leftover (0x7e) and the game crashed in the
# CRT sprintf (0x9480d0) at 2001-334 of the 340th batch's comparing real game. For the texts without a conversion
# the extra argument is not read. Run from a tree root (argument, or the current folder), with no mutacoes.py running
# on that tree.
import os, sys
raiz = sys.argv[1] if len(sys.argv) > 1 else '.'
p = os.path.join(raiz, 'rust/src/motor/comp/comp_util.rs')
s = open(p, encoding='utf-8').read()
a = "            lang_sprintf(buf, table[k].1);\n"
b = ("            // the empty text as the argument, as the original pushes it for its 40 texts with a \"<%s - COMMENT\"\n"
     "            // part (0x4b9043 for id 1106); the texts without a conversion do not read it (fixed 26/11/2026)\n"
     "            lang_sprintf(buf, table[k].1, 0xad9c64usize);\n")
assert s.count(a) == 1
s = s.replace(a, b)
a = ("/// `FN_LANGLIB_SPRINTF`, the text as the format, no arguments: the \"<%s - ...>\" parts are translator comments the\n"
     "/// langlib drops); any other id")
b = ("/// `FN_LANGLIB_SPRINTF`, the text as the format and the empty text 0xad9c64 as its argument, which the 40 texts with a\n"
     "/// \"<%s - COMMENT ...>\" part print); any other id")
assert s.count(a) == 1
s = s.replace(a, b)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
