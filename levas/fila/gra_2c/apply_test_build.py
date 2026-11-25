"""Scratch test build for check_routine.py: copy this file and codigo.rs next to a `git archive HEAD rust cm0102-base`
extracted in ./b, run it (optionally with another .rs, e.g. a mutated copy), then build with /MAP
(`cargo rustc --release -- -C link-arg=/MAP:<path>`). It puts the code after the integrar anchor of the copy's motor.rs
and exports `gra2c_daily` (lib.rs wrapper and ddraw.def). Never on the main tree."""
import os, sys
S = os.path.dirname(os.path.abspath(__file__))
cod_path = sys.argv[1] if len(sys.argv) > 1 else os.path.join(S, 'codigo.rs')
p = os.path.join(S, 'b', 'rust', 'src', 'motor.rs')
orig = os.path.join(S, 'motor.rs.orig')
if not os.path.exists(orig):
    open(orig, 'w', encoding='utf-8', newline='\n').write(open(p, encoding='utf-8').read())
s = open(orig, encoding='utf-8').read()
anc = 'compare_value!(compare_staff_criterion, staff_criterion, IDX_STAFF_CRITERION, () -> u32);\n'
assert s.count(anc) == 1
cod = open(cod_path, encoding='utf-8').read().replace('%%I0%%', '0')
s = s.replace(anc, anc + '\n' + cod)
open(p, 'w', encoding='utf-8', newline='\n').write(s)
lib = os.path.join(S, 'b', 'rust', 'src', 'lib.rs')
lo = lib + '.orig'
if not os.path.exists(lo):
    open(lo, 'w', encoding='utf-8', newline='\n').write(open(lib, encoding='utf-8').read())
open(lib, 'w', encoding='utf-8', newline='\n').write(open(lo, encoding='utf-8').read() + '\n#[no_mangle]\npub unsafe extern "C" fn gra2c_daily() {\n    motor::game_daily_staff_update()\n}\n')
df = os.path.join(S, 'b', 'rust', 'ddraw.def')
t = open(df, encoding='utf-8').read()
if 'gra2c_daily' not in t:
    open(df, 'w', encoding='utf-8', newline='\n').write(t.rstrip('\n') + '\n    gra2c_daily\n')
print('ok')
