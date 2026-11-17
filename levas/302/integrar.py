"""Integra a 302a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('rus_prm_set_up_rules', 8304448, [83, 86, 139, 241, 178, 3]), ('tur_first_set_up_rules', 9428880, [86, 139, 241, 186, 2, 0, 0, 0]), ('tur_second_set_up_rules', 9433728, [86, 139, 241, 186, 2, 0, 0, 0]), ('gre_prm_create_table', 6204768, [129, 236, 0, 2, 0, 0]), ('gre_second_create_table', 6208688, [129, 236, 0, 2, 0, 0]), ('rus_prm_create_table', 8304144, [129, 236, 0, 2, 0, 0]), ('rus_first_create_table', 8300224, [129, 236, 72, 2, 0, 0]), ('pol_second_create_table', 8175088, [129, 236, 0, 2, 0, 0]), ('tur_second_create_table', 9433424, [129, 236, 0, 2, 0, 0]), ('gre_prm_create_schedule', 6203088, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('gre_second_create_schedule', 6206832, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('tur_second_create_schedule', 9431472, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('construct_pol_cup', 8158592, [106, 255, 104, 233, 246, 149, 0]), ('pol_cup_create_round_table', 8159296, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('construct_pol_lge_cup', 8168672, [106, 255, 104, 233, 247, 149, 0]), ('construct_rus_cup', 8294928, [106, 255, 104, 249, 8, 150, 0]), ('rus_cup_create_round_table', 8295728, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('construct_tur_cup', 9421744, [106, 255, 104, 121, 79, 150, 0]), ('tur_cup_create_round_table', 9422448, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('construct_gre_super', 6209600, [106, 255, 104, 121, 170, 149, 0]), ('gre_super_create_round_table', 6210288, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('construct_pol_super', 8177952, [106, 255, 104, 169, 248, 149, 0]), ('pol_super_create_round_table', 8178640, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('pol_super_start_new_season', 8179600, [129, 236, 0, 2, 0, 0]), ('gre_prm_start_new_season', 6205264, [129, 236, 0, 2, 0, 0]), ('gre_second_start_new_season', 6209168, [129, 236, 0, 2, 0, 0]), ('gre_prm_swap_second_and_third_division_clubs', 6205728, [129, 236, 4, 2, 0, 0]), ('rus_first_start_new_season', 8300736, [129, 236, 0, 2, 0, 0]), ('rus_prm_start_new_season', 8304640, [129, 236, 0, 2, 0, 0]), ('rus_prm_exchange_first_and_second_division_clubs', 8306208, [129, 236, 44, 2, 0, 0]), ('tur_first_start_new_season', 9429056, [129, 236, 0, 2, 0, 0]), ('tur_second_start_new_season', 9433904, [129, 236, 0, 2, 0, 0]), ('tur_first_process_promotion_relegation_chain', 9429584, [86, 139, 241, 139, 6]), ('tur_first_create_table', 9428416, [129, 236, 0, 2, 0, 0]), ('tur_first_promote_lower_clubs_to_third_tier', 9429920, [129, 236, 8, 2, 0, 0]), ('gre_super_select_teams', 6210608, [129, 236, 0, 2, 0, 0]), ('pol_super_select_teams', 8178960, [129, 236, 0, 2, 0, 0])]
BASE = 2528
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
