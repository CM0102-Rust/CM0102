MOTOR = True
# Fora: 0x727fa0 (match_screens_draw_waiting_screen, 9 bytes: push 0 + call 0x41c0d0 relativo nos
# bytes 2..6, sem 5 bytes livres para o desvio).
ROTINAS = [('largar_partida_assistida', 0x6b6970, [0x6a, 0xff, 0x68, 0x8c, 0xd2, 0x95, 0x00]),
           ('ficha_do_registro', 0x729e30, [0x6a, 0xff, 0x68, 0x3b, 0xd6, 0x95, 0x00])]
DOCS = ('o fim das telas da partida: a partida que o humano deixa e a ficha do jogo de um registro de estatística',
        'the end of the match screens: the match the human leaves and the fixture of a statistics record',
        [('largar_partida_assistida', 'match_day_drop_viewer_and_finish_match', '0x6b6970', 'match_day.cpp', 'cdecl',
          'o humano da vez deixa a partida que assistia; sem outro humano assistindo, a partida anda até o fim, é registrada e sai da fila',
          'the current human leaves the match he was watching; with no other human watching, the match runs to the end, is recorded and leaves the queue'),
         ('ficha_do_registro', 'Match_Stats::find_record_fixture', '0x729e30', 'match_stats.cpp', 'thiscall',
          'acha na agenda o jogo de um registro de estatística (data, competição e clube) e copia a ficha',
          'finds in the fixture list the match of a statistics record (date, competition and club) and copies the fixture')])
CAMPOS = [('[0xb77c7a] vez (canal do humano da vez); valor; lê (largar_partida_assistida)', '[0xb77c7a] current slot (channel of the current human); value; reads (largar_partida_assistida)'),
          ('[0xb74c26 + 0x300 × vez] técnico humano do canal; ponteiro; lê (largar_partida_assistida)', '[0xb74c26 + 0x300 × slot] channel human manager; pointer; reads (largar_partida_assistida)'),
          ('[0xae23bc] tabela de pessoas (110 bytes); ponteiro; lê (largar_partida_assistida)', '[0xae23bc] staff table (110 bytes); pointer; reads (largar_partida_assistida)'),
          ('[0xae2364] quantidade de pessoas (as 16 últimas são os humanos); valor; lê (largar_partida_assistida)', '[0xae2364] staff count (the last 16 are the humans); value; reads (largar_partida_assistida)'),
          ('staff +0x5f: técnico humano ativo; valor; lê (largar_partida_assistida)', 'staff +0x5f: active human manager; value; reads (largar_partida_assistida)'),
          ('registro do humano (0x765480) +0xe2: partida que ele assiste; ponteiro; lê (largar_partida_assistida)', 'human record (0x765480) +0xe2: match he watches; pointer; reads (largar_partida_assistida)'),
          ('partida +0x5b: resumo (vetores por vez em +5/+0x45/+0x85/+0xc5); ponteiro; lê, e grava 0 na casa da vez (largar_partida_assistida)', 'match +0x5b: summary (per-slot vectors at +5/+0x45/+0x85/+0xc5); pointer; reads, and writes 0 at the slot entry (largar_partida_assistida)'),
          ('resumo +0x105..+0x108: contadores dos quatro vetores; valor; lê e grava −1 (largar_partida_assistida)', 'summary +0x105..+0x108: counters of the four vectors; value; reads and writes −1 (largar_partida_assistida)'),
          ('partida +0x57: motor da partida (0xf63b bytes); ponteiro; lê e grava (largar_partida_assistida)', 'match +0x57: match engine (0xf63b bytes); pointer; reads and writes (largar_partida_assistida)'),
          ('partida +0x67: estado da partida (2 ou 8 = fim); valor; lê e grava (largar_partida_assistida)', 'match +0x67: match state (2 or 8 = end); value; reads and writes (largar_partida_assistida)'),
          ('partida +0x5f: registro de estatística (0xd7a bytes); ponteiro; grava (largar_partida_assistida)', 'match +0x5f: statistics record (0xd7a bytes); pointer; writes (largar_partida_assistida)'),
          ('[0xb67a50] (MATCH_DAY) +0x25: sem nome (conta as partidas que chegam ao estado 2 ou 8); valor; grava +1 (largar_partida_assistida)', '[0xb67a50] (MATCH_DAY) +0x25: unnamed (counts matches reaching state 2 or 8); value; writes +1 (largar_partida_assistida)'),
          ('[0xb6b7c0] estatísticas da partida (Match_Stats); ponteiro; lê (largar_partida_assistida)', '[0xb6b7c0] match statistics (Match_Stats); pointer; reads (largar_partida_assistida)'),
          ('[0xde2f70] VIRTUAL_STAFF (fila das partidas); ponteiro; lê (largar_partida_assistida)', '[0xde2f70] VIRTUAL_STAFF (match queue); pointer; reads (largar_partida_assistida)'),
          ('Match_Stats +0: índice dos registros (12 bytes cada); ponteiro; lê (ficha_do_registro)', 'Match_Stats +0: record index (12 bytes each); pointer; reads (ficha_do_registro)'),
          ('Match_Stats +0x1c: quantidade de registros; valor; lê (ficha_do_registro)', 'Match_Stats +0x1c: record count; value; reads (ficha_do_registro)'),
          ('índice das estatísticas +0: competição; id; lê (ficha_do_registro)', 'statistics index +0: competition; id; reads (ficha_do_registro)'),
          ('índice das estatísticas +4: um dos clubes do jogo; id; lê (ficha_do_registro)', 'statistics index +4: one of the match clubs; id; reads (ficha_do_registro)'),
          ('índice das estatísticas +8/+0xa: data do jogo; valor; lê (ficha_do_registro)', 'statistics index +8/+0xa: match date; value; reads (ficha_do_registro)'),
          ('[0xae2a58] agenda (FIXTURE_MANAGER); ponteiro; lê (ficha_do_registro)', '[0xae2a58] fixture list (FIXTURE_MANAGER); pointer; reads (ficha_do_registro)'),
          ('ficha do jogo (0x4f bytes) +0xc/+0x10: clube da casa e de fora; id; lê, e copia a ficha inteira (ficha_do_registro)', 'fixture (0x4f bytes) +0xc/+0x10: home and away club; id; reads, and copies the whole fixture (ficha_do_registro)')]
INTERESSA = ['SGE']
