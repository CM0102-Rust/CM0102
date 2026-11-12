"""Integra a 288a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('get_transfer_budget', 5867520, [131, 236, 56, 139, 65, 8]), ('get_human_asking_price', 5063376, [129, 236, 0, 2, 0, 0]), ('calc_asking_price', 5051904, [131, 236, 124, 83, 85]), ('review_ai_club_bid', 9177104, [106, 255, 104, 115, 59, 150, 0]), ('assess_club_squad', 8637248, [129, 236, 164, 8, 0, 0]), ('fill_shortlist_slot', 8647584, [85, 139, 236, 131, 228, 248, 106, 255]), ('transfer_rules_get_loan_period', 9341904, [86, 87, 185, 144, 44, 174, 0]), ('transfer_rules_find_open_window', 9342544, [83, 86, 139, 241, 50, 219]), ('transfer_window_contains_date', 9342016, [100, 161, 0, 0, 0, 0]), ('staff_contract_calc_contract_value', 8808496, [106, 255, 104, 176, 42, 150, 0]), ('staff_contract_default_staff_contract', 8799568, [139, 13, 80, 16, 174, 0, 131, 236, 36]), ('staff_contract_set_to_desired_contract', 8780880, [106, 255, 104, 118, 42, 150, 0]), ('resolve_contract_offers', 5039328, [85, 139, 236, 131, 228, 248]), ('staff_contract_calc_unset_default_wage', 8791840, [139, 13, 80, 16, 174, 0]), ('staff_interest_in_job_at_club', 8669056, [85, 139, 236, 131, 228, 248])]
BASE = 2191
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
