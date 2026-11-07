# Os construtores das copas entre continentes: a Copa dos Campeões da CONCACAF, a Copa Interamericana e a Copa
# Intercontinental (con_champ.cpp, inter_amer_cup.cpp, world_club_cup.cpp, em eurocomp\). Frente das competições
# (Agente E, 07/11/2026, madrugada), carreira de 8 ligas (a MLS jogável: a CONCACAF de agosto de 2001 e a
# Interamericana de janeiro de 2002 têm clubes dela; a Intercontinental é em dezembro de 2001); uma mutação por
# rotina (o ano da copa).
# - 0x4c0c40 (ConChamp::ConChamp), 0x632080 (InterAmerCup::InterAmerCup), 0x92b4b0 (WorldClubCup::WorldClubCup).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): os construtores são thiscall, ret 8 (push comp; push ano; mov ecx, novo; call,
# na montagem das competições). Dentro: o construtor da base (mov ecx -> thiscall, ret 0), o carregamento (push 1; mov
# ecx -> thiscall, ret 4), 0x944e46, 0x944cf1 e 0x945501 (pushes; add esp -> cdecl), 0x49ee70 (push this; mov ecx,
# novo -> thiscall, ret 4) e as rotinas da própria copa (mov ecx -> thiscall).
# Mais: a virtual +0x3c (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx -> thiscall, ret 0x10), 0x518790 e 0x5223a0
# (mov ecx -> thiscall, ret 0).
# Comparadores: só a original, com o plano: o this, a vtable, o registro +4 e o ano.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas: ver PRONTO.txt; a Interamericana (0x632080) saiu: está no cqw_5 (07/11, sessão principal).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqw12_concacaf', 0x4c0c40, [0x6a, 0xff, 0x68, 0x39, 0x69, 0x95, 0x00]),
           ('cqw12_intercontinental', 0x92b4b0, [0x6a, 0xff, 0x68, 0x39, 0x5f, 0x96, 0x00])]
DOCS = ('os construtores das copas entre continentes: a Copa dos Campeões da CONCACAF e a Copa Intercontinental',
        'the constructors of the inter-continental cups: the CONCACAF Champions Cup and the Intercontinental Cup',
        [('cqw12_concacaf', 'ConChamp::ConChamp', '0x4c0c40', 'con_champ.cpp', 'thiscall, ret 8', 'regra: a Copa dos Campeões da CONCACAF é do ano do jogo, com 8 clubes e prêmio de 2.000.000; os clubes entram depois (a tabela começa zerada)', "rule: the CONCACAF Champions Cup is of the game's year, with 8 clubs and a 2,000,000 prize; the clubs come in later (the table starts empty)"),
         ('cqw12_intercontinental', 'WorldClubCup::WorldClubCup', '0x92b4b0', 'world_club_cup.cpp', 'thiscall, ret 8', 'regra: a Copa Intercontinental é do ano do jogo, com os dois clubes escolhidos na hora (o campeão europeu e o sul-americano)', "rule: the Intercontinental Cup is of the game's year, with the two clubs picked at once (the European and the South American winners)")])
CAMPOS = [('copa +0x40 ano (word), +4 registro, +0xa3 tabela das rodadas, +0x3c/+0x3a rodadas e extra, +0xa7 times, +0x28 prêmio, +8 estatísticas; valor/ponteiro; grava', 'cup +0x40 year (word), +4 record, +0xa3 round table, +0x3c/+0x3a rounds and extra, +0xa7 teams, +0x28 prize, +8 statistics; value/pointer; writes')]
INTERESSA = ['CMRTE']
