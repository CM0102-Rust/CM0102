ROTINAS = [('tirar_percentual_da_proxima_venda', 0x8cb4f0, [0x8b, 0x54, 0x24, 0x04, 0x53]),
           ('tirar_bonus_por_jogo', 0x8cb590, [0x8b, 0x54, 0x24, 0x04, 0x53]),
           ('tirar_bonus_pela_selecao', 0x8cb630, [0x8b, 0x54, 0x24, 0x04, 0x53])]
DOCS = ('as cláusulas do contrato: percentual da próxima venda, bônus por jogo e bônus pela seleção',
        'the contract clauses: next sale percentage, appearance bonus and national team bonus',
        [('tirar_percentual_da_proxima_venda', 'TRANSFER_MANAGER::remove_next_sale_percentage', '0x8cb4f0', 'transfer_manager.cpp', 'thiscall',
          'tira o percentual da próxima venda do jogador (marca e registro da lista)',
          'removes the player\'s next sale percentage (flag and list record)'),
         ('tirar_bonus_por_jogo', 'TRANSFER_MANAGER::remove_first_team_appearance_money', '0x8cb590', 'transfer_manager.cpp', 'thiscall',
          'tira o bônus por jogo no time principal (marca e registro da lista)',
          'removes the first-team appearance bonus (flag and list record)'),
         ('tirar_bonus_pela_selecao', 'TRANSFER_MANAGER::remove_international_team_appearance_money', '0x8cb630', 'transfer_manager.cpp', 'thiscall',
          'tira o bônus por jogo pela seleção (marca e registro da lista)',
          'removes the international appearance bonus (flag and list record)')])
