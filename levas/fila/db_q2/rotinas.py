ROTINAS = [('idade_da_pessoa', 0x53ab90, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nacao_com_transferencia_livre', 0x544670, [0x80, 0x3d, 0x3c, 0x23, 0xae, 0x00, 0x01]),
           ('pessoa_e_comunitaria', 0x53d740, [0x8b, 0x44, 0x24, 0x04, 0xb1, 0x02])]
DOCS = ('a idade da pessoa, se a nação tem transferência livre e se a pessoa é da UE',
        'the person\'s age, whether the nation has free transfers and whether the person is an EU national',
        [('idade_da_pessoa', 'database_get_staff_age', '0x53ab90', 'Database.cpp', 'cdecl',
          'a idade da pessoa em anos completos numa data, pelo nascimento',
          "the person's age in whole years on a date, from the birth date"),
         ('nacao_com_transferencia_livre', 'nation_allows_free_transfers', '0x544670', 'Database.cpp', 'cdecl',
          'se a nação tem transferência livre: regras novas da FIFA, Brasil, ou Europa fora de Polônia, Turquia, Croácia e Rússia',
          'whether the nation has free transfers: new FIFA rules, Brazil, or Europe except Poland, Turkey, Croatia and Russia'),
         ('pessoa_e_comunitaria', 'database_is_staff_eu_national', '0x53d740', 'Database.cpp', 'cdecl',
          'se a pessoa conta como da UE para um clube da nação: marca de UE da nacionalidade, Suíça e as exceções da Alemanha, [0x9cf360] e Dinamarca',
          'whether the person counts as an EU national for a club of the nation: the nationality EU mark, Switzerland and the Germany, [0x9cf360] and Denmark exceptions')])
CAMPOS = [('pessoa +0x10 / +0x12 [2]: nascimento (dia do ano e ano); valor; lê (idade_da_pessoa)', 'staff +0x10 / +0x12 [2]: birth date (day of year and year); value; reads (idade_da_pessoa)'),
          ('pessoa +0x1a / +0x1e: nacionalidade e segunda nacionalidade; ponteiro; lê (pessoa_e_comunitaria)', 'staff +0x1a / +0x1e: nationality and second nationality; pointer; reads (pessoa_e_comunitaria)'),
          ('nação +0x7f: marca de UE (2 = UE); valor; lê (pessoa_e_comunitaria)', 'nation +0x7f: EU mark (2 = EU); value; reads (pessoa_e_comunitaria)'),
          ('nação +0x71: continente; ponteiro; lê (nacao_com_transferencia_livre, pessoa_e_comunitaria)', 'nation +0x71: continent; pointer; reads (nacao_com_transferencia_livre, pessoa_e_comunitaria)'),
          ('nação +0x00: id; id; lê (comparado com [0x9cf260] Brasil, [0x9cf430] Polônia, [0x9cf4dc] Turquia, [0x9cf2ac] Croácia, [0x9cf448] Rússia, [0x9cf4ac] Suíça, [0x9cf314] Alemanha, [0x9cf360], [0x9cf2c0] Dinamarca, [0x9cf348], [0x9cf408] Noruega, [0x9cf3a0])', 'nation +0x00: id; id; reads (compared with [0x9cf260] Brazil, [0x9cf430] Poland, [0x9cf4dc] Turkey, [0x9cf2ac] Croatia, [0x9cf448] Russia, [0x9cf4ac] Switzerland, [0x9cf314] Germany, [0x9cf360], [0x9cf2c0] Denmark, [0x9cf348], [0x9cf408] Norway, [0x9cf3a0])'),
          ('continente +0x00: id (Europa = [0x9cfa10]); id; lê (nacao_com_transferencia_livre)', 'continent +0x00: id (Europe = [0x9cfa10]); id; reads (nacao_com_transferencia_livre)'),
          ('[0xae233c] regras novas da FIFA (1 = transferência livre em toda parte); valor; lê (nacao_com_transferencia_livre)', '[0xae233c] new FIFA rules (1 = free transfers everywhere); value; reads (nacao_com_transferencia_livre)'),
          ('[0xae23a4] tabela de continentes (198 bytes); ponteiro; lê (pessoa_e_comunitaria)', '[0xae23a4] continent table (198 bytes); pointer; reads (pessoa_e_comunitaria)')]
INTERESSA = ['CMScout', 'CMRTE']
