MOTOR = True
ROTINAS = [('evento_na_frente', 0x6d5d00, [0x66, 0x8b, 0x41, 0x08, 0x66, 0x85, 0xc0]),
           ('construir_fila_da_narracao', 0x6d5bf0, [0x53, 0x56, 0x8b, 0xf1, 0x83, 0xc8, 0xff]),
           ('liberar_tabelas_de_eventos', 0x6d5b50, [0x53, 0x56, 0x57, 0xbe, 0x90, 0x87, 0xb6, 0x00])]
DOCS = ('o fechamento do motor (3): a fila da narração',
        'closing the engine (3): the commentary queue',
        [('evento_na_frente', 'event_at_front', '0x6d5d00', 'match_events.cpp', 'thiscall',
          'põe um evento na frente da fila da narração (recua a posição no anel de 12, tira o repetido) e enfileira pela 0x6d6f70',
          'puts an event at the front of the commentary queue (steps the 12-slot ring back, drops the repeat) and queues it via 0x6d6f70'),
         ('construir_fila_da_narracao', 'construct_commentary_queue', '0x6d5bf0', 'match_events.cpp', 'thiscall',
          'o construtor da fila da narração: zera a fila, lê os eventos e, se pedido, toca o loop da torcida',
          'the commentary queue constructor: clears the queue, reads the events and, if asked, plays the crowd loop'),
         ('liberar_tabelas_de_eventos', 'free_event_tables', '0x6d5b50', 'match_events.cpp', 'cdecl',
          'libera as tabelas de eventos carregadas e os textos de cada registro',
          'frees the loaded event tables and each record\'s texts')])
