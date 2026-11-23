"""Integra a 323a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('wel_lge_cup_collect_entrants', 9596944, [129, 236, 0, 2, 0, 0]), ('wel_lge_cup_set_predicted_positions', 9598192, [129, 236, 32, 2, 0, 0]), ('wel_lge_cup_start_new_season', 9595040, [129, 236, 0, 2, 0, 0]), ('wel_lge_cup_create_schedule', 9593312, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('wel_prm_cup_create_schedule', 9599792, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('wel_prm_cup_start_new_season', 9601520, [129, 236, 0, 2, 0, 0]), ('construct_wel_lge_cup', 9592464, [106, 255, 104, 57, 93, 150, 0]), ('construct_wel_prm_cup', 9598960, [106, 255, 104, 217, 93, 150, 0]), ('wel_prm_cup_build_entrant_list', 9603168, [129, 236, 8, 2, 0, 0]), ('wel_prm_cup_set_predicted_positions', 9604608, [129, 236, 32, 2, 0, 0])]
BASE = 3764
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
