# O ranking da FIFA no jogo (fifa_rankings.cpp): o resultado no histórico da seleção (FIFA_RANKINGS::append_result
# 0x58dea0, pelo resultado internacional FIFA_RANKINGS::record_match_result 0x58cf70, que vem do
# NationalTeams::record_international_result 0x76b600, para cada uma das duas seleções), a faixa de força da seleção
# (FIFA_RANKINGS::get_nation_strength_tier 0x58e190, duas vezes por resultado) e as pontuações do ranking
# (FIFA_RANKINGS::recalculate_scores 0x58dfc0, pela atualização do dia 18 0x58d460 da cpt_1). A 0x58dea0 tem SEH só da
# data local (destrutor vazio). A tabela do ranking é do malloc (fora da impressão da memória): a carreira vê o
# histórico pela contagem da permissão de trabalho (cpt_1). Chamadas originais pelo endereço: 0x945667 e 0x944e46
# (realloc e malloc), 0x549ef0 e 0x54cfb0 (data e mês), 0x54c8e0 (hoje menos 365 dias). Iguais no cm0102-1080.exe.
# A record_match_result 0x58cf70 (1,2 KB de ponto flutuante x87 com tabela de saltos) fica de fora. Comparadores: o
# resultado só a original, com o plano antes (realoca); a faixa pela devolução; as pontuações lado a lado na tabela.
SAVE = False
ROTINAS = [('resultado_no_historico', 0x58dea0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('faixa_da_selecao', 0x58e190, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('pontuacoes_do_ranking', 0x58dfc0, [0x83, 0xec, 0x1c, 0xa1, 0x50, 0x23, 0xae, 0x00])]
DOCS = ('o ranking da FIFA no jogo: o resultado no histórico da seleção, a faixa de força da seleção e as pontuações do ranking',
        "the FIFA ranking in the match: the result in the national side's history, the national side's strength tier and the ranking scores",
        [('resultado_no_historico', 'FIFA_RANKINGS::append_result', '0x58dea0', 'fifa_rankings.cpp', 'thiscall, ret 0xc',
          'acrescenta ao histórico da seleção os pontos, o mês e o ano do jogo, crescendo o vetor de 5 em 5 entradas',
          "appends the points, month and year of the match to the national side's history, growing the array by 5 entries"),
         ('faixa_da_selecao', 'FIFA_RANKINGS::get_nation_strength_tier', '0x58e190', 'fifa_rankings.cpp', 'thiscall, ret 4',
          'a faixa de força de 1 a 4 da seleção, pela reputação e pela posição no ranking',
          "the national side's strength tier from 1 to 4, by reputation and ranking position"),
         ('pontuacoes_do_ranking', 'FIFA_RANKINGS::recalculate_scores', '0x58dfc0', 'fifa_rankings.cpp', 'thiscall',
          'o total do ano pelos oito melhores resultados e a pontuação pelos totais do ano e dos cinco anteriores',
          "the year total from the best eight results and the score from the totals of this year and the five before")])
CAMPOS = [('entrada do ranking +4/+8/+0xa: histórico, capacidade e conta; lê e grava (resultado_no_historico), lê (pontuacoes_do_ranking)',
           'ranking entry +4/+8/+0xa: history, capacity and count; reads and writes (resultado_no_historico), reads (pontuacoes_do_ranking)'),
          ('entrada do histórico (5 bytes): pontos +0, mês +2, ano +3; valor; grava (resultado_no_historico)',
           'history entry (5 bytes): points +0, month +2, year +3; value; writes (resultado_no_historico)'),
          ('entrada do ranking +0xc: pontuação; f32; grava (pontuacoes_do_ranking)',
           'ranking entry +0xc: score; f32; writes (pontuacoes_do_ranking)'),
          ('entrada do ranking +0x10: posição; valor; lê (faixa_da_selecao)',
           'ranking entry +0x10: position; value; reads (faixa_da_selecao)'),
          ('entrada do ranking +0x14: total do ano; valor; grava (pontuacoes_do_ranking); +0x18..+0x28 os cinco anteriores; lê',
           'ranking entry +0x14: year total; value; writes (pontuacoes_do_ranking); +0x18..+0x28 the five before; reads'),
          ('nação +0x8e: reputação; valor; lê (faixa_da_selecao)',
           'nation +0x8e: reputation; value; reads (faixa_da_selecao)'),
          ('partida +0x28/+0x2a: ano e dia do ano; valor; lê (resultado_no_historico)',
           'fixture +0x28/+0x2a: year and day of year; value; reads (resultado_no_historico)'),
          ('[0xdd6ec8] data de referência do ranking; valor; lê (pontuacoes_do_ranking)',
           '[0xdd6ec8] ranking reference date; value; reads (pontuacoes_do_ranking)')]
INTERESSA = ['CMRTE']
