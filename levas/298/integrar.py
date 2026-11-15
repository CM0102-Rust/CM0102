"""Integra a 298a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('sco_prm_create_schedule', 8326784, [129, 236, 8, 2, 0, 0]), ('sco_prm_start_new_season', 8333936, [129, 236, 0, 2, 0, 0]), ('sco_prm_update_for_date', 8334624, [139, 68, 36, 8, 85]), ('sco_prm_create_split_stages', 8334736, [100, 161, 0, 0, 0, 0]), ('construct_sco_lge_cup', 8321504, [106, 255, 104, 201, 11, 150, 0]), ('sco_lge_cup_create_round_table', 8322224, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('sco_lge_cup_update_for_date', 8323632, [129, 236, 0, 2, 0, 0]), ('sco_lge_cup_start_new_season', 8325424, [129, 236, 0, 2, 0, 0]), ('wel_lge_cup_create_table', 9594624, [129, 236, 4, 2, 0, 0]), ('wel_lge_cup_set_up_rules', 9594880, [83, 86, 139, 241, 176, 4]), ('wel_lge_cup_advance_stage', 9595504, [139, 81, 44, 139, 65, 48]), ('wel_lge_cup_record_season_honours', 9595024, [139, 65, 12, 139, 72, 20]), ('wel_lge_cup_update_staff_records_for_status', 9597408, [138, 68, 36, 12, 86]), ('wel_lge_cup_create_group_stage', 9595648, [106, 255, 104, 126, 93, 150, 0]), ('wel_prm_cup_create_group_table', 9601120, [129, 236, 4, 2, 0, 0]), ('wel_prm_cup_set_up_rules', 9601376, [86, 139, 241, 176, 4]), ('wel_prm_cup_advance_stage', 9601984, [139, 81, 44, 139, 65, 48]), ('wel_prm_cup_update_staff_records_for_status', 9603824, [138, 68, 36, 12, 86]), ('wel_prm_cup_create_group_stage', 9602128, [106, 255, 104, 30, 94, 150, 0])]
BASE = 2421
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
