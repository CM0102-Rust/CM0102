# O grupo A, os grupos B a D e as posicoes previstas do futebol olimpico (olympics.cpp; leitor cx07, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas e ATE 2004-11-01: o sorteio e em 20 de marco de 2004 (Olympics::update_for_date,
# ja em Rust pelo OLYMPICS_DAY, chama as tres no dia do sorteio), os grupos de 13 a 19 de setembro de 2004 (0 chamadas na
# cobertura do mundo ate 2002-08-01).
# - 0x7a0140 Olympics::init_group_a_table thiscall() ret 0 -> 1/0: club_count 4, malloc 0x944e46 (0xec; cdecl),
#   League::init_table_row 0x687430 (push 0; push 0; push clube; push i -> thiscall, ret 0x10) com as sorteadas 0 a 3,
#   League::send_group_draw_news 0x684230 (thiscall, ret 0); caixa na linha 0x1e3. Prologo sub esp, 0x204 (6 bytes).
# - 0x7a05d0 Olympics::create_group_stage thiscall(g) ret 4 -> 1/0 (g = 0 a 2): o molde da Copa da Asia
#   (nations_group_stage do cx05_3: League_Stage::League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos; a virtual
#   +0x3c) com as sorteadas 4(g+1)..4(g+1)+3; caixas nas linhas 0x2c5 e 0x2dc. Prologo mov eax, fs:[0] (6 bytes).
# - 0x7a13e0 Olympics::set_predicted_positions thiscall() ret 0 (virtual +0x5c): o molde da Copa Africana
#   (predicted_by_reputation_of_copy) com as 16 (+0xee, +0xf2 dword): 1, 2, 3, 3, quatro 5, quatro 9, quatro 13; qsort
#   0x945333 com 0x4b07d0 (cdecl), Comp_Stats::set_club_final_position 0x4a2540 (push pos; push clube -> thiscall,
#   ret 8); caixas nas linhas 0x44d e 0x455. Prologo sub esp, 0x200 (6 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx05_3 (nations_group_stage) e do cx07_6 (OLYMPICS_SOURCE): integrar depois deles.
# Comparadores: so a original nas tres (alocam, a noticia sai, as entradas das estatisticas sao criadas), com o plano: o
# grupo A nas linhas 0 a 3; a fase do grupo com as quatro sorteadas; a posicao de cada uma por reputacao.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('olympics_init_group_a_table', 0x7a0140, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('olympics_create_group_stage', 0x7a05d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('olympics_set_predicted_positions', 0x7a13e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os grupos e as posições previstas do futebol olímpico',
        'the Olympic football groups and predicted positions',
        [('olympics_init_group_a_table', 'Olympics::init_group_a_table', '0x7a0140', 'olympics.cpp', 'thiscall, ret 0',
          'monta a tabela do grupo A do futebol olímpico com as quatro primeiras sorteadas, cada uma na linha da sua ordem, e manda a notícia do sorteio do grupo',
          'builds the Olympic football group A table with the first four drawn nations, each in the row of its order, and sends the group draw news'),
         ('olympics_create_group_stage', 'Olympics::create_group_stage', '0x7a05d0', 'olympics.cpp', 'thiscall, ret 4',
          'monta os grupos B, C e D do futebol olímpico como fases de liga com as sorteadas de quatro em quatro e manda a notícia do sorteio',
          'builds Olympic football groups B, C and D as league stages with the drawn nations four by four and sends the draw news'),
         ('olympics_set_predicted_positions', 'Olympics::set_predicted_positions', '0x7a13e0', 'olympics.cpp', 'thiscall, ret 0',
          'as posições previstas das 16 seleções olímpicas por reputação: 1, 2, 3, 3, quatro 5, quatro 9 e quatro 13',
          'the predicted positions of the 16 Olympic nations by reputation: 1, 2, 3, 3, four 5s, four 9s and four 13s')])
CAMPOS = [('futebol olímpico +0xee lista das sorteadas (16 ponteiros), +0xf2 quantas (dword); lê', 'Olympic football +0xee drawn list (16 pointers), +0xf2 count (dword); reads')]
INTERESSA = []
