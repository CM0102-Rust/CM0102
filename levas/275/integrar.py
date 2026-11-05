"""Integra a 275a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('cqk9_destino_na_mls', 9470464, [138, 68, 36, 12, 86]), ('cqk9_resultado_na_mls', 9476560, [85, 86, 87, 139, 124, 36, 16]), ('cqk9_conferencia_das_estrelas', 9478144, [139, 76, 36, 4, 129, 236, 0, 2, 0, 0]), ('regras_da_premier', 5720944, [86, 139, 241, 185, 2, 0, 0, 0]), ('regras_da_first', 5713296, [86, 139, 241, 184, 2, 0, 0, 0]), ('regras_da_second', 5728128, [86, 139, 241, 184, 2, 0, 0, 0]), ('regras_da_third', 5734592, [83, 86, 139, 241, 176, 3]), ('tabela_da_premier', 5720608, [129, 236, 0, 2, 0, 0]), ('tabela_da_second', 5727792, [129, 236, 0, 2, 0, 0]), ('tabela_da_third', 5734256, [129, 236, 0, 2, 0, 0]), ('tabela_da_first', 5712880, [129, 236, 0, 2, 0, 0]), ('regras_da_serie_a', 6559296, [83, 86, 139, 241, 187, 1, 0, 0, 0]), ('regras_da_serie_b', 6580384, [83, 86, 139, 241, 187, 1, 0, 0, 0]), ('regras_da_c1a', 6600560, [83, 86, 139, 241, 178, 3]), ('regras_da_c1b', 6617376, [83, 86, 139, 241, 184, 2, 0, 0, 0]), ('tabela_da_serie_a', 6558992, [129, 236, 0, 2, 0, 0]), ('tabela_da_serie_b', 6580096, [129, 236, 0, 2, 0, 0]), ('tabela_da_c1a', 6600304, [129, 236, 0, 2, 0, 0]), ('tabela_da_c1b', 6617120, [129, 236, 0, 2, 0, 0]), ('regras_da_franca_1', 6013968, [86, 139, 241, 185, 2, 0, 0, 0]), ('regras_da_franca_2', 6025376, [86, 139, 241, 184, 2, 0, 0, 0]), ('regras_da_franca_3', 6031104, [86, 139, 241, 185, 2, 0, 0, 0]), ('tabela_da_franca_1', 6013616, [129, 236, 0, 2, 0, 0]), ('tabela_da_franca_2', 6024976, [129, 236, 0, 2, 0, 0]), ('tabela_da_franca_3', 6030800, [129, 236, 0, 2, 0, 0]), ('regras_da_primera', 8715760, [83, 86, 139, 241, 87]), ('regras_da_segunda', 8740032, [86, 139, 241, 185, 2, 0, 0, 0]), ('regras_da_divisao_de_baixo', 8730512, [86, 139, 241, 184, 2, 0, 0, 0]), ('tabela_da_primera', 8715376, [129, 236, 0, 2, 0, 0]), ('registros_pela_situacao_na_c2a', 6635904, [138, 68, 36, 12, 83]), ('situacao_na_primera', 8718464, [139, 68, 36, 4, 139, 73, 8]), ('tabela_do_grupo_da_cfa', 6004640, [129, 236, 0, 2, 0, 0]), ('tabela_da_divisao_de_baixo_francesa', 6021680, [129, 236, 0, 2, 0, 0]), ('situacao_na_c1a', 6602112, [139, 68, 36, 4, 139, 73, 8]), ('situacao_na_c1b', 6618944, [139, 68, 36, 4, 139, 73, 8])]
BASE = 1737
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
