"""Integra a 310a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('den_first_start_new_season', 5579856, [129, 236, 0, 2, 0, 0]), ('den_second_start_new_season', 5590016, [129, 236, 0, 2, 0, 0]), ('ger_second_start_new_season', 6157456, [129, 236, 0, 2, 0, 0]), ('hol_first_start_new_season', 6251504, [129, 236, 0, 2, 0, 0]), ('bel_second_start_new_season', 4325200, [129, 236, 0, 2, 0, 0]), ('bel_third_start_new_season', 4335536, [129, 236, 0, 2, 0, 0]), ('den_prm_start_new_season', 5585568, [129, 236, 0, 2, 0, 0]), ('den_prm_refresh_second_division_clubs', 5586096, [129, 236, 8, 2, 0, 0]), ('hol_prm_start_new_season', 6260240, [129, 236, 0, 2, 0, 0]), ('ger_first_start_new_season', 6141088, [129, 236, 0, 2, 0, 0]), ('ger_first_process_promotion_relegation_chain', 6141632, [161, 252, 173, 173, 0, 86]), ('ger_first_exchange_regional_clubs_unplayed', 6141872, [129, 236, 12, 2, 0, 0]), ('bel_fa_cup_collect_entrants', 4315344, [129, 236, 88, 4, 0, 0]), ('den_cup_collect_entrants', 5575232, [129, 236, 4, 2, 0, 0]), ('ger_cup_collect_entrants', 6132592, [129, 236, 16, 2, 0, 0]), ('ger_lge_cup_collect_entrants', 6145792, [129, 236, 24, 2, 0, 0]), ('bel_super_select_teams', 4330224, [129, 236, 0, 2, 0, 0]), ('hol_super_select_teams', 6261776, [129, 236, 0, 2, 0, 0]), ('bel_second_add_result', 4325648, [83, 139, 92, 36, 12, 86]), ('bel_second_advance_stage', 4325792, [139, 81, 44, 139, 65, 48]), ('hol_first_add_result', 6251952, [83, 139, 92, 36, 12, 86]), ('bel_first_start_new_season', 4319888, [129, 236, 0, 2, 0, 0]), ('hol_first_advance_stage', 6252112, [129, 236, 0, 2, 0, 0]), ('hol_cup_advance_stage', 6242704, [139, 81, 44, 139, 65, 48])]
BASE = 2690
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
