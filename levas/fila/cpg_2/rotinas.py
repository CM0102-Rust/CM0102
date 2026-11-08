# Cópias de registros, 2º item: a cópia da entrada da fila de regens (regen_queue_entry_assign 0x7a8b70, 30.195 chamadas
# na carreira até 2002-06-01; player_regen.cpp), a busca na lista de observados (Shortlist_Manager::is_on_shortlist
# 0x842080, 30.929; shortlist_manager.cpp; bsearch 0x946464 com a comparação 0x757ea0) e a escolha da pessoa que doa os
# dados de não jogador a um regen (player_regen_pick_staff_for_non_player_data 0x7a6e60, 22; a nota 0x7a7150 sorteia).
# Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem
# local lida sem valor (a data local é gravada pela 0x417c10; a chave do bsearch é toda gravada). Comparadores: a
# cópia lado a lado pela entrada; a busca pelo valor (o eax inteiro: os 3 bytes altos são os do ponteiro achado); a
# escolha lado a lado pelo estado do sorteio, a cópia em without_counting. Mais chamadas: a cópia e a busca (~80 por dia de
# jogo), sem laço por dentro; correm o risco da 0x689ad0 (241ª), de o desvio mudar o resíduo de pilha.
SAVE = False
ROTINAS = [('copiar_entrada_da_fila_de_regens', 0x7a8b70, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08]),
           ('esta_na_lista_de_observados', 0x842080, [0x8b, 0x01, 0x83, 0xec, 0x08]),
           ('escolher_doador_de_dados', 0x7a6e60, [0x51, 0xa1, 0x64, 0x23, 0xae, 0x00])]
DOCS = ('a cópia da entrada da fila de regens, a busca na lista de observados e a escolha do doador de dados de não jogador',
        'the copy of the regen queue entry, the shortlist lookup and the choice of the non-player data donor',
        [('copiar_entrada_da_fila_de_regens', 'regen_queue_entry_assign', '0x7a8b70', 'player_regen.cpp', 'thiscall, ret 4',
          'copia a entrada da fila de regens (0xc bytes): a pessoa (+0) e a data do regen (+4)',
          'copies the regen queue entry (0xc bytes): the person (+0) and the regen date (+4)'),
         ('esta_na_lista_de_observados', 'Shortlist_Manager::is_on_shortlist', '0x842080', 'shortlist_manager.cpp', 'thiscall, ret 8',
          'o par (jogador, clube ou −1) está nos pares observados? bsearch com a 0x757ea0',
          'is the (player, club or −1) pair in the shortlist pairs? bsearch with 0x757ea0'),
         ('escolher_doador_de_dados', 'player_regen_pick_staff_for_non_player_data', '0x7a6e60', 'player_regen.cpp', 'stdcall, ret 4',
          'a pessoa não jogadora de maior nota positiva (0x7a7150) da nação; sem nenhuma, de qualquer nação; senão 0',
          'the non-player with the highest positive score (0x7a7150) of the nation; with none, of any nation; else 0')])
CAMPOS = [('entrada da fila de regens (0xc bytes: pessoa + data); escreve (copiar_entrada_da_fila_de_regens); pares observados (8 bytes: jogador, clube ou −1), vetor em [this] e quantos em +0x10; lê (esta_na_lista_de_observados); não jogadores ([0xae23bc], 0x6e bytes, +0x1a nação); lê (escolher_doador_de_dados)',
           'regen queue entry (0xc bytes: person + date); writes (copiar_entrada_da_fila_de_regens); shortlist pairs (8 bytes: player, club or −1), array at [this] and count at +0x10; reads (esta_na_lista_de_observados); non-players ([0xae23bc], 0x6e bytes, +0x1a nation); reads (escolher_doador_de_dados)')]
INTERESSA = ['SGE', 'CMRTE']
