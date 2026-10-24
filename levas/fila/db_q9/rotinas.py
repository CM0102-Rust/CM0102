ROTINAS = [('ajustar_moral_da_pessoa', 0x544710, [0x8a, 0x4c, 0x24, 0x08, 0x84, 0xc9]),
           ('vetor_de_atributos_do_nao_jogador', 0x53a8b0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x0c]),
           ('marcar_elenco_da_pessoa', 0x540980, [0x8a, 0x4c, 0x24, 0x08, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('o ajuste do moral, o vetor de atributos do não jogador e as marcas de elenco da pessoa',
        "the morale adjustment, the non-player attribute vector and the person's squad flags",
        [('ajustar_moral_da_pessoa', 'staff_adjust_morale', '0x544710', 'Database.cpp', 'cdecl',
          'soma uma variação ao moral do jogador, amortecida por sorteio perto das pontas, presa em 1..20; devolve se mudou',
          "adds a delta to the player's morale, damped by a draw near the ends, clamped to 1..20; returns whether it changed"),
         ('vetor_de_atributos_do_nao_jogador', 'database_non_player_to_attribute_vector', '0x53a8b0', 'Database.cpp', 'cdecl',
          'estende os atributos de treinador (+0x0e..+0x22) do não jogador num vetor de 21 células de 3 bytes, com o corte em 13',
          "widens the non-player's coaching attributes (+0x0e..+0x22) into a 21-cell vector of 3 bytes, with the split at 13"),
         ('marcar_elenco_da_pessoa', 'staff_set_squad_flags', '0x540980', 'Database.cpp', 'cdecl',
          'grava os bits de elenco do clube ou da seleção da pessoa, mantendo o outro par',
          "sets the person's club or national squad bits, keeping the other pair")])
CAMPOS = [('staff +0x61: jogador; ponteiro; lê (ajustar_moral_da_pessoa)', 'staff +0x61: player; pointer; reads (ajustar_moral_da_pessoa)'),
          ('jogador +0x45: moral (1..20); valor; lê e grava (ajustar_moral_da_pessoa)', 'player +0x45: morale (1..20); value; reads and writes (ajustar_moral_da_pessoa)'),
          ('jogador +0x2f, staff +0x5a, staff +0x58: sem nome (a média / 3 é o teto do sorteio que amortece a queda do moral); valor; lê (ajustar_moral_da_pessoa)', 'player +0x2f, staff +0x5a, staff +0x58: unnamed (their sum / 3 caps the draw that damps a morale drop); value; reads (ajustar_moral_da_pessoa)'),
          ('staff +0x39: clube; ponteiro; lê (ajustar_moral_da_pessoa, com limite)', 'staff +0x39: club; pointer; reads (ajustar_moral_da_pessoa, with cap)'),
          ('vínculo pessoa-clube (0x4cf5f0) +0x45: sem nome (não nulo impede o moral subir com limite); valor; lê (ajustar_moral_da_pessoa)', 'staff-club link (0x4cf5f0) +0x45: unnamed (non-zero stops morale rising with the cap); value; reads (ajustar_moral_da_pessoa)'),
          ('[0xde1f5c], [0xde1f60], [0xad6c30] estado do gerador; valor; lê e grava (ajustar_moral_da_pessoa, por 0x90cfc0)', '[0xde1f5c], [0xde1f60], [0xad6c30] generator state; value; reads and writes (ajustar_moral_da_pessoa, through 0x90cfc0)'),
          ('não jogador +0x0e..+0x22: atributos de treinador (21 bytes com sinal); valor; lê (vetor_de_atributos_do_nao_jogador)', 'non-player +0x0e..+0x22: coaching attributes (21 signed bytes); value; reads (vetor_de_atributos_do_nao_jogador)'),
          ('staff +0x5e: marcas de elenco (& 3 clube, & 0xc seleção); valor; lê e grava (marcar_elenco_da_pessoa)', 'staff +0x5e: squad flags (& 3 club, & 0xc national team); value; reads and writes (marcar_elenco_da_pessoa)')]
INTERESSA = ['CMRTE', 'CMScout', 'GPF2']
