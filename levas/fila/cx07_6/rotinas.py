# A virada, a passagem de fase e o mata-mata do futebol olimpico (olympics.cpp; leitor cx07, rodada 5, 24/11/2026).
# Simulacao com as 8 ligas e ATE 2004-11-01: a edicao de 2004 (os grupos de 13 a 19 de setembro, o mata-mata de 23 a 30
# de setembro; 0 chamadas na cobertura do mundo ate 2002-08-01). Duvida para a prova: a data em que o jogo chama a virada
# nao foi determinada (a da Copa America roda logo depois da edicao, cqs_12); se nao rodar ate 2004-11-01, a virada fica
# para a auditoria.
# - 0x7a0450 Olympics::start_new_season thiscall() ret 0 (virtual +8): o molde da Copa America
#   (nations_tournament_new_season do cx05_5) com o ano + 4 e a caixa da linha 0x291. Prologo sub esp, 0x200 (6 bytes).
# - 0x7a0540 Olympics::advance_stage thiscall() ret 0 (virtual +0x28): con_merc_cup_advance_stage_cqs11_next_stage com o
#   mata-mata em 3 (0x7a07c0 pelo endereco) e a caixa da linha 0x2a3. Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30].
# - 0x7a07c0 Olympics::create_knockout_stage thiscall() ret 0: malloc 0x944e46 (cdecl), as oito lidas direto das tabelas
#   (+0xb1 de cada grupo, linhas 0 e 1), a virtual +0x3c (4 pushes), operator new 0x944cf1, Cup_Stage::Cup_Stage
#   0x522e00 (thiscall, ret 0x3c, 15 argumentos: 8 times, fase 3), free, Cup::set_up_round 0x51c800 (push 0 -> thiscall,
#   ret 4); caixas nas linhas 0x2fb e 0x316. Prologo mov eax, fs:[0] (6 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx05_5 (nations_tournament_new_season) e do cx07_2 (nations_knockout_from, check_nations_knockout):
# integrar depois deles.
# Comparadores: so a original nas tres (liberam, alocam, sorteiam), com o plano: o ano + 4, +0x30 = -1, +0x45 = 0 e
# +0x4c = 0; +0x30 sobe um; o mata-mata em [+0xc][3] e +0x30 = 3.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('olympics_start_new_season', 0x7a0450, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('olympics_advance_stage', 0x7a0540, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('olympics_create_knockout_stage', 0x7a07c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a virada, a passagem de fase e o mata-mata do futebol olímpico',
        'the Olympic football turnover, stage advance and knockout',
        [('olympics_start_new_season', 'Olympics::start_new_season', '0x7a0450', 'olympics.cpp', 'thiscall, ret 0',
          'o futebol olímpico é a cada quatro anos: ao fim, a tabela e as fases são desfeitas, a temporada vai para o histórico das estatísticas, o ano sobe 4 e as regras são montadas de novo',
          'the Olympic football is every four years: when it ends the table and stages are undone, the season goes to the statistics history, the year goes up 4 and the rules are set up again'),
         ('olympics_advance_stage', 'Olympics::advance_stage', '0x7a0540', 'olympics.cpp', 'thiscall, ret 0',
          'passa o futebol olímpico à fase seguinte quando a atual acaba; ao chegar à fase 3 (depois dos quatro grupos) monta o mata-mata; passar da última fase é erro',
          'moves the Olympic football on one stage when the current one ends; on reaching stage 3 (after the four groups) it builds the knockout; going past the last stage is an error'),
         ('olympics_create_knockout_stage', 'Olympics::create_knockout_stage', '0x7a07c0', 'olympics.cpp', 'thiscall, ret 0',
          'monta as quartas de final do futebol olímpico: A1, B2, C1, D2, A2, B1, C2 e D1, nessa ordem, e sorteia a primeira rodada',
          'builds the Olympic football quarter-finals: A1, B2, C1, D2, A2, B1, C2 and D1, in that order, and sets up the first round')])
CAMPOS = []
INTERESSA = []
