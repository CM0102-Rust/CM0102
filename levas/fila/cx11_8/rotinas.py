# O avanco de fase e as fases do fim da liga da Serie C2/B italiana (ita_ser_c2b.cpp; leitor cx11, rodada 5,
# 24/11/2026). Simulacao com a Italia e as divisoes inferiores, como o cx11_7 (maio de 2002). Como no cx11_2, a C2 so
# existe com a base NORMAL ou MAXIMA (BASE abaixo): a prova pede DATABASE=NORMAL na carreira.
# - 0x6582c0 ItaSerC2b::advance_stage thiscall() ret 0 (virtual 0x28): o corpo do ItaSerC1a::advance_stage (conferido
#   instrucao a instrucao; o fonte, a linha 0x256 e o construtor das fases 0x658350):
#   ita_ser_c1a_advance_stage_cql5_advance_stage. Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x658350 ItaSerC2b::create_playoff_stages thiscall() ret 0: o corpo do ItaSerC2a::create_play_off_stages (conferido
#   instrucao a instrucao; so o fonte e as linhas 0x27b, 0x28f, 0x2b2 mudam): create_serie_c2_playoff_stages do cx11_7.
#   Prologo push -1; push imm32 (7 bytes).
# Defeito do original (so sem memoria): o mesmo do cx11_7 (a lista e o gabarito ficam soltos depois da caixa 0x28f).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: como no cx11_7.
# Depende do cx11_7 (create_serie_c2_playoff_stages, compare_serie_c2_playoff_stages): integrar depois dele.
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2b_advance_stage', 0x6582c0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('ita_ser_c2b_create_playoff_stages', 0x658350, [0x6a, 0xff, 0x68, 0xcc, 0xbc, 0x95, 0x00])]
DOCS = ('o avanço de fase e as fases do fim da liga da Série C2/B italiana', 'the Italian Serie C2/B stage advance and end-of-league stages',
        [('ita_ser_c2b_advance_stage', 'ItaSerC2b::advance_stage', '0x6582c0', 'ita_ser_c2b.cpp', 'thiscall, ret 0',
          'o mesmo da C2/A: depois da liga a fase atual sobe; na primeira, cria a repescagem e o play-out; depois da última, a caixa de erro',
          'the same as C2/A: after the league the current stage goes up; on the first, the play-off and the play-out are created; past the last, the error box'),
         ('ita_ser_c2b_create_playoff_stages', 'ItaSerC2b::create_playoff_stages', '0x658350', 'ita_ser_c2b.cpp', 'thiscall, ret 0',
          'a mesma regra da C2/A: a repescagem de acesso do 2º ao 5º e o play-out do 14º ao 17º',
          'the same rule as C2/A: the promotion play-off of 2nd to 5th and the play-out of 14th to 17th')])
CAMPOS = []
INTERESSA = []
