# Os calendários da Série A, da Série B, da J1 e da NSL (o slot create_schedule, a virtual +0x3c, de bra_nat_first.cpp, bra_nat_second.cpp, jap_j1.cpp, aus_nsl.cpp; agente C
# da frente das competições, 06/11/2026). Rodam 2 a 4 vezes cada na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt): a fase de pontos corridos na criação da carreira (janeiro de 2001) e na virada, e o
# mata-mata ou a final quando a fase chega. Todas no mesmo molde dos calendários das copas da cqp_11: conforme a fase,
# grava as saídas (é_liga se o ponteiro não é nulo, as rodadas e o extra), aloca a tabela (0x41 bytes por rodada; sem
# memória, a caixa e 0) e grava as datas e os jogos de cada rodada; fase sem tabela: 0, sem gravar nada. O porte usa
# a máquina da cqp_11 (Cqp11Op, Cqp11Fase, cqp11_tabela, cqp11_comparar, já no motor.rs) com as tabelas de cada liga,
# extraídas seguindo o fluxo da original com cada fase de −128 a 127 (todas as outras fases devolvem 0 sem gravar,
# exceto na NSL, em que todas as outras são a fase final de uma rodada).
# - 0x429b50 BraNatFirst::create_schedule (bra_nat_first.cpp).
# - 0x42c140 BraNatSecond::create_schedule (bra_nat_second.cpp).
# - 0x662e30 JapJ1::create_schedule (jap_j1.cpp).
# - 0x4111a0 AusNsl::create_schedule (aus_nsl.cpp).
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
ROTINAS = [('bra_nat_first_create_schedule', 0x429b50, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bra_nat_second_create_schedule', 0x42c140, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('jap_j1_create_schedule', 0x662e30, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('aus_nsl_create_schedule', 0x4111a0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários da Série A, da Série B, da J1 e da NSL',
        'the schedules of the Brazilian first and second divisions, J1 and the NSL',
        [('bra_nat_first_create_schedule', 'BraNatFirst::create_schedule', '0x429b50', 'bra_nat_first.cpp', 'thiscall, ret 0x10', 'O calendário da Série A: a fase de pontos corridos (−1) (é liga): 27 rodadas, da 5/jul à 2/nov; a fase 0 (mata-mata ou fase final): 2 rodadas, da 3/nov à 12/nov; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the Brazilian first division: the round robin stage (−1) (league): 27 rounds, from 5/jul to 2/nov; stage 0 (knockout or final stage): 2 rounds, from 3/nov to 12/nov; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('bra_nat_second_create_schedule', 'BraNatSecond::create_schedule', '0x42c140', 'bra_nat_second.cpp', 'thiscall, ret 0x10', 'O calendário da Série B: a fase de pontos corridos (−1) (é liga): 27 rodadas, da 5/jul à 2/nov; a fase 0 (mata-mata ou fase final): 2 rodadas, da 3/nov à 12/nov; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the Brazilian second division: the round robin stage (−1) (league): 27 rounds, from 5/jul to 2/nov; stage 0 (knockout or final stage): 2 rounds, from 3/nov to 12/nov; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('jap_j1_create_schedule', 'JapJ1::create_schedule', '0x662e30', 'jap_j1.cpp', 'thiscall, ret 0x10', 'O calendário da J1: a fase de pontos corridos (−1) (é liga): 30 rodadas, da 11/fev à 26/out; a fase 2 (mata-mata ou fase final): 1 rodadas, da 27/out à 2/nov; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of J1: the round robin stage (−1) (league): 30 rounds, from 11/fev to 26/out; stage 2 (knockout or final stage): 1 rounds, from 27/out to 2/nov; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('aus_nsl_create_schedule', 'AusNsl::create_schedule', '0x4111a0', 'aus_nsl.cpp', 'thiscall, ret 0x10', 'O calendário da NSL: a fase as outras (mata-mata ou fase final): 1 rodadas, da 1/jun (+1) à 7/jun (+1); a fase de pontos corridos (−1) (é liga): 24 rodadas, da 5/set à 15/fev (+1); a fase 0 (é liga): 10 rodadas, da 29/fev (+1) à 29/mai (+1); cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of the NSL: stage as outras (knockout or final stage): 1 rounds, from 1/jun (+1) to 7/jun (+1); the round robin stage (−1) (league): 24 rounds, from 5/set to 15/fev (+1); stage 0 (league): 10 rounds, from 29/fev (+1) to 29/mai (+1); each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated')])
CAMPOS = [('tabela de rodadas (0x41 bytes por rodada): +0 o dia do ano, +2 os anos depois do da liga, +4 o código, +5 os jogos (7 bytes cada: +9 o dia da semana, +0xa e +0xb [inferência] o horário e o mando), +0x3d o valor; grava', 'round table (0x41 bytes per round): +0 day of year, +2 years after the league one, +4 code, +5 the games (7 bytes each: +9 weekday, +0xa and +0xb [inference] time and home side), +0x3d value; writes'),
          ('liga +0x40: o ano da temporada; valor; lê', 'league +0x40: the season year; value; reads')]
INTERESSA = ['CMRTE']
