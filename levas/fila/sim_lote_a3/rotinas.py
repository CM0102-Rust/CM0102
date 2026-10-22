ROTINAS = [('data_do_sorteio_da_rodada', 0x521e60, [0x8b, 0x4c, 0x24, 0x14, 0x8b, 0x44, 0x24, 0x18]),
           ('data_do_jogo_da_rodada', 0x521eb0, [0x8b, 0x4c, 0x24, 0x14, 0x8b, 0x44, 0x24, 0x18]),
           ('tirar_clube_dos_amistosos', 0x5c64e0, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('as datas do sorteio e do jogo da rodada da copa e o clube que sai dos amistosos planejados',
        'the draw and match dates of the cup round and the club removed from the planned friendlies',
        [('data_do_sorteio_da_rodada', 'cup_set_round_draw_date', '0x521e60', 'Cup.cpp', 'cdecl',
          'grava a data do sorteio da rodada da copa: o dia do ano e o ano contado do ano-base',
          'stores the cup round draw date: the day number and the year offset from the base year'),
         ('data_do_jogo_da_rodada', 'cup_set_round_match_date', '0x521eb0', 'Cup.cpp', 'cdecl',
          'grava a data do jogo da rodada da copa, o horário e as marcas do mando',
          'stores the cup round match date, its match time and the venue flags'),
         ('tirar_clube_dos_amistosos', 'Friendly::remove_club_from_ai_plans', '0x5c64e0', 'friendly.cpp', 'thiscall',
          'tira o clube dos amistosos planejados pelo computador e das listas de amistosos',
          'removes the club from the planned AI friendlies and from the friendly lists')])
