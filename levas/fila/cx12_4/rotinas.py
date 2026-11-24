# O calendario e a virada de temporada da Serie C2/C italiana (ita_ser_c2c.cpp; leitor cx12, rodada 5, 24/11/2026).
# Simulacao com a Italia e as divisoes inferiores: o calendario na fase -1 nas regras (criacao do jogo e virada de
# 2002), nas fases 0 e 1 quando a liga acaba (o advance_stage do cx12_5, maio de 2002); a virada uma vez no verao de
# 2002. ATE = 2002-09-30 para a temporada nova comecar (a primeira rodada da C2 e em 5 de setembro). Como no cx11_2, a
# C2 so existe com a base NORMAL ou MAXIMA (BASE abaixo): a prova pede DATABASE=NORMAL na carreira.
# - 0x65b7f0 ItaSerC2c::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (virtual 0x3c): o corpo do
#   ItaSerC2a::create_schedule (2288 bytes, conferido instrucao a instrucao contra o 0x6531a0; so o fonte 0x9ea83c muda;
#   as linhas das caixas 0xab, 0x155, 0x18f sao as mesmas): create_c1_schedule. Prologo mov al, [esp+4]; sub esp,
#   0x200 (10 bytes).
# - 0x65c290 ItaSerC2c::start_new_season thiscall() ret 0 (virtual 0x8): o corpo do ItaSerC2a::start_new_season
#   (conferido instrucao a instrucao contra o 0x653c40; so a tabela 0x65c0e0 e o fonte mudam; as linhas 0x220, 0x227,
#   0x22d, 0x234 sao as mesmas): rebuild_season e set_expectations_and_renew_officials. Prologo sub esp, 0x200.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: o calendario lado a lado sobre as saidas e a tabela escrita
# (compare_c1_schedule); a virada so a original, o ano tem de subir 1 (compare_state_league_new_season).
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2c_create_schedule', 0x65b7f0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_ser_c2c_start_new_season', 0x65c290, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário e a virada de temporada da Série C2/C italiana', 'the Italian Serie C2/C schedule and season turnover',
        [('ita_ser_c2c_create_schedule', 'ItaSerC2c::create_schedule', '0x65b7f0', 'ita_ser_c2c.cpp', 'thiscall, ret 0x10',
          'o calendário da Série C2/C, o mesmo da C2/A: a liga em 34 rodadas de 5 de setembro a 7 de maio; a repescagem de acesso em 2 rodadas de ida e volta (semifinais em 21 e 28 de maio, final em 4 e 11 de junho); o play-out em 1 rodada de ida e volta nas datas das semifinais',
          'the Serie C2/C schedule, the same as C2/A: the league in 34 rounds from 5 September to 7 May; the promotion play-off in 2 two-legged rounds (semi-finals on 21 and 28 May, the final on 4 and 11 June); the play-out in 1 two-legged round on the semi-final dates'),
         ('ita_ser_c2c_start_new_season', 'ItaSerC2c::start_new_season', '0x65c290', 'ita_ser_c2c.cpp', 'thiscall, ret 0',
          'a virada da Série C2/C: guarda a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela, os jogos e a ordem; depois as posições previstas, as expectativas das diretorias e a renovação dos árbitros',
          'the Serie C2/C turnover: archives the season in the statistics, clears the table, the dates and the stages, moves to the next year and rebuilds the rules, the table, the fixtures and the order; then the predicted positions, the board expectations and the referee renewal')])
CAMPOS = []
INTERESSA = []
