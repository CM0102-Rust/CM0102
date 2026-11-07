"""Integra a 284a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('cqt12_asia_fase_seguinte', 4243568, [139, 81, 44, 139, 65, 48]), ('cqt12_asia_grupos', 4243728, [106, 255, 104, 204, 55, 149, 0]), ('cqt12_asia_semifinais', 4244432, [100, 161, 0, 0, 0, 0]), ('cqs13_africana_dia', 4203936, [106, 255, 104, 70, 53, 149, 0]), ('cqs13_africana_sorteio', 4203200, [129, 236, 24, 2, 0, 0]), ('cqs13_africana_grupo', 4202144, [100, 161, 0, 0, 0, 0]), ('cqt13_africana_quartas', 4202640, [100, 161, 0, 0, 0, 0]), ('cqt13_recopa_asiatica_fase_seguinte', 4251552, [139, 81, 44, 139, 65, 48]), ('cqt13_recopa_asiatica_finais', 4251696, [100, 161, 0, 0, 0, 0]), ('cqr7_copa_da_italia_grupo_um', 6550096, [129, 236, 4, 2, 0, 0]), ('cqr7_copa_da_serie_c_grupo_um', 6542816, [129, 236, 4, 2, 0, 0]), ('cqr7_copa_da_italia_fase_seguinte', 6550960, [139, 81, 44, 139, 65, 48]), ('cqr7_copa_da_serie_c_fase_seguinte', 6543696, [139, 81, 44, 139, 65, 48]), ('cqr8_copa_da_italia_grupo', 6551104, [106, 255, 104, 14, 185, 149, 0]), ('cqr8_copa_da_serie_c_grupo', 6543840, [106, 255, 104, 110, 184, 149, 0]), ('cqr9_copa_da_italia_mata_mata', 6551664, [106, 255, 104, 46, 185, 149, 0]), ('cqr9_copa_da_serie_c_mata_mata', 6544400, [106, 255, 104, 156, 184, 149, 0]), ('cqs8_eurocopa_regras', 5761264, [106, 255, 104, 168, 136, 149, 0]), ('cqs8_eurocopa_tamanho_do_grupo', 5757392, [15, 190, 129, 246, 0, 0, 0]), ('cqs8_eurocopa_situacao', 5770064, [139, 68, 36, 4, 139, 73, 8]), ('cqs8_eurocopa_grupo_um', 5760992, [129, 236, 4, 2, 0, 0]), ('cqs9_uefa_inscritos', 9448336, [129, 236, 0, 2, 0, 0]), ('cqt5_copa_do_rei_dia', 8705856, [129, 236, 0, 2, 0, 0]), ('cqt5_historico_liga_dos_campeoes', 5786320, [139, 65, 12, 139, 72, 48]), ('cqt5_historico_copa_da_grecia', 6198240, [139, 65, 12, 139, 72, 40]), ('cqt5_historico_copa_da_liga_irlandesa', 6518608, [139, 65, 12, 139, 72, 28]), ('cqt5_historico_copa_da_serie_c', 6543216, [139, 65, 12, 139, 72, 60])]
BASE = 2065
m = re.search(r'\];\nconst N: usize = (\d+);', s)
assert m and int(m.group(1)) == BASE, ('o motor.rs mudou desde a materialização', m and m.group(1), BASE)
pro = ''.join('    Prologo { endereco: %#x, bytes: &[%s] },\n' % (e, ', '.join('0x%02x' % b for b in bs)) for _, e, bs in ROTINAS)
s = s[:m.start()] + pro + '];\nconst N: usize = %d;' % (BASE + len(ROTINAS)) + s[m.end():]
fins = [x.start() for x in re.finditer(r' as \*const \(\) as usize,\n    \];', s)]
assert len(fins) == 2, fins
for k, prefixo in ((1, 'comparar_'), (0, '')):
    i = fins[k] + len(' as *const () as usize,\n')
    s = s[:i] + ''.join('        %s%s as *const () as usize,\n' % (prefixo, a) for a, _, _ in ROTINAS) + s[i:]
anc = '\n    ];\n    for i in 0..N {'
assert s.count(anc) == 1
s = s.replace(anc, ' ' + ' '.join('"motor_%s",' % a for a, _, _ in ROTINAS) + anc)
anc = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
assert s.count(anc) == 1
s = s.replace(anc, anc + '\n' + open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read().strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
