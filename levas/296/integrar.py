"""Integra a 296a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('ire_first_start_new_season', 6511408, [129, 236, 0, 2, 0, 0]), ('ire_first_advance_stage', 6511856, [129, 236, 0, 2, 0, 0]), ('ire_first_create_promotion_playoff_stage', 6512064, [100, 161, 0, 0, 0, 0]), ('nir_prm_start_new_season', 7925296, [129, 236, 0, 2, 0, 0]), ('ire_prm_start_new_season', 6531504, [129, 236, 0, 2, 0, 0]), ('construct_ire_lge_cup', 6516176, [106, 255, 104, 121, 180, 149, 0]), ('ire_lge_cup_create_table', 6518208, [129, 236, 4, 2, 0, 0]), ('ire_lge_cup_set_up_rules', 6518464, [83, 86, 139, 241, 176, 4]), ('construct_nir_lge_cup', 7915392, [106, 255, 104, 25, 228, 149, 0]), ('nir_lge_cup_create_table', 7917520, [129, 236, 4, 2, 0, 0]), ('nir_lge_cup_set_up_rules', 7917776, [83, 86, 139, 241, 176, 4]), ('ire_lge_cup_advance_stage', 6519088, [139, 81, 44, 139, 65, 48]), ('ire_lge_cup_start_new_season', 6518624, [129, 236, 0, 2, 0, 0]), ('nir_lge_cup_advance_stage', 7918384, [139, 81, 44, 139, 65, 48]), ('nir_lge_cup_start_new_season', 7917920, [129, 236, 0, 2, 0, 0]), ('nir_prm_create_schedule', 7922480, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('nir_first_create_schedule', 7910160, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('construct_ire_super_cup', 6532304, [106, 255, 104, 41, 182, 149, 0]), ('ire_super_cup_set_up_rules', 6534336, [83, 86, 139, 241, 176, 2]), ('ire_super_cup_create_schedule', 6532960, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('ire_super_cup_create_table', 6533312, [129, 236, 0, 2, 0, 0]), ('ire_super_cup_update_for_date', 6533888, [129, 236, 0, 2, 0, 0]), ('ire_super_cup_start_new_season', 6534480, [129, 236, 0, 2, 0, 0])]
BASE = 2363
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
