"""Integra a 338a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('team_award_record_winners', 9109424, [81, 83, 85, 86, 139, 241]), ('team_award_send_club_news', 9109824, [106, 255, 104, 11, 56, 150, 0]), ('team_award_set_season_dates', 9109328, [131, 236, 16, 86, 139, 241]), ('team_award_write_news', 9110384, [129, 236, 244, 2, 0, 0]), ('team_award_set_dates', 9109248, [139, 68, 36, 4, 131, 236, 8]), ('year_award_record_winners', 9634544, [83, 85, 86, 139, 241]), ('year_award_post_winner_news', 9635056, [106, 255, 104, 219, 96, 150, 0]), ('year_award_write_news', 9635968, [129, 236, 72, 4, 0, 0]), ('world_cup_awards_construct', 9632784, [106, 255, 104, 40, 96, 150, 0]), ('world_cup_awards_init', 9632896, [100, 161, 0, 0, 0, 0]), ('world_cup_awards_daily_update', 9633264, [131, 236, 16, 86, 139, 116, 36, 24]), ('nation_awards_award_top_three_from_ratings', 7767840, [129, 236, 84, 2, 0, 0]), ('nation_awards_award_from_shortlist', 7768864, [139, 68, 36, 8, 129, 236, 80, 2, 0, 0]), ('nation_awards_award_tournament_player', 7770304, [131, 236, 124, 139, 132, 36, 128, 0, 0, 0]), ('nation_awards_award_tournament_team', 7772912, [131, 236, 20, 139, 68, 36, 24]), ('construct_wc_concacaf_cup', 9521648, [106, 255, 104, 41, 88, 150, 0]), ('wc_concacaf_cup_rank_teams', 9534896, [129, 236, 8, 2, 0, 0]), ('wc_concacaf_cup_build_team_table', 9526592, [129, 236, 0, 2, 0, 0]), ('wc_concacaf_cup_select_entrants', 9527888, [129, 236, 20, 2, 0, 0]), ('wc_concacaf_cup_create_central_american_groups', 9530576, [106, 255, 104, 156, 88, 150, 0]), ('wc_concacaf_cup_update_for_date', 9526848, [106, 255, 104, 104, 88, 150, 0]), ('construct_wc_europe_league', 9536000, [106, 255, 104, 87, 89, 150, 0])]
BASE = 3965
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
