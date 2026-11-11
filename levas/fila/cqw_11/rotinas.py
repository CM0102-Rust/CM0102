# A eliminatória da Oceania para a Copa do Mundo: as dez seleções e a final entre os campeões dos grupos; e o segundo
# grupo da Copa das Nações da Oceania (wc_oceania_league.cpp, oceania_nations.cpp). Frente das competições (Agente E,
# 07/11/2026, madrugada), carreira de 8 ligas (a Austrália jogável; o jogo de 2001 monta a eliminatória no começo).
# - 0x91fa80 (WcOceaniaLeague::build_entrant_pool): as dez. Duas mutações.
# - 0x91f640 (WcOceaniaLeague::create_final_stage): a final. Uma mutação.
# - 0x799a60 (OceaniaNations::create_group_stage): o segundo grupo da Copa da Oceania (os jogos são depois de junho).
#   Passageira.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): 0x91fa80 e 0x91f640 thiscall, ret 0; 0x799a60 thiscall, ret 4 (push 0; mov
# ecx; call, do dia). Dentro: 0x5fa730 (5 pushes; mov ecx, [0xb5e134] -> thiscall, ret 0x14), 0x4afd80 (2 pushes ->
# cdecl), 0x944e46, 0x944cf1, 0x9452ca e 0x53b3d0 (push; add esp, 4 -> cdecl), 0x945333 (4 pushes -> cdecl), 0x6826d0
# (push i; mov ecx -> thiscall, ret 4), a virtual +0x3c (4 pushes, ret 0x10), 0x522e00 (15 pushes, ret 0x3c), 0x51c800
# (push 0; mov ecx -> thiscall, ret 4), 0x68b100 (22 pushes, ret 0x58) e 0x684230.
# Comparadores: só a original, com o plano: as dez da Oceania sem as duas de fora, sem repetição e pela reputação; a
# final com os dois campeões na ordem; o grupo com as seleções 3 a 5.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas: comparação de 8 ligas até 2002-06-01 (os 12 itens do agente E) 0 div; Rust x2 iguais e detector igual (ab5806e63c96b235); mutações 3/3.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('wc_oceania_league_build_entrant_pool', 0x91fa80, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('wc_oceania_league_create_final_stage', 0x91f640, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('oceania_nations_create_group_stage', 0x799a60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a eliminatória da Oceania: as dez seleções e a final entre os campeões dos grupos; o segundo grupo da Copa das Nações da Oceania',
        'the Oceania qualifying: the ten nations and the final between the group winners; the second Oceania Nations Cup group',
        [('wc_oceania_league_build_entrant_pool', 'WcOceaniaLeague::build_entrant_pool', '0x91fa80', 'wc_oceania_league.cpp', 'thiscall, ret 0', 'regra: a eliminatória da Oceania tem dez vagas menos as isentas (as sedes da Copa do Mundo seguinte e o último campeão, se forem da Oceania); entram as seleções da Oceania pela ordem do banco, fora a Nova Caledônia e a Papua-Nova Guiné, e a lista é ordenada pela reputação (os grupos saem das vagas pares e ímpares); sem completar as vagas, nada se ordena', 'rule: the Oceania qualifying has ten places minus the exempt ones (the next World Cup hosts and the last winner, if from Oceania); the Oceania nations enter in database order, except New Caledonia and Papua New Guinea, and the list is sorted by reputation (the groups come from the even and odd places); short of places, nothing is sorted'),
         ('wc_oceania_league_create_final_stage', 'WcOceaniaLeague::create_final_stage', '0x91f640', 'wc_oceania_league.cpp', 'thiscall, ret 0', 'regra: a final da Oceania é um mata-mata em ida e volta [inferência: o 4] entre o campeão do primeiro grupo e o do segundo, nessa ordem; a primeira rodada é montada na hora', 'rule: the Oceania final is a two-legged [inference: the 4] knockout between the first group winner and the second, in that order; the first round is set up at once'),
         ('oceania_nations_create_group_stage', 'OceaniaNations::create_group_stage', '0x799a60', 'oceania_nations.cpp', 'thiscall, ret 4', 'regra: o segundo grupo da Copa das Nações da Oceania é uma fase de liga com as seleções das vagas 3, 4 e 5; os técnicos humanos recebem a notícia', 'rule: the second Oceania Nations Cup group is a league stage with the nations of places 3, 4 and 5; human managers get the news')])
CAMPOS = [('eliminatória +0xee isentas (byte), +0xef a lista, +0xf3 a conta, +0xa7 (word) = −1; valor/ponteiro; grava (seleções)', 'qualifying +0xee exempt count (byte), +0xef the list, +0xf3 its count, +0xa7 (word) = −1; value/pointer; writes (nations)'),
          ('[0xb5e134] as sedes (HostCountry); nação +0x71 continente; ponteiro; lê', '[0xb5e134] the hosts (HostCountry); nation +0x71 continent; pointer; reads'),
          ('eliminatória +0xc fases ([1] a final), +0xb1 tabela; ponteiro; grava e lê (final)', 'qualifying +0xc stages ([1] the final), +0xb1 table; pointer; writes and reads (final)'),
          ('copa +0xee a lista das 6, +0xc fases, +0x30 fase atual; ponteiro/valor; lê e grava (segundo grupo)', 'cup +0xee the 6-nation list, +0xc stages, +0x30 current stage; pointer/value; reads and writes (second group)')]
INTERESSA = ['CMRTE']
