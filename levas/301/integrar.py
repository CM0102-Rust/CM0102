"""Integra a 301a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('construct_cro_a1', 5324304, [106, 255, 104, 89, 115, 149, 0]), ('construct_cro_a2a', 5329680, [106, 255, 104, 233, 115, 149, 0]), ('construct_cro_a2b', 5334064, [106, 255, 104, 89, 116, 149, 0]), ('cro_a1_set_up_rules', 5327072, [83, 86, 139, 241, 178, 3]), ('cro_a2a_set_up_rules', 5332448, [83, 86, 139, 241, 178, 3]), ('cro_a2b_set_up_rules', 5336832, [83, 86, 139, 241, 178, 3]), ('cro_a1_create_table', 5326768, [129, 236, 0, 2, 0, 0]), ('cro_a2a_create_table', 5332144, [129, 236, 0, 2, 0, 0]), ('cro_a2b_create_table', 5336528, [129, 236, 0, 2, 0, 0]), ('cro_a1_create_schedule', 5325024, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cro_a2a_create_schedule', 5330400, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cro_a2b_create_schedule', 5334784, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cro_a1_start_new_season', 5327264, [129, 236, 0, 2, 0, 0]), ('cro_a2a_start_new_season', 5332640, [129, 236, 0, 2, 0, 0]), ('cro_a2b_start_new_season', 5337024, [129, 236, 0, 2, 0, 0]), ('cro_a1_advance_stage', 5327824, [139, 81, 44, 139, 65, 48]), ('cro_a2b_advance_stage', 5337472, [139, 81, 44, 139, 65, 48]), ('cro_a1_swap_second_and_third_tier_clubs', 5328592, [129, 236, 12, 2, 0, 0]), ('cro_a1_create_playoff_stage', 5327968, [106, 255, 104, 158, 115, 149, 0]), ('cro_a2b_create_tiebreak_playoff_stage', 5337616, [106, 255, 104, 158, 116, 149, 0]), ('construct_cro_cup', 5338240, [106, 255, 104, 201, 116, 149, 0]), ('cro_cup_create_round_table', 5338944, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cro_cup_collect_entrants', 5340080, [129, 236, 8, 2, 0, 0]), ('cro_cup_start_new_season', 5340512, [129, 236, 0, 2, 0, 0]), ('pol_cup_start_new_season', 8161440, [129, 236, 0, 2, 0, 0]), ('pol_lge_cup_start_new_season', 8171488, [129, 236, 0, 2, 0, 0]), ('rus_cup_start_new_season', 8297744, [129, 236, 0, 2, 0, 0]), ('tur_cup_start_new_season', 9424304, [129, 236, 0, 2, 0, 0]), ('gre_super_start_new_season', 6211248, [129, 236, 0, 2, 0, 0]), ('construct_gre_prm', 6202384, [106, 255, 104, 217, 169, 149, 0]), ('construct_gre_second', 6206128, [106, 255, 104, 41, 170, 149, 0]), ('construct_rus_first', 8298288, [106, 255, 104, 73, 9, 150, 0]), ('construct_tur_first', 9424736, [106, 255, 104, 201, 79, 150, 0]), ('construct_tur_second', 9430768, [106, 255, 104, 25, 80, 150, 0]), ('construct_rus_prm', 8301184, [106, 255, 104, 153, 9, 150, 0]), ('gre_prm_set_up_rules', 6205072, [85, 86, 139, 241, 178, 3]), ('gre_second_set_up_rules', 6208992, [86, 139, 241, 184, 2, 0, 0, 0]), ('rus_first_set_up_rules', 8300560, [86, 139, 241, 184, 2, 0, 0, 0])]
BASE = 2490
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
