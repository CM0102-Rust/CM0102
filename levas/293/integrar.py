"""Integra a 293a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('asia_nations_record_season_honours', 4257168, [139, 65, 12, 139, 72, 8]), ('oceania_nations_set_predicted_positions', 7971616, [129, 236, 0, 2, 0, 0]), ('wc_oceania_league_advance_stage', 9564368, [129, 236, 0, 2, 0, 0]), ('construct_euro_champ', 5744784, [106, 255, 104, 137, 135, 149, 0]), ('euro_champ_set_up_team_list', 5750144, [129, 236, 8, 2, 0, 0]), ('euro_champ_update_for_date', 5752352, [100, 161, 0, 0, 0, 0]), ('construct_world_cup', 9617232, [106, 255, 104, 137, 95, 150, 0]), ('world_cup_set_up_team_list', 9624128, [129, 236, 8, 2, 0, 0]), ('wc_south_american_league_update_for_date', 9572704, [106, 255, 104, 22, 92, 150, 0]), ('wc_asia_league_update_for_date', 9515328, [106, 255, 104, 118, 87, 150, 0]), ('wc_asia_league_build_entrant_list', 9518944, [129, 236, 12, 2, 0, 0]), ('wc_europe_league_build_entry_pool', 9543216, [129, 236, 12, 2, 0, 0]), ('wc_europe_league_update_for_date', 9544960, [106, 255, 104, 118, 90, 150, 0]), ('goldcup_build_fate_news_text', 6164304, [138, 68, 36, 12, 86])]
BASE = 2302
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
