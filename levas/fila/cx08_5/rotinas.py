# Os promedios do rebaixamento e o progresso dos clubes na Primeira Divisao argentina (arg_prm.cpp; leitor cx08, rodada 5,
# 24/11/2026). Simulacao com a Argentina (as ligas do cx01_8, ate 2002-08-01): o init roda na criacao do jogo (o
# construtor do mwf_23), o update no fim do Clausura (record_season_honours, push 0), na tela dos promedios e na virada
# (push 1), o progresso a cada atualizacao de status.
# - 0x405ef0 ArgPrm::init_relegation_averages thiscall() ret 0: +0x34 = +0x3e; calloc 0x945501 (cdecl; contagem, 0x13)
#   em +0x18 (caixa 0x36e); as 20 linhas fixas (o id no global de cada clube, +8 o ano, +0xa/+0xc e +0xd/+0xf as duas
#   temporadas anteriores, 38 jogos); depois cada linha abaixo da contagem: o clube do id (sem teste do -1) tem de estar
#   na Primeira ([[+0x57]] = [0x9cf654]); senao free e um calloc novo (caixa 0x3e7) com id -1 e o ano em cada linha.
#   Prologo sub esp, 0x208. As 20 linhas extraidas da desmontagem por script e conferidas uma a uma.
# - 0x406570 ArgPrm::update_relegation_averages thiscall(rolar) ret 4: os pontos (+0x19) e jogos (+0x11) da tabela da
#   fase 1 na terceira coluna (+0x10/+0x12), linha nova no primeiro id -1; cada linha: com rolar as colunas andam e o
#   ano + 1; o promedio +4 (float: fild/fidiv) = pontos / jogos das tres; qsort 0x945333 (cdecl) com 0x48f410; com rolar,
#   os clubes da tabela com status 3 ou fora da divisao (+0x57 != +4) tem a linha zerada, id -1 e o ano guardado.
#   Prologo sub esp, 8; push ebx; push ebp; push esi (6 bytes).
# - 0x406b80 ArgPrm::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: Comp_Stats::find_club_entry
#   0x4a4850; nas fases 0xff, 0 e 1 os codigos >= 0x12 viram 0x12; grava +0x74, +0x73, +0x75. Prologo mov eax, [esp+4];
#   mov ecx, [ecx+8] (7 bytes).
# Defeito do original: no init, um clube ausente do banco (id -1) e procurado 0x245 bytes antes da tabela dos clubes.
# Os auxiliares reset_relegation_averages, find_average_row, take_season_points, average_seasons, drop_departed_clubs e
# o comparador compare_relegation_averages_update ficam aqui (servem aos promedios da Segunda, que ficaram para a proxima
# rodada: ver cx08_10/FORA.md).
# Comparadores: o init so a original (as linhas semeadas ou vazias); o update lado a lado nas linhas dos promedios (com
# rolar so a original); o progresso lado a lado nos 3 bytes (compare_group_championship_progress do cx04_5).
# Depende do cx08_4 (os nomes dos promedios) e do cx04_5 (compare_group_championship_progress): integrar depois.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_prm_init_relegation_averages', 0x405ef0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('arg_prm_update_relegation_averages', 0x406570, [0x83, 0xec, 0x08, 0x53, 0x55, 0x56]),
           ('arg_prm_set_club_progress', 0x406b80, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os promédios do rebaixamento e o progresso no campeonato argentino', 'the Argentine Premier Division relegation averages and club progress',
        [('arg_prm_init_relegation_averages', 'ArgPrm::init_relegation_averages', '0x405ef0', 'arg_prm.cpp', 'thiscall, ret 0',
          'os promédios do rebaixamento argentino partem das temporadas reais de 1999/2000 e 2000/01 dos vinte clubes (pontos e jogos de cada uma); se algum deles não estiver na Primeira no banco, a tabela começa vazia',
          'the Argentine relegation averages start from the real 1999/2000 and 2000/01 seasons of the twenty clubs (the points and games of each); if any of them is not in the Premier Division in the database, the table starts empty'),
         ('arg_prm_update_relegation_averages', 'ArgPrm::update_relegation_averages', '0x406570', 'arg_prm.cpp', 'thiscall, ret 4',
          'os promédios tomam os pontos e jogos da temporada de cada clube na fase geral; numa temporada nova as colunas andam (a mais velha sai); o promédio é a soma dos pontos das três temporadas sobre a dos jogos; as linhas são ordenadas pelo promédio; numa temporada nova os rebaixados e quem saiu da divisão deixam a tabela',
          "the averages take each club's season points and games from the overall stage; on a new season the columns roll (the oldest leaves); the average is the three seasons' points over their games; the rows are sorted by the average; on a new season the relegated clubs and those who left the division leave the table"),
         ('arg_prm_set_club_progress', 'ArgPrm::set_club_progress', '0x406b80', 'arg_prm.cpp', 'thiscall, ret 0x14',
          'guarda o progresso de um clube nas estatísticas da Primeira Divisão argentina: na liga e nas duas fases os códigos acima de 17 viram 18; nas outras, como vêm',
          "stores a club's progress in the Argentine Premier Division statistics: in the league and its two stages the codes above 17 become 18; in others, as they come")])
CAMPOS = []
INTERESSA = []
