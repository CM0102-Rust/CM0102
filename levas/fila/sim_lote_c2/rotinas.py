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
CAMPOS = [('recordista (0x18 bytes) +0..+0xc: quatro campos de identificação, sem nome cada; valor; lê e grava (copiar_recordista)', 'record holder (0x18 bytes) +0..+0xc: four identity fields, each unnamed; value; reads and writes (copiar_recordista)'),
          ('recordista +0x10: data; valor; lê e grava (copiar_recordista)', 'record holder +0x10: date; value; reads and writes (copiar_recordista)'),
          ('registro de partida do recorde (0x1f bytes) +0: data; valor; lê e grava (copiar_registro_de_partida)', 'record match entry (0x1f bytes) +0: date; value; reads and writes (copiar_registro_de_partida)'),
          ('registro de partida do recorde +8: adversário; valor; lê e grava (copiar_registro_de_partida)', 'record match entry +8: opponent; value; reads and writes (copiar_registro_de_partida)'),
          ('registro de partida do recorde +0xc, +0xe, +0x10, +0x11, +0x15, +0x19, +0x1a, +0x1b: sem nome (placar e campos do jogo); valor; lê e grava (copiar_registro_de_partida)', 'record match entry +0xc, +0xe, +0x10, +0x11, +0x15, +0x19, +0x1a, +0x1b: unnamed (score and fixture fields); value; reads and writes (copiar_registro_de_partida)'),
          ('registro de sequência (0x14 bytes) +0: número de jogos; valor; lê e grava (copiar_registro_de_sequencia)', 'run record (0x14 bytes) +0: game count; value; reads and writes (copiar_registro_de_sequencia)'),
          ('registro de sequência +4/+0xc: datas de início e fim; valor; lê e grava (copiar_registro_de_sequencia)', 'run record +4/+0xc: start and end dates; value; reads and writes (copiar_registro_de_sequencia)')]
INTERESSA = ['SGE']
