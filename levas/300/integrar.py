"""Integra a 300a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('bel_fa_cup_start_new_season', 4316048, [129, 236, 0, 2, 0, 0]), ('den_cup_start_new_season', 5575936, [129, 236, 0, 2, 0, 0]), ('ger_cup_start_new_season', 6133632, [129, 236, 0, 2, 0, 0]), ('bel_super_start_new_season', 4330816, [129, 236, 0, 2, 0, 0]), ('hol_super_start_new_season', 6262336, [129, 236, 0, 2, 0, 0]), ('ger_lge_cup_start_new_season', 6146528, [129, 236, 0, 2, 0, 0]), ('construct_bel_fa_cup', 4313168, [106, 255, 104, 89, 63, 149, 0]), ('construct_den_cup', 5573040, [106, 255, 104, 137, 122, 149, 0]), ('construct_ger_cup', 6130720, [106, 255, 104, 57, 165, 149, 0]), ('construct_bel_super', 4329200, [106, 255, 104, 169, 64, 149, 0]), ('construct_hol_super', 6260768, [106, 255, 104, 57, 174, 149, 0]), ('construct_ger_lge_cup', 6144224, [106, 255, 104, 217, 165, 149, 0])]
BASE = 2478
m = re.search(r'\n\}\nconst N: usize = (\d+);', s)
assert m and int(m.group(1)) == BASE, ('o motor.rs mudou desde a materialização', m and m.group(1), BASE)
# one line per routine in `routines! { .. }` (rust/src/motor.rs; read by engenharia-reversa/tabela_motor.py)
linhas = ''.join('\n    Prologue { address: %#x, bytes: &[%s] } => %s, compare_%s;' % (e, ', '.join('0x%02x' % b for b in bs), a, a)
                 for a, e, bs in ROTINAS)
s = s[:m.start()] + linhas + '\n}\nconst N: usize = %d;' % (BASE + len(ROTINAS)) + s[m.end():]
anc = 'compare_value!(compare_staff_criterion, staff_criterion, IDX_STAFF_CRITERION, () -> u32);\n'
assert s.count(anc) == 1
s = s.replace(anc, anc + '\n' + open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read().strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
