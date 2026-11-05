"""Integra a 274a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('cqj11_libertadores_regras', 4995216, [106, 255, 104, 120, 106, 149, 0]), ('cqj11_merconorte_regras', 4988624, [106, 255, 104, 200, 105, 149, 0]), ('cqj11_oceania_clubes_regras', 7961008, [106, 255, 104, 232, 234, 149, 0]), ('cqj12_eliminatoria_europa_regras', 9540064, [106, 255, 104, 152, 89, 150, 0]), ('cqj12_eliminatoria_sul_americana_regras', 9572416, [100, 161, 0, 0, 0, 0]), ('cqj12_eliminatoria_asia_regras', 9514672, [106, 255, 104, 72, 87, 150, 0]), ('cqk10_copa_america_regras', 6165200, [106, 255, 104, 104, 168, 149, 0]), ('cqk10_copa_africana_regras', 4201136, [106, 255, 104, 216, 52, 149, 0]), ('cqk10_eliminatoria_oceania_regras', 9563376, [106, 255, 104, 248, 90, 150, 0]), ('cqk11_libertadores_classificados', 4996928, [129, 236, 0, 2, 0, 0]), ('cqk11_libertadores_registros', 4997248, [138, 68, 36, 12, 86]), ('cqk11_libertadores_fase_seguinte', 4995712, [139, 81, 44, 139, 65, 48]), ('cqk12_africana_fase_seguinte', 4202000, [139, 81, 44, 139, 65, 48]), ('cqk12_africana_grupo_a', 4200896, [129, 236, 4, 2, 0, 0]), ('cqk12_africana_historico', 4201360, [139, 65, 12, 139, 72, 12]), ('cqi7_regras_da_serie_a', 4367776, [83, 86, 139, 241, 187, 1, 0, 0, 0]), ('cqi7_regras_da_serie_b', 4377456, [83, 86, 139, 241, 187, 1, 0, 0, 0]), ('cqi7_regras_da_serie_c', 4382288, [86, 139, 241, 184, 2, 0, 0, 0]), ('cqi8_regras_do_centro_oeste', 4394992, [83, 86, 139, 241, 51, 192]), ('cqi8_regras_do_gaucho', 4399824, [83, 86, 139, 241, 51, 192]), ('cqi8_regras_do_catarinense', 4439152, [83, 86, 139, 241, 184, 2, 0, 0, 0]), ('cqi9_regras_do_paranaense', 4423616, [86, 139, 241, 184, 2, 0, 0, 0]), ('cqi9_regras_do_pernambucano', 4428848, [86, 139, 241, 51, 192]), ('cqi9_regras_do_carioca', 4433408, [86, 139, 241, 51, 192]), ('cqj7_regras_do_goiano', 4404800, [83, 86, 139, 241, 51, 192]), ('cqj7_regras_da_j1', 6698528, [86, 139, 241, 185, 2, 0, 0, 0]), ('cqj7_regras_da_j2', 6705472, [86, 139, 241, 178, 3]), ('cqj8_regras_da_nsl', 4267232, [86, 139, 241, 51, 192]), ('cqj8_regras_da_mls', 9471584, [178, 2, 184, 1, 0, 0, 0]), ('cqj8_historico_do_estadual', 4401520, [139, 65, 12, 139, 8]), ('cqj8_historico_do_primeiro_estagio', 4230832, [139, 65, 12, 139, 8]), ('cqj9_destino_na_serie_a', 4369344, [138, 68, 36, 12, 86]), ('cqj9_destino_na_serie_c', 4385536, [81, 138, 68, 36, 16]), ('cqj9_destino_no_baiano', 4391440, [138, 68, 36, 12, 83]), ('cqk7_destino_no_centro_oeste', 4396256, [138, 68, 36, 12, 83]), ('cqk7_destino_no_mineiro', 4410768, [138, 68, 36, 12, 83]), ('cqk7_destino_na_nsl', 4266368, [138, 68, 36, 12, 60, 255]), ('cqk8_progresso_no_mineiro', 4411600, [139, 68, 36, 4, 139, 73, 8]), ('cqk8_progresso_na_mls', 9477584, [139, 68, 36, 4, 139, 73, 8]), ('cqk8_progresso_na_serie_c', 4382976, [139, 68, 36, 4, 139, 73, 8])]
BASE = 1697
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
