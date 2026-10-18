MOTOR = True
ROTINAS = [('proximo_evento', 0x6d78b0, [0x51, 0x8b, 0xd1, 0x55, 0x56]),
           ('nome_do_jogador_na_narracao', 0x6db2d0, [0x8a, 0x44, 0x24, 0x18, 0x81, 0xec, 0x58, 0x02, 0x00, 0x00]),
           ('habilidade_vista', 0x6e86d0, [0x51, 0x8b, 0x15, 0x20, 0x71, 0xdd, 0x00])]
DOCS = ('o fechamento do motor (6): a leitura da narração, o nome do jogador e a habilidade vista',
        'closing the engine (6): reading the commentary, the player name and the perceived ability',
        [('proximo_evento', 'next_event', '0x6d78b0', 'match_events.cpp', 'thiscall',
          'o próximo evento da narração a mostrar, parte por parte do texto (separadas por |), avançando a leitura da fila',
          'the next commentary event to show, part by part of the text (split by |), advancing the queue reading'),
         ('nome_do_jogador_na_narracao', 'commentary_player_name', '0x6db2d0', 'match_events.cpp', 'cdecl',
          'o nome do jogador na narração: a forma curta, ou a longa quando outro jogador dos dois clubes tem o mesmo nome curto',
          'the player name in the commentary: the short form, or the long one when another player of either club has the same short name'),
         ('habilidade_vista', 'perceived_ability', '0x6e86d0', 'match_man.cpp', 'cdecl',
          'a habilidade atual ou potencial do jogador como um observador a vê (reputação, ruído e conhecimento do observador)',
          'a player\'s current or potential ability as an observer sees it (reputation, noise and the observer\'s knowledge)')])
