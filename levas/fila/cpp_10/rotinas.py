# O fim da temporada e as transferências nos registros (club_records.cpp, player_stats.cpp): o jogador do ano da
# torcida (Club_Records::pick_fans_player_of_year 0x452da0, pelo fim de temporada do clube 0x447880: o de maior nota
# do elenco, com o mínimo de jogos caindo de 0,6 a 0,3 dos jogos do clube), arquivar as estatísticas dos jogadores da
# nação no fim da temporada (PlayerStats::end_season_for_key_nation 0x7c1350, pela 0x5d5ed0: a 0x7c5fe0 em cada
# jogador; para na primeira seleção, como a original) e os recordes de valor de transferência pago e recebido
# (Club_Records::update_transfer_fee_records 0x448760, em cada transferência: os recordes da temporada e de sempre, a
# fila de notícias de recorde e, na compra grande, o técnico passa a gostar do jogador pela 0x547a10). Sem SEH; os
# doubles da nota lidos do .rdata. Iguais no cm0102-1080.exe. Comparadores: o jogador do ano lado a lado (a
# referência em +0x371); o arquivar só a original (a 0x7c5fe0 aloca), pela devolução; os recordes pelo plano (os
# quatro registros criados antes na mesma ordem; valor e clube de cada recorde, a conta da fila e o [0xae1080]).
SAVE = False
ROTINAS = [('jogador_do_ano_da_torcida', 0x452da0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('arquivar_estatisticas_da_nacao', 0x7c1350, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('recordes_de_valor_de_transferencia', 0x448760, [0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x18])]
DOCS = ('o fim da temporada e as transferências nos registros: o jogador do ano da torcida, arquivar as estatísticas da nação e os recordes de valor de transferência',
        'season end and transfers in the records: the fans\' player of the year, archiving the nation\'s statistics and the transfer fee records',
        [('jogador_do_ano_da_torcida', 'Club_Records::pick_fans_player_of_year', '0x452da0', 'club_records.cpp', 'thiscall, ret 8',
          'do elenco do clube, com jogos acima do mínimo (fator 0,6 caindo de 0,1 até 0,3), o de maior nota (média + 0,025 × +2 + 1/60 × +6, gostos do clube e do jogador); a referência dele em +0x371 do registro e 1, ou 0',
          'from the club\'s squad, with games above the minimum (factor 0.6 falling by 0.1 down to 0.3), the best score (average + 0.025 × +2 + 1/60 × +6, club and player likes); his reference at record +0x371 and 1, or 0'),
         ('arquivar_estatisticas_da_nacao', 'PlayerStats::end_season_for_key_nation', '0x7c1350', 'player_stats.cpp', 'thiscall, ret 4',
          'para os clubes da nação (até a primeira seleção), arquiva (0x7c5fe0) as estatísticas de cada jogador do elenco que não está emprestado a outro clube; 1 (0 e a caixa "Error" sem nação)',
          'for the nation\'s clubs (up to the first national team), archives (0x7c5fe0) the statistics of every squad player not on loan elsewhere; 1 (0 and the "Error" box without a nation)'),
         ('recordes_de_valor_de_transferencia', 'Club_Records::update_transfer_fee_records', '0x448760', 'club_records.cpp', 'thiscall, ret 4',
          'o recorde pago do comprador e o recebido do vendedor, da temporada e de sempre (valor, referência, outro clube, data, empréstimo); o de sempre acima de 1000 × (reputação + 300) vai à fila de notícias e, na compra, o técnico passa a gostar do jogador',
          'the buyer\'s paid record and the seller\'s received record, for the season and all time (fee, reference, other club, date, loan); an all-time one above 1000 × (reputation + 300) goes to the news queue and, on the purchase, the manager comes to favour the player')])
CAMPOS = [('Club_Records +0x1c: os registros de 106 bytes por clube; +0x10 os jogos (com sinal); ponteiro e valor; lê (jogador_do_ano_da_torcida)',
           'Club_Records +0x1c: the 106-byte records per club; +0x10 the games (signed); pointer and value; reads (jogador_do_ano_da_torcida)'),
          ('registro de recordes +0x371: a referência do jogador do ano da torcida (0x18 bytes); valor e id; grava (jogador_do_ano_da_torcida)',
           'records entry +0x371: the fans\' player of the year reference (0x18 bytes); value and id; writes (jogador_do_ano_da_torcida)'),
          ('registro de recordes +0x313..+0x33b e +0x33c..+0x364: recorde pago e recebido (valor, referência, clube, data, empréstimo); valor e id; lê e grava (recordes_de_valor_de_transferencia)',
           'records entry +0x313..+0x33b and +0x33c..+0x364: paid and received record (fee, reference, club, date, loan); value and id; reads and writes (recordes_de_valor_de_transferencia)'),
          ('Club_Records +0x24, +0x2c: a fila de notícias de recorde (6 bytes: id do clube, tipo) e a conta; ponteiro e valor; lê (recordes_de_valor_de_transferencia)',
           'Club_Records +0x24, +0x2c: the record news queue (6 bytes: club id, type) and its count; pointer and value; reads (recordes_de_valor_de_transferencia)'),
          ('estatística (0x7c3920, categoria 6) +0, +2, +6, +0xe: jogos, +2, +6 e a soma das notas; valor; lê (jogador_do_ano_da_torcida)',
           'statistics (0x7c3920, category 6) +0, +2, +6, +0xe: games, +2, +6 and the rating sum; value; reads (jogador_do_ano_da_torcida)'),
          ('transferência +4, +8, +0xc, +0x1b: empréstimo, índice do comprador, id da pessoa e valor; valor e id; lê (recordes_de_valor_de_transferencia)',
           'transfer +4, +8, +0xc, +0x1b: loan, buyer index, person id and fee; value and id; reads (recordes_de_valor_de_transferencia)'),
          ('clube +0x80, +0xbf, +0xd7: reputação, técnico e as 50 vagas; valor e ponteiro; lê',
           'club +0x80, +0xbf, +0xd7: reputation, manager and the 50 slots; value and pointer; reads'),
          ('[0xae1080] o contrato da última transferência; ponteiro; grava (recordes_de_valor_de_transferencia)',
           '[0xae1080] the last transfer\'s contract; pointer; writes (recordes_de_valor_de_transferencia)')]
INTERESSA = ['CMRTE', 'CMScout']
