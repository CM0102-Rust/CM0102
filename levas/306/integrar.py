"""Integra a 306a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('eng_charity_start_new_season', 5691824, [129, 236, 0, 2, 0, 0]), ('fra_cup_start_new_season', 6008400, [129, 236, 0, 2, 0, 0]), ('fra_lge_cup_start_new_season', 6018080, [129, 236, 0, 2, 0, 0]), ('kor_fa_cup_start_new_season', 6728896, [129, 236, 0, 2, 0, 0]), ('fra_super_start_new_season', 6027680, [129, 236, 0, 2, 0, 0]), ('ita_c1_super_start_new_season', 6539936, [129, 236, 0, 2, 0, 0]), ('ita_super_start_new_season', 6683792, [129, 236, 0, 2, 0, 0]), ('kor_super_cup_start_new_season', 6741648, [129, 236, 0, 2, 0, 0]), ('spa_super_start_new_season', 8752336, [129, 236, 0, 2, 0, 0]), ('eng_cc_cup_start_new_season', 5689648, [129, 236, 0, 2, 0, 0]), ('eng_fa_cup_start_new_season', 5702544, [129, 236, 0, 2, 0, 0]), ('spa_cup_start_new_season', 8707952, [129, 236, 0, 2, 0, 0]), ('eng_first_start_new_season', 5713456, [129, 236, 0, 2, 0, 0]), ('eng_second_start_new_season', 5728288, [129, 236, 0, 2, 0, 0]), ('eng_third_start_new_season', 5734768, [129, 236, 0, 2, 0, 0]), ('fra_second_start_new_season', 6025568, [129, 236, 0, 2, 0, 0]), ('fra_third_start_new_season', 6031296, [129, 236, 0, 2, 0, 0]), ('ita_ser_c1a_start_new_season', 6600736, [129, 236, 0, 2, 0, 0]), ('ita_ser_c1b_start_new_season', 6617568, [129, 236, 0, 2, 0, 0]), ('spa_second_start_new_season', 8740224, [129, 236, 0, 2, 0, 0]), ('spa_lower_start_new_season', 8730672, [129, 236, 0, 2, 0, 0]), ('ita_c_cup_start_new_season', 6543232, [129, 236, 0, 2, 0, 0]), ('ita_cup_start_new_season', 6550496, [129, 236, 0, 2, 0, 0]), ('kor_league_cup_start_new_season', 6736128, [129, 236, 0, 2, 0, 0]), ('ita_ser_b_start_new_season', 6580576, [129, 236, 0, 2, 0, 0]), ('ita_ser_c1b_record_final_positions', 6619216, [81, 139, 65, 12, 51, 210]), ('kor_league_start_new_season', 6733152, [129, 236, 0, 2, 0, 0]), ('kor_league_set_up_rules', 6732992, [86, 139, 241, 51, 192]), ('kor_league_cup_set_up_rules', 6735984, [86, 139, 241, 51, 192]), ('aus_nsl_record_season_honours', 4267408, [139, 65, 12, 86, 87]), ('aus_nsl_start_new_season', 4267488, [129, 236, 0, 2, 0, 0]), ('eng_auto_cup_start_new_season', 5682352, [129, 236, 0, 2, 0, 0]), ('eng_auto_cup_set_up_rounds', 5682944, [86, 139, 241, 106, 0])]
BASE = 2620
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
