ROTINAS = [('clube_do_contrato_acertado', 0x8c5450, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('registrar_jogo_pela_selecao', 0x8ce920, [0x8b, 0x54, 0x24, 0x04, 0x53]),
           ('punicao_guardada', 0x55cb60, [0x53, 0x55, 0x8b, 0x51, 0x14])]
DOCS = ('o clube do contrato acertado, o jogo pela seleção e a punição guardada',
        'the agreed contract club, the national team appearance and the held ban',
        [('clube_do_contrato_acertado', 'TRANSFER_MANAGER::get_agreed_contract_club', '0x8c5450', 'transfer_manager.cpp', 'thiscall',
          'o clube do contrato acertado da pessoa (desliga a marca quando o contrato sumiu)',
          'the club of the person\'s agreed contract (clears the flag when the contract is gone)'),
         ('registrar_jogo_pela_selecao', 'TRANSFER_MANAGER::record_international_appearance', '0x8ce920', 'transfer_manager.cpp', 'thiscall',
          'registra o jogo do jogador pela seleção nos bits dos últimos jogos',
          'records the player\'s national team match in the recent-matches bits'),
         ('punicao_guardada', 'Discipline::find_held_ban', '0x55cb60', 'discipline.cpp', 'thiscall',
          'a punição guardada do jogador numa vaga, ou nenhuma',
          'the player\'s held ban in a slot, or none')])
