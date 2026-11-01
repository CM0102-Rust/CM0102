"""Integra a 262a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('mata_mata_da_first', 5714048, [100, 161, 0, 0, 0, 0]), ('mata_mata_da_second', 5728880, [100, 161, 0, 0, 0, 0]), ('mata_mata_da_third', 5735360, [100, 161, 0, 0, 0, 0]), ('registros_pela_situacao_inglesa', 5729360, [138, 68, 36, 12, 83]), ('situacao_na_first', 5714528, [139, 68, 36, 4, 139, 73, 8]), ('situacao_na_third', 5736032, [139, 68, 36, 4, 139, 73, 8]), ('posicoes_e_situacao_na_liga', 6835408, [129, 236, 20, 2, 0, 0]), ('fase_seguinte_da_first', 5713904, [139, 81, 44, 139, 65, 48]), ('fase_seguinte_da_second', 5728736, [139, 81, 44, 139, 65, 48]), ('registros_da_liga_dos_campeoes', 5786336, [138, 68, 36, 12, 83]), ('sorteio_das_quartas', 5792560, [100, 161, 0, 0, 0, 0]), ('clubes_da_liga_dos_campeoes', 5793376, [138, 68, 36, 8, 129, 236, 4, 2, 0, 0]), ('registrar_resultado_da_tatica', 9003664, [85, 139, 236, 131, 228, 248]), ('capitao_guardado', 9008800, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('escolher_capitao', 9009280, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('selecao_que_tem_o_jogador', 7821264, [106, 255, 104, 240, 222, 149, 0]), ('jogador_serve_a_selecao', 7788976, [139, 68, 36, 8, 86]), ('juntar_competicoes_das_convocacoes', 7820672, [83, 85, 139, 233, 50, 219]), ('registro_da_partida_pelo_clube', 8275792, [100, 161, 0, 0, 0, 0]), ('referencia_ao_nome', 8276256, [139, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('codigo_do_resultado', 8279760, [139, 76, 36, 4, 129, 236, 0, 2, 0, 0]), ('mover_jogos_vizinhos_do_clube', 5958096, [106, 255, 104, 70, 152, 149, 0]), ('remarcar_jogos_perto_da_data', 5946128, [106, 255, 104, 64, 151, 149, 0]), ('noticia_do_jogo_remarcado', 5942384, [100, 161, 0, 0, 0, 0]), ('incidente_de_forma', 4304336, [106, 255, 104, 54, 63, 149, 0]), ('noticia_reacao_a_multa', 5928112, [100, 161, 0, 0, 0, 0]), ('noticia_retorno_do_treinador', 5930784, [100, 161, 0, 0, 0, 0]), ('sortear_comentarista', 8235920, [184, 148, 16, 0, 0]), ('nome_do_veiculo', 8236320, [83, 85, 86, 139, 241]), ('noticia_queixa_a_federacao', 7704032, [100, 161, 0, 0, 0, 0]), ('folha_salarial_do_clube', 5866352, [131, 236, 12, 83, 85, 86]), ('refazer_todas_as_folhas', 5885776, [161, 92, 35, 174, 0]), ('dias_ate_a_temporada', 5878976, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('receber_venda_de_jogador', 5871616, [139, 68, 36, 4, 139, 84, 36, 8]), ('pagar_compra_de_jogador', 5871760, [139, 68, 36, 4, 139, 84, 36, 8]), ('pagar_luvas', 5871840, [139, 68, 36, 4, 139, 145, 60, 1, 0, 0]), ('receber_premio_em_dinheiro', 5872032, [139, 68, 36, 4, 139, 145, 72, 1, 0, 0]), ('receber_direitos_de_tv', 5867456, [139, 68, 36, 4, 139, 145, 88, 1, 0, 0]), ('pagar_salarios', 5871904, [139, 68, 36, 4, 139, 145, 48, 1, 0, 0])]
BASE = 1380
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
