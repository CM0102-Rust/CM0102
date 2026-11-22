"""Integra a 317a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('ire_lge_cup_create_group_stage', 6519232, [106, 255, 104, 190, 180, 149, 0]), ('ire_lge_cup_update_staff_records_for_status', 6521104, [138, 68, 36, 12, 86]), ('ire_lge_cup_set_predicted_positions', 6521888, [129, 236, 32, 2, 0, 0]), ('ire_lge_cup_create_knockout_stage', 6519808, [100, 161, 0, 0, 0, 0]), ('ire_lge_cup_collect_entrants', 6520272, [129, 236, 160, 3, 0, 0]), ('ire_lge_cup_create_schedule', 6517024, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('nir_lge_cup_create_group_stage', 7918528, [106, 255, 104, 94, 228, 149, 0]), ('nir_lge_cup_update_staff_records_for_status', 7920064, [138, 68, 36, 12, 86]), ('nir_lge_cup_set_predicted_positions', 7920848, [129, 236, 32, 2, 0, 0]), ('nir_lge_cup_create_knockout_stage', 7919104, [100, 161, 0, 0, 0, 0]), ('nir_lge_cup_collect_entrants', 7919616, [129, 236, 4, 2, 0, 0]), ('nir_lge_cup_create_schedule', 7916240, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('ire_prm_create_schedule', 6528656, [100, 161, 0, 0, 0, 0]), ('ire_first_create_schedule', 6508352, [100, 161, 0, 0, 0, 0])]
BASE = 2773
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
