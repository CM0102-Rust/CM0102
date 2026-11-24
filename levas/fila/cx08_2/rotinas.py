# O calendario da Primeira Divisao argentina (arg_prm.cpp; leitor cx08, rodada 5, 24/11/2026). Simulacao com a Argentina
# (as ligas do cx01_8, ate 2002-08-01): roda na criacao do jogo (fase -1 pelo set_up_rules e fases 0 e 1 pelo
# create_stages) e na virada de 2002.
# - 0x404570 ArgPrm::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (virtual 0x3c): fase -1 o
#   Apertura (19 rodadas de 9/ago a 13/dez, extra 0x434, caixa 0xae), fase 0 o Clausura (19 de 31/jan a 9/mai do ano
#   seguinte, extra 0x435, caixa 0x14a, e depois a caixa 0x1c9 se +0xa9 != 19), as outras extra = 0 e 0. As datas por
#   0x68a160 e as faixas de TV por 0x68a1c0 (cdecl), malloc 0x944e46 (cdecl). Prologo mov al, [esp+4]; sub esp, 0x200
#   (10 bytes). Tabelas extraidas da desmontagem por script (as 38 datas e as 145 faixas conferidas uma a uma).
# Defeito do original: as rodadas 2 e 3 do Apertura caem em 23 e 30/set (mes 8 por 7): 9/ago, 16/ago, 23/set, 30/set,
# 6/set... (fora de ordem). A copia faz o mesmo.
# O comparador compare_staged_schedule (lado a lado, saidas iniciadas com as de quem chama) fica aqui: o cx08_7 e o
# cx08_14 o usam.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_prm_create_schedule', 0x404570, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Primeira Divisão argentina', 'the Argentine Premier Division schedule',
        [('arg_prm_create_schedule', 'ArgPrm::create_schedule', '0x404570', 'arg_prm.cpp', 'thiscall, ret 0x10',
          'monta o calendário da Primeira Divisão argentina: o Apertura (fase principal) em 19 rodadas semanais de 9 de agosto a 13 de dezembro, com três faixas de TV menos na última; o Clausura (fase 0) em 19 rodadas de 31 de janeiro a 9 de maio do ano seguinte, quatro delas no meio da semana; outra fase não tem calendário. Defeito do original: as rodadas 2 e 3 do Apertura caem em 23 e 30 de setembro, antes de 6 de setembro',
          'builds the Argentine Premier Division schedule: the Apertura (the main stage) in 19 weekly rounds from 9 August to 13 December, with three TV slots but for the last; the Clausura (stage 0) in 19 rounds from 31 January to 9 May of the next year, four of them midweek; no other stage has a schedule. Defect of the original: Apertura rounds 2 and 3 fall on 23 and 30 September, before 6 September')])
CAMPOS = []
INTERESSA = []
