# Os registros e o progresso da Serie B e a passagem de fase da Serie C (bra_nat_second.cpp, bra_nat_third.cpp; leitor
# cx09, rodada 5, 24/11/2026). Simulacao com as 8 ligas ate 2002-06-01 (o Brasileiro de 2001 vai de agosto a dezembro).
# - 0x42d040 BraNatSecond::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18 -> 0
#   (virtual 0x44): fase 0xff: status 0 add_promotion_record 0x869480 (0x64), 3 add_relegation_record 0x86a1c0. Fase 0,
#   status 1: League::find_club_row 0x68ab70 (clube, 0), add_league_champions_record 0x868c50 e +0x37 da linha = 5 (sem
#   testar a linha nula: defeito, derrubaria o jogo). Prologo mov al, [esp+0xc]; push esi (5 bytes).
# - 0x42d260 BraNatSecond::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 -> a entrada
#   (virtual 0x68): Comp_Stats::find_club_entry 0x4a4850 (sem conferir); fase 0xff: melhor < 5 (sem sinal) -> 1, pior
#   < 5 -> 4; +0x73, +0x74, +0x75. Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# - 0x42e380 BraNatThird::advance_stage thiscall() ret 0 (virtual 0x28): +0x30 >= +0x2c - 1: a caixa (linha 0x2b7);
#   senao +0x30 + 1 e na 7 0x42e430, na 8 0x42e700, na 10 0x42e980 (a 9 e posta pela 0x42e700). Prologo mov edx,
#   [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# Comparadores: os registros so a original (retorno 0 e o status 5 do campeao conferidos); o progresso lado a lado
# (compare_group_championship_progress do cx04_5); a passagem por compare_state_league_advance_stage.
# Iguais no cm0102-1080.exe. Depende do cx04_5.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('bra_nat_second_update_staff_records_for_status', 0x42d040, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('bra_nat_second_set_club_progress', 0x42d260, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('bra_nat_third_advance_stage', 0x42e380, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('os registros e o progresso da Série B e a passagem de fase da Série C',
        'the Brazilian Serie B staff records and progress, and the Serie C stage advance',
        [('bra_nat_second_update_staff_records_for_status', 'BraNatSecond::update_staff_records_for_status', '0x42d040', 'bra_nat_second.cpp', 'thiscall, ret 0x18',
          'na Série B, quem sobe e quem cai vão aos registros do técnico; o vencedor da final é campeão',
          "in the Brazilian Serie B, promotion and relegation go to the manager's records; the winner of the final is champion"),
         ('bra_nat_second_set_club_progress', 'BraNatSecond::set_club_progress', '0x42d260', 'bra_nat_second.cpp', 'thiscall, ret 0x14',
          'o progresso na Série B: na liga, o melhor código abaixo de 5 vira 1 e o pior abaixo de 5 vira 4',
          'the Serie B progress: in the league a best code below 5 becomes 1 and a worst code below 5 becomes 4'),
         ('bra_nat_third_advance_stage', 'BraNatThird::advance_stage', '0x42e380', 'bra_nat_third.cpp', 'thiscall, ret 0',
          'a Série C passa de fase: na 7 monta a segunda fase, na 8 os dois grupos da terceira e na 10 a final',
          'the Serie C moves on: stage 7 builds the second round, stage 8 the two third-round groups and stage 10 the final')])
CAMPOS = []
INTERESSA = []
