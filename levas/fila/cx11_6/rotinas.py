# A virada de temporada da Serie C2 italiana, grupos A e B (ita_ser_c2a.cpp, ita_ser_c2b.cpp; leitor cx11, rodada 5,
# 24/11/2026). Simulacao com a Italia e as divisoes inferiores: rodam uma vez cada na virada italiana do verao de 2002
# (a da C1/A, mwf_7, rodou na carreira do mundo ate 2002-08-01); ATE = 2002-09-30 para a temporada nova comecar (a
# primeira rodada da C2 e em 5 de setembro) e as mutacoes da virada serem acusadas. Como no cx11_2, a C2 so existe com
# a base NORMAL ou MAXIMA (BASE abaixo): a prova pede DATABASE=NORMAL na carreira.
# - 0x653c40 ItaSerC2a::start_new_season e 0x658100 ItaSerC2b::start_new_season thiscall() ret 0 (virtual 0x8): o corpo
#   do ItaSerC1a::start_new_season (conferido instrucao a instrucao; so a tabela 0x653a90 / 0x657f50, o fonte e, na
#   C2/B, as linhas das caixas 0x226, 0x22d, 0x233, 0x23a mudam; a C2/A usa as da C1/A 0x220, 0x227, 0x22d, 0x234):
#   rebuild_season e set_expectations_and_renew_officials. Prologo sub esp, 0x200 (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (libera e remonta a liga), o ano tem de subir 1
# (compare_state_league_new_season).
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2a_start_new_season', 0x653c40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_ser_c2b_start_new_season', 0x658100, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada de temporada da Série C2 italiana, grupos A e B', 'the season turnover of the Italian Serie C2 groups A and B',
        [('ita_ser_c2a_start_new_season', 'ItaSerC2a::start_new_season', '0x653c40', 'ita_ser_c2a.cpp', 'thiscall, ret 0',
          'a virada da Série C2/A: guarda a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela, os jogos e a ordem; depois as posições previstas, as expectativas das diretorias e a renovação dos árbitros',
          'the Serie C2/A turnover: the season archived in the statistics, the table, the dates and the stages cleared, the next year, and the rules, the table, the fixtures and the order rebuilt; then the predicted positions, the boards\' expectations and the referee renewal'),
         ('ita_ser_c2b_start_new_season', 'ItaSerC2b::start_new_season', '0x658100', 'ita_ser_c2b.cpp', 'thiscall, ret 0',
          'a virada da Série C2/B, como a da C2/A',
          'the Serie C2/B turnover, as C2/A\'s')])
CAMPOS = []
INTERESSA = []
