# Rodam na criação do jogo: load_database 0x528540 lê cada tabela do Data\*.dat uma vez (fread; no save,
# zread); clubes e competições duas vezes (club/nat_club, club_comp/nation_comp). A base de fábrica traz as
# três tabelas na versão 2 (leitura direta); a versão 1 vai ao conversor campo a campo da original, chamado
# pelo endereço (0x54dc90, 0x54ffd0, 0x54e9a0: sem SEH e sem variável local lida sem valor; leem o byte
# de reputação/peso no próprio argumento). A gravação do .qst da prova do save cria o jogo lendo o banco, e as
# tabelas lidas vão para o save.
SAVE = True
ROTINAS = [('ler_clubes', 0x54db50, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_competicoes', 0x54feb0, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_nao_jogadores', 0x54e880, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a leitura dos .dat com conversor: clubes, competições, não jogadores',
        'reading the .dat files with a converter: clubs, competitions, non-players',
        [('ler_clubes', 'db_files_read_clubs', '0x54db50', 'Db_files.cpp', 'cdecl',
          'lê a tabela club.dat e nat_club.dat (ids 0, 1) de registros de 581 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2, e troca a reputação 0 por 1; versão 1: o conversor 0x54dc90 (outra: caixa e saída); 1 se veio tudo',
          'reads the club.dat and nat_club.dat table (ids 0, 1) of 581-byte records in one go, through zread (save) or fread (Data), with the entry version 2, and turns reputation 0 into 1; version 1: the 0x54dc90 converter (other: box and exit); 1 if everything came'),
         ('ler_competicoes', 'db_files_read_comps', '0x54feb0', 'Db_files.cpp', 'cdecl',
          'lê a tabela club_comp.dat e nation_comp.dat (ids 12, 16) de registros de 107 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2; versão 1: o conversor 0x54ffd0 (outra: caixa e saída); 1 se veio tudo',
          'reads the club_comp.dat and nation_comp.dat table (ids 12, 16) of 107-byte records in one go, through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54ffd0 converter (other: box and exit); 1 if everything came'),
         ('ler_nao_jogadores', 'db_files_read_players', '0x54e880', 'Db_files.cpp', 'cdecl',
          'lê a tabela dos não jogadores do staff.dat (id 9; o mapa da comunidade a chama de players) de registros de 68 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2; versão 1: o conversor 0x54e9a0 (outra: caixa e saída); 1 se veio tudo',
          'reads the non-player table of staff.dat (id 9; the community map calls it players) of 68-byte records in one go, through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54e9a0 converter (other: box and exit); 1 if everything came')])
CAMPOS = [('index.dat entrada +0x37: quantidade de registros; valor; lê (as três)', 'index.dat entry +0x37: record count; value; reads (all three)'),
          ('index.dat entrada +0x3f: versão da tabela; valor; lê (as três; 1 vai ao conversor; fora de 1 e 2, a original abre a caixa)', 'index.dat entry +0x3f: table version; value; reads (all three; 1 goes to the converter; outside 1 and 2 the original opens the box)'),
          ('TClub (581 bytes, [0xae23b4 / 0xae23b8]): +0x0 id; +0x4 nome (51, a chave por nome); +0x53 nação; +0x65 caixa; +0x69 estádio; +0x73..+0x7b público; +0x80 reputação (u16; 0 vira 1); +0x83..+0x97 cores; +0xb3..+0xbb rivais; +0xd7 elenco (50 ids); registro inteiro; grava (ler_clubes)', 'TClub (581 bytes, [0xae23b4 / 0xae23b8]): +0x0 id; +0x4 name (51, the name key); +0x53 nation; +0x65 cash; +0x69 stadium; +0x73..+0x7b attendance; +0x80 reputation (u16; 0 becomes 1); +0x83..+0x97 colours; +0xb3..+0xbb rivals; +0xd7 squad (50 ids); whole record; writes (ler_clubes)'),
          ('TComp (107 bytes, [0xae23d0 / 0xae23d8]): +0x0 id; +0x4 nome (51, a chave por nome); +0x38 nome curto; +0x53 sigla; +0x57 escopo; +0x58 selecionada; +0x59 continente; +0x5d nação; +0x61/+0x65 cores; +0x69 reputação; registro inteiro; grava (ler_competicoes)', 'TComp (107 bytes, [0xae23d0 / 0xae23d8]): +0x0 id; +0x4 name (51, the name key); +0x38 short name; +0x53 three letters; +0x57 scope; +0x58 selected; +0x59 continent; +0x5d nation; +0x61/+0x65 colours; +0x69 reputation; whole record; writes (ler_competicoes)'),
          ('TNonPlayer (68 bytes, [0xae23c0]): +0x0 id; +0x4 capacidade atual; +0x6 potencial; +0x8/+0xa/+0xc reputações (casa, atual, mundial); +0xe.. atributos de técnico (+0xe ataque, +0xf negócios, +0x10 treino...); registro inteiro; grava (ler_nao_jogadores)', 'TNonPlayer (68 bytes, [0xae23c0]): +0x0 id; +0x4 current ability; +0x6 potential; +0x8/+0xa/+0xc reputations (home, current, world); +0xe.. staff attributes (+0xe attacking, +0xf business, +0x10 coaching...); whole record; writes (ler_nao_jogadores)')]
INTERESSA = ['SGE', 'CMRTE']
