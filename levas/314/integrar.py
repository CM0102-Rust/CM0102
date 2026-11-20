"""Integra a 314a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('eur_super_cup_start_new_season', 5744336, [129, 236, 0, 2, 0, 0]), ('european_cup_start_new_season', 5786976, [129, 236, 0, 2, 0, 0]), ('uefa_cup_start_new_season', 9449184, [129, 236, 0, 2, 0, 0]), ('intertoto_cup_start_new_season', 6502688, [129, 236, 0, 2, 0, 0]), ('uefa_cup_set_up_rounds', 9449648, [86, 139, 241, 106, 0]), ('world_club_cup_start_new_season', 9616800, [129, 236, 0, 2, 0, 0]), ('wc_asia_league_start_new_season', 9515872, [129, 236, 0, 2, 0, 0]), ('wc_south_american_league_start_new_season', 9573488, [129, 236, 0, 2, 0, 0]), ('wc_oceania_league_start_new_season', 9563968, [129, 236, 0, 2, 0, 0]), ('wc_europe_league_start_new_season', 9540656, [100, 161, 0, 0, 0, 0]), ('wc_african_cup_start_new_season', 9503760, [129, 236, 0, 2, 0, 0]), ('wc_concacaf_cup_start_new_season', 9527280, [129, 236, 0, 2, 0, 0]), ('world_cup_advance_stage', 9622704, [139, 81, 44, 139, 65, 48]), ('world_cup_create_knockout_stage', 9623408, [100, 161, 0, 0, 0, 0]), ('world_cup_record_season_honours', 9622288, [86, 139, 241, 139, 70, 12]), ('world_cup_build_fate_news_text', 9621152, [138, 68, 36, 12, 87]), ('write_award_news', 4277232, [139, 65, 8, 139, 76, 36, 4]), ('add_shortlist', 4278256, [129, 236, 0, 2, 0, 0]), ('construct_award_shortlist', 4289600, [106, 255, 104, 206, 61, 149, 0]), ('award_shortlist_add_entry', 4290720, [102, 139, 65, 4, 129, 236, 0, 2, 0, 0]), ('award_shortlist_announce', 4291104, [106, 255, 104, 235, 61, 149, 0]), ('fog_of_war_daily_update', 5962976, [86, 87, 139, 241, 191, 16, 0, 0, 0]), ('notes_post_due_reminders', 7951200, [81, 83, 85, 86, 87]), ('construct_ruling_body', 8288096, [106, 255, 104, 94, 8, 150, 0])]
BASE = 2743
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
