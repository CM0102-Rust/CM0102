"""Integra a 343a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('sweden_awards_construct', 8992544, [106, 255, 104, 88, 50, 150, 0]), ('sweden_awards_init', 8992688, [100, 161, 0, 0, 0, 0]), ('sweden_awards_daily_update', 8993776, [131, 236, 16, 86, 87]), ('sweden_awards_team_of_the_week', 8994144, [139, 81, 4, 139, 68, 36, 4]), ('construct_sweden_rules', 8994176, [139, 68, 36, 20, 139, 84, 36, 12]), ('nation_awards_award_player_of_year_by_stats', 7771008, [129, 236, 144, 2, 0, 0]), ('nation_awards_compare_score_ascending', 7775360, [139, 76, 36, 4, 139, 84, 36, 8]), ('nation_awards_compare_score_descending', 7775408, [139, 76, 36, 4, 139, 84, 36, 8]), ('norway_awards_on_month_change', 7947312, [161, 188, 246, 156, 0])]
BASE = 4178
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
