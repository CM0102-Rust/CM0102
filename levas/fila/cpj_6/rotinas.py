# Depois da partida e transferências: a diretoria de cada lado (MANAGER_MANAGER::update_board_after_fixture 0x6951e0;
# a 0x695240 muda a confiança, original) e a disciplina (Discipline::process_match 0x55ca00: o tribunal da competição
# ou, em amistosos, os das ligas dos dois clubes; 0x7e7ff0 original, 0x5c6710 já em Rust), as duas chamadas pelo fim
# da partida do dia 0x6b5f00 (já em Rust); as entradas do clube na shortlist (Shortlist_Manager::
# collect_club_shortlist_entries 0x8420e0, pelas 0x84abe0/0x8cfc90; bsearch 0x946464 com a comparação 0x844740, chave
# {−1, clube}) e a maior oferta pelo jogador (TRANSFER_MANAGER::find_highest_offer_for_player 0x8cc930, pela 0x8e5d60;
# o valor pela 0x8e7a70, original). Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem
# SEH, sem caixa, sem sorteio. Comparadores: diretoria e disciplina só pela original (mexem no estado do jogo); a
# shortlist pelos itens que a original acrescentou; a maior oferta pelo valor.
SAVE = False
ROTINAS = [('diretoria_depois_da_partida', 0x6951e0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('disciplina_da_partida', 0x55ca00, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('entradas_do_clube_na_shortlist', 0x8420e0, [0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10]),
           ('maior_oferta_pelo_jogador', 0x8cc930, [0x51, 0x53, 0x8b, 0xd9, 0x8b, 0x4c, 0x24, 0x0c])]
DOCS = ('depois da partida, a diretoria e a disciplina; as entradas do clube na shortlist e a maior oferta pelo jogador',
        'after the match, the board and the discipline; the club shortlist entries and the highest offer for a player',
        [('diretoria_depois_da_partida', 'MANAGER_MANAGER::update_board_after_fixture', '0x6951e0', 'manager_manager.cpp', 'thiscall, ret 8',
          'partida jogada: a confiança da diretoria de cada lado com técnico muda (0x695240, mandante com 1)',
          'played match: the board confidence of each side with a manager changes (0x695240, home with 1)'),
         ('disciplina_da_partida', 'Discipline::process_match', '0x55ca00', 'discipline.cpp', 'thiscall, ret 8',
          'o tribunal da competição julga a partida (0x7e7ff0); sem ele, em amistosos, os tribunais das ligas dos dois clubes',
          'the competition ruling body judges the match (0x7e7ff0); without it, in friendlies, the league ruling bodies of both clubs'),
         ('entradas_do_clube_na_shortlist', 'Shortlist_Manager::collect_club_shortlist_entries', '0x8420e0', 'shortlist_manager.cpp', 'thiscall, ret 8',
          'acrescenta à lista cada entrada (8 bytes) da shortlist do clube, achadas pela busca binária',
          'adds to the list every shortlist entry (8 bytes) of the club, found by binary search'),
         ('maior_oferta_pelo_jogador', 'TRANSFER_MANAGER::find_highest_offer_for_player', '0x8cc930', 'transfer_manager.cpp', 'thiscall, ret 8',
          'das ofertas pelo jogador do clube (ou marcadas) no estado 2, a de maior valor (0x8e7a70); 0 sem nenhuma',
          'of the offers for the player from the club (or flagged) in state 2, the highest (0x8e7a70); 0 when none')])
CAMPOS = [('partida +0 competição, +0x14, +0x1c/+0x20 clubes, +0x40 jogada; clube +0x57 liga, +0xcf técnico; competição +0x50 tribunal; shortlist +4 entradas (8 bytes), +0x14 quantas; gerenciador de transferências [+0] registros ([+4] deles: +0 lista, +8 clube, +0xc jogador, +0x2c estado, +0x2d marcada), +0x86e marcas (byte +1); lê (entradas_do_clube_na_shortlist, maior_oferta_pelo_jogador); escreve (diretoria_depois_da_partida, disciplina_da_partida)',
           'fixture +0 competition, +0x14, +0x1c/+0x20 clubs, +0x40 played; club +0x57 league, +0xcf manager; competition +0x50 ruling body; shortlist +4 entries (8 bytes), +0x14 count; transfer manager [+0] records ([+4] of them: +0 list, +8 club, +0xc player, +0x2c state, +0x2d flagged), +0x86e flags (byte +1); reads (entradas_do_clube_na_shortlist, maior_oferta_pelo_jogador); writes (diretoria_depois_da_partida, disciplina_da_partida)')]
INTERESSA = ['SGE']
