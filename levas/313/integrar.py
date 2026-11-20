"""Integra a 313a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('construct_kor_fa_cup', 6726752, [106, 255, 104, 137, 198, 149, 0]), ('kor_fa_cup_create_round_table', 6727456, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('kor_fa_cup_collect_entrants', 6728432, [129, 236, 148, 3, 0, 0]), ('construct_kor_super_cup', 6740000, [106, 255, 104, 185, 199, 149, 0]), ('kor_super_cup_create_round_table', 6740688, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('kor_super_cup_create_table', 6741008, [129, 236, 0, 2, 0, 0]), ('construct_kor_league', 6729328, [106, 255, 104, 217, 198, 149, 0]), ('construct_kor_league_cup', 6733568, [106, 255, 104, 41, 199, 149, 0]), ('construct_arg_prm', 4211280, [106, 255, 104, 105, 53, 149, 0]), ('construct_arg_second', 4221936, [106, 255, 104, 233, 53, 149, 0])]
BASE = 2733
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
