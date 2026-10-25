# As liberações rodam na criação do jogo: PlayerStats::PlayerStats e AWARD_MANAGER::AWARD_MANAGER soltam os
# históricos de pessoas e de prêmios (passam a viver nos .tmp dos subsistemas). A gravação do .qst da prova do
# save cria o jogo. O crescer (0x527f60) só roda no fim de competição (comp_util_add_history_record): a prova do
# save (criar e gravar) e a carga até 2001-08-28 não o alcançam; a mutação dele deve escapar (lacuna, como a do
# histórico de pessoas da 231a). O comparador da carreira comparando também não o cobre (realloc: só a original).
SAVE = True
ROTINAS = [('liberar_historico_de_pessoas', 0x527ee0, [0xa1, 0xcc, 0x23, 0xae, 0x00]),
           ('liberar_historico_de_premios', 0x527f20, [0xa1, 0xe4, 0x23, 0xae, 0x00]),
           ('crescer_historico_de_competicoes', 0x527f60, [0xa1, 0x7c, 0x23, 0xae, 0x00])]
DOCS = ('liberar os históricos de pessoas e de prêmios e crescer o histórico de competições',
        'freeing the staff and award histories and growing the competition history',
        [('liberar_historico_de_pessoas', 'database_free_staff_history_table', '0x527ee0', 'Database.cpp', 'cdecl',
          'com a tabela do histórico de pessoas presente, solta (GlobalFree no banco compartilhado, free no outro) e zera ponteiro e contagem',
          'with the staff history table present, frees it (GlobalFree in the shared database, free otherwise) and zeroes pointer and count'),
         ('liberar_historico_de_premios', 'database_free_staff_comp_history_table', '0x527f20', 'Database.cpp', 'cdecl',
          'o mesmo para o histórico de prêmios (competições de pessoal)',
          'the same for the award (staff competition) history'),
         ('crescer_historico_de_competicoes', 'database_grow_comp_history_table', '0x527f60', 'Database.cpp', 'cdecl',
          'realloc do histórico de competições para n linhas a mais (sem memória: 0); contagem, ponteiro das seleções e as linhas novas com o próprio índice em +0 e o resto zero; 1',
          'reallocs the competition history to n more rows (out of memory: 0); count, national part pointer and the new rows with their own index at +0 and the rest zero; 1')])
CAMPOS = [('[0xae23cc] / [0xae2374]: tabela e contagem do histórico de pessoas (17 bytes); ponteiro; solta e zera (liberar_historico_de_pessoas)', '[0xae23cc] / [0xae2374]: staff history table and count (17 bytes); pointer; frees and zeroes (liberar_historico_de_pessoas)'),
          ('[0xae23e4] / [0xae238c]: tabela e contagem do histórico de prêmios (58 bytes); ponteiro; solta e zera (liberar_historico_de_premios)', '[0xae23e4] / [0xae238c]: award history table and count (58 bytes); pointer; frees and zeroes (liberar_historico_de_premios)'),
          ('[0xae2340]: banco no bloco compartilhado (GlobalAlloc); valor; lê (as duas liberações)', '[0xae2340]: database in the shared block (GlobalAlloc); value; reads (both frees)'),
          ('[0xae23d4] / [0xae237c] / [0xae23dc]: histórico de competições (26 bytes), contagem e parte das seleções (a partir da linha [0xae2384]); tabela; escreve (crescer_historico_de_competicoes)', '[0xae23d4] / [0xae237c] / [0xae23dc]: competition history (26 bytes), count and national part (from row [0xae2384]); table; writes (crescer_historico_de_competicoes)')]
INTERESSA = ['SGE', 'CMRTE']
