"""Integra a 311a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('pol_first_start_new_season', 8165184, [129, 236, 0, 2, 0, 0]), ('pol_first_promote_lower_division_clubs', 8167472, [129, 236, 8, 2, 0, 0]), ('pol_second_start_new_season', 8175616, [129, 236, 0, 2, 0, 0]), ('pol_cup_collect_entrants', 8160592, [129, 236, 36, 3, 0, 0]), ('pol_lge_cup_collect_entrants', 8170800, [129, 236, 4, 2, 0, 0]), ('gre_cup_collect_entrants', 6199936, [129, 236, 144, 3, 0, 0]), ('tur_second_b_start_new_season', 9438464, [129, 236, 0, 2, 0, 0]), ('tur_second_b_advance_stage', 9438912, [129, 236, 0, 2, 0, 0]), ('pol_first_advance_stage', 8166096, [139, 81, 44, 139, 65, 48])]
BASE = 2714
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
