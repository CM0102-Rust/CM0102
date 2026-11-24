# O progresso do Gaucho e do Goiano (bra_reg_gaucho.cpp, bra_reg_goias.cpp; leitor cx09, rodada 5, 24/11/2026).
# Simulacao com as 8 ligas ate 2002-06-01 (os estaduais de 2002 vao de janeiro a maio).
# - 0x432af0 BraRegGaucho::set_club_progress e 0x433cf0 BraRegGoias::set_club_progress thiscall(clube, fase, atual,
#   melhor, pior) ret 0x14 -> a entrada (virtual 0x68): Comp_Stats::find_club_entry 0x4a4850 (sem conferir); fase 0xff:
#   melhor abaixo do limite (sem sinal) -> 1, pior abaixo do limite -> o valor (Gaucho 5 e 3, Goiano 9 e 5); +0x73,
#   +0x74, +0x75. Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes). O molde e o da Serie B (league_stage_progress do
#   cx09_16).
# Comparadores: lado a lado (compare_league_stage_progress do cx09_16).
# Iguais no cm0102-1080.exe. Depende do cx09_16.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('bra_reg_gaucho_set_club_progress', 0x432af0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('bra_reg_goias_set_club_progress', 0x433cf0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('o progresso do Gaúcho e do Goiano', 'the progress of the Gaucho and Goias state championships',
        [('bra_reg_gaucho_set_club_progress', 'BraRegGaucho::set_club_progress', '0x432af0', 'bra_reg_gaucho.cpp', 'thiscall, ret 0x14',
          'o progresso no Gaúcho: na liga, o melhor código abaixo de 5 vira 1 e o pior abaixo de 5 vira 3',
          'the Gaucho progress: in the league a best code below 5 becomes 1 and a worst code below 5 becomes 3'),
         ('bra_reg_goias_set_club_progress', 'BraRegGoias::set_club_progress', '0x433cf0', 'bra_reg_goias.cpp', 'thiscall, ret 0x14',
          'o progresso no Goiano: na liga, o melhor código abaixo de 9 vira 1 e o pior abaixo de 9 vira 5',
          'the Goias progress: in the league a best code below 9 becomes 1 and a worst code below 9 becomes 5')])
CAMPOS = []
INTERESSA = []
