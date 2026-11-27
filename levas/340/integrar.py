"""Integra a 340a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('construct_spa_second_b', 8740672, [106, 255, 104, 169, 39, 150, 0]), ('spa_second_b_set_up_rules', 8744688, [83, 86, 139, 241, 178, 3]), ('spa_second_b_create_schedule', 8741472, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('spa_second_b_load_group_clubs', 8744144, [129, 236, 0, 2, 0, 0]), ('spa_second_b_start_new_season', 8744848, [129, 236, 0, 2, 0, 0]), ('spa_second_b_advance_stage', 8745296, [129, 236, 0, 2, 0, 0]), ('spa_second_b_create_group_stage', 8745488, [106, 255, 104, 238, 39, 150, 0]), ('spa_second_b_create_promotion_playoff_groups', 8746352, [106, 255, 104, 14, 40, 150, 0]), ('spa_second_b_create_relegation_playoff', 8747104, [106, 255, 104, 46, 40, 150, 0]), ('spa_second_b_create_relegation_playoff_final', 8747520, [100, 161, 0, 0, 0, 0]), ('spa_second_b_update_for_date', 8749408, [86, 139, 241, 131, 126, 48, 7]), ('spa_second_b_update_staff_records_for_status', 8747888, [138, 68, 36, 12, 83]), ('spa_second_b_build_fate_news_text', 8748576, [139, 68, 36, 12, 131, 236, 100]), ('spa_second_b_get_club_array', 8749472, [129, 236, 4, 2, 0, 0]), ('spa_second_b_set_predicted_positions', 8749808, [129, 236, 4, 2, 0, 0]), ('spa_second_b_set_club_progress', 8750176, [139, 68, 36, 4, 139, 73, 8]), ('construct_swe_second', 8984304, [106, 255, 104, 185, 49, 150, 0]), ('swe_second_set_up_rules', 8987520, [83, 86, 139, 241, 184, 2, 0, 0, 0]), ('swe_second_create_schedule', 8985184, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('swe_second_create_table', 8987056, [129, 236, 8, 2, 0, 0]), ('swe_second_start_new_season', 8987664, [129, 236, 0, 2, 0, 0]), ('swe_second_advance_stage', 8988112, [129, 236, 0, 2, 0, 0]), ('swe_second_update_staff_records_for_status', 8990432, [138, 68, 36, 12, 129, 236, 0, 2, 0, 0]), ('swe_second_build_fate_news_text', 8991072, [139, 68, 36, 12, 86]), ('swe_second_get_club_array', 8991520, [129, 236, 4, 2, 0, 0]), ('swe_second_set_predicted_positions', 8991856, [129, 236, 4, 2, 0, 0]), ('swe_second_set_club_progress', 8992224, [139, 68, 36, 4, 139, 73, 8]), ('spa_first_exchange_second_division_b_clubs', 8717056, [129, 236, 52, 5, 0, 0]), ('spa_first_assign_second_division_b_groups', 8717616, [129, 236, 76, 3, 0, 0]), ('swe_prm_apply_second_division_moves', 8981312, [81, 161, 252, 173, 173, 0])]
BASE = 4018
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
