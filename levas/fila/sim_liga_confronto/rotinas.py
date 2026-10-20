ROTINAS = [('ordem_do_confronto', 0x6834f0, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('atualizar_confronto_direto', 0x6869a0, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('marcar_termos_oferecidos', 0x8c1f80, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o confronto direto da liga e os termos oferecidos na transferência',
        'the league head-to-head and the terms offered in the transfer',
        [('ordem_do_confronto', 'league_compare_h2h_rows', '0x6834f0', 'league.cpp', 'cdecl',
          'a ordem da minitabela do confronto direto: pontos e os critérios de desempate da liga',
          'the head-to-head mini table order: points and the league tie-breakers'),
         ('atualizar_confronto_direto', 'League::update_head_to_head', '0x6869a0', 'league.cpp', 'thiscall',
          'soma um jogo ao confronto direto (jogos, resultado, gols e pontos)',
          'adds one match to the head-to-head record (played, result, goals and points)'),
         ('marcar_termos_oferecidos', 'TRANSFER_MANAGER::note_terms_offered_to_main_player', '0x8c1f80', 'transfer_manager.cpp', 'thiscall',
          'marca que a oferta de transferência teve os termos de contrato oferecidos ao jogador',
          'marks that the transfer offer had contract terms offered to the player')])
