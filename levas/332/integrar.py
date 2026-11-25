"""Integra a 332a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('year_ratings_compare_mode_0', 9641872, [139, 68, 36, 4, 83]), ('year_ratings_compare_mode_1', 9642208, [139, 68, 36, 4, 83]), ('year_ratings_compare_mode_2', 9642544, [139, 68, 36, 4, 83]), ('year_ratings_compare_mode_3', 9642928, [139, 68, 36, 4, 83]), ('year_ratings_compare_mode_4', 9643264, [86, 139, 116, 36, 8]), ('year_ratings_compare_mode_5', 9643408, [139, 76, 36, 4, 83]), ('year_ratings_reset_if_year', 9640608, [86, 139, 241, 139, 76, 36, 8]), ('year_ratings_collect_best_players', 9640672, [131, 236, 12, 83, 85]), ('year_ratings_collect_candidates', 9641296, [131, 236, 8, 138, 68, 36, 28]), ('team_award_add_candidate', 9108032, [131, 236, 8, 83, 85]), ('team_award_clear_selection', 9108880, [86, 141, 65, 82, 190, 20, 0, 0, 0]), ('team_award_init_slot_positions', 9108928, [138, 129, 92, 2, 0, 0])]
BASE = 3852
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
