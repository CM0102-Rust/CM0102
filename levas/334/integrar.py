"""Integra a 334a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('database_trim_staff_tables', 5393776, [106, 255, 104, 167, 120, 149, 0]), ('fog_of_war_new', 5961264, [85, 139, 233, 87, 185, 16, 0, 0, 0]), ('fog_of_war_load', 5961440, [106, 255, 104, 108, 152, 149, 0]), ('fog_of_war_map_load', 5963792, [129, 236, 4, 2, 0, 0]), ('fog_of_war_save', 5962448, [129, 236, 12, 2, 0, 0]), ('fog_of_war_map_save', 5964144, [129, 236, 4, 2, 0, 0]), ('fog_of_war_destructor', 5961376, [83, 86, 87, 139, 249]), ('new_media', 7516624, [86, 139, 241, 139, 76, 36, 8]), ('drop_media', 7516672, [86, 139, 241, 139, 70, 12]), ('load_media_dat', 7516736, [139, 68, 36, 4, 129, 236, 16, 2, 0, 0]), ('save_media_dat', 7517680, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('injury_save', 6468304, [139, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('human_manager_save', 6289888, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('key_nation_save_all_comps', 6722288, [86, 87, 190, 108, 61, 182, 0]), ('awol_construct', 4293696, [100, 161, 0, 0, 0, 0]), ('awol_load', 4294256, [106, 255, 104, 137, 62, 149, 0]), ('awol_save', 4295040, [139, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('month_award_load_from_disk', 7761744, [129, 236, 0, 2, 0, 0]), ('month_award_save_to_disk', 7762032, [129, 236, 0, 2, 0, 0]), ('month_ratings_load_from_disk', 7764768, [129, 236, 0, 2, 0, 0]), ('nationalteams_load_dat', 7824304, [106, 255, 104, 25, 223, 149, 0]), ('nationalteams_constructor', 7775456, [106, 255, 104, 172, 220, 149, 0]), ('nationalteams_save_dat', 7825952, [129, 236, 12, 2, 0, 0]), ('nationalteams_destructor', 7776016, [106, 255, 104, 203, 220, 149, 0]), ('fine_load', 5919104, [106, 255, 104, 6, 147, 149, 0]), ('fine_save', 5919696, [139, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('dispute_load', 5675280, [106, 255, 104, 179, 128, 149, 0]), ('dispute_save', 5675744, [139, 68, 36, 4, 129, 236, 8, 2, 0, 0])]
BASE = 3890
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
