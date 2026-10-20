ROTINAS = [('ler_registro_da_partida', 0x729600, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('registrar_partida', 0x7293f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gravar_registro_da_partida', 0x729820, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as estatísticas da partida: ler e gravar o registro no arquivo temporário e registrar a partida',
        'the match statistics: reading and writing the record in the temporary file and recording the match',
        [('ler_registro_da_partida', 'Match_Stats::read_match_record', '0x729600', 'match_stats.cpp', 'thiscall',
          'lê um registro de estatística da partida (0xd7a bytes) do match_stats_matches.tmp pelo índice',
          'reads one match statistics record (0xd7a bytes) from match_stats_matches.tmp by index'),
         ('registrar_partida', 'Match_Stats::record_match', '0x7293f0', 'match_stats.cpp', 'thiscall',
          'preenche o registro de uma partida que terminou e o grava, salvo os jogos excluídos',
          'fills the record of a finished match and writes it, except for excluded matches'),
         ('gravar_registro_da_partida', 'Match_Stats::write_match_record', '0x729820', 'match_stats.cpp', 'thiscall',
          'grava os eventos e o registro na vaga do jogo (ou na vaga do cursor do balde, que anda em círculo) e atualiza o índice',
          "writes the events and the record into the fixture's slot (or the bucket cursor's slot, which wraps around) and updates the index")])
