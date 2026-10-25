# Rodam na criação do jogo: database_player_initialisation normaliza cada jogador (0x538ec0 e 0x538bc0, no
# segundo laço, 0x538a62/0x538a68) e OfficialsManager::set_up_officials cria árbitros com o segundo nome pelo
# peso (0x53a0e0). Jogadores e árbitros vão para o .qst da prova do save.
# Troca: Staff::operator= (0x5335f0) saiu: só é chamada pelo embaralhamento de "Use Real Players: No"
# (0x533080) e pelos jogadores virtuais (VIRTUAL_STAFF::relocate_virtual_players 0x90e9a0, na temporada): a
# prova do save não a alcança. Entrou 0x538bc0 (C4, MAPA §4), irmã da 0x538ec0 (o mesmo reequilíbrio).
SAVE = True
ROTINAS = [('normalizar_atributos_fisicos_do_jogador', 0x538ec0, [0x81, 0xec, 0x78, 0x02, 0x00, 0x00]),
           ('segundo_nome_pelo_peso', 0x53a0e0, [0x51, 0x8b, 0x44, 0x24, 0x08]),
           ('normalizar_atributos_do_jogador', 0x538bc0, [0x81, 0xec, 0x74, 0x02, 0x00, 0x00])]
DOCS = ('a normalização física e geral dos atributos do jogador e o segundo nome pelo peso',
        'the physical and general normalisation of player attributes and the second name by weight',
        [('normalizar_atributos_fisicos_do_jogador', 'staff_normalise_player_physical_attributes', '0x538ec0', 'Database.cpp', 'cdecl',
          'baixa os físicos que passam da habilidade atual (corte pela soma ponderada com as constantes do .rdata) e reequilibra o vetor de atributos sem as células 0x18 e 0x1e, alvo de −40 a 0; com +0xf = 20 tira médias de pares e trios; tira e devolve o progresso do treino',
          'lowers the physical attributes above the current ability (cut by the weighted sum with the .rdata constants) and rebalances the attribute vector without cells 0x18 and 0x1e, target −40 to 0; with +0xf = 20 removes pair and trio averages; removes and restores the training progress'),
         ('segundo_nome_pelo_peso', 'database_pick_second_name_by_weight', '0x53a0e0', 'Database.cpp', 'cdecl',
          'um segundo nome: com a nação tendo mais de 10, o de maior peso entre 3 sorteios na faixa dela; senão sorteio na tabela inteira',
          'a second name: with the nation having more than 10, the heaviest of 3 draws in its range; otherwise a draw over the whole table'),
         ('normalizar_atributos_do_jogador', 'staff_normalise_player_attributes', '0x538bc0', 'Database.cpp', 'cdecl',
          'reequilibra o vetor de atributos do jogador com alvo −20 (12 células no limite, excesso acima de 85 repartido); com +0xf ≥ 15 tira a média de +0x2a/+0x3a; tira e devolve o progresso do treino',
          'rebalances the player attribute vector with target −20 (12 cells within the limit, excess above 85 spread); with +0xf ≥ 15 removes the +0x2a/+0x3a average; removes and restores the training progress')])
CAMPOS = [('pessoa +0x61: jogador; ponteiro; lê. Jogador (70 bytes) +0x0f..+0x44: atributos; bytes com sinal; lê e escreve (as duas normalizações, pelo 0x53a610/0x53aa10 e direto: +0x1b, +0x36, +0x40, +0x44, +0x2a, +0x3a, +0x1e, +0x24, +0x39)', 'staff +0x61: player; pointer; reads. Player (70 bytes) +0x0f..+0x44: attributes; signed bytes; reads and writes (both normalisations, through 0x53a610/0x53aa10 and directly: +0x1b, +0x36, +0x40, +0x44, +0x2a, +0x3a, +0x1e, +0x24, +0x39)'),
          ('jogador +5: habilidade atual; palavra com sinal; lê (normalizar_atributos_fisicos_do_jogador)', 'player +5: current ability; signed word; reads (normalizar_atributos_fisicos_do_jogador)'),
          ('pessoa +0x58: célula 54 do vetor; byte; escreve (pelo 0x53aa10)', 'staff +0x58: vector cell 54; byte; writes (through 0x53aa10)'),
          ('[0xde1f44]: TRAINING_MANAGER; ponteiro; lê (as duas normalizações: tirar e devolver o progresso, 0x8b3640)', '[0xde1f44]: TRAINING_MANAGER; pointer; reads (both normalisations: remove and restore the progress, 0x8b3640)'),
          ('índice de nomes da nação ([0xae2338] + id × 36) +4 / +0x1c: primeiro e quantos segundos nomes; valor; lê. [0xae23f4] (60 bytes) +0x3b: peso; byte com sinal; lê (segundo_nome_pelo_peso)', 'nation name index ([0xae2338] + id × 36) +4 / +0x1c: first and count of second names; value; reads. [0xae23f4] (60 bytes) +0x3b: weight; signed byte; reads (segundo_nome_pelo_peso)')]
INTERESSA = ['SGE', 'CMRTE']
