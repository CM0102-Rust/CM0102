ROTINAS = [('hora_das_ferias', 0x5fdc30, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('escalacao_guardada', 0x5fed50, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tecnico_da_busca', 0x7ae570, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o registro do técnico humano (hora das férias e escalação guardada) e o técnico da busca',
        'the human manager record (holiday stamp and saved line-up) and the search manager',
        [('hora_das_ferias', 'HUMAN_MANAGER::get_holiday_stamp', '0x5fdc30', 'human_manager.cpp', 'thiscall',
          'a hora guardada quando o técnico humano saiu de férias',
          'the clock stamp stored when the human manager went on holiday'),
         ('escalacao_guardada', 'HUMAN_MANAGER::get_saved_lineup', '0x5fed50', 'human_manager.cpp', 'thiscall',
          'o endereço da escalação guardada do técnico humano (16 pares)',
          'the address of the human manager\'s saved line-up (16 pairs)'),
         ('tecnico_da_busca', 'PLAYER_SEARCH::set_manager', '0x7ae570', 'player_search.cpp', 'thiscall',
          'grava na busca de jogadores o id do técnico humano',
          'stores the human manager id in the player search')])
