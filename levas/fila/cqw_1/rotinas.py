# Os calendários do Baiano, do Centro-Oeste, do Gaúcho e do Goiano (o slot create_schedule, a virtual +0x3c, de bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp, bra_reg_goias.cpp; agente C
# da frente das competições, 06/11/2026). Rodam 2 a 4 vezes cada na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt): a fase de pontos corridos na criação da carreira (janeiro de 2001) e na virada, e o
# mata-mata ou a final quando a fase chega. Todas no mesmo molde dos calendários das copas da cqp_11: conforme a fase,
# grava as saídas (é_liga se o ponteiro não é nulo, as rodadas e o extra), aloca a tabela (0x41 bytes por rodada; sem
# memória, a caixa e 0) e grava as datas e os jogos de cada rodada; fase sem tabela: 0, sem gravar nada. O porte usa
# a máquina da cqp_11 (Cqp11Op, Cqp11Fase, cqp11_tabela, cqp11_comparar, já no motor.rs) com as tabelas de cada liga,
# extraídas seguindo o fluxo da original com cada fase de −128 a 127 (todas as outras fases devolvem 0 sem gravar,
# exceto na NSL, em que todas as outras são a fase final de uma rodada).
# - 0x42f5d0 BraRegBahia::create_schedule (bra_reg_bahia.cpp).
# - 0x4307b0 BraRegCentral::create_schedule (bra_reg_central.cpp).
# - 0x431ac0 BraRegGaucho::create_schedule (bra_reg_gaucho.cpp).
# - 0x432e50 BraRegGoias::create_schedule (bra_reg_goias.cpp).
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
ROTINAS = [('cqw1_calendario_baiano', 0x42f5d0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqw1_calendario_centro_oeste', 0x4307b0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqw1_calendario_gaucho', 0x431ac0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqw1_calendario_goiano', 0x432e50, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários do Baiano, do Centro-Oeste, do Gaúcho e do Goiano',
        'the schedules of the Bahia, Centre-West, Rio Grande do Sul and Goiás state leagues',
        [('cqw1_calendario_baiano', 'BraRegBahia::create_schedule', '0x42f5d0', 'bra_reg_bahia.cpp', 'thiscall, ret 0x10', 'O calendário do Baiano: a fase de pontos corridos (−1) (é liga): 18 rodadas, da 11/jan à 20/abr; a fase 0 (mata-mata ou fase final): 2 rodadas, da 21/abr à 10/mai; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the Bahia state league: the round robin stage (−1) (league): 18 rounds, from 11/jan to 20/abr; stage 0 (knockout or final stage): 2 rounds, from 21/abr to 10/mai; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('cqw1_calendario_centro_oeste', 'BraRegCentral::create_schedule', '0x4307b0', 'bra_reg_central.cpp', 'thiscall, ret 0x10', 'O calendário do Centro-Oeste: a fase de pontos corridos (−1) (é liga): 15 rodadas, da 11/jan à 6/abr; a fase 0 (mata-mata ou fase final): 3 rodadas, da 7/abr à 10/mai; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the Centre-West state league: the round robin stage (−1) (league): 15 rounds, from 11/jan to 6/abr; stage 0 (knockout or final stage): 3 rounds, from 7/abr to 10/mai; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('cqw1_calendario_gaucho', 'BraRegGaucho::create_schedule', '0x431ac0', 'bra_reg_gaucho.cpp', 'thiscall, ret 0x10', 'O calendário do Gaúcho: a fase de pontos corridos (−1) (é liga): 17 rodadas, da 11/jan à 20/abr; a fase 0 (mata-mata ou fase final): 2 rodadas, da 21/abr à 10/mai; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the Rio Grande do Sul state league: the round robin stage (−1) (league): 17 rounds, from 11/jan to 20/abr; stage 0 (knockout or final stage): 2 rounds, from 21/abr to 10/mai; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('cqw1_calendario_goiano', 'BraRegGoias::create_schedule', '0x432e50', 'bra_reg_goias.cpp', 'thiscall, ret 0x10', 'O calendário do Goiano: a fase de pontos corridos (−1) (é liga): 13 rodadas, da 11/jan à 6/abr; a fase 0 (mata-mata ou fase final): 3 rodadas, da 7/abr à 10/mai; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the Goiás state league: the round robin stage (−1) (league): 13 rounds, from 11/jan to 6/abr; stage 0 (knockout or final stage): 3 rounds, from 7/abr to 10/mai; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated')])
CAMPOS = [('tabela de rodadas (0x41 bytes por rodada): +0 o dia do ano, +2 os anos depois do da liga, +4 o código, +5 os jogos (7 bytes cada: +9 o dia da semana, +0xa e +0xb [inferência] o horário e o mando), +0x3d o valor; grava', 'round table (0x41 bytes per round): +0 day of year, +2 years after the league one, +4 code, +5 the games (7 bytes each: +9 weekday, +0xa and +0xb [inference] time and home side), +0x3d value; writes'),
          ('liga +0x40: o ano da temporada; valor; lê', 'league +0x40: the season year; value; reads')]
INTERESSA = ['CMRTE']
