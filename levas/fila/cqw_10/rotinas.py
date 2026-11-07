# Os construtores dos torneios de seleções: a Copa Africana de Nações, a Copa América e a Copa das Nações da Oceania
# (african_nations.cpp, goldcup.cpp, oceania_nations.cpp, em intercomp\). Frente das competições (Agente E, 07/11/2026,
# madrugada), carreira de 8 ligas: o construtor escolhe a edição (a Copa América de julho de 2001, a Copa Africana de
# janeiro de 2002, o sorteio da Copa da Oceania de março de 2002); cada rotina tem uma mutação no ano da edição.
# - 0x401000 (AfricanNations::AfricanNations), 0x5e0660 (GoldCup::GoldCup), 0x7988b0 (OceaniaNations::OceaniaNations).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): os construtores são thiscall, ret 8 (push comp; push ano; mov ecx, novo; call,
# na montagem das competições). Dentro: o construtor da base (mov ecx -> thiscall, ret 0), o carregamento (push 1; mov
# ecx -> thiscall, ret 4), 0x944e46, 0x944cf1 e 0x945501 (pushes; add esp -> cdecl), 0x49ee70 (push this; mov ecx,
# novo -> thiscall, ret 4) e as rotinas da própria copa (mov ecx -> thiscall).
# Comparadores: só a original (alocam as fases e as estatísticas), com o plano: o this, a vtable, o registro +4, o ano
# da edição e +0x51.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas: comparação de 8 ligas até 2002-06-01 (os 12 itens do agente E) 0 div; Rust x2 iguais e detector igual (ab5806e63c96b235); mutações 3/3.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqw10_africana', 0x401000, [0x6a, 0xff, 0x68, 0x99, 0x34, 0x95, 0x00]),
           ('cqw10_copa_america', 0x5e0660, [0x6a, 0xff, 0x68, 0x29, 0xa8, 0x95, 0x00]),
           ('cqw10_oceania_nacoes', 0x7988b0, [0x6a, 0xff, 0x68, 0x79, 0xeb, 0x95, 0x00])]
DOCS = ('os construtores dos torneios de seleções: a Copa Africana de Nações, a Copa América e a Copa das Nações da Oceania',
        'the constructors of the national team tournaments: the African Nations Cup, the Copa América and the Oceania Nations Cup',
        [('cqw10_africana', 'AfricanNations::AfricanNations', '0x401000', 'african_nations.cpp', 'thiscall, ret 8', 'regra: a Copa Africana de Nações é de dois em dois anos, nos anos pares (1996, 1998...); a edição do jogo é a do ano seguinte ao do jogo levada ao ano par (num jogo de 2001, a de 2002); 4 fases, sem nada salvo as regras e as estatísticas', "rule: the African Nations Cup is every two years, in even years (1996, 1998...); the game's edition is the year after the game's, moved to an even year (in a 2001 game, 2002); 4 stages, with nothing saved the rules and statistics"),
         ('cqw10_copa_america', 'GoldCup::GoldCup', '0x5e0660', 'goldcup.cpp', 'thiscall, ret 8', 'regra: a Copa América é de dois em dois anos, nos anos ímpares; a edição é a do ano do jogo levada ao ano ímpar (num jogo de 2001, a de 2001, em julho); 3 fases', "rule: the Copa América is every two years, in odd years; the edition is the game's year moved to an odd year (in a 2001 game, 2001, in July); 3 stages"),
         ('cqw10_oceania_nacoes', 'OceaniaNations::OceaniaNations', '0x7988b0', 'oceania_nations.cpp', 'thiscall, ret 8', 'regra: a Copa das Nações da Oceania é de dois em dois anos, nos anos pares, a partir de 2002; 2 fases', 'rule: the Oceania Nations Cup is every two years, in even years, from 2002; 2 stages')])
CAMPOS = [('competição +0x40 ano da edição (word), +4 registro, +0xc/+0x2c fases, +8 estatísticas, +0x51, +0xd9, +0x45, +0x44, +0x30; valor/ponteiro; grava', 'competition +0x40 edition year (word), +4 record, +0xc/+0x2c stages, +8 statistics, +0x51, +0xd9, +0x45, +0x44, +0x30; value/pointer; writes')]
INTERESSA = ['CMRTE']
