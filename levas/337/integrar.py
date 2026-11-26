"""Integra a 337a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('game_daily_staff_update', 6087376, [85, 139, 236, 131, 228, 248]), ('build_board_match_reaction_text', 6973888, [129, 236, 232, 12, 0, 0]), ('news_board_match_reaction_text', 6972176, [106, 255, 104, 43, 207, 149, 0]), ('daily_update', 6879680, [106, 255, 104, 86, 204, 149, 0]), ('construct_nation_awards', 7766672, [139, 193, 51, 201, 199, 0, 16, 218, 150, 0]), ('construct_month_ratings', 7764720, [139, 68, 36, 4, 86]), ('construct_month_award', 7761424, [106, 255, 104, 136, 219, 149, 0]), ('month_award_post_winner_news', 7762736, [106, 255, 104, 171, 219, 149, 0]), ('month_award_write_news', 7763328, [129, 236, 128, 2, 0, 0]), ('nation_awards_record_match_ratings', 7766944, [139, 73, 10, 133, 201]), ('nation_awards_award_player_of_year_from_ratings', 7766976, [129, 236, 96, 2, 0, 0]), ('write_club_players_international_text', 7802144, [106, 255, 104, 187, 222, 149, 0]), ('write_call_up_news_text', 7791536, [129, 236, 120, 4, 0, 0]), ('post_qualified_nations_news', 7829984, [106, 255, 104, 187, 223, 149, 0]), ('write_qualified_nations_text', 7830688, [129, 236, 32, 6, 0, 0]), ('clear_competition_for_comp_clubs', 7820320, [131, 236, 8, 139, 68, 36, 12]), ('reschedule_clashing_club_fixtures', 7829472, [106, 255, 104, 152, 223, 149, 0]), ('post_player_praise_story', 7619136, [106, 255, 104, 6, 216, 149, 0]), ('create_player_media_story', 7520752, [106, 255, 104, 134, 214, 149, 0])]
BASE = 3946
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
