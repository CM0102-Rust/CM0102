# Jogadores: rodam na criação do jogo (load_database 0x528540 lê o Data\staff.dat; no save, zread), e a
# tabela vai para o .qst da prova do save. Pessoas de 110 bytes (0x54e630) e preferências (0x54fc10) só
# rodam com staff.dat na versão 2: o save (load_database(..., 1), chamadas em 0x52c2b0 e 0x52c370) e as bases
# da comunidade; a base de fábrica é a versão 1 (pessoas de 157 bytes, 0x54e780, preferências convertidas).
# Por isso a marca CARGA: a prova carrega o FeriasTesteSave com e sem as rotinas. A versão 1 dos jogadores
# vai ao conversor 0x54f150 pelo endereço (sem SEH e sem variável local lida sem valor).
SAVE = True
CARGA = True
ROTINAS = [('ler_jogadores', 0x54f030, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_pessoas_110', 0x54e630, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_preferencias', 0x54fc10, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a leitura dos .dat: jogadores, pessoas 110, preferências',
        'reading the .dat files: players, staff 110, preferences',
        [('ler_jogadores', 'db_files_read_non_players', '0x54f030', 'Db_files.cpp', 'cdecl',
          'lê a tabela dos jogadores do staff.dat (id 10; o mapa da comunidade a chama de non_players) de registros de 70 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2; versão 1: o conversor 0x54f150 (outra: caixa e saída); 1 se veio tudo',
          'reads the player table of staff.dat (id 10; the community map calls it non_players) of 70-byte records in one go, through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54f150 converter (other: box and exit); 1 if everything came'),
         ('ler_pessoas_110', 'db_files_read_staff', '0x54e630', 'Db_files.cpp', 'cdecl',
          'lê a tabela staff.dat (pessoas, versão de arquivo de 110 bytes: o save e as bases da comunidade) (id 6) de registros de 110 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2 (outra: caixa e saída); 1 se veio tudo',
          'reads the staff.dat (staff, 110-byte file version: the save and community databases) table (id 6) of 110-byte records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if everything came'),
         ('ler_preferencias', 'db_files_read_staff_extra', '0x54fc10', 'Db_files.cpp', 'cdecl',
          'lê a tabela das preferências das pessoas (id 22: o save e as bases da comunidade) de registros de 52 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the staff preferences table (id 22: the save and community databases) of 52-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came')])
CAMPOS = [('index.dat entrada +0x37: quantidade de registros; valor; lê (as três)', 'index.dat entry +0x37: record count; value; reads (all three)'),
          ('index.dat entrada +0x3f: versão da tabela; valor; lê (as três; jogadores na versão 1 vão ao conversor; fora da versão aceita, a original abre a caixa)', 'index.dat entry +0x3f: table version; value; reads (all three; players in version 1 go to the converter; outside the accepted version the original opens the box)'),
          ('TPlayer (70 bytes, [0xae23c4]): +0x0 id; +0x4 número; +0x5 capacidade atual; +0x7 potencial; +0x9/+0xb/+0xd reputações (casa, atual, mundial); +0xf.. posições (+0xf goleiro, +0x10 líbero, +0x11 zagueiro...); atributos; registro inteiro; grava (ler_jogadores)', 'TPlayer (70 bytes, [0xae23c4]): +0x0 id; +0x4 squad number; +0x5 current ability; +0x7 potential; +0x9/+0xb/+0xd reputations (home, current, world); +0xf.. positions (+0xf goalkeeper, +0x10 sweeper, +0x11 defender...); attributes; whole record; writes (ler_jogadores)'),
          ('TStaff (110 bytes, [0xae23bc]): +0x0 id; +0x4/+0x8/+0xc nomes (ids); +0x10 nascimento; +0x18 ano; +0x1a nação; +0x39 clube; +0x3e/+0x46 contrato; +0x4e salário; +0x52 valor; +0x56..+0x5d traços; +0x61 jogador, +0x65 preferências, +0x69 não jogador (índices); registro inteiro; grava (ler_pessoas_110)', 'TStaff (110 bytes, [0xae23bc]): +0x0 id; +0x4/+0x8/+0xc names (ids); +0x10 birth date; +0x18 year; +0x1a nation; +0x39 club; +0x3e/+0x46 contract; +0x4e wage; +0x52 value; +0x56..+0x5d traits; +0x61 player, +0x65 preferences, +0x69 non-player (indexes); whole record; writes (ler_pessoas_110)'),
          ('TPreferences (52 bytes, [0xae23c8]): +0x0 id; +0x4..+0xc clubes favoritos; +0x10..+0x18 clubes detestados; +0x1c..+0x24 pessoas favoritas; +0x28..+0x30 pessoas detestadas (ids); registro inteiro; grava (ler_preferencias)', 'TPreferences (52 bytes, [0xae23c8]): +0x0 id; +0x4..+0xc favourite clubs; +0x10..+0x18 disliked clubs; +0x1c..+0x24 favourite staff; +0x28..+0x30 disliked staff (ids); whole record; writes (ler_preferencias)')]
INTERESSA = ['SGE', 'CMRTE']
