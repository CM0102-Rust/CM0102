# Rodam na criação do jogo: load_database 0x528540 lê cada tabela do Data\*.dat uma vez (fread; no save,
# zread); a de nomes três vezes (primeiros nomes, sobrenomes e nomes comuns). A gravação do .qst da prova do
# save cria o jogo lendo o banco, e as tabelas lidas vão para o save.
SAVE = True
ROTINAS = [('ler_arbitros', 0x5505c0, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_cores', 0x550710, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_nomes', 0x550860, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a leitura dos .dat: árbitros, cores, nomes',
        'reading the .dat files: officials, colours, names',
        [('ler_arbitros', 'db_files_read_officials', '0x5505c0', 'Db_files.cpp', 'cdecl',
          'lê a tabela officials.dat (id 7) de registros de 43 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the officials.dat table (id 7) of 43-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came'),
         ('ler_cores', 'db_files_read_colours', '0x550710', 'Db_files.cpp', 'cdecl',
          'lê a tabela colour.dat (id 2) de registros de 58 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the colour.dat table (id 2) of 58-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came'),
         ('ler_nomes', 'db_files_read_names', '0x550860', 'Db_files.cpp', 'cdecl',
          'lê as tabelas first_names.dat, second_names.dat e common_names.dat (ids 13, 14, 15) de registros de 60 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the first_names.dat, second_names.dat and common_names.dat tables (ids 13, 14, 15) of 60-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came')])
CAMPOS = [('index.dat entrada +0x37: quantidade de registros; valor; lê (as três)', 'index.dat entry +0x37: record count; value; reads (all three)'),
          ('index.dat entrada +0x3f: versão da tabela; valor; lê (as três; fora da versão aceita, a original abre a caixa)', 'index.dat entry +0x3f: table version; value; reads (all three; outside the accepted version the original opens the box)'),
          ('TOfficials (43 bytes, [0xae23e8]): +0x0 id; +0x4 primeiro nome e +0x8 sobrenome (ids); +0xc nascimento; +0x14 ano; +0x16 nação; +0x1a cidade; +0x1e capacidade atual; +0x20 potencial; +0x22 reputação; +0x24..+0x2a notas (deixar correr, disciplina, jogos importantes, pressão, arbitragem, bandeira, cronometragem); registro inteiro; grava (ler_arbitros)', 'TOfficials (43 bytes, [0xae23e8]): +0x0 id; +0x4 first name and +0x8 second name (ids); +0xc birth date; +0x14 year; +0x16 nation; +0x1a city; +0x1e current ability; +0x20 potential; +0x22 reputation; +0x24..+0x2a ratings (allowing flow, discipline, important matches, pressure, refereeing, running the line, time keeping); whole record; writes (ler_arbitros)'),
          ('TColour (58 bytes, [0xae23ec]): +0x0 id; +0x4 nome (51); +0x37 vermelho; +0x38 verde; +0x39 azul; registro inteiro; grava (ler_cores)', 'TColour (58 bytes, [0xae23ec]): +0x0 id; +0x4 name (51); +0x37 red; +0x38 green; +0x39 blue; whole record; writes (ler_cores)'),
          ('TNames (60 bytes, [0xae23f0 / 0xae23f4 / 0xae23f8]): +0x0 nome (51); +0x33 id; +0x37 nação; +0x3b contagem (peso); registro inteiro; grava (ler_nomes)', 'TNames (60 bytes, [0xae23f0 / 0xae23f4 / 0xae23f8]): +0x0 name (51); +0x33 id; +0x37 nation; +0x3b count (weight); whole record; writes (ler_nomes)')]
INTERESSA = ['SGE', 'CMRTE']
