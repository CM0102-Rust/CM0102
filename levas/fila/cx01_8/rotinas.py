# As ordens das linhas das medias do rebaixamento (comp.cpp, usadas pelas ligas argentinas; leitor cx01, rodada 5,
# 24/11/2026). Competicoes com a Argentina: rodam no fim da temporada argentina (mundo ate 2002-08-01:
# ArgPrm::update_relegation_averages 2 vezes, ArgSecond::create_promotion_cup_stage, create_promotion_playoffs e
# update_relegation_averages 1 vez cada).
# - 0x48f410 comp_compare_average_rows cdecl(a, b) -> int: qsort(linhas, n, 0x13, 0x48f410) em ArgPrm/ArgSecond
#   update_relegation_averages e nas fases de acesso da segunda (0x406713, 0x4087ec, 0x408b94, 0x409f04).
# - 0x48f450 comp_compare_average_rows_by_season cdecl(a, b) -> int: a coluna da temporada no byte 0xadae04 (2, gravado
#   antes por ArgSecond::create_promotion_cup_stage e create_promotion_playoffs; 0x408707, 0x408ac1).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Comparadores: compare_order (o valor).
# Defeito do original (0x48f450): a coluna 3 le a palavra em +0x13, fora da linha de 0x13 bytes.
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('comp_compare_average_rows', 0x48f410, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('comp_compare_average_rows_by_season', 0x48f450, [0xa0, 0x04, 0xae, 0xad, 0x00])]
DOCS = ('as ordens das linhas das médias do rebaixamento',
        'the orders of the relegation average rows',
        [('comp_compare_average_rows', 'comp_compare_average_rows', '0x48f410', 'comp.cpp', 'cdecl',
          'ordena as linhas das médias do rebaixamento pela média de pontos, a maior primeiro; médias iguais pelo endereço do clube',
          'orders the relegation average rows by the points average, the highest first; equal averages by the club address'),
         ('comp_compare_average_rows_by_season', 'comp_compare_average_rows_by_season', '0x48f450', 'comp.cpp', 'cdecl',
          'ordena as linhas pelos pontos de uma temporada (a coluna escolhida, de 0 a 3; outra responde 0), mais pontos primeiro; empate pelo endereço do clube',
          'orders the rows by the points of one season (the chosen column, 0 to 3; another answers 0), more points first; ties by the club address')])
CAMPOS = [('linha das médias (0x13 bytes): +0 clube, +4 média (float), +0xa pontos da temporada (colunas de 3 bytes)', 'average row (0x13 bytes): +0 club, +4 average (float), +0xa season points (3-byte columns)')]
INTERESSA = []
