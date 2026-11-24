# O avanco de fase, as fases do fim da liga e o progresso dos clubes da Serie C2/C italiana (ita_ser_c2c.cpp; leitor
# cx12, rodada 5, 24/11/2026). Simulacao com a Italia e as divisoes inferiores: o avanco e as fases no fim da liga
# (maio de 2002), jogadas em maio e junho; o progresso a cada linha da tabela em cada atualizacao de status (a
# temporada inteira). Como no cx11_2, a C2 so existe com a base NORMAL ou MAXIMA (BASE abaixo): a prova pede
# DATABASE=NORMAL na carreira.
# - 0x65c450 ItaSerC2c::advance_stage thiscall() ret 0: o corpo do ItaSerC2a::advance_stage (conferido instrucao a
#   instrucao contra o 0x653e00; so o fonte, a linha da caixa 0x250 e a chamada 0x65c4e0 mudam):
#   ita_ser_c1a_advance_stage_cql5_advance_stage. Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x65c4e0 ItaSerC2c::create_playoff_stages thiscall() ret 0: o corpo do ItaSerC2a::create_play_off_stages (conferido
#   instrucao a instrucao contra o 0x653e90; so o fonte, o manipulador do SEH e as linhas das caixas 0x275, 0x289, 0x2ac
#   mudam): create_serie_c2_playoff_stages do cx11_7. Prologo push -1; push imm32 (7 bytes).
# - 0x65c7d0 ItaSerC2c::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68): o corpo do
#   ItaSerC2a::set_club_progress (conferido instrucao a instrucao contra o 0x6544b0, as duas tabelas de salto e os
#   indices iguais): set_c1_club_progress. Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: o avanco lado a lado quando nao cria fase, senao so a original
# (ita_ser_c1a_compare_advance_stage); as fases so a original (compare_serie_c2_playoff_stages); o progresso lado a lado
# nos 3 bytes da entrada (compare_c1_club_progress).
# Depende do cx11_7 (create_serie_c2_playoff_stages, compare_serie_c2_playoff_stages): integrar depois dele.
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2c_advance_stage', 0x65c450, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('ita_ser_c2c_create_playoff_stages', 0x65c4e0, [0x6a, 0xff, 0x68, 0x6c, 0xbd, 0x95, 0x00]),
           ('ita_ser_c2c_set_club_progress', 0x65c7d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('o avanço de fase, as fases do fim da liga e o progresso dos clubes da Série C2/C italiana',
        'the Italian Serie C2/C stage advance, end-of-league stages and club progress',
        [('ita_ser_c2c_advance_stage', 'ItaSerC2c::advance_stage', '0x65c450', 'ita_ser_c2c.cpp', 'thiscall, ret 0',
          'passa a Série C2/C à fase seguinte; quando a liga acaba, monta a repescagem de acesso e o play-out',
          'moves Serie C2/C to the next stage; when the league is over, builds the promotion play-off and the play-out'),
         ('ita_ser_c2c_create_playoff_stages', 'ItaSerC2c::create_playoff_stages', '0x65c4e0', 'ita_ser_c2c.cpp', 'thiscall, ret 0',
          'as fases do fim da Série C2/C: o 2º contra o 5º e o 3º contra o 4º na repescagem de acesso; o 14º contra o 17º e o 15º contra o 16º no play-out do rebaixamento',
          'the Serie C2/C end-of-league stages: 2nd v 5th and 3rd v 4th in the promotion play-off; 14th v 17th and 15th v 16th in the relegation play-out'),
         ('ita_ser_c2c_set_club_progress', 'ItaSerC2c::set_club_progress', '0x65c7d0', 'ita_ser_c2c.cpp', 'thiscall, ret 0x14',
          'a previsão de colocação do clube na C2/C, a mesma regra das outras C2: na liga, melhor e pior colocação viram faixas (2 a 5 = a repescagem: melhor 2, pior 5; 14 a 17 = o play-out: melhor 14, pior 17); na repescagem, as colocações + 1; no play-out, quem fica fica em 14º e quem perde em 16º',
          "the club's place forecast in C2/C, the same rule as the other C2 groups': in the league, best and worst place become bands (2 to 5 = the play-off: best 2, worst 5; 14 to 17 = the play-out: best 14, worst 17); in the play-off, places + 1; in the play-out, the winner stays at 14 and the loser at 16")])
CAMPOS = []
INTERESSA = []
