"""Integra a 333a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('date_get_weekday_name', 5558640, [15, 191, 68, 36, 4]), ('date_get_short_month_name', 5558800, [15, 190, 68, 36, 4]), ('date_get_month_name', 5559216, [15, 190, 68, 36, 4]), ('date_get_time_of_day_name', 5558336, [139, 68, 36, 4, 37, 255, 0, 0, 0]), ('date_get_time_of_day_gender', 5558576, [232, 155, 35, 18, 0]), ('date_get_weekday_gender', 5559632, [139, 68, 36, 4, 131, 248, 9]), ('date_get_short_weekday_date_string', 5549584, [139, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('date_get_long_date_string', 5552672, [139, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('date_get_long_date_string_no_year', 5553904, [139, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('date_get_date_string_no_weekday', 5555120, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('database_load_language_names', 5454240, [100, 161, 0, 0, 0, 0]), ('formation_new', 5973952, [83, 85, 86, 139, 233]), ('game_run_to_start_date', 6080592, [106, 255, 104, 11, 164, 149, 0]), ('compare_shorts_ascending', 6119936, [139, 68, 36, 4, 139, 76, 36, 8]), ('game_date_static_init', 6080080, [185, 144, 44, 174, 0]), ('last_processed_date_static_init', 6080128, [185, 120, 44, 174, 0]), ('apply_team_story_response', 7596144, [131, 236, 32, 83, 85]), ('queue_obscene_gesture_story', 7696560, [106, 255, 104, 6, 217, 149, 0]), ('queue_wants_first_team_story', 7710272, [106, 255, 104, 187, 217, 149, 0]), ('create_player_outburst_story', 7551584, [106, 255, 104, 230, 214, 149, 0]), ('get_player_injury', 6483312, [129, 236, 0, 2, 0, 0]), ('key_nation_continent_has_selected_nation', 6724048, [139, 84, 36, 4, 83]), ('key_nation_number_active_entries', 6721856, [50, 210, 83, 136, 21, 56, 71, 182, 0]), ('key_nation_sorted_order', 6725824, [50, 192, 185, 56, 61, 182, 0]), ('key_nation_compare_entries', 6725888, [139, 76, 36, 4, 83]), ('nation_menu_split_part_count', 6726480, [128, 121, 9, 16, 15, 159, 192])]
BASE = 3864
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
