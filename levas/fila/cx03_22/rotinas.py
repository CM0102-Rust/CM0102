# As fases preliminares, a fase seguinte e a chave principal da Copa da Suecia (swe_cup.cpp; leitor cx03, rodada 5,
# 24/11/2026). Competicoes da Suecia: LIGAS do grupo ENG SWE FIN NOR POR, ate 2002-08-01 (no mundo ate essa data: as
# preliminares 14 vezes, a fase seguinte e a chave principal 1 vez cada).
# - 0x88dda0 SweCup::create_preliminary_stage thiscall(fase) ret 4: n = 31 (fase par) ou 30 (impar: and 0x80000001; neg;
#   sbb; add 0x1f); malloc 4 x n (a caixa da linha 0x32b); os primeiros clubes nao vazios da reserva +0xb2 (ate +0xb6),
#   zerados nela; a virtual 0x3c (fase, &rodadas, &extra, 0; a fase como veio, dword); o Cup_Stage (0x944cf1 de 0xb2;
#   0x522e00 com 15 argumentos, thiscall, ret 0x3c: a copa, n, o vetor, rodadas, +4, a tabela, o ano, a fase, 1, extra, o
#   byte +0x51, 0, 0, 0, 0) em [+0xc][fase com sinal]; free do vetor e da tabela; sem fase a caixa 0x347; senao +0x30 =
#   fase. Sem sorteio. SEH so para o new. Prologo push -1; push imm32.
# - 0x88dfd0 SweCup::advance_stage thiscall() ret 0 (a virtual 0x28): +0x30 >= +0x2c - 1: a caixa 0x35b; senao +0x30 + 1 e,
#   em 7, create_main_draw_stage 0x88e060. Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30].
# - 0x88e060 SweCup::create_main_draw_stage thiscall() ret 0: malloc 0x100 (a caixa 0x380); a copa e as fases 0..6, cada
#   time com estado +5 = 1 no vetor (o 65o: a caixa 0x393 e fim, o vetor fica); o Cup_Stage da fase 7 (64 clubes,
#   0x14); free da tabela; sem fase a caixa 0x3a8 (o vetor fica); senao Cup::draw_round 0x51c410 (push 0) e free do vetor.
# Usa swe_cup_pool e SWE_CUP_CLUB_POOL do cx03_21 (integrar depois dele).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: so a original (alocam e sorteiam), com o plano (a fase no slot e +0x30; a fase seguinte; a chave de 64).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('swe_cup_create_preliminary_stage', 0x88dda0, [0x6a, 0xff, 0x68, 0xae, 0x30, 0x96, 0x00]),
           ('swe_cup_advance_stage', 0x88dfd0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('swe_cup_create_main_draw_stage', 0x88e060, [0x6a, 0xff, 0x68, 0xce, 0x30, 0x96, 0x00])]
DOCS = ('as fases preliminares, a fase seguinte e a chave principal da Copa da Suécia',
        "the Swedish Cup's preliminary stages, the next stage and the main draw",
        [('swe_cup_create_preliminary_stage', 'SweCup::create_preliminary_stage', '0x88dda0', 'swe_cup.cpp', 'thiscall, ret 4',
          'regra: cada uma das sete seções preliminares da Copa da Suécia é uma fase própria: os 31 clubes seguintes da reserva nas fases pares, 30 nas ímpares (as seções, depois dos 30 da própria copa), tirados da reserva; a primeira rodada não é sorteada aqui',
          "rule: each of the Swedish Cup's seven preliminary sections is a stage of its own: the next 31 clubs of the pool for even stages, 30 for odd ones (the sections, after the cup's own 30), taken out of the pool; its first round is not drawn here"),
         ('swe_cup_advance_stage', 'SweCup::advance_stage', '0x88dfd0', 'swe_cup.cpp', 'thiscall',
          'regra: a Copa da Suécia passa à fase seguinte; ao chegar à fase 7 monta a chave principal',
          'rule: the Swedish Cup moves to its next stage; on reaching stage 7 it builds the main draw'),
         ('swe_cup_create_main_draw_stage', 'SweCup::create_main_draw_stage', '0x88e060', 'swe_cup.cpp', 'thiscall',
          'regra: a chave principal da Copa da Suécia (fase 7) tem os 64 clubes ainda vivos: os classificados da primeira seção da própria copa e de cada uma das sete seções preliminares, nessa ordem; a primeira rodada é sorteada na hora',
          "rule: the Swedish Cup's main draw (stage 7) has the 64 clubs still in: those through in the cup's own first section and in each of the seven preliminary sections, in that order; its first round is drawn at once")])
CAMPOS = []
INTERESSA = []
