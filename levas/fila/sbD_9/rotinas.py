# Os promedios do rebaixamento da Segunda Divisao argentina (arg_second.cpp; leitor sbD, rodada 5, 24/11/2026; as sobras
# do cx08, ver levas/fila/cx08_10/FORA.md). Simulacao com a Argentina ('ENG KOR ARG ITA FRA ESP AUS', ate 2002-08-01): o
# init roda na criacao do jogo (o construtor do mwf_23), o update no fim da temporada e na virada (como o da Primeira,
# cx08_5).
# - 0x4093f0 ArgSecond::init_relegation_averages thiscall() ret 0: +0x34 = +0x3e; calloc 0x945501 (contagem, 0x13) em
#   +0x18 (caixa 0x638); as linhas abaixo da contagem com id -1 e o ano (indice em byte); as 25 linhas fixas (o id no
#   global de cada clube, as duas temporadas anteriores, o ano), tiradas pela emulacao da rotina (scratchpad
#   sbD\avg_rows.py) e conferidas contra a desmontagem; depois cada linha abaixo da contagem: o clube do id tem de estar
#   na Segunda ([[+0x57]] = [0x9cf658]); senao free e reset_relegation_averages (caixa 0x6d1). Prologo sub esp, 0x204.
# - 0x409c20 ArgSecond::update_relegation_averages thiscall(rolar) ret 4: a propria tabela na terceira coluna
#   (take_season_points), as fases de zona somadas (0 ate +0x30; o indice da fase e o registro reusado pela busca da
#   linha: depois de uma fase com clubes, a proxima e o ultimo indice da busca + 1, como no original), rolar/promedio/
#   qsort (average_seasons) e, com rolar, as linhas dos clubes com status 3, status 0 ou fora da divisao zeradas (id -1,
#   o ano). Prologo sub esp, 0xc; push ebx; push ebp (5 bytes).
# Defeitos do original: no init, com menos de 25 clubes as linhas fixas sao gravadas alem do bloco, e uma linha de id -1
# e procurada 0x245 bytes antes da tabela dos clubes; no update, o indice das fases de zona e o da busca (as zonas depois
# da primeira quase nunca sao somadas, e o laco pode voltar a uma fase anterior e nao terminar: visto na emulacao com
# dados inventados, nas duas, original e copia).
# Comparadores: o init so a original (as linhas semeadas ou vazias); o update lado a lado nas linhas dos promedios (com
# rolar so a original), pelo compare_relegation_averages_update do cx08_5.
# Depende do cx08_4 (ARG_AVERAGES, ARG_AVERAGE_COUNT, ARG_AVERAGE_ROW_SIZE, arg_average_row_club, this_average_count) e do
# cx08_5 (AVERAGE_ROW_*, reset_relegation_averages, find_average_row, take_season_points, average_seasons,
# compare_relegation_averages_update). Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_second_init_relegation_averages', 0x4093f0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('arg_second_update_relegation_averages', 0x409c20, [0x83, 0xec, 0x0c, 0x53, 0x55])]
DOCS = ('os promédios do rebaixamento da Segunda Divisão argentina', 'the Argentine Second Division relegation averages',
        [('arg_second_init_relegation_averages', 'ArgSecond::init_relegation_averages', '0x4093f0', 'arg_second.cpp', 'thiscall, ret 0',
          'os promédios da Segunda Divisão argentina partem das temporadas reais de 1999/2000 e 2000/01 de vinte e cinco clubes (pontos e jogos de cada uma); se algum clube da tabela não estiver na Segunda no banco, a tabela começa vazia',
          'the Argentine Second Division averages start from the real 1999/2000 and 2000/01 seasons of twenty-five clubs (the points and games of each); if any club of the table is not in the Second Division in the database, the table starts empty'),
         ('arg_second_update_relegation_averages', 'ArgSecond::update_relegation_averages', '0x409c20', 'arg_second.cpp', 'thiscall, ret 4',
          'os promédios tomam os pontos e jogos da temporada de cada clube na tabela geral e somam os das zonas jogadas; numa temporada nova as colunas andam; o promédio é a soma dos pontos das três temporadas sobre a dos jogos; as linhas são ordenadas pelo promédio; numa temporada nova os rebaixados, os promovidos e quem saiu da divisão deixam a tabela',
          "the averages take each club's season points and games from the overall table and add those of the zones played; on a new season the columns roll; the average is the three seasons' points over their games; the rows are sorted by the average; on a new season the relegated, the promoted and those who left the division leave the table")])
CAMPOS = []
INTERESSA = []
