# As rodadas da Copa do Imperador, os pontos da J. League e os potes da Copa da Italia (jap_emp_cup.cpp,
# kor_league_cup.cpp, ita_cup.cpp; leitor cx02, rodada 5, 24/11/2026). Competicoes com o Japao e a Italia (8 ligas);
# ATE 2002-08-01 para a virada da Copa do Imperador (a set_up_rounds roda na virada).
# - 0x662b00 JapEmpCup::set_up_rounds: premio 7.000.000 e a tabela (set_up_prize_rounds do cx02_4: integrar depois dele).
# - 0x66d770 KorLeagueCup::points_for_result thiscall(clube, partida) ret 8 -> al (a virtual 0x90 da League; vtables da
#   J1 0x96ca40, da J2 0x96caf4 e da Copa da Liga coreana 0x96ce84: roda a cada resultado da J. League). Como
#   League::points_for_result, mas quem vence na prorrogacao leva 2 (e nao os pontos da vitoria). Prologo mov eax,
#   [esp+8]; push ebx.
# - 0x640590 compare_clubs_by_seeding_pot cdecl(a, b) -> int: o qsort 0x945333 da ItaCup::select_entrants (cqo_11) na
#   criacao do jogo e na virada; as chaves 0x9cf570 (Serie A) e 0x9cf574 (Serie B). Prologo mov eax, [esp+4]; mov ecx,
#   [esp+8] (8 bytes).
# Iguais no cm0102-1080.exe.
# Comparadores: as rodadas lado a lado (cx02_4); o byte devolvido; a ordem (compare_order).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('jap_emp_cup_set_up_rounds', 0x662b00, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('kor_league_cup_points_for_result', 0x66d770, [0x8b, 0x44, 0x24, 0x08, 0x53]),
           ('compare_clubs_by_seeding_pot', 0x640590, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08])]
DOCS = ('as rodadas da Copa do Imperador, os pontos da J. League e os potes da Copa da Itália',
        "the Emperor's Cup rounds, the J. League points and the Coppa Italia seeding pots",
        [('jap_emp_cup_set_up_rounds', 'JapEmpCup::set_up_rounds', '0x662b00', 'jap_emp_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa do Imperador tem um prêmio total de 7.000.000; monta a tabela de rodadas',
          "rule: the Emperor's Cup has a 7,000,000 prize fund; builds the round table"),
         ('kor_league_cup_points_for_result', 'KorLeagueCup::points_for_result', '0x66d770', 'kor_league_cup.cpp', 'thiscall, ret 8',
          'regra: na J1, na J2 e na Copa da Liga coreana a vitória no tempo normal vale os pontos da vitória; empatado, quem vence na prorrogação leva 2 pontos e quem perde 0; ainda empatado, quem vence nos pênaltis leva os pontos do empate e quem perde 0; sem desempate, os pontos do empate',
          'rule: in the J1, the J2 and the Korean League Cup a win in normal time is worth the win points; level, the extra-time winner takes 2 points and the loser 0; still level, the shoot-out winner takes the draw points and the loser 0; with no decider, the draw points'),
         ('compare_clubs_by_seeding_pot', 'compare_clubs_by_seeding_pot', '0x640590', 'ita_cup.cpp', 'cdecl',
          'regra: os potes da Copa da Itália pela temporada passada: pote 4 para quem foi do 3º ao 6º na Série B ou caiu da Série A para a B; pote 3 do 7º ao 14º na Série B; pote 2 do 15º ao 18º na Série B ou quem subiu para a Série B; pote 1 os outros; a ordem é a diferença dos potes',
          'rule: the Coppa Italia pots by last season: pot 4 for 3rd to 6th in Serie B or relegated from Serie A to B; pot 3 for 7th to 14th in Serie B; pot 2 for 15th to 18th in Serie B or promoted into Serie B; pot 1 the others; the order is the difference of the pots')])
CAMPOS = []
INTERESSA = []
