# A semana do treino (TRAINING_MANAGER::advance_week_counter 0x8b1730, todo dia pelo laço do dia 0x5cccd0: a semana que
# a update_player_training 0x8b1750 usa para pôr em dia o treino de cada jogador), a reserva de ficha de não jogador
# (PLAYER_REGEN::take_spare_non_player_record 0x7a6e30, pela contract_manager 0x4ce4e0, já em Rust) e a lesão que volta
# (Injury::get_recurring_injury_type 0x62ed30, pelas notícias 0x77bc80, pela avaliação do fisioterapeuta 0x7a2650 e
# pelas telas). Chamadas originais pelo endereço: nenhuma (a 0x7a82b0 já em Rust). Iguais no cm0102-1080.exe. Trocas:
# das candidatas da família, as de player_regen e injury que sobram são de tela ou de notícia montada na pilha
# (0x7a8ba0, 0x7a8eb0, 0x62ee30, 0x6306e0, 0x630af0, 0x631dc0) ou só de tela (0x62bf60, 0x62ec20, 0x631550); as de
# media.cpp que sobram são notícias; youth*.cpp não tem rotina fora do motor. Simulação: as mutações pela carreira até
# 2002-06-01, todas na semana do treino. Comparadores: a semana lado a lado (+0x688d) com a devolução; a lesão que volta
# pelo al; a reserva só a original (tira o item).
SAVE = False
ROTINAS = [('avancar_a_semana', 0x8b1730, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('tirar_a_ficha_reservada', 0x7a6e30, [0x8b, 0x41, 0x1c, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('lesao_que_volta', 0x62ed30, [0x8b, 0x54, 0x24, 0x04, 0x56])]
DOCS = ('a semana do treino, a reserva de ficha de não jogador e a lesão que volta',
        'the training week, the spare non-player record and the recurring injury',
        [('avancar_a_semana', 'TRAINING_MANAGER::advance_week_counter', '0x8b1730', 'training_manager.cpp', 'thiscall, ret 4',
          'no dia do ano múltiplo de 7, a semana do treino sobe 1 (o treino de cada jogador é posto em dia pelas semanas que faltam); devolve o dia / 7',
          'on a day of the year that is a multiple of 7 the training week goes up by 1 (each player training is caught up by the missing weeks); returns the day / 7'),
         ('tirar_a_ficha_reservada', 'PLAYER_REGEN::take_spare_non_player_record', '0x7a6e30', 'player_regen.cpp', 'thiscall, ret 0xc',
          'copia o primeiro item da reserva de fichas de não jogador (id, nação, segunda nação) para as saídas e tira-o da reserva',
          'copies the first spare non-player record entry (id, nation, second nation) to the outputs and removes it from the list'),
         ('lesao_que_volta', 'Injury::get_recurring_injury_type', '0x62ed30', 'injury.cpp', 'thiscall, ret 4',
          'o tipo da lesão que volta do jogador, ou 0xff sem ela ou quando a pessoa não é jogador',
          "the player's recurring injury type, or 0xff when he has none or is not a player")])
CAMPOS = [('[0xae2c90] data do jogo (dia do ano, 16 bits); valor; lê (avancar_a_semana)',
           '[0xae2c90] game date (day of the year, 16 bits); value; reads (avancar_a_semana)'),
          ('TRAINING_MANAGER +0x688d: semana do treino (16 bits); valor; lê e grava (avancar_a_semana)',
           'TRAINING_MANAGER +0x688d: training week (16 bits); value; reads and writes (avancar_a_semana)'),
          ('PLAYER_REGEN +0x1c: reserva de fichas de não jogador (itens de 12 bytes: id, nação, segunda nação); id; lê e tira o primeiro (tirar_a_ficha_reservada)',
           'PLAYER_REGEN +0x1c: spare non-player records (12-byte entries: id, nation, second nation); id; reads and removes the first (tirar_a_ficha_reservada)'),
          ('Injury +0 fichas de forma (31 bytes por pessoa; +0x12 tipo da lesão que volta, 0xff sem ela); valor; lê (lesao_que_volta)',
           'Injury +0 fitness records (31 bytes per person; +0x12 recurring injury type, 0xff for none); value; reads (lesao_que_volta)'),
          ('pessoa +0 e +0x61: id e ficha de jogador; id e ponteiro; lê (lesao_que_volta)',
           'person +0 and +0x61: id and player record; id and pointer; reads (lesao_que_volta)')]
INTERESSA = ['CMRTE', 'SGE']
