# As fichas de treino iniciais dos jogadores (TRAINING_MANAGER::init_player_entries 0x8b3c50, pelo construtor 0x8b0ec0
# num jogo novo: a vaga do programa de cada ficha de jogador, goleiros no 6 e os outros no 1; a primeira atualizacao do
# treino de cada jogador, 0x8b1750, usa essa vaga antes de refaze-la no computador, e o clube humano a usa sempre), o
# banco de jogadores livres do regen (PLAYER_REGEN::add_player_data_to_pool 0x7a8350, pela process_retirements 0x7a59e0
# quando um jogador com ficha de nao jogador deixa de jogar; ~1 chamada na carreira) e se ha reserva de ficha de nao
# jogador (PLAYER_REGEN::has_spare_non_player_record 0x7a6e20, pela 0x4ceb03). Chamadas originais pelo endereco: calloc
# 0x945501, realloc 0x945667 e as caixas de erro (aviso_em, cpk2_sem_memoria); os ajudantes cpk2_* ja estao no motor.rs.
# Iguais no cm0102-1080.exe. Trocas: das candidatas da familia sobraram poucas que rodem na carreira; a 0x62b7b0
# (Injury::daily_update) chama oito originais nao portadas e fica de fora; as de noticia (0x7a8ba0, 0x7a92e0, 0x7aa340,
# 0x62dca0) e as so de tela (0x62bf60, 0x631550) tambem. Simulacao: as mutacoes na 0x8b3c50 pela carreira ate 2002-06-01
# (a montagem roda no comeco de cada carreira nova); a 0x7a8350 e a 0x7a6e20 vao de passageiras. Comparadores: a 0x8b3c50
# pelo plano (aloca: so a original roda, confere cada ficha pela regra); a 0x7a8350 pelo item novo (realoca: so a
# original); a 0x7a6e20 pela devolucao.
SAVE = False
ROTINAS = [('fichas_de_treino_iniciais', 0x8b3c50, [0xa1, 0x6c, 0x23, 0xae, 0x00]),
           ('guardar_jogador_no_banco', 0x7a8350, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tem_reserva_de_tecnico', 0x7a6e20, [0x8b, 0x41, 0x20, 0x85, 0xc0])]
DOCS = ('as fichas de treino iniciais dos jogadores, o banco de jogadores livres do regen e se há reserva de não jogador',
        "the players' initial training entries, the regen pool of free player records and whether there is a spare non-player record",
        [('fichas_de_treino_iniciais', 'TRAINING_MANAGER::init_player_entries', '0x8b3c50', 'training_manager.cpp', 'thiscall',
          'aloca a ficha de treino de 6 bytes de cada ficha de jogador e põe os goleiros (nota de goleiro de 15 para cima) no programa 6 e os outros no 1',
          'allocates the 6-byte training entry of every player data record and puts goalkeepers (goalkeeping rating 15 or more) on schedule 6 and the others on 1'),
         ('guardar_jogador_no_banco', 'PLAYER_REGEN::add_player_data_to_pool', '0x7a8350', 'player_regen.cpp', 'thiscall, ret 0xc',
          'acrescenta ao banco de jogadores livres o id da ficha de jogador de uma pessoa que deixa de jogar, com a nação e a segunda nação, crescendo a tabela de 250 em 250',
          'appends to the pool of free player records the player data id of a person who stops playing, with the nation and second nation, growing the table by 250'),
         ('tem_reserva_de_tecnico', 'PLAYER_REGEN::has_spare_non_player_record', '0x7a6e20', 'player_regen.cpp', 'thiscall',
          'se a reserva de fichas de não jogador tem alguma',
          'whether the spare non-player record list holds any entry')])
CAMPOS = [('TRAINING_MANAGER +0x651d: fichas de treino de 6 bytes por ficha de jogador (+0 vaga do programa, 6 goleiros ou 1 geral; +5 bit 0x20); ponteiro e valor; grava (fichas_de_treino_iniciais)',
           'TRAINING_MANAGER +0x651d: 6-byte training entries per player data record (+0 schedule slot, 6 goalkeepers or 1 general; +5 bit 0x20); pointer and value; writes (fichas_de_treino_iniciais)'),
          ('ficha de jogador (0x46 bytes em [0xae23c4], [0xae236c] fichas) +0: id; +0xf: nota de goleiro (com sinal); id e valor; lê (fichas_de_treino_iniciais)',
           'player data record (0x46 bytes at [0xae23c4], [0xae236c] records) +0: id; +0xf: goalkeeping rating (signed); id and value; reads (fichas_de_treino_iniciais)'),
          ('pessoa +0x61: ficha de jogador (o id em +0); ponteiro e id; lê (guardar_jogador_no_banco)',
           'person +0x61: player data record (id at +0); pointer and id; reads (guardar_jogador_no_banco)'),
          ('PLAYER_REGEN +0x3c banco de jogadores livres (16 bytes: id da ficha, nação, segunda nação), +0x40 conta, +0x44 capacidade; ponteiro, id e valor; acrescenta (guardar_jogador_no_banco)',
           'PLAYER_REGEN +0x3c pool of free player records (16 bytes: record id, nation, second nation), +0x40 count, +0x44 capacity; pointer, id and value; appends (guardar_jogador_no_banco)'),
          ('PLAYER_REGEN +0x20: conta da reserva de fichas de não jogador; valor; lê (tem_reserva_de_tecnico)',
           'PLAYER_REGEN +0x20: count of the spare non-player records; value; reads (tem_reserva_de_tecnico)')]
INTERESSA = ['CMRTE', 'GPF2']
