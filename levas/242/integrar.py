"""Integra a 242a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('encerrar_convocacoes_da_competicao', 7819056, [131, 236, 8, 86, 87]), ('encerrar_convocacoes_da_nacao', 7819152, [131, 236, 8, 86, 87]), ('podar_registros_de_convocacao', 7828656, [161, 80, 35, 174, 0]), ('por_no_elenco_da_selecao', 7823952, [131, 236, 8, 83, 138, 92, 36, 20]), ('tirar_do_elenco_da_selecao', 7824112, [138, 68, 36, 8, 86]), ('dispensar_acima_da_idade_do_sub21', 7788672, [81, 86, 139, 116, 36, 12]), ('limpar_competicao_atual', 7819264, [139, 84, 36, 4, 86]), ('liberar_competicoes_do_continente', 7819376, [81, 161, 80, 35, 174, 0]), ('convocar_substituto', 7823248, [131, 236, 24, 139, 68, 36, 40]), ('montar_reservas_das_selecoes', 7777264, [131, 236, 40, 161, 80, 35, 174, 0]), ('ordem_das_reservas', 7835344, [139, 68, 36, 4, 139, 8]), ('ordem_pelo_clube', 7835376, [139, 68, 36, 4, 86]), ('adversario_do_amistoso', 6048944, [131, 236, 28, 137, 76, 36, 0]), ('adversario_da_excursao', 6049856, [131, 236, 32, 139, 68, 36, 36]), ('aplicar_resultado_na_tabela', 6840160, [131, 236, 12, 139, 209]), ('pontos_do_resultado', 6859472, [139, 68, 36, 8, 83]), ('ordem_pela_posicao_do_confronto', 6829072, [161, 252, 121, 182, 0]), ('ordem_das_datas_da_liga', 6839536, [139, 84, 36, 4, 86]), ('linha_do_clube', 6859632, [83, 86, 139, 116, 36, 16]), ('liga_tem_o_clube', 6859728, [83, 86, 139, 177, 177, 0, 0, 0])]
BASE = 788
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
