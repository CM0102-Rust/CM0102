"""Integra a 304a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('construct_nor_prm', 7935232, [106, 255, 104, 25, 230, 149, 0]), ('construct_por_prm', 8185632, [106, 255, 104, 41, 250, 149, 0]), ('construct_por_second', 8194384, [106, 255, 104, 121, 250, 149, 0]), ('construct_nor_first', 7929152, [106, 255, 104, 105, 229, 149, 0]), ('construct_swe_first', 8971744, [106, 255, 104, 249, 48, 150, 0]), ('construct_fin_first', 5844656, [106, 255, 104, 9, 143, 149, 0]), ('nor_prm_set_up_rules', 7939840, [83, 86, 139, 241, 184, 2, 0, 0, 0]), ('nor_first_set_up_rules', 7932672, [83, 86, 139, 241, 184, 2, 0, 0, 0]), ('swe_prm_set_up_rules', 8979792, [83, 86, 139, 241, 178, 3]), ('swe_first_set_up_rules', 8974480, [83, 86, 139, 241, 184, 2, 0, 0, 0]), ('por_prm_set_up_rules', 8190256, [86, 139, 241, 186, 2, 0, 0, 0]), ('por_second_set_up_rules', 8197792, [86, 139, 241, 186, 2, 0, 0, 0]), ('construct_swe_prm', 8977168, [106, 255, 104, 105, 49, 150, 0]), ('fin_prm_set_up_rules', 5856400, [83, 86, 139, 241, 184, 3, 0, 0, 0]), ('fin_first_set_up_rules', 5847936, [83, 86, 139, 241, 187, 1, 0, 0, 0]), ('nor_prm_create_table', 7939552, [129, 236, 0, 2, 0, 0]), ('swe_prm_create_table', 8979504, [129, 236, 0, 2, 0, 0]), ('fin_prm_create_table', 5856112, [129, 236, 0, 2, 0, 0]), ('swe_first_create_table', 8974224, [129, 236, 0, 2, 0, 0]), ('fin_first_create_table', 5847664, [129, 236, 0, 2, 0, 0]), ('nor_first_create_table', 7932320, [129, 236, 0, 2, 0, 0]), ('por_prm_create_table', 8189952, [129, 236, 0, 2, 0, 0]), ('por_second_create_table', 8197408, [129, 236, 8, 2, 0, 0]), ('por_second_start_new_season', 8197984, [129, 236, 0, 2, 0, 0]), ('swe_first_start_new_season', 8974672, [129, 236, 0, 2, 0, 0]), ('nor_first_start_new_season', 7932816, [129, 236, 0, 2, 0, 0]), ('fin_first_start_new_season', 5848096, [129, 236, 0, 2, 0, 0]), ('swe_first_advance_stage', 8975120, [129, 236, 0, 2, 0, 0]), ('nor_first_advance_stage', 7933264, [129, 236, 0, 2, 0, 0]), ('swe_first_create_play_off_stage', 8975328, [100, 161, 0, 0, 0, 0]), ('nor_first_create_play_off_stage', 7933472, [100, 161, 0, 0, 0, 0]), ('fin_first_advance_stage', 5848976, [129, 236, 0, 2, 0, 0]), ('fin_first_set_club_progress', 5848592, [139, 73, 8, 83, 86]), ('swe_prm_create_schedule', 8978096, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('fin_prm_create_schedule', 5854384, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('por_prm_create_schedule', 8186336, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('por_second_create_schedule', 8195136, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('swe_first_create_schedule', 8972512, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('fin_first_create_schedule', 5845440, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0])]
BASE = 2570
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
