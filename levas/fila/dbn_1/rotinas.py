# Rodam uma vez na criação do jogo (create_game 0x828110): a 0x524d70 (cortes das pessoas) chama a 0x527240 logo no
# começo; a create_game chama a 0x539ae0 e a 0x5267c0 sem condição. A gravação do .qst da prova do save cria o jogo pelo
# início rápido e grava o banco já cortado: a tabela de clubes cortada (club.dat, com [0xae23fc] na cauda do index.dat),
# o histórico de competições podado e o pessoal extra que sobreviver aos cortes vão para o save.
# Sem SEH; sem variável local lida sem valor (a local do par da 0x527240 só é lida quando 0x540a50 devolve parceiro, e
# então ela foi gravada); sorteios só pela 0x90cfc0 (e pelas 0x539c60/0x53a1b0 já em Rust), sem rand direto; realloc só
# na 0x5267c0 (no lugar, a original confere o endereço) e na 0x539ae0 (o resultado vira a tabela), pelas rotinas do jogo.
# Trocadas: 0x524d50 (repasse da carga; desviada, a load_database rodaria na pilha particular e a create_game lê resíduo
# de pilha, como na 0x827110 desligada) e 0x528080 (liberar tudo: efeito único, as mutações não aparecem em prova nenhuma)
# saíram; entraram a 0x539ae0 e a 0x53db40 (no dbn_2), órfãs da 232ª.
SAVE = True
ROTINAS = [('gerar_comissao_extra', 0x527240, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00]),
           ('cortar_tabela_de_clubes', 0x5267c0, [0x81, 0xec, 0x28, 0x06, 0x00, 0x00]),
           ('podar_historico_de_competicoes', 0x539ae0, [0x8b, 0x0d, 0x7c, 0x23, 0xae, 0x00])]
DOCS = ('a criação do jogo: gerar a comissão que falta, cortar a tabela de clubes, podar o histórico de competições',
        'game creation: generating the missing backroom staff, trimming the club table, pruning the competition history',
        [('gerar_comissao_extra', 'database_generate_extra_non_playing_staff', '0x527240', 'Database.cpp', 'cdecl',
          'completa técnicos, olheiros e fisioterapeutas dos clubes de nações escolhidas pela reputação (pessoas das reservas, nomes da nação, nascimento sorteado) e cria treinadores sem clube para os clubes sem treinador; mais de 23.000: caixa e 0',
          'fills coaches, scouts and physios of the clubs of chosen nations by reputation (reserve rows, names of the nation, random birth date) and creates unattached managers for clubs without one; over 23,000: box and 0'),
         ('cortar_tabela_de_clubes', 'database_trim_club_table', '0x5267c0', 'Database.cpp', 'cdecl',
          'corta a tabela de clubes pelo tamanho da base: os melhores por nação, os referidos por pessoas e históricos, as seleções, as chaves e os de nações escolhidas; troca os ponteiros, compacta, monta [0xae23fc] e realoca no lugar',
          'trims the club table by the database size: the best per nation, those referred by staff and histories, national teams, keys and clubs of chosen nations; swaps the pointers, compacts, builds [0xae23fc] and reallocates in place'),
         ('podar_historico_de_competicoes', 'database_prune_comp_history_table', '0x539ae0', 'Database.cpp', 'cdecl',
          'deixa no histórico de competições de clube só as linhas de competições de nação nula ou escolhida, realoca, acerta a parte das seleções e ordena (qsort com 0x540780)',
          'keeps in the club competition history only the rows of competitions with no nation or a chosen one, reallocates, sets the national part and sorts (qsort with 0x540780)')])
CAMPOS = [('TStaff +4/+8/+0xc nomes, +0x10 nascimento, +0x1a nação, +0x24 seleção, +0x28 cargo na seleção, +0x39 clube, +0x3d cargo, +0x5f tipo; grava (gerar_comissao_extra)', 'TStaff +4/+8/+0xc names, +0x10 birth date, +0x1a nation, +0x24 national team, +0x28 national job, +0x39 club, +0x3d job, +0x5f type; writes (gerar_comissao_extra)'),
          ('TClub +0x19f (5), +0x1b3 (7), +0x1cf (3): comissão; ponteiros; grava (gerar_comissao_extra)', 'TClub +0x19f (5), +0x1b3 (7), +0x1cf (3): backroom staff; pointers; writes (gerar_comissao_extra)'),
          ('[0x9b59b4]: primeira linha de nome comum vazia; grava (gerar_comissao_extra)', '[0x9b59b4]: first empty common name row; writes (gerar_comissao_extra)'),
          ('TClub +0 id, +0x80 reputação, +0xb3/+0xb7/+0xbb clubes; [0xae23fc] linha nova → id antigo; [0xae235c], [0xae23b8]; grava (cortar_tabela_de_clubes)', 'TClub +0 id, +0x80 reputation, +0xb3/+0xb7/+0xbb clubs; [0xae23fc] new row → old id; [0xae235c], [0xae23b8]; writes (cortar_tabela_de_clubes)'),
          ('TNation +0x11c: nação escolhida; byte; lê (as três)', 'TNation +0x11c: chosen nation; byte; reads (all three)'),
          ('histórico de competições de clube (26 bytes): +4 competição; [0xae23d4], [0xae237c], [0xae23dc]; grava (podar_historico_de_competicoes)', 'club competition history (26 bytes): +4 competition; [0xae23d4], [0xae237c], [0xae23dc]; writes (podar_historico_de_competicoes)')]
INTERESSA = ['SGE', 'CMRTE']
