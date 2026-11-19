"""Integra a 307a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('spa_first_process_promotion_relegation_chain', 8718080, [161, 252, 173, 173, 0]), ('spa_first_exchange_second_division_clubs_unplayed', 8716480, [131, 236, 12, 161, 252, 173, 173, 0]), ('spa_first_start_new_season', 8715952, [129, 236, 0, 2, 0, 0]), ('fra_first_exchange_cfa_national_clubs', 6014848, [129, 236, 12, 2, 0, 0]), ('fra_first_start_new_season', 6014160, [100, 161, 0, 0, 0, 0]), ('ita_ser_a_process_promotion_relegation_chain', 6562832, [161, 252, 173, 173, 0]), ('ita_ser_a_exchange_serie_c2_clubs_unplayed', 6561728, [129, 236, 8, 2, 0, 0]), ('ita_ser_a_regroup_serie_c1_clubs', 6563376, [100, 161, 0, 0, 0, 0]), ('ita_ser_a_start_new_season', 6559488, [129, 236, 0, 2, 0, 0]), ('eng_prm_exchange_conference_clubs_unplayed', 5721696, [161, 252, 173, 173, 0]), ('kor_league_cup_select_teams', 6737696, [129, 236, 0, 2, 0, 0]), ('kor_league_cup_create_group_a_table', 6735728, [129, 236, 4, 2, 0, 0]), ('kor_league_cup_advance_stage', 6736592, [139, 81, 44, 139, 65, 48])]
BASE = 2653
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
