# O ranking da FIFA no mês (fifa_rankings.cpp): os jogos de uma seleção no período
# (FIFA_RANKINGS::count_results_in_period 0x58d710, pela permissão de trabalho TRANSFER_MANAGER::is_work_permit_granted
# 0x8c7950, que limita a 16 os jogos dos dois últimos anos e confere quantos deles o jogador jogou), a atualização do dia
# 18 (FIFA_RANKINGS::update_rankings 0x58d460, pelo dia das seleções 0x76b100: tira os resultados de mais de um ano,
# ordena os históricos pelos pontos, gira os totais dos anos e refaz pontuações e posições) e a ordem dos resultados
# por pontos (compare_result_points_desc 0x58e220, o qsort da atualização). A tabela do ranking ([0xb74318]+0x1c,
# entradas de 0x2c bytes por nação) é alocada pelo malloc e fica fora da impressão da memória: o que a carreira vê é a
# contagem dos jogos na permissão de trabalho (aceita ou não, e os sorteios dela). Chamadas originais pelo endereço:
# 0x54cf70 e 0x54cfb0 (dia e mês), 0x417c10 (cópia da data), 0x945333 (qsort), 0x58dfc0 e 0x58d610 (cpt_2 e cpt_4),
# 0x9452ca (free). Iguais no cm0102-1080.exe. Famílias do brief (player.cpp, person*.cpp, continent*.cpp, city*.cpp)
# não existem no mapa da comunidade; nation*.cpp só tem nation_awards (prêmios) e national_teams (do agente D): o
# ranking das seleções e a nação-chave ficam com os itens cpt_1..cpt_4. Comparadores: os jogos e a ordem pela devolução;
# a atualização lado a lado (objeto, tabela e históricos).
SAVE = False
ROTINAS = [('jogos_da_selecao_no_periodo', 0x58d710, [0x8b, 0x54, 0x24, 0x0c, 0x53]),
           ('atualizar_ranking_da_fifa', 0x58d460, [0x83, 0xec, 0x18, 0x53, 0x8b, 0x5c, 0x24, 0x20]),
           ('ordem_dos_resultados_por_pontos', 0x58e220, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('o ranking da FIFA no mês: os jogos de uma seleção no período, a atualização do dia 18 e a ordem dos resultados por pontos',
        'the FIFA ranking in the month: the matches of a national side in a period, the update of the 18th and the order of the results by points',
        [('jogos_da_selecao_no_periodo', 'FIFA_RANKINGS::count_results_in_period', '0x58d710', 'fifa_rankings.cpp', 'thiscall, ret 0x14',
          'conta os resultados do histórico da seleção entre dois meses (ano × 12 + mês), do fim para o começo, para a permissão de trabalho',
          "counts the results of the national side's history between two months (year × 12 + month), from the end, for the work permit"),
         ('atualizar_ranking_da_fifa', 'FIFA_RANKINGS::update_rankings', '0x58d460', 'fifa_rankings.cpp', 'thiscall, ret 4',
          'no dia 18: tira os resultados de mais de um ano, ordena os históricos, gira os totais dos anos e refaz pontuações e posições',
          'on the 18th: drops the results older than a year, sorts the histories, rolls the yearly totals and redoes scores and positions'),
         ('ordem_dos_resultados_por_pontos', 'compare_result_points_desc', '0x58e220', 'fifa_rankings.cpp', 'cdecl',
          'ordem do qsort dos históricos: os pontos de b menos os de a (do maior para o menor)',
          'qsort order of the histories: the points of b minus those of a (highest first)')])
CAMPOS = [('[0xb74318]+0x1c: o ranking da FIFA (FIFA_RANKINGS); ponteiro; lê (permissão de trabalho 0x8c7950)',
           '[0xb74318]+0x1c: the FIFA ranking (FIFA_RANKINGS); pointer; reads (work permit 0x8c7950)'),
          ('FIFA_RANKINGS +0: tabela das nações (0x2c bytes por id de nação); ponteiro; lê (as três)',
           'FIFA_RANKINGS +0: nation table (0x2c bytes per nation id); pointer; reads (all three)'),
          ('FIFA_RANKINGS +4: mês da virada dos totais; valor; lê (atualizar_ranking_da_fifa)',
           'FIFA_RANKINGS +4: month of the yearly roll; value; reads (atualizar_ranking_da_fifa)'),
          ('FIFA_RANKINGS +5: data da última atualização (8 bytes); valor; grava (atualizar_ranking_da_fifa)',
           'FIFA_RANKINGS +5: date of the last update (8 bytes); value; writes (atualizar_ranking_da_fifa)'),
          ('entrada do ranking +4/+8/+0xa: histórico (entradas de 5 bytes: pontos +0, mês +2, ano +3), capacidade e conta; lê e grava (atualizar_ranking_da_fifa), lê (jogos_da_selecao_no_periodo)',
           'ranking entry +4/+8/+0xa: history (5-byte entries: points +0, month +2, year +3), capacity and count; reads and writes (atualizar_ranking_da_fifa), reads (jogos_da_selecao_no_periodo)'),
          ('entrada do ranking +0x10/+0x12: posição e posição anterior; valor; grava (atualizar_ranking_da_fifa)',
           'ranking entry +0x10/+0x12: position and previous position; value; writes (atualizar_ranking_da_fifa)'),
          ('entrada do ranking +0x14..+0x28: totais do ano e dos cinco anteriores; valor; grava (atualizar_ranking_da_fifa)',
           'ranking entry +0x14..+0x28: totals of this year and the five before; value; writes (atualizar_ranking_da_fifa)'),
          ('[0xae2350] número de nações; valor; lê (atualizar_ranking_da_fifa)',
           '[0xae2350] number of nations; value; reads (atualizar_ranking_da_fifa)')]
INTERESSA = ['CMRTE']
