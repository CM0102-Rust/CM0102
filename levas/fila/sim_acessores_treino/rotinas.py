ROTINAS = [('lista_de_observados', 0x5d5ab0, [0x8b, 0x81, 0x56, 0x08, 0x00, 0x00]),
           ('entrada_do_treino', 0x5d5ac0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x89, 0x1d, 0x65, 0x00, 0x00]),
           ('gravar_carga_de_treino', 0x62c660, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x00])]
DOCS = ('a lista de observados, a entrada do treino e a carga de treino',
        'the shortlist manager, the training entry and the training load',
        [('lista_de_observados', 'transfer_manager_get_shortlist_manager', '0x5d5ab0', 'game.cpp', 'thiscall',
          'o gerente da lista de observados guardado no gerente de transferências',
          'the shortlist manager kept in the transfer manager'),
         ('entrada_do_treino', 'training_manager_get_player_entry', '0x5d5ac0', 'game.cpp', 'thiscall',
          'a entrada de 6 bytes do jogador na tabela do treino',
          'the player\'s 6-byte entry in the training table'),
         ('gravar_carga_de_treino', 'Injury::set_training_load', '0x62c660', 'injury.cpp', 'thiscall',
          'grava a carga de treino do jogador no registro de condição',
          'stores the player\'s training load in the fitness record')])
