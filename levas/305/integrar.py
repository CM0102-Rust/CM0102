"""Integra a 305a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('fin_prm_start_new_season', 5856576, [129, 236, 0, 2, 0, 0]), ('nor_prm_start_new_season', 7940000, [129, 236, 0, 2, 0, 0]), ('swe_prm_start_new_season', 8979952, [129, 236, 0, 2, 0, 0]), ('por_prm_process_promotion_relegation_chain', 8190960, [161, 252, 173, 173, 0]), ('por_prm_start_new_season', 8190432, [129, 236, 0, 2, 0, 0]), ('fin_cup_start_new_season', 5844224, [129, 236, 0, 2, 0, 0]), ('nor_cup_start_new_season', 7928720, [129, 236, 0, 2, 0, 0]), ('por_cup_start_new_season', 8185152, [129, 236, 0, 2, 0, 0]), ('por_super_select_teams', 8204896, [129, 236, 0, 2, 0, 0]), ('por_super_create_round_table', 8204528, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('por_super_start_new_season', 8205456, [129, 236, 0, 2, 0, 0])]
BASE = 2609
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
