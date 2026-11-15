"""Integra a 297a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('construct_sco_chal_cup', 8309120, [106, 255, 104, 185, 10, 150, 0]), ('sco_chal_cup_create_round_table', 8309824, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('sco_chal_cup_collect_entrants', 8310800, [129, 236, 0, 2, 0, 0]), ('construct_sco_fa_cup', 8311568, [106, 255, 104, 9, 11, 150, 0]), ('sco_fa_cup_create_round_table', 8312272, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('sco_fa_cup_collect_entrants', 8313584, [129, 236, 124, 2, 0, 0]), ('construct_wel_cup', 9585696, [106, 255, 104, 153, 92, 150, 0]), ('wel_cup_create_round_table', 9586400, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('wel_cup_collect_entrants', 9587376, [129, 236, 4, 2, 0, 0]), ('sco_chal_cup_start_new_season', 8311088, [129, 236, 0, 2, 0, 0]), ('sco_fa_cup_start_new_season', 8314384, [129, 236, 0, 2, 0, 0]), ('wel_cup_start_new_season', 9587792, [129, 236, 0, 2, 0, 0]), ('construct_sco_first', 8314816, [106, 255, 104, 89, 11, 150, 0]), ('sco_first_create_table', 8318256, [129, 236, 0, 2, 0, 0]), ('sco_first_set_up_rules', 8318544, [83, 86, 139, 241, 178, 3]), ('sco_first_create_schedule', 8315616, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('sco_first_start_new_season', 8318720, [129, 236, 0, 2, 0, 0]), ('sco_first_set_club_progress', 8319232, [102, 129, 121, 64, 207, 7]), ('construct_sco_second', 8335952, [106, 255, 104, 153, 12, 150, 0]), ('sco_second_create_table', 8339072, [129, 236, 0, 2, 0, 0]), ('sco_second_set_up_rules', 8339328, [86, 139, 241, 178, 3]), ('construct_sco_third', 8339952, [106, 255, 104, 233, 12, 150, 0]), ('sco_third_create_table', 8343072, [129, 236, 0, 2, 0, 0]), ('sco_third_set_up_rules', 8343328, [86, 139, 241, 176, 2]), ('sco_second_create_schedule', 8336704, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('sco_second_start_new_season', 8339504, [129, 236, 0, 2, 0, 0]), ('sco_third_create_schedule', 8340704, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('sco_third_start_new_season', 8343472, [129, 236, 0, 2, 0, 0]), ('construct_wel_first', 9588272, [106, 255, 104, 233, 92, 150, 0]), ('wel_first_set_up_rules', 9591888, [86, 139, 241, 51, 192]), ('wel_first_start_new_season', 9592048, [129, 236, 0, 2, 0, 0]), ('wel_first_create_schedule', 9588976, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('construct_sco_prm', 8326016, [106, 255, 104, 25, 12, 150, 0]), ('sco_prm_create_table', 8333344, [129, 236, 0, 2, 0, 0]), ('sco_prm_set_up_rules', 8333680, [83, 86, 139, 241, 87])]
BASE = 2386
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
