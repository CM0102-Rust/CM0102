# Os campeoes dos periodos e o progresso da Terceira Divisao belga (bel_third.cpp; leitor cx09, rodada 5, 24/11/2026).
# Simulacao com a Belgica ('ENG BEL HOL GER DEN', ate 2002-08-01); na cobertura do mundo a 0x422f70 roda 88 vezes.
# - 0x422f70 BelThird::update_positions_and_status thiscall() ret 0 (virtual 0x94): League::update_positions_and_status
#   0x684cd0; para o grupo A (a liga) e o B (fase 0): o primeiro periodo em aberto (+0xfe/+0x100/+0x102 + grupo; os tres
#   marcados: ret, sem olhar o grupo B: defeito), a fase por 0x422cf0 (periodo, +0x3a do grupo, 0; 0xfe: o proximo
#   grupo); com o status da 1a linha da tabela do periodo = 1, as linhas do grupo com o clube e status 0xff viram 1,
#   0x4242e0 (clube, periodo) e a virtual 0x68 (clube, +0x44, 2, 2, 7); o primeiro flag em aberto = 1. Prologo sub esp,
#   0x14; push ebx; push ebp; push esi; push edi; mov esi, ecx (9 bytes).
# - 0x4242e0 BelThird::add_period_winner_news thiscall(clube, periodo) ret 8: o NewsItem local 0x77bbe0 (0x11, 0), os
#   slots 0x785790 (0: id do clube, 1: periodo com sinal, 2: id da competicao), News::send_to_division_managers 0x7862d0
#   em [0xb7431c]. O mesmo corpo da BelSecond (cx08_17, em andamento: pode unificar). Prologo push -1; push imm32.
# - 0x4241b0 BelThird::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68): entrada
#   0x4a4850 (sem conferir); fase com sinal < 2 (a liga 0xff inclusive): 3 x codigo - 2 em byte; +0x73, +0x74, +0x75.
#   Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# Comparadores: update e noticia so a original; o progresso lado a lado (compare_group_championship_progress do cx04_5).
# Iguais no cm0102-1080.exe. Depende do cx09_8 (as fases dos periodos pelo desvio) e do cx04_5.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_third_update_positions_and_status', 0x422f70, [0x83, 0xec, 0x14, 0x53, 0x55, 0x56, 0x57, 0x8b, 0xf1]),
           ('bel_third_add_period_winner_news', 0x4242e0, [0x6a, 0xff, 0x68, 0xdb, 0x41, 0x95, 0x00]),
           ('bel_third_set_club_progress', 0x4241b0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os campeões dos períodos e o progresso da Terceira Divisão belga', 'the Belgian Third Division period winners and club progress',
        [('bel_third_update_positions_and_status', 'BelThird::update_positions_and_status', '0x422f70', 'bel_third.cpp', 'thiscall, ret 0',
          'depois da atualização comum da tabela, em cada grupo, o líder decidido do próximo período em aberto é campeão do período: marcado na tabela do grupo, com a notícia e o progresso; o período conta como decidido. Com os três períodos do grupo A decididos, o grupo B não é mais olhado (defeito do original)',
          'after the usual table update, in each group the decided leader of the next open period is the period winner: marked in the group table, with the news and the progress; the period counts as decided. Once group A has its three winners group B is no longer looked at (a defect of the original)'),
         ('bel_third_add_period_winner_news', 'BelThird::add_period_winner_news', '0x4242e0', 'bel_third.cpp', 'thiscall, ret 8',
          'a notícia (tipo 0x11) do campeão de um período, com o clube, o período e a competição, aos técnicos da divisão',
          'the news (type 0x11) of a period winner, with the club, the period and the competition, to the managers of the division'),
         ('bel_third_set_club_progress', 'BelThird::set_club_progress', '0x4241b0', 'bel_third.cpp', 'thiscall, ret 0x14',
          'grava os três códigos de progresso do clube nas estatísticas; na liga e nos grupos cada código vira 3 x código - 2',
          "stores the club's three progress codes in the statistics; in the league and the groups each code becomes 3 x code - 2")])
CAMPOS = [('terceira divisão belga +0xfe..+0x103: o período (1 a 3) do grupo (A, B) já tem campeão', 'Belgian Third Division +0xfe..+0x103: the period (1 to 3) of the group (A, B) has its winner')]
INTERESSA = []
