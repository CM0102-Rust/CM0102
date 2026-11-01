# O treino do jogador posto em dia: o progresso das semanas, o programa do jogador e o nivel da categoria
# (training_manager.cpp).
# - 0x8b1750 (TRAINING_MANAGER::update_player_training, 2.784 bytes): pela iniciacao do jogador na partida (0x6eaea0,
#   match_man.cpp, todo jogador de toda partida, com 0), pela lesao nova (0x62d250, ja em Rust) e pelas telas. Poe em dia
#   as semanas do treino (+0x688d contra +1 da entrada de 6 bytes): os sorteios do progresso das cinco categorias pelo
#   peso do programa (nivel x 2 + 60 + o fator do treinador), os treinos de posicao e o progresso nos atributos
#   (0x8b2be0). Os programas padrao tem niveis de 10 a 50 (0x8b4e50): pesos abaixo de 100 (a descida) e acima (a
#   subida) rodam na carreira. As mutacoes estao nela.
# - 0x8b39f0 (TRAINING_MANAGER::get_player_schedule): pelo diario das pessoas (0x5ce2d0), pela 0x5d6aa0 e pela 0x8b1750.
# - 0x8b4f50 (TRAINING_SCHEDULE::get_category_level, 12 bytes; o prologo movsx de 5 bytes): pela 0x8b1750, cinco vezes
#   por atualizacao.
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x8b1750: thiscall (mov ecx, [0xde1f44] em 0x6eb03f; push 0; push ebp), 2 na pilha (ret 8; a 2a e um short).
#   Dentro: 0x8b39f0 thiscall (mov ecx, ebp = this), 1; 0x4cf5a0 thiscall (mov ecx, [0xae1050]), 1, o retorno guardado
#   em [0xae1080]; 0x8b4f60 thiscall (mov ecx, eax = o programa), 0; 0x62c660 thiscall (mov ecx, [0xb63d28]), 2 (pessoa,
#   carga); 0x8b4b20 thiscall (mov ecx, ebp), 2 (pessoa, &primeiro); 0x8b4450 thiscall (mov ecx, ebp), 2 (item, 0);
#   0x8b4f50 thiscall (mov ecx, [esp+0x40] = o programa), 1 (o dword da categoria com bytes de cima da pilha: le o
#   byte); 0x8b41d0 thiscall (mov ecx, [esp+0x24] = this), 2 (clube, categoria); 0x90cfc0 cdecl (add esp, 4), 1;
#   0x4cf5f0 thiscall (mov ecx, [0xae1050]), 2 (pessoa, +0x39); 0x8b4770 thiscall (mov ecx, [esp+0x28] = this depois do
#   primeiro push), 2 (pessoa, mascara no ecx com os 16 bits de cima que sobram: ela le a palavra); 0x62c400 thiscall
#   (mov ecx, [0xb63d28]), 1; 0x867700 thiscall (mov ecx, edi = o contrato), 2 (0x8000, 0); 0x5885c0 cdecl (add esp, 4),
#   1; 0x8b2be0 thiscall (mov ecx, ebx = this), 2 (pessoa, &progresso); 0x945333 cdecl (add esp, 0x10), 4 (base, conta,
#   8, 0x8b4e00).
#   0x8b39f0: thiscall (mov ecx, ebp em 0x8b1805; mov ecx, [0xde1f44] em 0x5cf367, 0x5d3786 e 0x5d6b48), 1 (ret 4,
#   push ebp/esi). Dentro: 0x4cf5a0
#   thiscall (mov ecx, [0xae1050]), 1; 0x62c400 thiscall (mov ecx, [0xb63d28]), 1.
#   0x8b4f50: thiscall (mov ecx, [esp+0x40] em 0x8b1d8b), 1 (ret 4): movsx eax, byte [esp+4]; mov al, [eax+ecx+4].
# Comparadores: 0x8b1750 lado a lado (estado do sorteador, [0xae1080], a entrada do treino, a ficha +0xf..+0x41 e a
# carga da ficha de forma), com os sorteios da copia fora da contagem; so a original nos casos de erro e no jogador com
# treino de posicao (ali a 0x8b4450, a 0x867700 e a 0x5885c0 mexem fora das regioes). 0x8b39f0 pela devolucao e
# [0xae1080] lado a lado. 0x8b4f50 pela devolucao inteira. Iguais no cm0102-1080.exe (conferido byte a byte).
SAVE = False
ROTINAS = [('atualizar_o_treino_do_jogador', 0x8b1750, [0x81, 0xec, 0x34, 0x04, 0x00, 0x00]),
           ('programa_de_treino_do_jogador', 0x8b39f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nivel_da_categoria_do_programa', 0x8b4f50, [0x0f, 0xbe, 0x44, 0x24, 0x04])]
DOCS = ('o treino do jogador posto em dia: o progresso das semanas, o programa do jogador e o nível da categoria',
        'player training brought up to date: the weekly progress, the player schedule and the category level',
        [('atualizar_o_treino_do_jogador', 'TRAINING_MANAGER::update_player_training', '0x8b1750', 'training_manager.cpp', 'thiscall',
          'põe em dia as semanas de treino do jogador (até 30): o programa do clube de técnico humano, os sorteios do progresso de cada categoria pelo peso do programa e do treinador, os treinos de posição (a nota, o fim e a reclamação do jogador) e o progresso nos atributos',
          'brings the player training weeks up to date (up to 30): the human club schedule, the progress rolls of each category by the schedule and coach weight, the position training (the rating, the end and the player complaint) and the progress on the attributes'),
         ('programa_de_treino_do_jogador', 'TRAINING_MANAGER::get_player_schedule', '0x8b39f0', 'training_manager.cpp', 'thiscall',
          'o programa de treino do jogador: o do técnico humano do clube pela vaga da entrada (o de descanso se lesionado) ou o programa padrão da vaga',
          'the player training schedule: the club human manager one by the entry slot (the rest one when injured) or the default schedule of the slot'),
         ('nivel_da_categoria_do_programa', 'TRAINING_SCHEDULE::get_category_level', '0x8b4f50', 'training_manager.cpp', 'thiscall',
          'o nível de uma das cinco categorias do programa de treino',
          'the level of one of the five training schedule categories')])
CAMPOS = [('treino (TRAINING_MANAGER [0xde1f44]) +0x651d entradas de 6 bytes por ficha de jogador (+0 vaga, +1 semana, +3..+5 meios bytes do progresso, +5 marcas 0x10/0x20); ponteiro/valor; lê e grava (atualizar_o_treino_do_jogador)',
           'training (TRAINING_MANAGER [0xde1f44]) +0x651d 6-byte entries per player record (+0 slot, +1 week, +3..+5 progress nibbles, +5 flags 0x10/0x20); pointer/value; reads and writes (atualizar_o_treino_do_jogador)'),
          ('treino +0x688d semana do treino; valor; lê; +0x6521/+0x6525 lista do treino de posição (8 bytes: +0 id, +4 máscara, +6 nota, +7 taxa); ponteiro/valor; lê e grava',
           'training +0x688d training week; value; reads; +0x6521/+0x6525 position training list (8 bytes: +0 id, +4 mask, +6 rating, +7 rate); pointer/value; reads and writes'),
          ('treino + 0x9b × vaga programas (+4..+8 níveis, +0x15 tipo); os dos humanos em +0x4d8 + 0x60e × índice; valor; lê (programa_de_treino_do_jogador, nivel_da_categoria_do_programa)',
           'training + 0x9b × slot schedules (+4..+8 levels, +0x15 type); the human ones at +0x4d8 + 0x60e × index; value; reads (programa_de_treino_do_jogador, nivel_da_categoria_do_programa)'),
          ('pessoa +0x39 clube, +0x58 e +0x5b (limites do progresso), +0x61 ficha (+0xf goleiro, +0x1b..+0x41 atributos pela 0x8b2be0); valor/ponteiro; lê',
           'person +0x39 club, +0x58 and +0x5b (progress limits), +0x61 record (+0xf goalkeeper, +0x1b..+0x41 attributes through 0x8b2be0); value/pointer; reads'),
          ('contrato do clube +0x44 felicidade, +0x45 marca 0x8000; valor; lê e grava (a reclamação do treino de posição)',
           'club contract +0x44 happiness, +0x45 flag 0x8000; value; reads and writes (the position training complaint)'),
          ('[0xae1080] o último contrato consultado; ponteiro; grava; [[0xb63d28]] + 31 × id + 0x1e carga de treino; valor; grava',
           '[0xae1080] last contract looked up; pointer; writes; [[0xb63d28]] + 31 × id + 0x1e training load; value; writes')]
INTERESSA = ['CMRTE', 'SGE', 'CMScout']
