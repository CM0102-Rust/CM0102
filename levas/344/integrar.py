"""Integra a 344a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('turkey_awards_construct', 9444464, [106, 255, 104, 40, 81, 150, 0]), ('turkey_awards_init', 9444608, [100, 161, 0, 0, 0, 0]), ('turkey_awards_daily_update', 9445328, [131, 236, 16, 86, 87]), ('turkey_awards_team_of_the_week', 9445504, [139, 81, 4, 139, 68, 36, 4]), ('poland_awards_construct', 8180016, [106, 255, 104, 232, 248, 149, 0]), ('poland_awards_init', 8180160, [106, 255, 104, 140, 249, 149, 0]), ('poland_awards_daily_update', 8181296, [131, 236, 16, 86, 87]), ('poland_awards_on_month_change', 8181648, [161, 88, 248, 156, 0]), ('russia_awards_construct', 8307216, [106, 255, 104, 248, 9, 150, 0]), ('russia_awards_init', 8307360, [106, 255, 104, 114, 10, 150, 0]), ('russia_awards_daily_update', 8308224, [131, 236, 16, 86, 87])]
BASE = 4187
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
