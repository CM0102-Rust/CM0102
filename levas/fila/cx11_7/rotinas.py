# O avanco de fase e as fases do fim da liga da Serie C2/A italiana (ita_ser_c2a.cpp; leitor cx11, rodada 5,
# 24/11/2026). Simulacao com a Italia e as divisoes inferiores: o advance_stage roda quando a liga acaba (maio de 2002)
# e cria a repescagem e o play-out; de novo no fim de cada um. Como no cx11_2, a C2 so existe com a base NORMAL ou
# MAXIMA (BASE abaixo): a prova pede DATABASE=NORMAL na carreira.
# - 0x653e00 ItaSerC2a::advance_stage thiscall() ret 0 (virtual 0x28): o corpo do ItaSerC1a::advance_stage (conferido
#   instrucao a instrucao; so o fonte, a linha 0x251 e o construtor das fases 0x653e90 mudam):
#   ita_ser_c1a_advance_stage_cql5_advance_stage. Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x653e90 ItaSerC2a::create_play_off_stages thiscall() ret 0: o corpo do ItaSerC1a::create_playoff_stages sem a
#   Supercopa da C1 (o campeao da C2 nao vai a ela), com a lista solta antes do gabarito (como a C1/A), conferido
#   instrucao a instrucao: malloc 0x944e46 (push 0x10, cdecl; caixa 0x276), a virtual 0x3c (push 0; push &extra; push
#   &rodadas; push 0 -> thiscall, ret 0x10), operator new 0x944cf1 (0xb2, cdecl), Cup_Stage::Cup_Stage 0x522e00
#   (thiscall, ret 0x3c, 15 args: liga, +0xbf com sinal, lista, rodadas, +4, gabarito, ano, 0/1, 1, extra, 0x14, 0, 0,
#   0, &posicoes), Cup::set_up_round 0x51c800 (push 0, thiscall, ret 4), free 0x9452ca (cdecl); caixas 0x28a e 0x2ad;
#   +0x30 = 1. create_serie_c2_playoff_stages (aqui; o cx11_8 usa). Prologo push -1; push imm32 (7 bytes).
# Defeito do original (so sem memoria): depois da caixa da fase 0 (linha 0x28a) a lista dos 4 clubes e o gabarito nao
# sao liberados (como na C1).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: o avanco lado a lado no +0x30 quando nao cria fases, senao so a
# original (ita_ser_c1a_compare_advance_stage); as fases so a original, com as duas fases criadas e +0x30 = 1.
# Depende do cx11_5 so na carreira (o calendario das fases 0 e 1 pela virtual); compila sozinho.
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2a_advance_stage', 0x653e00, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('ita_ser_c2a_create_play_off_stages', 0x653e90, [0x6a, 0xff, 0x68, 0x2c, 0xbc, 0x95, 0x00])]
DOCS = ('o avanço de fase e as fases do fim da liga da Série C2/A italiana', 'the Italian Serie C2/A stage advance and end-of-league stages',
        [('ita_ser_c2a_advance_stage', 'ItaSerC2a::advance_stage', '0x653e00', 'ita_ser_c2a.cpp', 'thiscall, ret 0',
          'depois da liga a fase atual sobe; na primeira, cria a repescagem de acesso e o play-out; depois da última, a caixa de erro',
          'after the league the current stage goes up; on the first, the promotion play-off and the play-out are created; past the last, the error box'),
         ('ita_ser_c2a_create_play_off_stages', 'ItaSerC2a::create_play_off_stages', '0x653e90', 'ita_ser_c2a.cpp', 'thiscall, ret 0',
          'fim da liga na C2/A: o 2º contra o 5º e o 3º contra o 4º jogam a repescagem de acesso; o 14º contra o 17º e o 15º contra o 16º jogam o play-out do rebaixamento; cada um é uma fase de copa com o calendário da própria liga e o sorteio da primeira rodada (diferente da C1, o campeão não vai à Supercopa). Defeito do original: sem memória para a fase da repescagem, a lista dos clubes e o calendário não são liberados',
          'end of the C2/A league: 2nd v 5th and 3rd v 4th play the promotion play-off; 14th v 17th and 15th v 16th play the relegation play-out; each is a cup stage with the league\'s own schedule and the first-round draw (unlike C1, the champion does not go to the Super Cup). Defect of the original: out of memory for the play-off stage, the club list and the schedule are not freed')])
CAMPOS = []
INTERESSA = []
