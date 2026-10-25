# Rodam uma vez na criação do jogo: create_game 0x828110 chama 0x541e60 (0x8282f1), 0x539750 (0x82836c) e 0x544970
# (0x8283d9) sem condição; a 0x539750 também na carga do jogo. A gravação do .qst da prova do save cria o jogo pelo
# início rápido (create_game_from_setup_file -> create_game) e grava o banco já criado: os elencos e a comissão dos
# times A e B e o pessoal que a 0x527240 cria com os nomes do índice vão para o save.
# Sem SEH, sem sorteio, sem variável local lida sem valor (eh_a só é lido quando 0x540a50 o gravou).
# A dbk1_indice_de_nomes só aparece no .qst se a 0x527240 criar pessoal na prova (clubes sem comissão): pode escapar.
SAVE = True
ROTINAS = [('cortar_elencos_de_a_e_b', 0x541e60, [0xa1, 0x5c, 0x23, 0xae, 0x00]),
           ('juntar_o_time_b_ao_a', 0x544970, [0x83, 0xec, 0x10, 0xa1, 0x5c, 0x23, 0xae, 0x00]),
           ('montar_indice_de_nomes', 0x539750, [0xa1, 0x50, 0x23, 0xae, 0x00])]
DOCS = ('a criação do jogo: os elencos dos times A e B, a comissão do time B, o índice de nomes por nação',
        'game creation: the senior and B team squads, the B team staff, the per-nation name index',
        [('cortar_elencos_de_a_e_b', 'database_trim_senior_and_b_team_squads', '0x541e60', 'Database.cpp', 'cdecl',
          'marca em cada clube se ele tem time parceiro (+0x244) e, em cada time A, corta o elenco conjunto A+B a 48 pela reputação, de trás para a frente, mantendo 3 goleiros',
          'flags every club that has a partner team (+0x244) and, for every senior team, trims the joint senior+B squad to 48 by reputation, from the bottom, keeping 3 goalkeepers'),
         ('juntar_o_time_b_ao_a', 'database_merge_b_team_staff', '0x544970', 'Database.cpp', 'cdecl',
          'em cada time A com time B: corta o elenco do B ao que couber em 50 com o do A e passa a comissão do B (técnicos, olheiros, fisioterapeutas e o treinador quando o A não tem) para o A',
          'for every senior team with a B team: trims the B squad to what fits in 50 with the senior one and moves the B backroom staff (coaches, scouts, physios and the manager when the senior team has none) to the senior club'),
         ('montar_indice_de_nomes', 'database_build_nation_name_index', '0x539750', 'Database.cpp', 'cdecl',
          'aloca e monta o índice por nação das tabelas de primeiros, segundos e nomes comuns (primeira linha, soma dos pesos e quantidade de cada grupo); o último grupo de cada tabela fica sem gravar, como na original',
          'allocates and builds the per-nation index of the first, second and common name tables (first row, weight total and count of every group); the last group of each table is left unwritten, as in the original')])
CAMPOS = [('TClub +0x244: tem time parceiro; byte; grava (cortar_elencos_de_a_e_b), lê (juntar_o_time_b_ao_a)', 'TClub +0x244: has a partner team; byte; writes (cortar_elencos_de_a_e_b), reads (juntar_o_time_b_ao_a)'),
          ('TClub +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): casas do clube; ponteiros; grava (as duas, por 0x53c190 e 0x53bd10)', 'TClub +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): club slots; pointers; writes (both, through 0x53c190 and 0x53bd10)'),
          ('TStaff +0x39 clube, +0x3d cargo, +0x3e data de entrada; grava (as duas, por 0x53c190 e 0x53bd10)', 'TStaff +0x39 club, +0x3d job, +0x3e joined date; writes (both, through 0x53c190 and 0x53bd10)'),
          ('TNames (60 bytes): +0x37 nação, +0x3b peso; lê (montar_indice_de_nomes)', 'TNames (60 bytes): +0x37 nation, +0x3b weight; reads (montar_indice_de_nomes)'),
          ('índice de nomes [0xae2338] (0x24 bytes por nação): +0/+4/+8 primeira linha, +0xc/+0x10/+0x14 soma dos pesos, +0x18/+0x1c/+0x20 quantidade (primeiros, segundos, comuns); grava (montar_indice_de_nomes)', 'name index [0xae2338] (0x24 bytes per nation): +0/+4/+8 first row, +0xc/+0x10/+0x14 weight total, +0x18/+0x1c/+0x20 count (first, second, common); writes (montar_indice_de_nomes)')]
INTERESSA = ['SGE', 'CMRTE']
