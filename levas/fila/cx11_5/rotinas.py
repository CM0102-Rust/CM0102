# Os calendarios da Serie C2 italiana, grupos A e B (ita_ser_c2a.cpp, ita_ser_c2b.cpp; leitor cx11, rodada 5,
# 24/11/2026). Simulacao com a Italia e as divisoes inferiores: a fase -1 nas regras (criacao do jogo e virada de 2002),
# as fases 0 e 1 quando a liga acaba (o advance_stage do cx11_7/cx11_8, maio de 2002). Como no cx11_2, a C2 so existe
# com a base NORMAL ou MAXIMA (BASE abaixo): a prova pede DATABASE=NORMAL na carreira.
# - 0x6531a0 ItaSerC2a::create_schedule e 0x657660 ItaSerC2b::create_schedule thiscall(fase, &rodadas, &extra, &e_liga)
#   ret 0x10 (virtual 0x3c): o corpo do ItaSerC1a::create_schedule (2288 bytes, conferido instrucao a instrucao; so o
#   fonte 0x9ea654 / 0x9ea75c muda; as linhas das caixas 0xab, 0x155, 0x18f sao as da C1/A nas duas): create_c1_schedule.
#   O mapa diz "play-out de um jogo" na C2/A e "em ida e volta" na C2/B; o codigo e o mesmo nas duas (1 rodada de ida e
#   volta). Prologo mov al, [esp+4]; sub esp, 0x200 (10 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: lado a lado sobre as saidas e a tabela escrita
# (compare_c1_schedule).
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2a_create_schedule', 0x6531a0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_ser_c2b_create_schedule', 0x657660, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários da Série C2 italiana, grupos A e B', 'the Italian Serie C2 groups A and B schedules',
        [('ita_ser_c2a_create_schedule', 'ItaSerC2a::create_schedule', '0x6531a0', 'ita_ser_c2a.cpp', 'thiscall, ret 0x10',
          'o calendário da Série C2/A, o mesmo da C1: 34 rodadas de 5 de setembro a 7 de maio; a repescagem de acesso com semifinais em 21 e 28 de maio e a final em 4 e 11 de junho; o play-out em ida e volta nas datas das semifinais',
          'the Serie C2/A schedule, the same as C1\'s: 34 rounds from 5 September to 7 May; the promotion play-off with the semi-finals on 21 and 28 May and the final on 4 and 11 June; the play-out home and away on the semi-final dates'),
         ('ita_ser_c2b_create_schedule', 'ItaSerC2b::create_schedule', '0x657660', 'ita_ser_c2b.cpp', 'thiscall, ret 0x10',
          'o calendário da Série C2/B, o mesmo da C2/A',
          'the Serie C2/B schedule, the same as C2/A\'s')])
CAMPOS = []
INTERESSA = []
