"""Integra a 320a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('award_rating_filter_reputation_over_2000', 4279072, [139, 68, 36, 4, 133, 192]), ('award_rating_filter_selected_nations', 4279104, [139, 84, 36, 4, 86]), ('get_human_manager_record', 7754880, [139, 68, 36, 4, 133, 192]), ('match_team_pair_destructor', 4537280, [104, 16, 60, 69, 0]), ('rating_record_construct', 4537424, [86, 139, 241, 141, 78, 20]), ('squad_view_settings_construct', 4749328, [139, 193, 128, 201, 255]), ('player_age_record_assign', 4857776, [131, 236, 8, 141, 68, 36, 0]), ('attendance_record_assign', 4896176, [139, 68, 36, 4, 86]), ('date_equals', 4882208, [102, 139, 68, 36, 6]), ('text_set_string', 4908640, [129, 236, 0, 2, 0, 0]), ('text_append_string', 4908912, [129, 236, 0, 2, 0, 0]), ('text_add_link', 4909216, [100, 161, 0, 0, 0, 0]), ('text_construct', 4908432, [139, 68, 36, 4, 86]), ('text_destruct', 4908592, [86, 139, 241, 139, 6]), ('text_clear', 4913728, [86, 139, 241, 87, 139, 6]), ('pundit_list_construct', 8233360, [106, 255, 104, 76, 253, 149, 0]), ('pundit_list_destruct', 8234640, [86, 87, 139, 249, 139, 55]), ('apply_obscene_gesture_ban', 8236880, [106, 255, 104, 112, 253, 149, 0]), ('comp_stats_find_club_entry', 4868176, [106, 255, 104, 136, 103, 149, 0]), ('comp_stats_find_player_entry', 4868656, [129, 236, 48, 2, 0, 0]), ('comp_stats_get_club_stat', 4861456, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('comp_stats_get_player_stat', 4862368, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('comp_stats_get_sorted_team_table', 4868896, [129, 236, 12, 2, 0, 0]), ('comp_stats_get_sorted_player_table', 4869568, [129, 236, 16, 2, 0, 0]), ('comp_stats_update_season_end_records', 4856304, [85, 86, 139, 241, 139, 134, 2, 5, 0, 0]), ('comp_stats_archive_season_and_reset', 4856912, [106, 255, 104, 11, 103, 149, 0]), ('comp_stats_send_record_news', 4858960, [100, 161, 0, 0, 0, 0]), ('comp_stats_construct', 4845168, [106, 255, 104, 243, 100, 149, 0]), ('comp_stats_destruct', 4846672, [106, 255, 104, 153, 102, 149, 0]), ('comp_stats_get_club_projected_positions', 4907776, [100, 161, 0, 0, 0, 0]), ('club_history_record_comp_season', 4468352, [129, 236, 60, 2, 0, 0])]
BASE = 3682
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
