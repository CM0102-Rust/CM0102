"""Applies rxdb_1 to the scratch copy ./b (like integrar.py) plus a test export `rxdb_save` for the emulator.
python apply.py [codigo.rs] [--sem-export]"""
import os, re, sys
S = os.path.dirname(os.path.abspath(__file__))
cod_path = next((a for a in sys.argv[1:] if not a.startswith('--')), r'D:\Sistemas\CM0102\levas\fila\rxdb_1\codigo.rs')
p = os.path.join(S, 'b', 'rust', 'src', 'motor.rs')
orig = p + '.orig'
if not os.path.exists(orig):
    open(orig, 'w', encoding='utf-8', newline='\n').write(open(p, encoding='utf-8').read())
s = open(orig, encoding='utf-8').read()
m = re.search(r'\n\}\nconst N: usize = (\d+);', s)
base = int(m.group(1))
line = '\n    Prologue { address: 0x52dad0, bytes: &[0x81, 0xec, 0x10, 0x0a, 0x00, 0x00] } => save_database, compare_save_database;'
s = s[:m.start()] + line + '\n}\nconst N: usize = %d;' % (base + 1) + s[m.end():]
anc = 'compare_value!(compare_staff_criterion, staff_criterion, IDX_STAFF_CRITERION, () -> u32);\n'
assert s.count(anc) == 1
cod = open(cod_path, encoding='utf-8').read().replace('%%I0%%', 'IDX_SAVE_DATABASE')
s = s.replace(anc, anc + '\n' + cod)
open(p, 'w', encoding='utf-8', newline='\n').write(s)
lib = os.path.join(S, 'b', 'rust', 'src', 'lib.rs')
lo = lib + '.orig'
if not os.path.exists(lo):
    open(lo, 'w', encoding='utf-8', newline='\n').write(open(lib, encoding='utf-8').read())
t = open(lo, encoding='utf-8').read()
df = os.path.join(S, 'b', 'rust', 'ddraw.def')
do = df + '.orig'
if not os.path.exists(do):
    open(do, 'w', encoding='utf-8', newline='\n').write(open(df, encoding='utf-8').read())
d = open(do, encoding='utf-8').read()
if '--sem-export' not in sys.argv:
    t += '\n#[no_mangle]\npub unsafe extern "C" fn rxdb_save(a: usize, b: i32, c: u32, e: u32) -> u32 {\n    motor::save_database(a, b, c, e)\n}\n'
    d = d.rstrip('\n') + '\n    rxdb_save\n'
open(lib, 'w', encoding='utf-8', newline='\n').write(t)
open(df, 'w', encoding='utf-8', newline='\n').write(d)
print('ok', base)
