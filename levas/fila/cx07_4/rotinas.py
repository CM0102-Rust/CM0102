# A virada, a passagem de fase e o mata-mata da Copa das Nacoes da Oceania (oceania_nations.cpp; leitor cx07, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas e ATE 2002-12-31: a edicao de 2002 (grupos em 10, 13 e 15 de novembro, o
# mata-mata em 17 e 19 de novembro; 0 chamadas na cobertura do mundo ate 2002-08-01). Duvida para a prova: a data em
# que o jogo chama a virada nao foi determinada (a da Copa America, molde destas, roda logo depois da edicao, cqs_12);
# se nao rodar ate 2002-12-31, a virada fica para a auditoria.
# - 0x7998e0 OceaniaNations::start_new_season thiscall() ret 0 (virtual +8): o molde da Copa America
#   (nations_tournament_new_season do cx05_5) com o ano + 2 e a caixa da linha 0x2fd; dentro, League::free_table 0x687970
#   (push 0 -> thiscall, ret 4), free 0x9452ca (cdecl), Comp_Stats::archive_season_and_reset 0x4a1c50 (push 1 ->
#   thiscall, ret 4), o destrutor escalar de cada fase (virtual 0 com 1), a virtual +0x8c (thiscall, ret 0). Prologo sub
#   esp, 0x200 (6 bytes).
# - 0x7999d0 OceaniaNations::advance_stage thiscall() ret 0 (virtual +0x28): con_merc_cup_advance_stage_cqs11_next_stage
#   com o mata-mata em 1 (0x799c50 pelo endereco: o desvio leva a copia) e a caixa da linha 0x30f. Prologo mov edx,
#   [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x799c50 OceaniaNations::create_knockout_stage thiscall() ret 0: malloc 0x944e46 (cdecl), League::get_table_row
#   0x6826d0 (push k -> thiscall, ret 4), a virtual +0x3c (4 pushes), operator new 0x944cf1, Cup_Stage::Cup_Stage
#   0x522e00 (thiscall, ret 0x3c, 15 argumentos: 4 times, fase 1), free, Cup::set_up_round 0x51c800 (push 0 ->
#   thiscall, ret 4); caixas nas linhas 0x366 e 0x37d. Prologo mov eax, fs:[0] (6 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx05_5 (nations_tournament_new_season), do cx05_3 (table_row_club) e do cx07_2 (nations_knockout_from,
# check_nations_knockout): integrar depois deles.
# Comparadores: so a original nas tres (liberam, alocam, sorteiam), com o plano: o ano + 2, +0x30 = -1, +0x45 = 0 e
# +0x4c = 0; +0x30 sobe um; o mata-mata em [+0xc][1] e +0x30 = 1.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-12-31'
ROTINAS = [('oceania_nations_start_new_season', 0x7998e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('oceania_nations_advance_stage', 0x7999d0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('oceania_nations_create_knockout_stage', 0x799c50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a virada, a passagem de fase e o mata-mata da Copa das Nações da Oceania',
        'the Oceania Nations Cup turnover, stage advance and knockout',
        [('oceania_nations_start_new_season', 'OceaniaNations::start_new_season', '0x7998e0', 'oceania_nations.cpp', 'thiscall, ret 0',
          'a Copa da Oceania é a cada dois anos: ao fim, a tabela e as fases são desfeitas, a temporada vai para o histórico das estatísticas, o ano sobe 2 e as regras são montadas de novo',
          'the Oceania Nations Cup is every two years: when it ends the table and stages are undone, the season goes to the statistics history, the year goes up 2 and the rules are set up again'),
         ('oceania_nations_advance_stage', 'OceaniaNations::advance_stage', '0x7999d0', 'oceania_nations.cpp', 'thiscall, ret 0',
          'passa a Copa da Oceania à fase seguinte quando a atual acaba; ao chegar à fase 1 (depois dos dois grupos) monta o mata-mata; passar da última fase é erro',
          'moves the Oceania Nations Cup on one stage when the current one ends; on reaching stage 1 (after the two groups) it builds the knockout; going past the last stage is an error'),
         ('oceania_nations_create_knockout_stage', 'OceaniaNations::create_knockout_stage', '0x799c50', 'oceania_nations.cpp', 'thiscall, ret 0',
          'monta as semifinais da Copa da Oceania: o primeiro do grupo A contra o segundo do B e o segundo do A contra o primeiro do B, e sorteia a primeira rodada',
          'builds the Oceania Nations Cup semi-finals: group A\'s winner against group B\'s runner-up and group A\'s runner-up against group B\'s winner, and sets up the first round')])
CAMPOS = []
INTERESSA = []
