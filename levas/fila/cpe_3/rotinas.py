# Bloco das competições, a tabela da liga (league.cpp), as pequenas: a ordem pelo confronto guardado
# (league_compare_table_rows_h2h 0x683410, callback do qsort da 0x6827d0 na regra 4 sendo a primeira e da 0x6471a0 da
# Série B italiana), a ordem das datas (league_compare_dates 0x685cf0, callback do bsearch da 0x685640, 6.533 chamadas),
# a linha do clube (League::find_club_row 0x68ab70, 31 chamadas diretas) e se a liga tem o clube (League::contains_club
# 0x68abd0, só pela virtual +0x80 das ligas). Fora do mapa de cobertura (sem arquivo no mapa das funções): quantas vezes
# rodam na carreira não se sabe; todas só leem (a comparação própria [0xb67a00], chamada pela 0x683410, é a das ligas).
# Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem sorteio,
# sem caixa, sem locais. Comparadores pelo valor. Em laço quente: a 0x683410 e a 0x685cf0 (dentro do qsort/bsearch);
# correm o risco da 0x689ad0 (241ª), de o desvio mudar o resíduo de pilha que uma original lê depois.
SAVE = False
ROTINAS = [('ordem_pela_posicao_do_confronto', 0x683410, [0xa1, 0xfc, 0x79, 0xb6, 0x00]),
           ('ordem_das_datas_da_liga', 0x685cf0, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('linha_do_clube', 0x68ab70, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('liga_tem_o_clube', 0x68abd0, [0x53, 0x56, 0x8b, 0xb1, 0xb1, 0x00, 0x00, 0x00])]
DOCS = ('a tabela da liga: a ordem pelo confronto guardado, a ordem das datas, a linha do clube e se a liga tem o clube',
        'the league table: the order by stored head-to-head, the date order, the club row and whether the league has the club',
        [('ordem_pela_posicao_do_confronto', 'league_compare_table_rows_h2h', '0x683410', 'league.cpp', 'cdecl',
          'callback do qsort: a comparação própria, os pontos, o id dos clubes sem jogos, a posição do confronto guardada (+4) e o nome',
          'qsort callback: the own compare, the points, the club id with no games, the stored head-to-head position (+4) and the name'),
         ('ordem_das_datas_da_liga', 'league_compare_dates', '0x685cf0', 'league.cpp', 'cdecl',
          'callback do bsearch: o ano (+2, sem sinal) e o dia (+0): −1, 0 ou 1',
          'bsearch callback: the year (+2, unsigned) and the day (+0): −1, 0 or 1'),
         ('linha_do_clube', 'League::find_club_row', '0x68ab70', 'league.cpp', 'thiscall, ret 8',
          'a linha do clube nas linhas dadas (nulas: a tabela da liga), ou 0',
          'the club row in the given rows (null: the league table), or 0'),
         ('liga_tem_o_clube', 'League::contains_club', '0x68abd0', 'league.cpp', 'thiscall, ret 4',
          '1 se o clube tem linha na tabela da liga',
          '1 when the club has a row in the league table')])
CAMPOS = [('linha da tabela (0x3b bytes): +0 clube, +4 posição do confronto, +0x11 jogos e +0x19 pontos por coluna; data da liga: +0 dia, +2 ano; lê (ordem_pela_posicao_do_confronto, ordem_das_datas_da_liga, linha_do_clube, liga_tem_o_clube)',
           'table row (0x3b bytes): +0 club, +4 head-to-head position, +0x11 played and +0x19 points per column; league date: +0 day, +2 year; reads (ordem_pela_posicao_do_confronto, ordem_das_datas_da_liga, linha_do_clube, liga_tem_o_clube)')]
INTERESSA = ['SGE', 'CMRTE']
