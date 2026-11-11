# Os calendários do Mineiro, do Norte, do Nordeste e do Paranaense (o slot create_schedule, a virtual +0x3c, de bra_reg_minas_gerais.cpp, bra_reg_north.cpp, bra_reg_northeast.cpp, bra_reg_parana.cpp; agente C
# da frente das competições, 06/11/2026). Rodam 2 a 4 vezes cada na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt): a fase de pontos corridos na criação da carreira (janeiro de 2001) e na virada, e o
# mata-mata ou a final quando a fase chega. Todas no mesmo molde dos calendários das copas da cqp_11: conforme a fase,
# grava as saídas (é_liga se o ponteiro não é nulo, as rodadas e o extra), aloca a tabela (0x41 bytes por rodada; sem
# memória, a caixa e 0) e grava as datas e os jogos de cada rodada; fase sem tabela: 0, sem gravar nada. O porte usa
# a máquina da cqp_11 (Cqp11Op, Cqp11Fase, cqp11_tabela, cqp11_comparar, já no motor.rs) com as tabelas de cada liga,
# extraídas seguindo o fluxo da original com cada fase de −128 a 127 (todas as outras fases devolvem 0 sem gravar,
# exceto na NSL, em que todas as outras são a fase final de uma rodada).
# - 0x434050 BraRegMinasGerais::create_schedule (bra_reg_minas_gerais.cpp).
# - 0x435640 BraRegNorth::create_schedule (bra_reg_north.cpp).
# - 0x4366f0 BraRegNortheast::create_schedule (bra_reg_northeast.cpp).
# - 0x437700 BraRegParana::create_schedule (bra_reg_parana.cpp).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas:
# - as rotinas, a virtual +0x3c (de League::create_fixtures 0x6835c0 e das viradas: push &é_liga; push &extra; push
#   &rodadas; push fase; mov ecx, liga; call [vtable + 0x3c]) -> thiscall, ret 0x10;
# - 0x944e46 (malloc): push tamanho; call; add esp, 4 -> cdecl;
# - 0x68a160 (a data da rodada) e 0x68a1c0 (o jogo da rodada), 0x521e60 (o sorteio), 0x521eb0 (o jogo da copa): 9, 7, 7 e
#   9 pushes; call; add esp -> cdecl (o ano: palavra +0x40 limpa na 0x68a160, com sinal nas de copa);
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, pela cqp11_tabela.
# Comparador: lado a lado (cqp11_comparar): a original com as saídas de quem chama, a cópia com saídas próprias que
# começam com os mesmos valores, a partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas iguais;
# a tabela da cópia é liberada. Na fase sem tabela, só a original (tem de devolver 0).
# Determinismo: só constantes e o ano da liga.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('bra_reg_minas_gerais_create_schedule', 0x434050, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bra_reg_north_create_schedule', 0x435640, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bra_reg_northeast_create_schedule', 0x4366f0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bra_reg_parana_create_schedule', 0x437700, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários do Mineiro, do Norte, do Nordeste e do Paranaense',
        'the schedules of the Minas Gerais, North, Northeast and Paraná state leagues',
        [('bra_reg_minas_gerais_create_schedule', 'BraRegMinasGerais::create_schedule', '0x434050', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 0x10', 'O calendário do Mineiro: a fase de pontos corridos (−1) (é liga): 22 rodadas, da 11/jan à 3/mai; a fase 2 (mata-mata ou fase final): 1 rodadas, da 4/mai à 10/mai; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the Minas Gerais state league: the round robin stage (−1) (league): 22 rounds, from 11/jan to 3/mai; stage 2 (knockout or final stage): 1 rounds, from 4/mai to 10/mai; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('bra_reg_north_create_schedule', 'BraRegNorth::create_schedule', '0x435640', 'bra_reg_north.cpp', 'thiscall, ret 0x10', 'O calendário do Norte: a fase de pontos corridos (−1) (é liga): 18 rodadas, da 11/jan à 20/abr; a fase 0 (mata-mata ou fase final): 2 rodadas, da 21/abr à 10/mai; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the North state league: the round robin stage (−1) (league): 18 rounds, from 11/jan to 20/abr; stage 0 (knockout or final stage): 2 rounds, from 21/abr to 10/mai; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('bra_reg_northeast_create_schedule', 'BraRegNortheast::create_schedule', '0x4366f0', 'bra_reg_northeast.cpp', 'thiscall, ret 0x10', 'O calendário do Nordeste: a fase de pontos corridos (−1) (é liga): 15 rodadas, da 18/jan à 20/abr; a fase 0 (mata-mata ou fase final): 2 rodadas, da 21/abr à 10/mai; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the Northeast league: the round robin stage (−1) (league): 15 rounds, from 18/jan to 20/abr; stage 0 (knockout or final stage): 2 rounds, from 21/abr to 10/mai; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('bra_reg_parana_create_schedule', 'BraRegParana::create_schedule', '0x437700', 'bra_reg_parana.cpp', 'thiscall, ret 0x10', 'O calendário do Paranaense: a fase de pontos corridos (−1) (é liga): 22 rodadas, da 11/jan à 3/mai; a fase 2 (mata-mata ou fase final): 1 rodadas, da 4/mai à 10/mai; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the Paraná state league: the round robin stage (−1) (league): 22 rounds, from 11/jan to 3/mai; stage 2 (knockout or final stage): 1 rounds, from 4/mai to 10/mai; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated')])
CAMPOS = [('tabela de rodadas (0x41 bytes por rodada): +0 o dia do ano, +2 os anos depois do da liga, +4 o código, +5 os jogos (7 bytes cada: +9 o dia da semana, +0xa e +0xb [inferência] o horário e o mando), +0x3d o valor; grava', 'round table (0x41 bytes per round): +0 day of year, +2 years after the league one, +4 code, +5 the games (7 bytes each: +9 weekday, +0xa and +0xb [inference] time and home side), +0x3d value; writes'),
          ('liga +0x40: o ano da temporada; valor; lê', 'league +0x40: the season year; value; reads')]
INTERESSA = ['CMRTE']
