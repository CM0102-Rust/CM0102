# As listas do regen (player_regen.cpp): acrescentar a reserva de não jogador (PLAYER_REGEN::add_spare_non_player_record
# 0x7a81a0; tirada pela 0x7a82b0, já em Rust), a aposentadoria pendente (PLAYER_REGEN::add_retirement 0x7a8510, ~800
# chamadas na carreira) e a vaga para o regen (PLAYER_REGEN::queue_staff_for_regen 0x7a87d0, ~260; a fila do regen, a
# reserva de não jogador e o banco de jogadores livres, tirado pela 0x7a8470). Trocas: as candidatas 0x7a7310 e
# 0x7a6f10 (79 e 98 bytes no mapa) têm 1,7 KB (x87) e 576 bytes (SEH); as de media.cpp 0x736720 e 0x73e870 têm 1 KB
# e 10 KB; entram as listas do regen. Quirks fiéis: a 0x7a81a0 decide crescer pela conta do banco (+0x40/+0x44) e a
# 0x7a87d0, no banco, confere a conta e não o ponteiro depois do realloc. Simulação: as mutações pela carreira e a
# confirmação no jogo real comparando. Sem SEH; caixa só sem memória ou id fora (a original); a 0x7a87d0 sorteia a
# nação de quem não tem. Comparadores: a original roda (realoca, sorteia) e o resultado é conferido com o plano.
SAVE = False
ROTINAS = [('acrescentar_reserva_de_tecnico', 0x7a81a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('acrescentar_aposentadoria', 0x7a8510, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('guardar_vaga_para_regen', 0x7a87d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as listas do regen: a reserva de técnico, a aposentadoria pendente e a vaga para o regen',
        'the regen lists: the spare non-player record, the pending retirement and the regen slot',
        [('acrescentar_reserva_de_tecnico', 'PLAYER_REGEN::add_spare_non_player_record', '0x7a81a0', 'player_regen.cpp', 'thiscall, ret 0xc',
          'acrescenta (id, nação, segunda nação) à lista de 16 bytes das reservas de não jogador (+0x1c); cresce 250 quando o banco de jogadores (+0x40/+0x44) enche, como a original',
          'appends (id, nation, second nation) to the 16-byte spare non-player list (+0x1c); grows by 250 when the player pool (+0x40/+0x44) is full, as the original'),
         ('acrescentar_aposentadoria', 'PLAYER_REGEN::add_retirement', '0x7a8510', 'player_regen.cpp', 'thiscall, ret 4',
          'acrescenta os campos do registro à tabela de 26 bytes das aposentadorias pendentes (+4), que cresce 250',
          'appends the record fields to the 26-byte pending retirement table (+4), growing by 250'),
         ('guardar_vaga_para_regen', 'PLAYER_REGEN::queue_staff_for_regen', '0x7a87d0', 'player_regen.cpp', 'thiscall, ret 4',
          'a pessoa entra na fila do regen (+0x10) com as datas; os dados de não jogador na reserva (+0x1c, nação sorteada se não houver) e os de jogador no banco de livres (+0x3c)',
          'the person enters the regen queue (+0x10) with the dates; the non-player data in the spare list (+0x1c, random nation if none) and the player data in the free pool (+0x3c)')])
CAMPOS = [('regen +0x10/+0x14/+0x18: fila do regen, itens de 12 bytes (+0 id da pessoa, +4 e +6 palavras da entrada), conta, capacidade; grava (guardar_vaga_para_regen)',
           'regen +0x10/+0x14/+0x18: regen queue, 12-byte items (+0 person id, +4 and +6 entry words), count, capacity; writes (guardar_vaga_para_regen)'),
          ('regen +0x1c/+0x20/+0x24: reservas de não jogador, itens de 16 bytes (+0 id, +4 nação, +8 segunda nação ou +0xc pessoa), conta, capacidade; grava (acrescentar_reserva_de_tecnico, guardar_vaga_para_regen)',
           'regen +0x1c/+0x20/+0x24: spare non-player records, 16-byte items (+0 id, +4 nation, +8 second nation or +0xc person), count, capacity; writes (acrescentar_reserva_de_tecnico, guardar_vaga_para_regen)'),
          ('regen +0x3c/+0x40/+0x44: banco de jogadores livres, itens de 16 bytes (+0 id dos dados de jogador, +4 nação, +0xc pessoa), conta, capacidade; lê (acrescentar_reserva_de_tecnico) e grava (guardar_vaga_para_regen)',
           'regen +0x3c/+0x40/+0x44: free player pool, 16-byte items (+0 player data id, +4 nation, +0xc person), count, capacity; reads (acrescentar_reserva_de_tecnico) and writes (guardar_vaga_para_regen)'),
          ('regen +4/+8/+0xc: aposentadorias pendentes, itens de 26 bytes (+0, +4, +8, +0xa, +0x10, +0x12, +0x18, +0x19 do registro), conta, capacidade; grava (acrescentar_aposentadoria)',
           'regen +4/+8/+0xc: pending retirements, 26-byte items (+0, +4, +8, +0xa, +0x10, +0x12, +0x18, +0x19 of the record), count, capacity; writes (acrescentar_aposentadoria)'),
          ('pessoa (0x6e bytes em [0xae23bc]) +0 id, +0x1a nação, +0x61 dados de jogador, +0x69 dados de não jogador; ponteiros; lê (guardar_vaga_para_regen)',
           'person (0x6e bytes at [0xae23bc]) +0 id, +0x1a nation, +0x61 player data, +0x69 non-player data; pointers; reads (guardar_vaga_para_regen)')]
INTERESSA = ['GPF2', 'SGE', 'CMRTE']
