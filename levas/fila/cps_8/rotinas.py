# O jogo com jogador observado (SCOUT_MANAGER::fixture_has_scouted_player 0x7f9720, pela partida_sem_humano 0x6b66a0
# já em Rust: decide se a partida pode ser simulada sem o humano), a lista do olheiro do humano
# (SCOUT_MANAGER::is_player_shortlisted 0x7f7490, pelos clubes interessados 0x842180, pelo estatuto 0x849d60 e pela
# 0x8cde70, já em Rust, e por telas) e o registro da primeira convocação (Staff_Records::add_first_cap_record 0x86cc50,
# pelo resultado internacional 0x76b600). A 0x86cc50 tem SEH só da data local (destrutor vazio 0x5a53c0) e usa os
# ajudantes do histórico da 253ª (cpq4_*). Chamadas originais pelo endereço: 0x68d1b0 (LLIST), 0x8c7560, 0x55d270,
# 0x7757d0, 0x8cf5e0, 0x804540, 0x7f6cd0, 0x7af600; o resto já em Rust. Iguais no cm0102-1080.exe. Trocas: das
# candidatas da família, a scout_match 0x7f8c00 só é chamada pela fechar_partida_do_dia (SO_NA_COMPARACAO); a
# add_scout_request 0x7f9290 e a record_scout_report 0x7f9520 só rodam pelas buscas do humano (telas); as de notícia
# (0x7f9890, 0x7f9a60) ficam de fora. Simulação: as mutações pela carreira até 2002-06-01. Comparadores: as duas
# consultas pela devolução (a original e a cópia; o cursor da lista vigiada é reposto no começo), sem os casos de erro;
# o registro só a original (o histórico vai para o arquivo).
SAVE = False
ROTINAS = [('jogo_com_observado', 0x7f9720, [0x83, 0xec, 0x0c, 0x53, 0x8b, 0x5c, 0x24, 0x14]),
           ('na_lista_do_olheiro', 0x7f7490, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('registrar_a_primeira_convocacao', 0x86cc50, [0x6a, 0xff, 0x68, 0x3b, 0x2d, 0x96, 0x00])]
DOCS = ('o jogo com jogador observado, a lista do olheiro do humano e o registro da primeira convocação',
        'the fixture with a scouted player, the human scout list and the first cap record',
        [('jogo_com_observado', 'SCOUT_MANAGER::fixture_has_scouted_player', '0x7f9720', 'scout_manager.cpp', 'thiscall, ret 4',
          'diz se a partida tem clube da lista vigiada ou jogador com pedido de observação disponível (no clube ou na seleção, não cedido, não lesionado, não suspenso, não impedido e que ainda não jogou)',
          'tells whether the fixture has a watched club or an available player with a scout request (at the club or national team, not loaned out, injured, suspended or barred, and not yet played)'),
         ('na_lista_do_olheiro', 'SCOUT_MANAGER::is_player_shortlisted', '0x7f7490', 'scout_manager.cpp', 'thiscall, ret 8',
          'diz se o jogador está na lista do olheiro do humano (pela edição aberta ou pela primeira busca de tipo 1)',
          'tells whether the player is on the human scout shortlist (through the open edit session or the first type 1 search)'),
         ('registrar_a_primeira_convocacao', 'Staff_Records::add_first_cap_record', '0x86cc50', 'staff_records.cpp', 'thiscall, ret 8',
          'o histórico da primeira convocação da pessoa (tipo 6: o clube, o adversário e a idade)',
          'the first cap history of the person (type 6: the club, the opponent and the age)')])
CAMPOS = [('partida +0, +0xc, +0x10, +0x14, +0x1c, +0x20: competição, dois ids comparados à lista vigiada, competição para a suspensão e os dois clubes; id e ponteiro; lê (jogo_com_observado)',
           'fixture +0, +0xc, +0x10, +0x14, +0x1c, +0x20: competition, two ids checked against the watched list, competition for the ban and the two clubs; id and pointer; reads (jogo_com_observado)'),
          ('[0xb74438] lista dos clubes vigiados (LLIST estática, cursor +0xa reposto do +2); valor; lê e grava o cursor (jogo_com_observado)',
           '[0xb74438] watched club list (static LLIST, cursor +0xa reset from +2); value; reads and writes the cursor (jogo_com_observado)'),
          ('SCOUT_MANAGER +0x5c pedidos de observação (0x11 bytes, a pessoa em +4), +0x64 conta; ponteiro e valor; lê (jogo_com_observado)',
           'SCOUT_MANAGER +0x5c scout requests (0x11 bytes, person at +4), +0x64 count; pointer and value; reads (jogo_com_observado)'),
          ('SCOUT_MANAGER +0xc + vaga × 4: edição aberta do humano; ponteiro; lê (na_lista_do_olheiro)',
           'SCOUT_MANAGER +0xc + slot × 4: human open edit session; pointer; reads (na_lista_do_olheiro)'),
          ('busca do olheiro +0xa: tipo (1 = lista do olheiro); valor; lê (na_lista_do_olheiro)',
           'scout search +0xa: type (1 = shortlist); value; reads (na_lista_do_olheiro)'),
          ('[0xb74c26 + 0x300 × (i16)[0xb77c7a]] gerente do humano da vez; ponteiro; lê (na_lista_do_olheiro)',
           '[0xb74c26 + 0x300 × (i16)[0xb77c7a]] manager of the current human; pointer; reads (na_lista_do_olheiro)'),
          ('pessoa +0x39 e +0x24: clube e seleção; ponteiro; lê (jogo_com_observado, registrar_a_primeira_convocacao)',
           'person +0x39 and +0x24: club and national team; pointer; reads (jogo_com_observado, registrar_a_primeira_convocacao)'),
          ('pessoa +0x18: idade; valor; lê (registrar_a_primeira_convocacao)',
           'person +0x18: age; value; reads (registrar_a_primeira_convocacao)'),
          ('Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (registrar_a_primeira_convocacao)',
           'Staff_Records +0 and +0xc: staff history file and lists; appends (registrar_a_primeira_convocacao)')]
INTERESSA = ['CMScout']
