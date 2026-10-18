ROTINAS = [('zagueiros_do_lado_sem_conflito', 0x6d5b20, [0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0]),
           ('construir_arbitro', 0x6e9460, [0x8b, 0xc1, 0xc7, 0x00, 0x00, 0x00, 0x00, 0x00]),
           ('destruir_jogador_da_partida', 0x6ebd60, [0xc7, 0x81, 0x9e, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o fechamento do motor (2): os zagueiros do lado, o árbitro e o jogador da partida',
        'closing the engine (2): the side\'s centre-backs, the referee and the match player',
        [('zagueiros_do_lado_sem_conflito', 'side_centre_backs_without_conflict', '0x6d5b20', 'match_eng.cpp', 'thiscall',
          'a 0x5b8dc0 na formação do lado', '0x5b8dc0 on the side\'s formation'),
         ('construir_arbitro', 'construct_referee', '0x6e9460', 'match_official.cpp', 'thiscall',
          'o construtor do árbitro (zera +0 e +4)', 'the referee constructor (zeroes +0 and +4)'),
         ('destruir_jogador_da_partida', 'destroy_match_player', '0x6ebd60', 'match_pl.cpp', 'thiscall',
          'o destrutor do jogador da partida (zera +0x19e)', 'the match player destructor (zeroes +0x19e)')])
