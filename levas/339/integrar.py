"""Integra a 339a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('pol_second_update_staff_records_for_status', 8176784, [139, 21, 88, 248, 156, 0]), ('pol_second_build_fate_news_text', 8177280, [83, 139, 92, 36, 40]), ('pol_second_set_club_progress', 8177664, [139, 73, 8, 83, 86]), ('tur_second_b_get_club_array', 9443328, [129, 236, 4, 2, 0, 0]), ('tur_second_b_set_predicted_positions', 9443664, [129, 236, 4, 2, 0, 0]), ('tur_second_b_set_club_progress', 9444032, [139, 68, 36, 4, 139, 73, 8]), ('tur_second_b_update_staff_records_for_status', 9442000, [138, 68, 36, 12, 129, 236, 0, 2, 0, 0]), ('tur_second_b_build_fate_news_text', 9442800, [139, 68, 36, 12, 86]), ('tur_second_b_record_final_positions', 9444256, [81, 83, 85, 86, 87]), ('tur_second_b_create_group_stage', 9439104, [106, 255, 104, 174, 80, 150, 0]), ('tur_second_b_create_promotion_group', 9439920, [106, 255, 104, 206, 80, 150, 0]), ('tur_second_b_create_classification_groups', 9440512, [106, 255, 104, 238, 80, 150, 0]), ('compare_clubs_by_last_division', 4983360, [139, 68, 36, 4, 139, 76, 36, 8]), ('gre_cup_create_schedule', 6196208, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('gre_cup_create_table', 6197856, [129, 236, 4, 2, 0, 0]), ('gre_cup_create_group_stage', 6198864, [106, 255, 104, 142, 169, 149, 0]), ('construct_gre_cup', 6195360, [106, 255, 104, 73, 169, 149, 0]), ('gre_cup_create_knockout_stage', 6199376, [106, 255, 104, 174, 169, 149, 0]), ('gre_cup_set_up_rules', 6198096, [83, 86, 139, 241, 176, 4]), ('gre_cup_start_new_season', 6198256, [129, 236, 0, 2, 0, 0]), ('gre_cup_set_predicted_positions', 6201440, [129, 236, 32, 2, 0, 0]), ('gre_cup_advance_stage', 6198720, [139, 81, 44, 139, 65, 48]), ('rus_cup_update_for_date', 8298176, [131, 236, 16, 83, 139, 92, 36, 28]), ('cro_a2a_advance_stage', 5333088, [139, 81, 44, 139, 65, 48]), ('cro_a2a_create_tiebreak_playoff_stage', 5333232, [106, 255, 104, 46, 116, 149, 0]), ('cro_a2a_update_for_date', 5333856, [139, 68, 36, 4, 83]), ('cro_a1_process_promotion_relegation_chain', 5329136, [161, 252, 173, 173, 0]), ('cro_a1_realign_second_division_groups', 5329296, [100, 161, 0, 0, 0, 0]), ('pol_first_update_staff_records_for_status', 8168096, [138, 68, 36, 12, 60, 1]), ('pol_first_build_fate_news_text', 8168224, [83, 138, 92, 36, 16]), ('pol_first_set_club_progress', 8168432, [139, 68, 36, 4, 139, 73, 8])]
BASE = 3987
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
