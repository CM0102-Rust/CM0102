"""Integra a 278a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('cqn9_mata_mata_do_gaucho', 4400560, [106, 255, 104, 14, 71, 149, 0]), ('cqn9_mata_mata_do_centro_oeste', 4395728, [106, 255, 104, 158, 70, 149, 0]), ('cqn9_mata_mata_do_goiano', 4405552, [106, 255, 104, 126, 71, 149, 0]), ('desempate_da_serie_a', 6563968, [131, 236, 28, 139, 68, 36, 32]), ('desempate_da_serie_b', 6582352, [131, 236, 24, 176, 2]), ('ordem_dos_empatados_na_italia', 6582688, [83, 86, 139, 241, 87]), ('resultado_do_desempate_da_serie_a', 6561216, [138, 68, 36, 12, 131, 236, 60]), ('fase_seguinte_da_c1a', 6601184, [139, 81, 44, 139, 65, 48]), ('fases_do_fim_da_c1a', 6601328, [106, 255, 104, 236, 186, 149, 0]), ('fase_seguinte_da_c1b', 6618016, [139, 81, 44, 139, 65, 48]), ('fases_do_fim_da_c1b', 6618160, [106, 255, 104, 140, 187, 149, 0]), ('fase_seguinte_da_third', 5735216, [139, 81, 44, 139, 65, 48]), ('construir_premier', 5714816, [106, 255, 104, 201, 131, 149, 0]), ('construir_first', 5707392, [106, 255, 104, 89, 131, 149, 0]), ('construir_second', 5723296, [106, 255, 104, 25, 132, 149, 0]), ('construir_third', 5729744, [106, 255, 104, 137, 132, 149, 0]), ('construir_serie_a', 6555344, [106, 255, 104, 89, 185, 149, 0]), ('construir_serie_b', 6576144, [106, 255, 104, 9, 186, 149, 0]), ('construir_c1a', 6597232, [106, 255, 104, 153, 186, 149, 0]), ('construir_c1b', 6614048, [106, 255, 104, 57, 187, 149, 0]), ('construir_franca_2', 6021936, [106, 255, 104, 137, 154, 149, 0]), ('construir_franca_3', 6028096, [106, 255, 104, 41, 155, 149, 0]), ('construir_segunda', 8731040, [106, 255, 104, 89, 39, 150, 0]), ('construir_primera', 8708400, [106, 255, 104, 167, 38, 150, 0]), ('construir_divisao_de_baixo_espanhola', 8720416, [106, 255, 104, 11, 39, 150, 0]), ('tabela_da_segunda', 8739648, [129, 236, 8, 2, 0, 0]), ('tabela_da_divisao_de_baixo_espanhola', 8729552, [129, 236, 156, 3, 0, 0]), ('regras_da_cfa', 6004896, [86, 139, 241, 184, 2, 0, 0, 0]), ('construir_cfa', 6001088, [106, 255, 104, 171, 152, 149, 0]), ('construir_divisao_de_baixo_francesa', 6018560, [106, 255, 104, 59, 154, 149, 0]), ('destruir_cfa', 6001760, [106, 255, 104, 200, 152, 149, 0]), ('destruir_divisao_de_baixo_francesa', 6019184, [106, 255, 104, 88, 154, 149, 0]), ('calendario_fixo_da_serie_a', 6564464, [106, 255, 104, 222, 185, 149, 0]), ('calendario_fixo_da_serie_b', 6582832, [106, 255, 104, 110, 186, 149, 0]), ('calendario_fixo_da_c1a', 6602384, [106, 255, 104, 14, 187, 149, 0]), ('calendario_fixo_da_c1b', 6619392, [106, 255, 104, 174, 187, 149, 0]), ('toco_um_com_dois', 6010176, [184, 1, 0, 0, 0]), ('toco_um_com_um', 6010192, [184, 1, 0, 0, 0])]
BASE = 1849
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
