"""Integra a 259a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('satisfacao_com_o_contrato', 8807968, [106, 255, 104, 144, 42, 150, 0]), ('noticia_quer_contrato_novo', 5087104, [100, 161, 0, 0, 0, 0]), ('noticia_pedido_de_transferencia_retirado', 5084256, [100, 161, 0, 0, 0, 0]), ('confirmar_oferta_pelo_clube', 9188112, [106, 255, 104, 16, 60, 150, 0]), ('tentar_concluir_oferta', 9188576, [131, 236, 8, 83, 85]), ('acertar_clausulas_anteriores', 9212384, [83, 85, 86, 87, 139, 124, 36, 20]), ('tentar_emprestimo_pelo_jogador', 8660800, [100, 161, 0, 0, 0, 0]), ('noticia_contrato_acabando', 5072784, [100, 161, 0, 0, 0, 0]), ('noticia_saida_pela_lei_bosman', 5074288, [100, 161, 0, 0, 0, 0]), ('acrescentar_oferta_de_contrato', 5031056, [106, 255, 104, 41, 109, 149, 0]), ('tirar_oferta_de_contrato_da_vaga', 5032752, [83, 85, 86, 87, 139, 249]), ('menor_salario_pedido', 5039184, [106, 255, 104, 136, 109, 149, 0]), ('guardar_notas_da_partida', 7765728, [139, 68, 36, 8, 83]), ('nota_do_jogador_no_mes', 7765920, [129, 236, 0, 2, 0, 0]), ('melhor_do_mes', 7766320, [131, 236, 12, 83, 85]), ('premiar_o_melhor_do_mes', 7762256, [86, 87, 139, 249, 139, 71, 9]), ('gravar_vencedor_do_mes', 7762368, [81, 83, 139, 92, 36, 20]), ('reputacao_do_premiado', 4271872, [83, 85, 139, 76, 36, 28]), ('fisico_do_dia', 6473344, [106, 255, 104, 248, 176, 149, 0]), ('faixa_da_confianca_na_liga', 6884304, [139, 68, 36, 4, 139, 9]), ('gravar_notas_da_diretoria', 6944704, [139, 68, 36, 4, 86]), ('dia_das_lesoes', 6469552, [131, 236, 28, 83, 139, 217]), ('noticias_de_volta_aos_treinos', 6479888, [106, 255, 104, 104, 177, 149, 0]), ('um_cartao_da_suspensao', 5624672, [86, 139, 116, 36, 8]), ('fila_de_regens', 8021456, [129, 236, 16, 2, 0, 0]), ('veiculo_pelo_clube', 7700272, [139, 68, 36, 4, 102, 129, 184, 128, 0, 0, 0, 106, 24]), ('veiculo_pela_pessoa', 7700384, [139, 68, 36, 4, 139, 72, 97]), ('somar_resultado_na_liga', 6839600, [129, 236, 16, 2, 0, 0]), ('copiar_tabela_da_liga', 6846224, [129, 236, 4, 2, 0, 0]), ('posicoes_previstas_da_liga', 6858832, [129, 236, 0, 2, 0, 0]), ('resultado_entre_selecoes', 7779840, [131, 236, 96, 86, 87]), ('nome_do_elenco_da_selecao', 7820816, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('rodada_com_jogos_conhecidos', 4772880, [139, 84, 36, 4, 139, 193]), ('registrar_convocados_por_clube', 7827904, [106, 255, 104, 107, 223, 149, 0]), ('importancia_do_jogo_na_competicao', 4776336, [139, 84, 36, 4, 86]), ('fase_da_competicao', 4772352, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0])]
BASE = 1272
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
