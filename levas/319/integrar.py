"""Integra a 319a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('create_world_cup_qualifying_comps', 8586784, [100, 161, 0, 0, 0, 0]), ('create_other_int_comps', 8587968, [100, 161, 0, 0, 0, 0]), ('create_world_club_comps', 8589280, [100, 161, 0, 0, 0, 0]), ('create_european_club_comps', 8590048, [100, 161, 0, 0, 0, 0]), ('create_north_american_club_comps', 8590928, [100, 161, 0, 0, 0, 0]), ('create_south_american_club_comps', 8591600, [100, 161, 0, 0, 0, 0]), ('create_asian_club_comps', 8592288, [100, 161, 0, 0, 0, 0]), ('create_oceania_club_comps', 8593072, [100, 161, 0, 0, 0, 0]), ('create_argentine_comps', 8593664, [100, 161, 0, 0, 0, 0]), ('create_australian_comps', 8594304, [100, 161, 0, 0, 0, 0]), ('create_belgian_comps', 8594896, [100, 161, 0, 0, 0, 0]), ('create_brazilian_comps', 8595840, [100, 161, 0, 0, 0, 0]), ('create_croatian_comps', 8597680, [100, 161, 0, 0, 0, 0]), ('create_danish_comps', 8598528, [100, 161, 0, 0, 0, 0]), ('create_english_comps', 8599376, [100, 161, 0, 0, 0, 0]), ('create_finnish_comps', 8600816, [100, 161, 0, 0, 0, 0]), ('create_french_comps', 8601584, [100, 161, 0, 0, 0, 0]), ('create_german_comps', 8602608, [100, 161, 0, 0, 0, 0]), ('create_greek_comps', 8603584, [100, 161, 0, 0, 0, 0]), ('create_dutch_comps', 8604432, [100, 161, 0, 0, 0, 0]), ('create_irish_comps', 8605280, [100, 161, 0, 0, 0, 0]), ('create_italian_comps', 8606496, [100, 161, 0, 0, 0, 0]), ('create_japanese_comps', 8608000, [100, 161, 0, 0, 0, 0]), ('create_northern_irish_comps', 8608912, [100, 161, 0, 0, 0, 0]), ('create_norwegian_comps', 8609872, [100, 161, 0, 0, 0, 0]), ('create_polish_comps', 8610640, [100, 161, 0, 0, 0, 0]), ('create_portuguese_comps', 8611552, [100, 161, 0, 0, 0, 0]), ('create_russian_comps', 8612544, [100, 161, 0, 0, 0, 0]), ('create_scottish_comps', 8613312, [100, 161, 0, 0, 0, 0]), ('create_south_korean_comps', 8614400, [100, 161, 0, 0, 0, 0]), ('create_spanish_comps', 8615264, [100, 161, 0, 0, 0, 0]), ('create_swedish_comps', 8616432, [100, 161, 0, 0, 0, 0]), ('create_turkish_comps', 8617328, [100, 161, 0, 0, 0, 0]), ('create_united_states_comps', 8618176, [100, 161, 0, 0, 0, 0]), ('create_welsh_comps', 8618928, [100, 161, 0, 0, 0, 0])]
BASE = 3647
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
