# O progresso do treino nos atributos: aplicar o progresso das cinco categorias (TRAINING_MANAGER::apply_training_progress
# 0x8b2be0, pela update_player_training 0x8b1750 em cada jogador a cada atualização do treino, e pelas 0x8b3640, já em
# Rust, e 0x8b37c0, das telas), achar o treino de posição do jogador (TRAINING_MANAGER::find_position_training 0x8b4b20,
# pela 0x8b1750, pela 0x8b2be0, pela 0x8b42d0 e pela 0x8b4db0) e se ele aprende uma posição depressa
# (TRAINING_MANAGER::has_fast_position_training 0x8b4db0, pelo diário das pessoas 0x5ce2d0). Chamadas originais pelo
# endereço: 0x946464 (bsearch da CRT) e a caixa de erro (aviso_em); a ordem 0x8b4e00 já em Rust. Iguais no
# cm0102-1080.exe (as tabelas de salto da 0x8b2be0 também). Trocas: das candidatas da família, a update_player_training
# (0x8b1750, 2,7 KB) fica para outra leva; a set_position_training_rate 0x8b4450 e a get_position_aptitude 0x8b4770 só
# servem ao treino de posição, que o computador quase não usa. Simulação: as mutações pela carreira até 2002-06-01.
# Comparadores: o progresso lado a lado na ficha e na entrada de treino; a busca pela devolução e pela saída (reposta
# entre as duas); a pergunta pela devolução; sem os casos de erro.
SAVE = False
ROTINAS = [('aplicar_o_progresso', 0x8b2be0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('achar_o_treino_de_posicao', 0x8b4b20, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('aprende_depressa', 0x8b4db0, [0x51, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('o progresso do treino nos atributos: aplicar o progresso das cinco categorias, achar o treino de posição do jogador e se ele aprende uma posição depressa',
        "training progress on the attributes: applying the progress of the five categories, finding the player's position training and whether he learns a position fast",
        [('aplicar_o_progresso', 'TRAINING_MANAGER::apply_training_progress', '0x8b2be0', 'training_manager.cpp', 'thiscall, ret 8',
          'soma o progresso de cada categoria ao meio byte guardado do jogador; a cada dois passos os atributos da categoria andam (os de 1 a 20 um, os de −125 a 125 seis); depois a posição ou o lado do treino de posição vai a 20 quando aprendida, ou volta a 1 quando o treino para',
          "adds each category's progress to the player's stored nibble; every two steps the category attributes move (1 to 20 ones by one, −125 to 125 ones by six); then the trained position or side goes to 20 when learnt, or back to 1 when the training stops"),
         ('achar_o_treino_de_posicao', 'TRAINING_MANAGER::find_position_training', '0x8b4b20', 'training_manager.cpp', 'thiscall, ret 8',
          'acha na lista ordenada do treino de posição o primeiro item do jogador e devolve quantos são dele (0 sem a marca de treino de posição)',
          "finds the player's first entry in the sorted position training list and returns how many are his (0 without the position training mark)"),
         ('aprende_depressa', 'TRAINING_MANAGER::has_fast_position_training', '0x8b4db0', 'training_manager.cpp', 'thiscall, ret 4',
          '1 quando algum treino de posição do jogador tem aprendizado acima de 50',
          '1 when one of the player position training entries has a learning rate above 50')])
CAMPOS = [('pessoa +0 e +0x61: id e ficha de jogador; id e ponteiro; lê (aplicar_o_progresso, achar_o_treino_de_posicao)',
           'person +0 and +0x61: id and player record; id and pointer; reads (aplicar_o_progresso, achar_o_treino_de_posicao)'),
          ('ficha +0x1b, +0x1d, +0x22, +0x29, +0x2e, +0x34, +0x36, +0x3c, +0x3d, +0x3f, +0x40: atributos de 1 a 20; valor; lê e grava (aplicar_o_progresso)',
           'player record +0x1b, +0x1d, +0x22, +0x29, +0x2e, +0x34, +0x36, +0x3c, +0x3d, +0x3f, +0x40: 1 to 20 attributes; value; reads and writes (aplicar_o_progresso)'),
          ('ficha +0x23, +0x24, +0x26, +0x27, +0x2a, +0x2b, +0x31, +0x32, +0x33, +0x35, +0x37, +0x38, +0x39, +0x3a, +0x3e, +0x41: atributos de −125 a 125; valor; lê e grava (aplicar_o_progresso)',
           'player record +0x23, +0x24, +0x26, +0x27, +0x2a, +0x2b, +0x31, +0x32, +0x33, +0x35, +0x37, +0x38, +0x39, +0x3a, +0x3e, +0x41: −125 to 125 attributes; value; reads and writes (aplicar_o_progresso)'),
          ('ficha +0xf..+0x15 e +0x17..+0x19: posições e lados (1 a 20); valor; lê e grava 20 ou 1 (aplicar_o_progresso)',
           'player record +0xf..+0x15 and +0x17..+0x19: positions and sides (1 to 20); value; reads and writes 20 or 1 (aplicar_o_progresso)'),
          ('TRAINING_MANAGER +0x651d: entradas de treino de 6 bytes por ficha (+3 e +4 meio bytes do progresso das categorias 0, 1, 2 e 3; +5 baixo: categoria 4; +5 bit 0x10: tem treino de posição); valor; lê e grava (aplicar_o_progresso), lê (achar_o_treino_de_posicao)',
           'TRAINING_MANAGER +0x651d: 6-byte training entries by player record (+3 and +4 progress nibbles of categories 0, 1, 2 and 3; +5 low: category 4; +5 bit 0x10: has position training); value; reads and writes (aplicar_o_progresso), reads (achar_o_treino_de_posicao)'),
          ('TRAINING_MANAGER +0x6521 lista do treino de posição (itens de 8 bytes: +0 id da pessoa, +4 máscara da posição ou lado, +6 estado, +7 aprendizado), +0x6525 conta; ponteiro e valor; lê (as três)',
           'TRAINING_MANAGER +0x6521 position training list (8-byte entries: +0 person id, +4 position or side mask, +6 state, +7 learning rate), +0x6525 count; pointer and value; reads (all three)')]
INTERESSA = ['CMRTE', 'CMScout']
