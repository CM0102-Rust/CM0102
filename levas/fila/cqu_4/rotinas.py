# Os calendários da MLS e da J2 (o slot create_schedule, a virtual +0x3c, de usa_mls.cpp, jap_j2.cpp; agente C
# da frente das competições, 06/11/2026). Rodam 2 a 4 vezes cada na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt): a fase de pontos corridos na criação da carreira (janeiro de 2001) e na virada, e o
# mata-mata ou a final quando a fase chega. Todas no mesmo molde dos calendários das copas da cqp_11: conforme a fase,
# grava as saídas (é_liga se o ponteiro não é nulo, as rodadas e o extra), aloca a tabela (0x41 bytes por rodada; sem
# memória, a caixa e 0) e grava as datas e os jogos de cada rodada; fase sem tabela: 0, sem gravar nada. O porte usa
# a máquina da cqp_11 (Cqp11Op, Cqp11Fase, cqp11_tabela, cqp11_comparar, já no motor.rs) com as tabelas de cada liga,
# extraídas seguindo o fluxo da original com cada fase de −128 a 127 (todas as outras fases devolvem 0 sem gravar).
# A MLS tem três fases de pontos corridos (−3, −2 e −1: a terceira, a segunda e a primeira divisão), o play-off (2) e a
# final (3); a J2, só a de pontos corridos (−1). Este item ficou no lugar das posições previstas dos estaduais (cqu_4
# antiga, cqv_1, cqv_2: fora, ver cqv_1/FORA.md).
# - 0x9072d0 UsaMls::create_schedule (usa_mls.cpp).
# - 0x664780 JapJ2::create_schedule (jap_j2.cpp).
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
ROTINAS = [('cqu4_calendario_mls', 0x9072d0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqu4_calendario_j2', 0x664780, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários da MLS e da J2',
        'the schedules of MLS and J2',
        [('cqu4_calendario_mls', 'UsaMls::create_schedule', '0x9072d0', 'usa_mls.cpp', 'thiscall, ret 0x10', 'O calendário da MLS: a fase -3 (é liga): 12 rodadas, da 29/fev à 30/jul; a fase -2 (é liga): 22 rodadas, da 22/fev à 6/ago; a fase de pontos corridos (−1) (é liga): 32 rodadas, da 22/fev à 6/ago; a fase 2 (mata-mata ou fase final): 2 rodadas, da 7/ago à 30/ago; a fase 3 (mata-mata ou fase final): 1 rodadas, da 8/set à 15/set; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of MLS: stage -3 (league): 12 rounds, from 29/fev to 30/jul; stage -2 (league): 22 rounds, from 22/fev to 6/ago; the round robin stage (−1) (league): 32 rounds, from 22/fev to 6/ago; stage 2 (knockout or final stage): 2 rounds, from 7/ago to 30/ago; stage 3 (knockout or final stage): 1 rounds, from 8/set to 15/set; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated'),
         ('cqu4_calendario_j2', 'JapJ2::create_schedule', '0x664780', 'jap_j2.cpp', 'thiscall, ret 0x10', 'O calendário da J2: a fase de pontos corridos (−1) (é liga): 44 rodadas, da 12/fev à 26/out; cada rodada tem a data (dia, mês, anos depois do da temporada, dia da semana e código) e os jogos, e o resto da tabela fica como veio da alocação', 'The schedule of J2: the round robin stage (−1) (league): 44 rounds, from 12/fev to 26/out; each round has the date (day, month, years after the season one, weekday and code) and the games, and the rest of the table is left as allocated')])
CAMPOS = [('tabela de rodadas (0x41 bytes por rodada): +0 o dia do ano, +2 os anos depois do da liga, +4 o código, +5 os jogos (7 bytes cada: +9 o dia da semana, +0xa e +0xb [inferência] o horário e o mando), +0x3d o valor; grava', 'round table (0x41 bytes per round): +0 day of year, +2 years after the league one, +4 code, +5 the games (7 bytes each: +9 weekday, +0xa and +0xb [inference] time and home side), +0x3d value; writes'),
          ('liga +0x40: o ano da temporada; valor; lê', 'league +0x40: the season year; value; reads')]
INTERESSA = ['CMRTE']
