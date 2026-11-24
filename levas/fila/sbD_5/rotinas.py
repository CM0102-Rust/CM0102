# O calendario da Terceira Divisao belga (bel_third.cpp; leitor sbD, rodada 5, 24/11/2026; as sobras do cx09).
# Simulacao com a Belgica ('ENG BEL HOL GER DEN', ate 2002-08-01, como os irmaos cx09_8 a cx09_13): o virtual +0x3c dos
# dois grupos (fases -1 e 0) roda na criacao do jogo e na virada; as fases 7, 8 e 9 (a final e as repescagens) em
# abril/maio de 2002, se a liga chegar a elas.
# - 0x421a00 BelThird::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10: fases abaixo de 1 (com
#   sinal): e_liga = 1, 30 rodadas, extra 0x41f (fase -1) ou 0x420; malloc(0x79e) (caixa 0xb8) e as 30 rodadas pelo
#   League::set_round_date 0x68a160 e set_round_match_slot 0x68a1c0 (cdecl, ano = a palavra +0x40); fases 7, 8 e 9:
#   e_liga = 0, 1/3/3 rodadas, extra 0/0x458/0x452, malloc de 0x68 por rodada (caixas 0x14a, 0x172, 0x1c0), as rodadas
#   pelo Cup::set_round_draw_date 0x521e60 e set_round_match_date 0x521eb0 (cdecl, ano com sinal) e os campos da
#   rodada; outras fases: 0. A tabela das 30 rodadas tirada pela emulacao (scratchpad cx14\tabela_emulada.py) e
#   conferida contra a desmontagem; as copas lidas da desmontagem e conferidas pela emulacao (draw/match).
#   O tamanho do mapa (1582 bytes) e so o primeiro pedaco: a rotina vai ate 0x4224d1 (fases 7, 8 e 9).
#   Prologo mov al, [esp+4]; sub esp, 0x200 (10 bytes). Igual no cm0102-1080.exe.
# Comparador: compare_stage_schedule (lado a lado do mesmo [0xb67a34], as saidas e os bytes gravados das duas tabelas:
# as rodadas de liga nas fases abaixo de 1, as de copa nas 7 a 9; a tabela da copia liberada).
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_third_create_schedule', 0x421a00, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Terceira Divisão belga', 'the Belgian Third Division schedule',
        [('bel_third_create_schedule', 'BelThird::create_schedule', '0x421a00', 'bel_third.cpp', 'thiscall, ret 0x10',
          'o calendário da Terceira Divisão belga: os dois grupos (fases -1 e 0) com 30 rodadas, de 3 de setembro a 17 de dezembro e de 7 de janeiro a 6 de maio, sem rodada em 5 de novembro, 11 de fevereiro, 11 de março e 1º de abril; a fase 7 (uma rodada de ida e volta em maio) e as duas repescagens (fases 8 e 9, três rodadas em maio cada); as outras fases não têm calendário',
          'the Belgian Third Division schedule: the two groups (stages -1 and 0) with 30 rounds, from 3 September to 17 December and from 7 January to 6 May, with no round on 5 November, 11 February, 11 March and 1 April; stage 7 (one two-legged round in May) and the two promotion play-offs (stages 8 and 9, three rounds in May each); the other stages have no schedule')])
CAMPOS = []
INTERESSA = []
