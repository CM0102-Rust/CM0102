# As lesões (injury.cpp): sortear a lesão recorrente (Injury::roll_recurring_injury 0x62e500, ~7 mil chamadas na
# carreira, pela lesão nova 0x62d250), recomeçar o histórico de lesões da pessoa (Injury::reset_player_history
# 0x631f90, ~500, pelos regens 0x7a59e0/0x7abac0 e pela 0x62bfa0) e reiniciar o físico do jogador (Injury::reset_player
# 0x62bfa0, ~250, pelo regen jovem 0x7abac0). Trocas: das candidatas da família, as 0x7a7310/0x7a6f10 (x87, SEH) já
# tinham ficado de fora na 247ª e a 0x7a8350 roda uma vez; entram as três de injury.cpp ligadas aos regens. 0x62d250,
# 0x62e700, 0x58f110, 0x417c10 e a lista 0x68d1b0/0x68d1f0 originais; 0x600a50 e o sorteio 0x90cfc0 já em Rust.
# Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem SEH; caixa só com pessoa nula na
# 0x631f90 (fatal, a original). A 0x62e500 sorteia e chama a 0x62d250 (que pode chamá-la de novo); a 0x62bfa0 libera
# itens da lista. Comparadores: a 0x62e500 pelo plano com o estado do sorteio guardado; a 0x631f90 lado a lado; a
# 0x62bfa0 pelo plano (a original roda).
SAVE = False
ROTINAS = [('sortear_lesao_recorrente', 0x62e500, [0x83, 0xec, 0x10, 0x53, 0x55]),
           ('recomecar_historico_de_lesoes', 0x631f90, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('reiniciar_fisico_do_jogador', 0x62bfa0, [0x83, 0xec, 0x08, 0x53, 0x55])]
DOCS = ('as lesões: sortear a lesão recorrente, recomeçar o histórico de lesões e reiniciar o físico do jogador',
        "injuries: rolling a recurring injury, rewinding the injury history and resetting a player's fitness",
        [('sortear_lesao_recorrente', 'Injury::roll_recurring_injury', '0x62e500', 'injury.cpp', 'thiscall, ret 8',
          '1 em 20, a lesão que pode voltar fica recorrente no registro (+0x12 tipo, +0x13 = 0); dos registros com recorrente, o de maior nota (sorteio 0x65 − 5 × nível − propensão, para em 95): jogador de clube grande (reputação > 4250) sem humano e 1 em 3, lesão nova 0x4d/0x4e/0x4f (0x62d250); senão a marca dele cai',
          'with 1 in 20, a recurring-capable injury becomes recurring in the record (+0x12 type, +0x13 = 0); among the recurring records, the best score (roll 0x65 − 5 × level − proneness, stops at 95): a player at a big club (reputation > 4250) without a human and 1 in 3 gets a new injury 0x4d/0x4e/0x4f (0x62d250); otherwise his mark is cleared'),
         ('recomecar_historico_de_lesoes', 'Injury::reset_player_history', '0x631f90', 'injury.cpp', 'thiscall, ret 4',
          'o último deslocamento do histórico de lesões da pessoa (+0x18 do registro de 31 bytes) = −1 (0x58f110)',
          "the last offset of the person's injury history (+0x18 of the 31-byte record) = −1 (0x58f110)"),
         ('reiniciar_fisico_do_jogador', 'Injury::reset_player', '0x62bfa0', 'injury.cpp', 'thiscall, ret 4',
          'tira os itens do id da lista +0x28, termina a lesão (0x62e700), recomeça o histórico (0x631f90) e o registro de 31 bytes (data de hoje, condição e forma 10000, desgaste 0, sem lesão nem recorrente); o bit 0 do +0xc do registro de 79 bytes cai',
          'removes the id items from list +0x28, ends the injury (0x62e700), rewinds the history (0x631f90) and the 31-byte record (today, condition and form 10000, wear 0, no injury nor recurrence); bit 0 of +0xc of the 79-byte record is cleared')])
CAMPOS = [('físico (INJURY [this]): registro de 31 bytes por pessoa: +0 data, +8 condição, +0xa forma, +0xc desgaste, +0xe índice da lesão (−1 sem), +0x12 tipo recorrente (0xff sem), +0x13 nível, +0x14/+0x18 histórico, +0x1d; valores; lê e grava (sortear_lesao_recorrente, recomecar_historico_de_lesoes, reiniciar_fisico_do_jogador)',
           'fitness (INJURY [this]): 31-byte record per person: +0 date, +8 condition, +0xa form, +0xc wear, +0xe injury index (−1 none), +0x12 recurring type (0xff none), +0x13 level, +0x14/+0x18 history, +0x1d; values; reads and writes (sortear_lesao_recorrente, recomecar_historico_de_lesoes, reiniciar_fisico_do_jogador)'),
          ('INJURY +0x28: lista de itens por id; lê e tira (reiniciar_fisico_do_jogador); +0xc arquivo do histórico (recomecar_historico_de_lesoes)',
           'INJURY +0x28: list of items by id; reads and removes (reiniciar_fisico_do_jogador); +0xc history file (recomecar_historico_de_lesoes)'),
          ('[0x9e7ae0 + 13 × tipo]: a lesão pode voltar; valor; lê (sortear_lesao_recorrente)',
           '[0x9e7ae0 + 13 × type]: the injury can recur; value; reads (sortear_lesao_recorrente)'),
          ('pessoa +0x39 clube, +0x61 jogador; jogador +0x2d propensão a lesões; clube +0x80 reputação; lê (sortear_lesao_recorrente)',
           'person +0x39 club, +0x61 player; player +0x2d injury proneness; club +0x80 reputation; reads (sortear_lesao_recorrente)'),
          ('[0xae2d0c] registro de 79 bytes +0xc: bit 0; grava 0 (reiniciar_fisico_do_jogador)',
           '[0xae2d0c] 79-byte record +0xc: bit 0; writes 0 (reiniciar_fisico_do_jogador)')]
INTERESSA = ['CMRTE', 'SGE', 'GPF2', 'CMScout']
