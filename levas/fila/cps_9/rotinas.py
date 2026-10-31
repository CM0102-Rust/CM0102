# Os gols na carreira do jogador (player_stats_get_career_goals 0x7c3d20, pela MATCH_PLAYER::setup 0x6eaea0 em cada
# jogador de cada partida, gravados em +0x43 do jogador da partida), o registro do empréstimo
# (Staff_Records::add_loan_records 0x86da20, pela transferência aceita 0x8c2180) e a ordem da fila das voltas aos
# treinos (injury_compare_entry_dates 0x632040, o comparador do bsearch em Injury::post_due_recovery_news 0x62e010, todo
# dia pela 0x62b7b0). A 0x86da20 tem SEH só da data local (destrutor vazio 0x5a53c0) e usa os ajudantes do histórico da
# 253ª (cpq4_*). Chamadas originais pelo endereço: 0x7c3e40 (monta a tabela da carreira, malloc), 0x9452ca (free),
# 0x456770 e 0x456740 (datas); o resto já em Rust. Iguais no cm0102-1080.exe. Trocas: das candidatas da família,
# player_stats_years_since_at_club 0x7c6e30 só roda pela névoa do humano (FOG_OF_WAR); a schedule_injury_retirement
# 0x7a7f20 monta a entrada de aposentadoria para a add_retirement 0x7a8510 (DESLIGADA): fora; a apps_for_club 0x7c3a40
# já é porta do pós-jogo; as de texto (0x62ef30, 0x631580, 0x7aa4c0) ficam de fora. Simulação: as mutações pela
# carreira até 2002-06-01. Comparadores: os gols e a ordem pela devolução (a original e a cópia), sem os casos de erro;
# o registro só a original (o histórico vai para o arquivo).
SAVE = False
ROTINAS = [('gols_na_carreira', 0x7c3d20, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('registrar_o_emprestimo', 0x86da20, [0x6a, 0xff, 0x68, 0x9b, 0x2d, 0x96, 0x00]),
           ('ordem_da_volta_aos_treinos', 0x632040, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('os gols na carreira do jogador, o registro do empréstimo e a ordem da fila das voltas aos treinos',
        "the player's career goals, the loan record and the order of the return to training queue",
        [('gols_na_carreira', 'player_stats_get_career_goals', '0x7c3d20', 'player_stats.cpp', 'stdcall, ret 4',
          'soma os gols de todas as temporadas e clubes da tabela da carreira do jogador (0x7c3e40), em 16 bits',
          "sums the goals of every season and club of the player's career table (0x7c3e40), in 16 bits"),
         ('registrar_o_emprestimo', 'Staff_Records::add_loan_records', '0x86da20', 'staff_records.cpp', 'thiscall, ret 0xc',
          'os históricos do empréstimo: o do técnico do clube que recebe (tipo 0x69) e o da pessoa (tipo 0x6a)',
          'the loan histories: the one of the receiving club manager (type 0x69) and the one of the person (type 0x6a)'),
         ('ordem_da_volta_aos_treinos', 'injury_compare_entry_dates', '0x632040', 'injury.cpp', 'cdecl',
          'compara a data da chave com a de uma entrada de 0x16 bytes da fila das voltas aos treinos, para o bsearch',
          'compares the key date with the one of a 0x16-byte return to training queue entry, for bsearch')])
CAMPOS = [('pessoa +0x61: ficha de jogador; ponteiro; lê (gols_na_carreira)',
           'person +0x61: player record; pointer; reads (gols_na_carreira)'),
          ('tabela da carreira (0x7c3e40): entradas de 0x2b bytes, seis por temporada e clube; +0xdf do grupo: gols; valor; lê (gols_na_carreira)',
           'career table (0x7c3e40): 0x2b-byte entries, six per season and club; group +0xdf: goals; value; reads (gols_na_carreira)'),
          ('jogador da partida +0x43: gols na carreira; valor; grava (MATCH_PLAYER::setup, pela gols_na_carreira)',
           'match player +0x43: career goals; value; written (MATCH_PLAYER::setup, by gols_na_carreira)'),
          ('clube +0xcf: técnico; ponteiro; lê (registrar_o_emprestimo)',
           'club +0xcf: manager; pointer; reads (registrar_o_emprestimo)'),
          ('pessoa +4, +8, +0xc → +0x33: os três ids do nome; id; lê (registrar_o_emprestimo)',
           'person +4, +8, +0xc → +0x33: the three name ids; id; reads (registrar_o_emprestimo)'),
          ('fila das voltas aos treinos (Injury +0x10, conta +0x18; 0x16 bytes, a data em +8); valor; lê (ordem_da_volta_aos_treinos)',
           'return to training queue (Injury +0x10, count +0x18; 0x16 bytes, date at +8); value; reads (ordem_da_volta_aos_treinos)'),
          ('Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (registrar_o_emprestimo)',
           'Staff_Records +0 and +0xc: staff history file and lists; appends (registrar_o_emprestimo)')]
INTERESSA = ['CMScout', 'CMRTE']
