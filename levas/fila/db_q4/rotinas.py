ROTINAS = [('ordem_do_historico_de_competicao', 0x540780, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('vetor_de_atributos_do_jogador', 0x53a610, [0x8b, 0x44, 0x24, 0x0c, 0xba, 0x3f, 0x00, 0x00, 0x00]),
           ('nacao_britanica_ou_irlanda', 0x5446d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xe4, 0xf2, 0x9c, 0x00])]
DOCS = ('a ordem do histórico de competições, o vetor de atributos do jogador e se a nação é britânica ou a Irlanda',
        'the competition history order, the player attribute vector and whether the nation is British or Ireland',
        [('ordem_do_historico_de_competicao', 'database_compare_comp_history_rows', '0x540780', 'Database.cpp', 'cdecl',
          'comparação do qsort do histórico de competições: sem competição no fim, pelo id da competição e pela temporada da mais nova para a mais velha',
          'qsort comparison of the competition history: rows without a competition last, by competition id and by season, newest first'),
         ('vetor_de_atributos_do_jogador', 'database_player_to_attribute_vector', '0x53a610', 'Database.cpp', 'cdecl',
          'estende posições, lados e atributos do jogador num vetor de 63 células (valor e peso) e devolve os fins dos grupos (9, 0xc, 0x20)',
          "widens the player's positions, sides and attributes into a 63 cell vector (value and weight) and returns the group ends (9, 0xc, 0x20)"),
         ('nacao_britanica_ou_irlanda', 'nation_is_uk_or_ireland', '0x5446d0', 'Database.cpp', 'cdecl',
          'se a nação é Inglaterra, Escócia, Irlanda, País de Gales ou Irlanda do Norte',
          'whether the nation is England, Scotland, Ireland, Wales or Northern Ireland')])
CAMPOS = [('histórico de competição +0x04: competição; ponteiro; lê (ordem_do_historico_de_competicao)', 'competition history +0x04: competition; pointer; reads (ordem_do_historico_de_competicao)'),
          ('histórico de competição +0x08 [2]: temporada (sem nome; −1 = sem); valor; lê (ordem_do_historico_de_competicao)', 'competition history +0x08 [2]: season (unnamed; −1 = none); value; reads (ordem_do_historico_de_competicao)'),
          ('competição +0x00: id; id; lê (ordem_do_historico_de_competicao)', 'competition +0x00: id; id; reads (ordem_do_historico_de_competicao)'),
          ('jogador +0x0f..+0x1a: posições e lados (sem nome em CAMPOS.md; +0x0f goleiro); valor; lê (vetor_de_atributos_do_jogador)', 'player +0x0f..+0x1a: positions and sides (unnamed in CAMPOS.md; +0x0f goalkeeper); value; reads (vetor_de_atributos_do_jogador)'),
          ('jogador +0x1b..+0x44: atributos (aceleração +0x1b, antecipação +0x1e, regularidade +0x21, drible +0x26, cabeceio +0x2b, impulsão +0x2e, liderança +0x2f, pé esquerdo +0x30, chute de longe +0x31, marcação +0x32, movimentação sem bola +0x33, mano a mano +0x35, velocidade +0x36, passe +0x37, pé direito +0x3b, resistência +0x3c, desarme +0x3e, trabalho em equipe +0x3f, técnica +0x40, criatividade +0x43, disposição +0x44; os demais sem nome); valor; lê (vetor_de_atributos_do_jogador)', 'player +0x1b..+0x44: attributes (acceleration +0x1b, anticipation +0x1e, consistency +0x21, dribbling +0x26, heading +0x2b, jumping +0x2e, leadership +0x2f, left foot +0x30, long shots +0x31, marking +0x32, off the ball +0x33, one on ones +0x35, pace +0x36, passing +0x37, right foot +0x3b, stamina +0x3c, tackling +0x3e, teamwork +0x3f, technique +0x40, creativity +0x43, work rate +0x44; the others unnamed); value; reads (vetor_de_atributos_do_jogador)'),
          ('pessoa +0x58: sem nome (o atributo 10 de 0x543540); valor; lê (vetor_de_atributos_do_jogador)', 'staff +0x58: unnamed (attribute 10 of 0x543540); value; reads (vetor_de_atributos_do_jogador)'),
          ('nação +0x00: id; id; lê (nacao_britanica_ou_irlanda: comparado com [0x9cf2e4] Inglaterra, [0x9cf45c] Escócia, [0x9cf2dc] Irlanda, [0x9cf510] País de Gales, [0x9cf3e4] Irlanda do Norte)', 'nation +0x00: id; id; reads (nacao_britanica_ou_irlanda: compared with [0x9cf2e4] England, [0x9cf45c] Scotland, [0x9cf2dc] Ireland, [0x9cf510] Wales, [0x9cf3e4] Northern Ireland)')]
INTERESSA = ['CMScout']
