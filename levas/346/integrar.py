"""Integra a 346a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('wales_awards_construct', 9497296, [106, 255, 104, 201, 85, 150, 0]), ('wales_awards_init', 9497616, [106, 255, 104, 52, 86, 150, 0]), ('wales_awards_daily_update', 9498384, [131, 236, 16, 86, 87]), ('wales_awards_on_month_change', 9498656, [161, 76, 249, 156, 0]), ('wales_awards_team_of_the_week', 9498720, [139, 81, 4, 139, 68, 36, 4]), ('scotland_awards_construct', 8343920, [106, 255, 104, 57, 13, 150, 0]), ('scotland_awards_init', 8344240, [106, 255, 104, 202, 14, 150, 0]), ('scotland_awards_daily_update', 8347072, [131, 236, 16, 86, 87])]
BASE = 4255
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
