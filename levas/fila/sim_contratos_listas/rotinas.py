ROTINAS = [('tirar_contrato_de_emprestimo', 0x4d0bd0, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('desmarcar_lances_pelo_jogador', 0x84aeb0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tecnico_substituto', 0x600e90, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('o contrato de empréstimo, as marcas de lance da lista de observados e o técnico substituto',
        'the loan contract, the shortlist bid markers and the stand-in manager',
        [('tirar_contrato_de_emprestimo', 'CONTRACT_MANAGER::remove_loan_contract', '0x4d0bd0', 'contract_manager.cpp', 'thiscall',
          'libera a vaga do contrato de empréstimo da pessoa, se houver',
          'frees the person\'s loan contract slot, if any'),
         ('desmarcar_lances_pelo_jogador', 'Shortlist_Manager::clear_bid_markers_for_player', '0x84aeb0', 'shortlist_manager.cpp', 'thiscall',
          'tira a marca de lance das vagas da lista de observados do clube que miram o jogador',
          'clears the bid marker of the club shortlist slots that target the player'),
         ('tecnico_substituto', 'human_manager_get_stand_in_manager', '0x600e90', 'human_manager.cpp', 'cdecl',
          'o auxiliar do clube ou, sem ele, o membro da comissão de maior reputação',
          'the club\'s assistant or, without one, the highest-reputation staff member')])
