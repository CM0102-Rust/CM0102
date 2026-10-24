ROTINAS = [('vetor_para_o_nao_jogador', 0x53a980, [0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x04]),
           ('vetor_para_o_jogador', 0x53aa10, [0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x04]),
           ('normalizar_atributos_do_nao_jogador', 0x539490, [0x81, 0xec, 0x6c, 0x02, 0x00, 0x00])]
DOCS = ('os vetores de atributos de volta ao não jogador e ao jogador e o reequilíbrio dos atributos do não jogador',
        "the attribute vectors back into the non-player and the player and the rebalancing of the non-player's attributes",
        [('vetor_para_o_nao_jogador', 'database_vector_to_non_player', '0x53a980', 'Database.cpp', 'cdecl',
          'devolve o byte de baixo das 21 células do vetor aos atributos de treinador do não jogador (+0x0e..+0x22)',
          "writes the low byte of the vector's 21 cells back to the non-player's coaching attributes (+0x0e..+0x22)"),
         ('vetor_para_o_jogador', 'database_vector_to_player', '0x53aa10', 'Database.cpp', 'cdecl',
          'devolve o byte de baixo das células 0..53 do vetor aos atributos do jogador e o da 54 à pessoa +0x58',
          "writes the low byte of the vector's cells 0..53 back to the player's attributes and cell 54 to staff +0x58"),
         ('normalizar_atributos_do_nao_jogador', 'staff_normalise_non_player_attributes', '0x539490', 'Database.cpp', 'cdecl',
          'reequilibra os atributos do não jogador em duas passadas: média das células perto de zero levada a −5, excesso positivo cortado, teto −80 para o cargo 10',
          "rebalances the non-player's attributes in two passes: mean of the near-zero cells pulled to −5, positive excess trimmed, cap −80 for job 10")])
CAMPOS = [('não jogador +0x0e..+0x22: atributos de treinador (21 bytes com sinal); valor; lê e grava (vetor_para_o_nao_jogador grava, normalizar_atributos_do_nao_jogador lê e grava)', 'non-player +0x0e..+0x22: coaching attributes (21 signed bytes); value; reads and writes (vetor_para_o_nao_jogador writes, normalizar_atributos_do_nao_jogador reads and writes)'),
          ('jogador +0x0f..+0x44: atributos e posições (54 bytes de CELULAS_DO_VETOR); valor; grava (vetor_para_o_jogador)', 'player +0x0f..+0x44: attributes and positions (54 bytes of CELULAS_DO_VETOR); value; writes (vetor_para_o_jogador)'),
          ('staff +0x58: sem nome (célula 54 do vetor do jogador); valor; grava (vetor_para_o_jogador)', 'staff +0x58: unnamed (cell 54 of the player vector); value; writes (vetor_para_o_jogador)'),
          ('staff +0x69: não jogador; ponteiro; lê (normalizar_atributos_do_nao_jogador)', 'staff +0x69: non-player; pointer; reads (normalizar_atributos_do_nao_jogador)'),
          ('staff +0x28 / +0x3d: sem nome (cargos; 10 limita os atributos a −80); valor; lê (normalizar_atributos_do_nao_jogador)', 'staff +0x28 / +0x3d: unnamed (jobs; 10 caps the attributes at −80); value; reads (normalizar_atributos_do_nao_jogador)')]
INTERESSA = ['CMRTE', 'CMScout', 'GPF2']
