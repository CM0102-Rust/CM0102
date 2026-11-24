"""Integra a 330a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('write_team_media_story_news_text', 7650256, [106, 255, 100, 161, 0, 0, 0, 0]), ('nation_awards_daily_update', 7766800, [106, 255, 104, 200, 219, 149, 0]), ('month_award_new_season', 7762656, [83, 138, 153, 161, 0, 0, 0]), ('month_ratings_compare_average', 7766528, [86, 139, 116, 36, 8]), ('nation_awards_award_player_of_year', 7769328, [129, 236, 128, 2, 0, 0]), ('nation_awards_award_manager_of_year', 7768384, [139, 68, 36, 12, 139, 13, 252, 173, 173, 0]), ('nation_awards_award_team_of_season', 7772400, [129, 236, 24, 2, 0, 0]), ('database_compare_staff_by_birth_date', 5505328, [106, 255, 104, 70, 121, 149, 0]), ('init_transfer_rule_change_date', 5393616, [106, 255, 104, 209, 7, 0, 0]), ('staff_add_disliked_staff', 5536496, [83, 139, 92, 36, 8]), ('staff_add_favoured_club', 5537168, [86, 87, 139, 124, 36, 12]), ('eng_lng_load_tables', 5525440, [129, 236, 0, 2, 0, 0]), ('construct_transfer_manager', 9157168, [106, 255, 104, 136, 57, 150, 0]), ('setup_transfer_rules', 9250480, [106, 255, 104, 152, 66, 150, 0]), ('clear_old_past_transfers', 9191088, [106, 255, 104, 75, 60, 150, 0]), ('cancel_club_offers', 9193808, [106, 255, 104, 171, 60, 150, 0]), ('compare_next_sale_entries', 9327824, [139, 76, 36, 4, 139, 84, 36, 8]), ('compare_window_dates', 9328016, [139, 68, 36, 4, 139, 76, 36, 8]), ('compare_work_permit_entries', 9328064, [139, 68, 36, 4, 139, 76, 36, 8]), ('apply_story_effect_flags', 7564288, [131, 236, 32, 83, 85]), ('queue_player_sent_off_story', 7706128, [106, 255, 104, 131, 217, 149, 0]), ('date_set', 5546288, [129, 236, 0, 2, 0, 0]), ('date_sub_weeks', 5556912, [81, 86, 139, 241, 138, 13, 170, 36, 174, 0]), ('date_get_difference', 5557296, [106, 255, 104, 88, 122, 149, 0]), ('date_get_day_of_month', 5558128, [139, 65, 4, 86, 133, 192]), ('date_get_month', 5558192, [139, 65, 4, 186, 96, 132, 155, 0]), ('date_is_leap_year', 5558240, [139, 76, 36, 4, 129, 225, 255, 255, 0, 0])]
BASE = 3823
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
