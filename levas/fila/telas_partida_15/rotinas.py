MOTOR = True
ROTINAS = [('desenhar_conferencia_da_escalacao', 0x7260f0, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('carregar_estatisticas_da_partida', 0x72aa80, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('salvar_estatisticas_da_partida', 0x72adf0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as telas da partida (15): a conferência da escalação e o índice das estatísticas da partida no save',
        'the match screens (15): the team selection check and the match statistics index in the saved game',
        [('desenhar_conferencia_da_escalacao', 'match_screens_draw_team_selection', '0x7260f0', 'match_screens.cpp', 'cdecl',
          'desenha a conferência da escalação: avisa (com os nomes ligados) os convocados, emprestados, indisponíveis, inelegíveis, suspensos, lesionados e sem permissão de trabalho, o limite do banco ou a falta de titulares; senão pede a confirmação (capitão, cobradores, reservas) com "No" e "Yes"',
          'draws the team selection check: warns (with linked names) about players on international duty, on loan, unavailable, ineligible, banned, injured or without a work permit, the bench limit or missing starters; otherwise asks for confirmation (captain, set piece takers, substitutes) with "No" and "Yes"'),
         ('carregar_estatisticas_da_partida', 'Match_Stats::load', '0x72aa80', 'match_stats.cpp', 'thiscall',
          'lê do match_stats.dat do save o índice dos registros, os cursores dos quatro baldes e as contagens',
          'reads the record index, the four bucket cursors and the counts from match_stats.dat in the saved game'),
         ('salvar_estatisticas_da_partida', 'Match_Stats::save', '0x72adf0', 'match_stats.cpp', 'thiscall',
          'grava no match_stats.dat do save o índice dos registros, os cursores dos quatro baldes e as contagens',
          'writes the record index, the four bucket cursors and the counts to match_stats.dat in the saved game')])
