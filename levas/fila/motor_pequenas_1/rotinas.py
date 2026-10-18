ROTINAS = [('bola_na_area', 0x6d58f0, [0x8a, 0x81, 0xa8, 0x8e, 0x00, 0x00]),
           ('partida_abandonada', 0x6d5930, [0x8a, 0x81, 0x38, 0xf6, 0x00, 0x00]),
           ('vagas_do_lado_sem_conflito', 0x6d5af0, [0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0])]
DOCS = ('o fechamento do motor (1): o gol da bola, a partida abandonada e as vagas do lado',
        'closing the engine (1): the ball\'s goal end, the abandoned match and the side\'s slots',
        [('bola_na_area', 'ball_goal_end', '0x6d58f0', 'match_eng.cpp', 'thiscall',
          '1 com a bola na área de cima, −1 na de baixo, senão 0; devolve só `al`', '1 with the ball in the top box, −1 in the bottom one, otherwise 0; returns only `al`'),
         ('partida_abandonada', 'match_abandoned', '0x6d5930', 'match_eng.cpp', 'thiscall',
          'o minuto do abandono × 11 abaixo do relógio (16 bits); devolve só `al`', 'the abandonment minute × 11 below the clock (16-bit); returns only `al`'),
         ('vagas_do_lado_sem_conflito', 'side_slots_without_conflict', '0x6d5af0', 'match_eng.cpp', 'thiscall',
          'a 0x5b8960 na formação do lado', '0x5b8960 on the side\'s formation')])
