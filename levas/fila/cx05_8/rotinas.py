# Os grupos e as posicoes previstas da Eurocopa (euro_champ.cpp; leitor cx05, rodada 5, 24/11/2026). Simulacao com as
# 8 ligas e ATE 2004-07-15: as tres rodam no sorteio de 10 de fevereiro de 2004 (EuroChamp::run_group_draw, cx05_7);
# 0 chamadas na cobertura do mundo ate 2002-08-01.
# - 0x57b530 EuroChamp::create_group_table thiscall(lista) ret 4: o grupo 1 na propria tabela (malloc 0x944e46 cdecl;
#   League::init_table_row 0x687430: 4 pushes; mov ecx -> thiscall, ret 0x10; League::send_group_draw_news 0x684230
#   thiscall). Prologo sub esp, 0x200 (6 bytes).
# - 0x57b910 EuroChamp::create_group_stage thiscall(g, lista) ret 8: as entradas do grupo g+2 e nations_group_stage do
#   cx05_3 (League_Stage::League_Stage 0x68b100, thiscall, ret 0x58; o numero de times empurrado como word de +0x3e).
#   Prologo mov eax, fs:[0] (6 bytes).
# - 0x57c9f0 EuroChamp::set_predicted_positions thiscall() ret 0 (virtual +0x5c): NationalTeams::release_competitions_
#   of_continent 0x775070 (push 1; push continente; mov ecx, [0xb74318] -> thiscall, ret 8), qsort com 0x4b07d0,
#   Comp_Stats::set_club_final_position 0x4a2540 (thiscall, ret 8), NationalTeams::set_current_competition 0x775220
#   (push competicao; push nacao; mov ecx, [0xb74318] -> thiscall, ret 8). Prologo sub esp, 0x200.
# A lista do sorteio: 16 entradas de 5 bytes (o id da selecao, o grupo de 1 a 4).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx05_3 (nations_group_stage): integrar depois dele.
# Comparadores: so a original nas tres (alocam, a noticia sai, as selecoes mudam de competicao), com o plano: as
# entradas do grupo 1 nas linhas; a fase com as do grupo g+2; a posicao de cada finalista por reputacao.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-07-15'
ROTINAS = [('euro_champ_create_group_table', 0x57b530, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('euro_champ_create_group_stage', 0x57b910, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('euro_champ_set_predicted_positions', 0x57c9f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os grupos e as posições previstas da Eurocopa', 'the European Championship groups and predicted positions',
        [('euro_champ_create_group_table', 'EuroChamp::create_group_table', '0x57b530', 'euro_champ.cpp', 'thiscall, ret 4',
          'regra: o grupo 1 da Eurocopa fica na tabela da própria copa, com as quatro sorteadas para ele na ordem do sorteio; os técnicos recebem a notícia',
          'rule: group 1 of the European Championship stays in the cup\'s own table, with the four nations drawn into it in draw order; the managers get the news'),
         ('euro_champ_create_group_stage', 'EuroChamp::create_group_stage', '0x57b910', 'euro_champ.cpp', 'thiscall, ret 8',
          'regra: os grupos 2 a 4 da Eurocopa são fases de liga com as quatro sorteadas de cada um, as regras e o ano da copa e o calendário do grupo',
          'rule: groups 2 to 4 of the European Championship are league stages with each group\'s four drawn nations, the cup\'s rules and year and the group\'s round table'),
         ('euro_champ_set_predicted_positions', 'EuroChamp::set_predicted_positions', '0x57c9f0', 'euro_champ.cpp', 'thiscall, ret 0',
          'regra: as posições previstas das 16 finalistas por reputação: 1, 2, dois 3ºs, quatro 5ºs, quatro 9ºs e quatro 13ºs; as seleções europeias deixam as suas competições e as finalistas passam a ter a Eurocopa como competição atual',
          'rule: the predicted places of the 16 finalists by reputation: 1, 2, two 3rds, four 5ths, four 9ths and four 13ths; the European national teams leave their competitions and the finalists get the European Championship as their current competition')])
CAMPOS = []
INTERESSA = []
