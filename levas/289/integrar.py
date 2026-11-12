"""Integra a 289a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('finland_rules_get_loan_period', 5937792, [86, 87, 185, 144, 44, 174, 0]), ('finland_rules_can_loan_player', 5937904, [131, 236, 100, 86, 106, 0]), ('finland_rules_can_end_loan', 5938080, [106, 255, 104, 104, 149, 149, 0]), ('northern_ireland_rules_get_loan_period', 7944304, [83, 86, 87, 185, 144, 44, 174, 0]), ('northern_ireland_rules_can_loan_player', 7944448, [139, 68, 36, 8, 139, 13, 80, 16, 174, 0]), ('norway_rules_can_loan_player', 7948128, [131, 236, 100, 86, 106, 0]), ('germany_rules_can_end_loan', 6160912, [139, 68, 36, 20, 131, 236, 100]), ('ireland_rules_can_end_loan', 6538544, [139, 68, 36, 16, 131, 236, 100]), ('ireland_rules_record_loan', 6538464, [139, 68, 36, 4, 139, 21, 180, 35, 174, 0])]
BASE = 2206
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
