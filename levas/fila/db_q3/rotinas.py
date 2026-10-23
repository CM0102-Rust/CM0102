ROTINAS = [('pessoa_listada_para_transferencia', 0x544520, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('atributo_calculado_do_jogador', 0x543540, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('atributo_da_pessoa', 0x5431e0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('se a pessoa está na lista de transferências e os atributos de jogador de 1 a 20',
        'whether the person is transfer listed and the 1 to 20 player attributes',
        [('pessoa_listada_para_transferencia', 'staff_is_transfer_listed_or_requested', '0x544520', 'Database.cpp', 'cdecl',
          'se o vínculo da pessoa com o clube (ou o do contrato) está na lista de transferências, pelo clube ou a pedido',
          'whether the person\'s link with the club (or the contract club) is transfer listed, by the club or at request'),
         ('atributo_calculado_do_jogador', 'staff_compute_player_attribute', '0x543540', 'Database.cpp', 'cdecl',
          'um atributo de jogador (1 a 31) de 1 a 20: o byte do registro ou, nos relativos, a nota de 2 × byte + habilidade atual',
          'a player attribute (1 to 31) as 1 to 20: the record byte or, for relative ones, the rating of 2 × byte + current ability'),
         ('atributo_da_pessoa', 'staff_get_player_attribute', '0x5431e0', 'Database.cpp', 'cdecl',
          'um atributo de jogador da pessoa, pelos dados de jogador dela (0x543540)',
          "a player attribute of the person, from the person's player data (0x543540)")])
CAMPOS = [('pessoa +0x61: dados de jogador; ponteiro; lê (atributo_da_pessoa)', 'staff +0x61: player data; pointer; reads (atributo_da_pessoa)'),
          ('pessoa +0x58: atributo 10 (byte direto); valor; lê (atributo_calculado_do_jogador)', 'staff +0x58: attribute 10 (raw byte); value; reads (atributo_calculado_do_jogador)'),
          ('jogador +0x05 [2]: habilidade atual; valor; lê (atributo_calculado_do_jogador)', 'player +0x05 [2]: current ability; value; reads (atributo_calculado_do_jogador)'),
          ('jogador +0x0f: goleiro (1..20; = 20 ou < 15 muda a fórmula); valor; lê (atributo_calculado_do_jogador)', 'player +0x0f: goalkeeper (1..20; = 20 or < 15 changes the formula); value; reads (atributo_calculado_do_jogador)'),
          ('jogador +0x1b..+0x20, +0x28, +0x29, +0x2e, +0x2f, +0x36, +0x3c, +0x3d, +0x3f, +0x40, +0x44: atributos absolutos (aceleração, agilidade, ..., disposição); valor; lê (atributo_calculado_do_jogador)', 'player +0x1b..+0x20, +0x28, +0x29, +0x2e, +0x2f, +0x36, +0x3c, +0x3d, +0x3f, +0x40, +0x44: absolute attributes (acceleration, agility, ..., work rate); value; reads (atributo_calculado_do_jogador)'),
          ('jogador +0x1e, +0x23, +0x24, +0x26, +0x27, +0x2a, +0x2b, +0x31, +0x32, +0x33, +0x37, +0x39, +0x3a, +0x3e, +0x43: atributos relativos (antecipação, ..., desarme, criatividade; nota com a habilidade atual); valor; lê (atributo_calculado_do_jogador)', 'player +0x1e, +0x23, +0x24, +0x26, +0x27, +0x2a, +0x2b, +0x31, +0x32, +0x33, +0x37, +0x39, +0x3a, +0x3e, +0x43: relative attributes (anticipation, ..., tackling, creativity; rated with the current ability); value; reads (atributo_calculado_do_jogador)'),
          ('contrato (0x4cf5a0) +4: id do clube; id; lê (pessoa_listada_para_transferencia)', 'contract (0x4cf5a0) +4: club id; id; reads (pessoa_listada_para_transferencia)'),
          ('vínculo pessoa-clube (0x4cf5f0) +0x4e: marcas (1 lista de transferências, 2 lista de empréstimos, 8 a pedido); valor; lê (pessoa_listada_para_transferencia)', 'staff-club link (0x4cf5f0) +0x4e: flags (1 transfer list, 2 loan list, 8 at request); value; reads (pessoa_listada_para_transferencia)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava (pessoa_listada_para_transferencia, sem clube)', '[0xae1080] last looked-up contract; pointer; writes (pessoa_listada_para_transferencia, without a club)')]
INTERESSA = ['CMScout', 'CMRTE']
