# O progresso do treino guardado: tirar e devolver o progresso, o quanto o treino soma a um atributo e a aptidao para
# uma posicao (training_manager.cpp).
# - 0x8b3640 (TRAINING_MANAGER::remove_or_restore_training_progress, 384 bytes): pelas normalizacoes dos atributos do
#   jogador (0x538bc0, 0x538ec0), que rodam no envelhecimento do diario das pessoas (0x5ce2d0, 0x5d5772), no regen
#   jovem (0x7abac0, 0x7acf9b/0x7acfa1) e na carga; tira o progresso (guardando-o em 0xde1f3c..0xde1f40) e devolve. As
#   mutacoes estao nela.
# - 0x8b37c0 (TRAINING_MANAGER::get_training_attribute_change): passageira (so a tela do treino 0x8b8184 a chama).
# - 0x8b4770 (TRAINING_MANAGER::get_position_aptitude): passageira (a reclamacao do treino de posicao na 0x8b1750 e a
#   0x8b4450; o treino de posicao so o tecnico humano liga).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x8b3640: thiscall (mov ecx, [0xde1f44] em 0x538be9; push ebx = 0 ou 1; push esi), 2 (ret 8). Dentro: 0x8b2be0
#   thiscall (o ecx do this que chegou), 2 (pessoa, &progresso de 5 bytes).
#   0x8b37c0: thiscall (mov ecx, [0xde1f44] em 0x8b8177; push eax = o atributo no al, push ebp), 2 (ret 8). Dentro:
#   0x5431e0 cdecl (add esp, 8), 2 (pessoa, o dword do atributo que chegou: ela le o byte); 0x8b2be0 thiscall (mov ecx,
#   ebp = this), 2, duas vezes.
#   0x8b4770: thiscall (mov ecx, [esp+0x10] em 0x8b4620 e [esp+0x28] em 0x8b206a; push mascara, push pessoa), 2 (ret 8;
#   le a palavra da mascara; devolve no al, os chamadores fazem movsx/mov bl, al). Dentro: 0x544380 cdecl (add esp,
#   0x10 ou 0x18 juntando as chamadas), 2 (a ficha, o byte do atributo num registrador com bytes de cima que sobram:
#   ela le o byte).
# Comparadores: 0x8b3640 lado a lado (ficha +0xf..+0x41, a entrada do treino e o guardado em 0xde1f3c); 0x8b37c0 lado a
# lado (ficha e entrada) com a devolucao; 0x8b4770 pela devolucao inteira (so le). Os casos de erro (a caixa), so a
# original. Iguais no cm0102-1080.exe (conferido byte a byte).
SAVE = False
ROTINAS = [('tirar_ou_devolver_o_progresso', 0x8b3640, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('mudanca_do_atributo_pelo_treino', 0x8b37c0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('aptidao_para_a_posicao', 0x8b4770, [0x0f, 0xbf, 0x44, 0x24, 0x08])]
DOCS = ('o progresso do treino guardado: tirar e devolver o progresso, o quanto o treino soma a um atributo e a aptidão para uma posição',
        'the stored training progress: remove and restore the progress, how much training adds to an attribute and the aptitude for a position',
        [('tirar_ou_devolver_o_progresso', 'TRAINING_MANAGER::remove_or_restore_training_progress', '0x8b3640', 'training_manager.cpp', 'thiscall',
          'tira dos atributos o progresso guardado na entrada do treino (guardando os cinco meios bytes) ou o devolve, em volta das normalizações dos atributos do jogador',
          'takes the progress stored in the training entry off the attributes (keeping the five nibbles) or puts it back, around the player attribute normalisations'),
         ('mudanca_do_atributo_pelo_treino', 'TRAINING_MANAGER::get_training_attribute_change', '0x8b37c0', 'training_manager.cpp', 'thiscall',
          'quanto o progresso do treino soma a um atributo: o atributo menos o mesmo atributo sem o progresso',
          'how much the training progress adds to an attribute: the attribute minus the same attribute without the progress'),
         ('aptidao_para_a_posicao', 'TRAINING_MANAGER::get_position_aptitude', '0x8b4770', 'training_manager.cpp', 'thiscall',
          'a aptidão do jogador para aprender uma posição ou um lado: a média da adaptabilidade com os atributos que a posição pede',
          'the player aptitude to learn a position or side: the average of the adaptability and the attributes the position needs')])
CAMPOS = [('treino (TRAINING_MANAGER [0xde1f44]) +0x651d entradas de 6 bytes por ficha (+3..+5 meios bytes do progresso); ponteiro/valor; lê (e grava pela 0x8b2be0)',
           'training (TRAINING_MANAGER [0xde1f44]) +0x651d 6-byte entries per record (+3..+5 progress nibbles); pointer/value; reads (and writes through 0x8b2be0)'),
          ('0xde1f3c..0xde1f40 o progresso tirado (categorias 3, 2, 4, 0, 1); valor; grava e lê (tirar_ou_devolver_o_progresso)',
           '0xde1f3c..0xde1f40 the removed progress (categories 3, 2, 4, 0, 1); value; writes and reads (tirar_ou_devolver_o_progresso)'),
          ('pessoa +0x56 adaptabilidade; valor; lê; ficha +0x27, +0x2a, +0x30, +0x32, +0x33, +0x35..+0x37, +0x39, +0x3b, +0x3e..+0x40 atributos; valor; lê (aptidao_para_a_posicao)',
           'person +0x56 adaptability; value; reads; record +0x27, +0x2a, +0x30, +0x32, +0x33, +0x35..+0x37, +0x39, +0x3b, +0x3e..+0x40 attributes; value; reads (aptidao_para_a_posicao)')]
INTERESSA = ['CMRTE', 'CMScout']
