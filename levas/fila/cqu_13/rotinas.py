# As copas entre continentes: os dois clubes da Copa Intercontinental e da Copa Interamericana; e as posições previstas
# da Merconorte (world_club_cup.cpp, inter_amer_cup.cpp, con_merc_cup.cpp, em eurocomp\). Frente das competições (Agente
# E, 06/11/2026, noite), carreira de 8 ligas (a Europa, a América do Norte e a do Sul com ligas carregadas: as copas
# tomam os campeões continentais; a Intercontinental joga em dezembro de 2001, a Interamericana em janeiro de 2002).
# - 0x92b8b0 (WorldClubCup::pick_qualifiers): o europeu e o sul-americano. Uma mutação (o vice europeu).
# - 0x632480 (InterAmerCup::select_teams): o norte-americano e o sul-americano. Duas mutações (o vice da CONCACAF; o
#   vice da Libertadores).
# - 0x4c2a90 (ConMercCup::set_predicted_positions): as posições previstas da Merconorte. Passageira.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são thiscall, ret 0 (mov ecx; call: do construtor; a última pela
# virtual +0x5c). Dentro: 0x944e46 (push 0xc; add esp, 4 -> cdecl), 0x6699d0 e 0x53b900 (push; add esp, 4 -> cdecl),
# 0x4afd80 (2 pushes; add esp, 8 -> cdecl), 0x4afbd0 (3 pushes; add esp, 0xc -> cdecl), 0x945333 (4 pushes; add esp
# -> cdecl), a virtual +0x3c (push 0; push; push; push fase; mov ecx -> thiscall, ret 0x10), 0x4a2540 (push posição;
# push clube; mov ecx, [+8] -> thiscall, ret 8) e 0x9452ca.
# Um defeito da original reproduzido: sem clube para sortear (continente sem liga carregada fora de 2001), o campeão de
# reserva vai sempre para a vaga 1 (+6), mesmo na busca da vaga 0 (que fica com o que a memória alocada tinha).
# Comparadores: só a original, com o plano: os dois clubes nos seus continentes e os bytes de estado zerados; as
# previstas conferem cada posição gravada (a tabela das rodadas pedida de novo e liberada).
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas: comparação de 8 ligas até 2002-06-01 (os 12 itens do agente E) 0 div; Rust x2 iguais e detector igual (ab5806e63c96b235); mutações 3/3.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('world_club_cup_pick_qualifiers', 0x92b8b0, [0xb8, 0xa4, 0x11, 0x00, 0x00]),
           ('inter_amer_cup_select_teams', 0x632480, [0xb8, 0xa4, 0x11, 0x00, 0x00]),
           ('con_merc_cup_set_predicted_positions', 0x4c2a90, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00])]
DOCS = ('as copas entre continentes: os dois clubes da Copa Intercontinental e da Copa Interamericana; as posições previstas da Merconorte',
        'the inter-continental cups: the two clubs of the Intercontinental Cup and of the Interamerican Cup; the Merconorte predicted positions',
        [('world_club_cup_pick_qualifiers', 'WorldClubCup::pick_qualifiers', '0x92b8b0', 'world_club_cup.cpp', 'thiscall, ret 0', 'regra: a Copa Intercontinental é entre o campeão da Liga dos Campeões e o da Libertadores (com as ligas do continente carregadas ou em 2001); sem liga carregada, um clube do continente com reputação acima de 5.000 é sorteado; se o campeão da Libertadores e o vice são sul-americanos, vai o vice [inferência: um defeito, que na base de 2001 não muda nada porque o vice de 2001 é o mexicano Cruz Azul]', 'rule: the Intercontinental Cup is between the Champions League winner and the Libertadores winner (with the continent leagues loaded or in 2001); without a loaded league, a club of the continent with reputation above 5,000 is drawn; if both the Libertadores winner and runner-up are South American, the runner-up goes [inference: a defect, which changes nothing in the 2001 database because the 2001 runner-up is Mexican Cruz Azul]'),
         ('inter_amer_cup_select_teams', 'InterAmerCup::select_teams', '0x632480', 'inter_amer_cup.cpp', 'thiscall, ret 0', 'regra: a Copa Interamericana é entre o campeão da Copa dos Campeões da CONCACAF e o da Libertadores (com as ligas carregadas ou em 2001); sem liga carregada, sorteia um clube norte-americano de reputação acima de 1.000 ou sul-americano acima de 5.000', 'rule: the Interamerican Cup is between the CONCACAF Champions Cup winner and the Libertadores winner (with the leagues loaded or in 2001); without a loaded league, it draws a North American club above reputation 1,000 or a South American one above 5,000'),
         ('con_merc_cup_set_predicted_positions', 'ConMercCup::set_predicted_positions', '0x4c2a90', 'con_merc_cup.cpp', 'thiscall, ret 0', 'regra: as posições previstas da Merconorte: os convidados pela reputação; os que saem em cada rodada do mata-mata (da final para trás) ganham a posição corrente (o campeão e o vice a 1 [inferência: a última rodada conta duas vezes]); os que ficam nos grupos, em blocos de 4', 'rule: the Merconorte predicted positions: the invited clubs by reputation; those leaving in each knockout round (from the final backwards) get the running position (winner and runner-up 1 [inference: the last round counts twice]); those staying in the groups, in blocks of 4')])
CAMPOS = [('copa +0x3e times (2), +0xa7 tabela de 2 times de 6 bytes (+0 clube, +4/+5 estado); valor/ponteiro; grava (clubes)', 'cup +0x3e teams (2), +0xa7 table of 2 six-byte teams (+0 club, +4/+5 state); value/pointer; writes (clubs)'),
          ('copa +0x40 ano; [0xae23a4] + 0xc6 × continente: o registro do continente; valor/ponteiro; lê', 'cup +0x40 year; [0xae23a4] + 0xc6 × continent: the continent record; value/pointer; reads'),
          ('copa +0xee a lista dos convidados, +0xf2 a conta; ponteiro/valor; ordena e lê (previstas)', 'cup +0xee the invited list, +0xf2 its count; pointer/value; sorts and reads (predicted)'),
          ('registro de estatística +0x72: a posição prevista do clube; valor; grava (previstas)', 'stats record +0x72: the club predicted position; value; writes (predicted)')]
INTERESSA = ['CMRTE']
