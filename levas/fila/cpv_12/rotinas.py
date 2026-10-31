# A convocação (national_teams.cpp) e o amistoso (friendly.cpp): national_teams_can_change_squad (0x76db00, 977
# chamadas na cobertura de 17/10, todas pela convocação 0x76c120, sem entrar e sem texto), national_teams_player_in_squad
# (0x76da80) e Friendly::schedule_assign_opponent (0x5c57d0), essas duas passageiras (não rodam na carreira automática).
# A 0x76db00 é thiscall (o mapa diz stdcall, mas o ecx é guardado e lido: o registro [this+4] e a lista [this]); o
# quadro SEH só protege a lista local da busca de jogos do dia (0x5acee0, 18 argumentos), construída e destruída como
# a original (0x68d020/0x68d040). A marca do tipo vai num dword da local com só o byte de baixo gravado (a 0x53c590 lê
# só o byte). Iguais no cm0102-1080.exe. Comparadores: a 0x76db00 lado a lado pela devolução e pelo motivo (destino
# próprio para a cópia); a 0x76da80 pura; a 0x5c57d0 lado a lado pelas vagas da agenda.
SAVE = False
ROTINAS = [('pode_mudar_a_convocacao', 0x76db00, [0x6a, 0xff, 0x68, 0xdb, 0xdd, 0x95, 0x00]),
           ('esta_na_convocacao', 0x76da80, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('marcar_adversario_do_amistoso', 0x5c57d0, [0x8b, 0x54, 0x24, 0x04, 0x53])]
DOCS = ('a convocação pode mudar agora?, o jogador está na convocação e o adversário marcado no amistoso',
        'can the squad change now?, is the player in the squad and the opponent booked for the friendly',
        [('pode_mudar_a_convocacao', 'national_teams_can_change_squad', '0x76db00', 'national_teams.cpp', 'thiscall, ret 0x18',
          'diz se o jogador pode entrar ou sair da convocação agora (aposentado, de outra nação, elenco cheio, machucado, janela do tipo, data e jogo do dia, técnico humano) e escreve o motivo',
          'tells whether the player may join or leave the squad now (retired, other nation, full squad, injured, squad window, date and match day, human manager) and writes the reason'),
         ('esta_na_convocacao', 'national_teams_player_in_squad', '0x76da80', 'national_teams.cpp', 'stdcall, ret 0xc',
          'o jogador está no time do tipo (principal, segundo ou sub-21) com a marca do tipo?',
          'is the player in the squad of the type (senior, second or under-21) with the type flag?'),
         ('marcar_adversario_do_amistoso', 'Friendly::schedule_assign_opponent', '0x5c57d0', 'friendly.cpp', 'thiscall, ret 8',
          'grava o adversário na primeira vaga vazia da agenda de amistosos do clube e marca o pedido',
          "stores the opponent in the first empty slot of the club's friendly schedule and marks the request")])
CAMPOS = [('staff +0x1a: a nação do jogador; ponteiro; lê (pode_mudar_a_convocacao)',
           "staff +0x1a: the player's nation; pointer; reads (pode_mudar_a_convocacao)"),
          ('staff +0x24: o clube da seleção; ponteiro; lê (pode_mudar_a_convocacao, esta_na_convocacao)',
           'staff +0x24: the national side club; pointer; reads (pode_mudar_a_convocacao, esta_na_convocacao)'),
          ('staff +0x5e: as marcas de elenco (4 principal e sub-21, 8 o segundo); valor; lê (esta_na_convocacao)',
           'staff +0x5e: the squad flags (4 senior and under-21, 8 the second team); value; reads (esta_na_convocacao)'),
          ('registro da nação nas seleções ([NationalTeams+4] + 0x6e × id) + 20 × tipo: +0x3a a data do tipo, +0x42 o teto do elenco, +0x44 a janela, +0x45 as vagas abertas; valor; lê (pode_mudar_a_convocacao)',
           'nation record in the national teams ([NationalTeams+4] + 0x6e × id) + 20 × type: +0x3a the squad date, +0x42 the squad cap, +0x44 the window, +0x45 the open slots; value; reads (pode_mudar_a_convocacao)'),
          ('[NationalTeams+0] a lista dos times com data; ponteiro; lê e anda o cursor +0xa (pode_mudar_a_convocacao)',
           '[NationalTeams+0] the list of teams with a date; pointer; reads and moves the cursor +0xa (pode_mudar_a_convocacao)'),
          ('agenda de amistosos +8: as vagas (byte); +0x19 + 25·i o adversário; +0x21 + 25·i a marca do pedido; ponteiro e valor; lê e grava (marcar_adversario_do_amistoso)',
           'friendly schedule +8: the slots (byte); +0x19 + 25·i the opponent; +0x21 + 25·i the request flag; pointer and value; reads and writes (marcar_adversario_do_amistoso)')]
INTERESSA = ['CMRTE']
