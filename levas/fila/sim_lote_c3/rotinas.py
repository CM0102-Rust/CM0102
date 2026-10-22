ROTINAS = [('humano_assiste_a_partida', 0x5fe420, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('humano_deu_apelido', 0x600740, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('copiar_aposentadoria_pendente', 0x7a8780, [0x83, 0xec, 0x08, 0x56, 0x57])]
DOCS = ('a partida assistida por algum humano, o apelido dado pelo humano e a cópia da aposentadoria pendente',
        'the fixture watched by any human, the nickname given by the human and the pending retirement copy',
        [('humano_assiste_a_partida', 'HUMAN_MANAGER::is_fixture_watched', '0x5fe420', 'human_manager.cpp', 'thiscall',
          'se algum dos técnicos humanos ativos escolheu assistir a partida',
          'whether any of the active human managers chose to watch the fixture'),
         ('humano_deu_apelido', 'HUMAN_MANAGER::has_nickname', '0x600740', 'human_manager.cpp', 'thiscall',
          'se o técnico humano (ou o da vez) deu apelido à pessoa, por busca binária na lista dele',
          'whether the human manager (or the current one) gave the person a nickname, by binary search in his list'),
         ('copiar_aposentadoria_pendente', 'retire_record_assign', '0x7a8780', 'player_regen.cpp', 'thiscall',
          'copia a aposentadoria pendente: a pessoa, o clube, as duas datas, o motivo e a marca de anunciada',
          'copies the pending retirement record: the person, the club, the two dates, the reason and the announced flag')])
