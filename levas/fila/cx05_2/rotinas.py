# O sorteio da Copa da Asia: as doze finalistas, as posicoes previstas e o grupo A (asia_nations.cpp; leitor cx05,
# rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2004-11-01: a Copa da Asia do jogo e a de 2004 (o construtor,
# ja em Rust, leva o ano ao ciclo de 4 a partir de 1996); o sorteio e no dia 5 de junho de 2004 (AsiaNations::
# update_for_date, ja em Rust, chama as tres no dia do sorteio), os grupos de 13 a 19 de outubro e o mata-mata ate 29
# de outubro. Na cobertura do mundo ate 2002-08-01 as tres tem 0 chamadas: sem a data longa o item nao prova nada (se a
# cadeia nao puder ir ate la, deixar para a auditoria ou tirar da fila).
# - 0x40fdf0 AsiaNations::select_and_draw_teams thiscall() ret 0 (do dia do sorteio: mov ecx -> call 0x40fdf0 pelo
#   ASIA_NATIONS_DAY). Dentro: HostCountry::find_hosts 0x5fa730 (5 pushes; mov ecx, [0xb5e134] -> thiscall, ret 0x14; o
#   ano empurrado como word), database_get_national_team 0x53b3d0 (cdecl), comp_util_sort_and_shuffle_top_clubs
#   0x4afbd0 (3 pushes; add esp, 0xc -> cdecl), qsort 0x945333 com 0x4b07d0 (cdecl), utils_random 0x90cfc0 (cdecl), o
#   malloc 0x944e46 e o free 0x9452ca (cdecl). Prologo sub esp, 0x3a8 (6 bytes).
# - 0x4103a0 AsiaNations::set_predicted_positions thiscall() ret 0 (virtual +0x5c): o molde da Copa Africana
#   (predicted_by_reputation_of_copy) com 12 posicoes. Prologo sub esp, 0x200.
# - 0x40f3d0 AsiaNations::create_group_table thiscall() ret 0: 4 linhas com os sorteados 0 a 3 (a lista +0xee guarda os
#   grupos em sequencia). Prologo sub esp, 0x204.
# Defeitos do original: (1) os dois indices de sede sao locais nao iniciados, lidos so os que o find_hosts gravou (ele
# grava os dois quando acha a edicao; sem edicao devolve 0 e nada e lido): a copia inicia com -1, sem efeito; (2) com
# menos de 12 candidatas a rotina devolve 0 deixando a lista +0xee alocada e +0xf2 = 12 (o dia do sorteio segue e a
# tabela do grupo A le a lista incompleta); (3) uma vaga de pote que ja esteja vazia faz o sorteio tentar de novo ate
# achar uma cheia (sem limite; com 12 times sempre acha).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: so a original nas tres (sorteiam, alocam, a noticia sai), com o plano: as 12 distintas; a posicao de
# cada uma por reputacao; o grupo A nas linhas 0 a 3.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('asia_nations_select_and_draw_teams', 0x40fdf0, [0x81, 0xec, 0xa8, 0x03, 0x00, 0x00]),
           ('asia_nations_set_predicted_positions', 0x4103a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('asia_nations_create_group_table', 0x40f3d0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o sorteio da Copa da Ásia', 'the Asian Cup draw',
        [('asia_nations_select_and_draw_teams', 'AsiaNations::select_and_draw_teams', '0x40fdf0', 'asia_nations.cpp', 'thiscall, ret 0',
          'regra: a Copa da Ásia tem 12 seleções: as sedes da edição primeiro e depois as melhores seleções asiáticas (até 100 candidatas, ordenadas e as primeiras embaralhadas); as 12 vão por reputação para quatro potes de três e cada um dos três grupos tira, de cada pote, uma seleção ao acaso; com menos de 12 candidatas não há sorteio',
          'rule: the Asian Cup has 12 nations: the edition\'s hosts first and then the best Asian nations (up to 100 candidates, sorted and the best shuffled); the 12 go by reputation into four pots of three and each of the three groups draws one nation at random from each pot; with fewer than 12 candidates there is no draw'),
         ('asia_nations_set_predicted_positions', 'AsiaNations::set_predicted_positions', '0x4103a0', 'asia_nations.cpp', 'thiscall, ret 0',
          'regra: as posições previstas das 12 finalistas, por reputação: 1, 2, 3, 3, quatro 5ºs, dois 9ºs e dois 11ºs',
          'rule: the predicted places of the 12 finalists, by reputation: 1, 2, 3, 3, four 5ths, two 9ths and two 11ths'),
         ('asia_nations_create_group_table', 'AsiaNations::create_group_table', '0x40f3d0', 'asia_nations.cpp', 'thiscall, ret 0',
          'regra: o grupo A fica na tabela da própria copa, com as quatro primeiras sorteadas na ordem; os técnicos recebem a notícia do sorteio',
          'rule: group A stays in the cup\'s own table, with the first four drawn nations in order; the managers get the draw news')])
CAMPOS = [('Copa da Ásia +0xee lista das sorteadas (ponteiros), +0xf2 quantas (word); lê e grava', 'Asian Cup +0xee drawn list (pointers), +0xf2 count (word); reads and writes')]
INTERESSA = []
