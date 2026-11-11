# Os construtores das copas asiáticas de clubes: a Liga dos Campeões e a Recopa
# (asia_club_champ.cpp, asia_cup_winner.cpp, asia_super_cup.cpp, em eurocomp\). Frente das competições (Agente E,
# 07/11/2026, madrugada), carreira de 8 ligas (o Japão jogável: clubes japoneses nas três); uma mutação por rotina (o
# ano da copa).
# - 0x40aae0 (AsiaClubChamp::AsiaClubChamp), 0x40cbe0 (AsiaCupWinner::AsiaCupWinner), 0x410650
#   (AsiaSuperCup::AsiaSuperCup).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): os construtores são thiscall, ret 8 (push comp; push ano; mov ecx, novo; call,
# na montagem das competições). Dentro: o construtor da base (mov ecx -> thiscall, ret 0), o carregamento (push 1; mov
# ecx -> thiscall, ret 4), 0x944e46, 0x944cf1 e 0x945501 (pushes; add esp -> cdecl), 0x49ee70 (push this; mov ecx,
# novo -> thiscall, ret 4) e as rotinas da própria copa (mov ecx -> thiscall).
# Mais: a virtual +0x3c (push 0; push &+0x3a; push &+0x3c; push −1, ret 0x10), 0x518790 e 0x5223a0 (thiscall, ret 0) e
# 0x9452ca (push; add esp, 4 -> cdecl).
# Comparadores: só a original, com o plano: o this, a vtable, o registro +4 e o ano.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas: ver PRONTO.txt; a Supercopa da Ásia (0x410650) saiu: está no cqw_5 (07/11, sessão principal).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_asia_club_champ', 0x40aae0, [0x6a, 0xff, 0x68, 0x59, 0x37, 0x95, 0x00]),
           ('construct_asia_cup_winner', 0x40cbe0, [0x6a, 0xff, 0x68, 0x19, 0x38, 0x95, 0x00]),]
DOCS = ('os construtores das copas asiáticas de clubes: a Liga dos Campeões e a Recopa',
        "the constructors of the Asian club cups: the Club Championship and the Cup Winners' Cup",
        [('construct_asia_club_champ', 'AsiaClubChamp::AsiaClubChamp', '0x40aae0', 'asia_club_champ.cpp', 'thiscall, ret 8', 'regra: a Liga dos Campeões asiática é do ano do jogo, com 4 fases e prêmio de 4.000.000; sem nada salvo, os 32 classificados, a tabela dos times, as rodadas, as estatísticas, o mata-mata da zona oeste, a árvore dos jogos e as posições previstas', "rule: the Asian Club Championship is of the game's year, with 4 stages and a 4,000,000 prize; with nothing saved, the 32 qualifiers, the team table, the rounds, the statistics, the west zone knockout, the fixture tree and the predicted positions"),
         ('construct_asia_cup_winner', 'AsiaCupWinner::AsiaCupWinner', '0x40cbe0', 'asia_cup_winner.cpp', 'thiscall, ret 8', 'regra: a Recopa asiática é do ano do jogo, com 2 fases e prêmio de 4.000.000, montada como a Liga dos Campeões asiática', "rule: the Asian Cup Winners' Cup is of the game's year, with 2 stages and a 4,000,000 prize, built like the Asian Club Championship")])
CAMPOS = [('copa +0x40 ano (word), +4 registro, +0x2c/+0xc fases, +0xb2 classificados (liberados no fim), +0xa3 rodadas, +0x28 prêmio, +8 estatísticas; valor/ponteiro; grava', 'cup +0x40 year (word), +4 record, +0x2c/+0xc stages, +0xb2 qualifiers (freed at the end), +0xa3 rounds, +0x28 prize, +8 statistics; value/pointer; writes')]
INTERESSA = ['CMRTE']
