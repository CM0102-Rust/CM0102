ROTINAS = [('familia_da_cor', 0x540520, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xe4, 0xf1, 0x9c, 0x00]),
           ('cores_se_confundem', 0x540490, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('atributo_calculado_do_nao_jogador', 0x543fd0, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a família da cor, se duas cores se confundem e o atributo calculado do não jogador',
        "the colour family, whether two colours clash and the non-player's computed attribute",
        [('familia_da_cor', 'database_get_colour_family', '0x540520', 'Database.cpp', 'cdecl',
          'a família (1..10) da cor pelas chaves de cor do index.cpp',
          "the colour's family (1..10) from the index.cpp colour keys"),
         ('cores_se_confundem', 'database_colours_clash', '0x540490', 'Database.cpp', 'cdecl',
          'se duas cores se confundem: mesma família ou os pares 8-9, 8-7, 6-7 e a família 6 com a cor da chave [0x9cf1dc]',
          'whether two colours clash: same family or the pairs 8-9, 8-7, 6-7 and family 6 against the colour keyed at [0x9cf1dc]'),
         ('atributo_calculado_do_nao_jogador', 'staff_compute_non_player_attribute', '0x543fd0', 'Database.cpp', 'cdecl',
          'o atributo 1..12 de treinador: quatro direto do registro, oito pela fórmula de 1 a 20 com a palavra +4 do não jogador',
          'the coaching attribute 1..12: four straight from the record, eight through the 1-to-20 formula with the non-player word +4')])
CAMPOS = [('cor +0x00: id (comparado com as chaves 0x9cf178..0x9cf1fc); id; lê (familia_da_cor, cores_se_confundem)', 'colour +0x00: id (compared with the keys 0x9cf178..0x9cf1fc); id; reads (familia_da_cor, cores_se_confundem)'),
          ('[0x9cf178..0x9cf1fc] chaves de cor do index.cpp; id; lê (familia_da_cor, cores_se_confundem)', '[0x9cf178..0x9cf1fc] index.cpp colour keys; id; reads (familia_da_cor, cores_se_confundem)'),
          ('staff +0x56, +0x58: atributos 1 e 4 de treinador; valor; lê (atributo_calculado_do_nao_jogador)', 'staff +0x56, +0x58: coaching attributes 1 and 4; value; reads (atributo_calculado_do_nao_jogador)'),
          ('não jogador +0x04: sem nome (palavra com sinal da fórmula); valor; lê (atributo_calculado_do_nao_jogador)', 'non-player +0x04: unnamed (signed word of the formula); value; reads (atributo_calculado_do_nao_jogador)'),
          ('não jogador +0x10, +0x11, +0x14, +0x17, +0x18, +0x19, +0x1b, +0x1e, +0x21, +0x22: atributos de treinador; valor; lê (atributo_calculado_do_nao_jogador)', 'non-player +0x10, +0x11, +0x14, +0x17, +0x18, +0x19, +0x1b, +0x1e, +0x21, +0x22: coaching attributes; value; reads (atributo_calculado_do_nao_jogador)')]
INTERESSA = ['CMRTE', 'CMScout']
