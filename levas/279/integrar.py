"""Integra a 279a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('estadio_da_partida', 8766048, [106, 255, 104, 209, 41, 150, 0]), ('estadio_neutro', 8767488, [131, 236, 24, 83, 85]), ('publico_do_jogo', 8768416, [85, 139, 236, 131, 228, 248]), ('calendario_da_terceira_francesa', 6028848, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('calendario_da_segunda_francesa', 6022688, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('calendario_da_divisao_de_baixo_francesa', 6019360, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('calendario_da_cfa', 6001936, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('calendario_da_c1_a', 6598016, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('calendario_da_c1_b', 6614832, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('calendario_da_serie_a', 6556080, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('calendario_da_serie_b', 6576880, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('calendario_da_primeira_francesa', 6010512, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqo7_fase_seguinte_do_mineiro', 4410080, [139, 81, 44, 139, 65, 48]), ('cqo7_final_do_mineiro', 4410224, [106, 255, 104, 238, 71, 149, 0]), ('cqo7_fim_da_temporada_do_mineiro', 4411088, [86, 139, 241, 131, 126, 48, 2]), ('cqo8_fase_seguinte_do_paranaense', 4424240, [139, 81, 44, 139, 65, 48]), ('cqo8_final_do_paranaense', 4424384, [106, 255, 104, 94, 73, 149, 0]), ('cqo8_fim_da_temporada_do_paranaense', 4424928, [86, 139, 241, 131, 126, 48, 2]), ('cqo9_fase_seguinte_do_carioca', 4434032, [139, 81, 44, 139, 65, 48]), ('cqo9_final_do_carioca', 4434176, [106, 255, 104, 94, 74, 149, 0]), ('cqo9_fim_da_temporada_do_carioca', 4435232, [86, 139, 241, 131, 126, 48, 2]), ('cqo10_copa_da_italia_tabela', 6548560, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqo10_copa_c_tabela', 6541200, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqo10_supercopa_tabela', 6682912, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqo10_supercopa_c1_tabela', 6539408, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqo11_copa_da_italia_inscritos', 6552304, [129, 236, 140, 2, 0, 0]), ('cqo11_copa_c_inscritos', 6545376, [129, 236, 0, 2, 0, 0]), ('cqo11_supercopa_clubes', 6683232, [129, 236, 0, 2, 0, 0]), ('cqo12_copa_do_brasil_tabela', 4355840, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqo12_times_da_copa', 4357152, [129, 236, 0, 2, 0, 0]), ('cqo12_regras_da_copa', 4357824, [86, 139, 241, 106, 0]), ('cqo13_libertadores_tabela', 4993472, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqo13_mercosul_tabela', 4999472, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqo13_mercosul_convidados', 5003344, [129, 236, 0, 2, 0, 0]), ('calendario_da_segunda_inglesa', 5724064, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('calendario_da_terceira_inglesa', 5730528, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0])]
BASE = 1887
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
