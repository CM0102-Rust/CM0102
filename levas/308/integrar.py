"""Integra a 308a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('fill_fixture_squads', 9491184, [129, 236, 20, 2, 0, 0]), ('release_fixture', 9492144, [131, 236, 12, 83, 139, 92, 36, 20]), ('get_player_position_group', 9494560, [139, 68, 36, 4, 139, 64, 97]), ('count_available_players', 9492720, [131, 236, 16, 83, 85, 86, 87]), ('top_up_club_squad', 9493216, [131, 236, 44, 83, 85, 86, 87]), ('create_virtual_player', 9494688, [106, 255, 104, 160, 85, 150, 0]), ('find_free_human_manager_row', 8621152, [139, 21, 188, 35, 174, 0]), ('count_human_managers', 8621328, [161, 188, 35, 174, 0]), ('is_current_manager_being_created', 8621536, [15, 191, 5, 122, 124, 183, 0]), ('year_record_match_ratings', 9639376, [81, 87, 139, 124, 36, 12]), ('year_add_player_rating', 9639568, [129, 236, 16, 2, 0, 0]), ('month_ratings_reset', 7765856, [86, 139, 241, 139, 6]), ('award_manager_add_year_award_record', 4276928, [139, 68, 36, 4, 139, 84, 36, 12]), ('award_manager_add_team_award_record', 4276992, [139, 68, 36, 4, 139, 84, 36, 12]), ('find_nation_awards', 4277056, [129, 236, 0, 2, 0, 0]), ('add_calendar_year_entry', 6222448, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('add_league_entry', 6222640, [139, 84, 36, 4, 129, 236, 4, 2, 0, 0]), ('try_add_entry', 6223328, [106, 255, 104, 27, 172, 149, 0]), ('match_record_has_more_goals', 8276960, [129, 236, 0, 2, 0, 0]), ('format_duration_text', 8279936, [139, 68, 36, 12, 129, 236, 0, 2, 0, 0]), ('init_loan_offer', 9336848, [139, 84, 36, 4, 129, 236, 0, 2, 0, 0])]
BASE = 2666
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
