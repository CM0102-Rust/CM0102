ROTINAS = [('copiar_recordista', 0x448490, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08]),
           ('copiar_registro_de_partida', 0x448110, [0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x00]),
           ('copiar_registro_de_sequencia', 0x448170, [0x83, 0xec, 0x08, 0x8d, 0x54, 0x24, 0x00])]
DOCS = ('as cópias do recordista, do registro de partida e do registro de sequência dos recordes do clube',
        'the club records copies of the record holder, the match record and the run record',
        [('copiar_recordista', 'record_holder_assign', '0x448490', 'club_records.cpp', 'thiscall',
          'copia o recordista: quatro campos de identificação e a data',
          'copies the record holder: four identity fields and the date'),
         ('copiar_registro_de_partida', 'match_record_assign', '0x448110', 'club_records.cpp', 'thiscall',
          'copia o registro de partida: a data, o adversário, o placar e os campos do jogo',
          'copies the match record: the date, the opponent, the score and the fixture fields'),
         ('copiar_registro_de_sequencia', 'run_record_assign', '0x448170', 'club_records.cpp', 'thiscall',
          'copia o registro de sequência: o número de jogos e as datas de início e fim',
          'copies the run record: the game count and the start and end dates')])
