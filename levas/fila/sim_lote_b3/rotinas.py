ROTINAS = [('mapa_de_casas_da_formacao', 0x5b7160, [0x83, 0xec, 0x48, 0x53, 0x55]),
           ('nota_do_clube_para_o_jovem', 0x7ad220, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('partida_assistida_pelo_humano', 0x5fe290, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o mapa de casas da formação, a nota do clube para o jovem e a partida assistida pelo humano',
        'the formation position lookup, the club score for a youth player and the match watched by the human',
        [('mapa_de_casas_da_formacao', 'Formation::build_position_lookup', '0x5b7160', 'formation.cpp', 'thiscall',
          'expande as posições de cada casa da formação nas tabelas de células do campo e no espelho delas',
          "expands each formation slot's positions into the pitch cell tables and their mirrored copy"),
         ('nota_do_clube_para_o_jovem', 'player_regen_score_club_for_player', '0x7ad220', 'player_regen.cpp', 'stdcall',
          'a nota sorteada de quanto o clube serve ao jovem: reputação contra habilidade, juniores, controle humano e nação',
          'the random score of how well the club suits the youth player: reputation against ability, youth facilities, human control and nation'),
         ('partida_assistida_pelo_humano', 'HUMAN_MANAGER::is_watching_fixture', '0x5fe290', 'human_manager.cpp', 'thiscall',
          'se a partida é a que o técnico humano escolheu assistir',
          'whether the fixture is the one the human manager chose to watch')])
