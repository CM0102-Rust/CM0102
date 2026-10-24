# Rodam na criação do jogo: load_database 0x528540 lê cada tabela do Data\*.dat uma vez (fread; no save,
# zread). A gravação do .qst da prova do save cria o jogo lendo o banco, e as tabelas lidas vão para o
# save (os históricos de pessoas e de competições de pessoal pelos .tmp dos subsistemas).
SAVE = True
ROTINAS = [('ler_continentes', 0x54d5e0, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_nacoes', 0x54d740, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_cidades', 0x54d8a0, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a leitura dos .dat: continentes, nações, cidades',
        'reading the .dat files: continents, nations, cities',
        [('ler_continentes', 'db_files_read_continents', '0x54d5e0', 'Db_files.cpp', 'cdecl',
          'lê a tabela continent.dat (id 3) de registros de 198 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the continent.dat table (id 3) of 198-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came'),
         ('ler_nacoes', 'db_files_read_nations', '0x54d740', 'Db_files.cpp', 'cdecl',
          'lê a tabela nation.dat (id 4) de registros de 290 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2 (outra: caixa e saída); 1 se veio tudo',
          'reads the nation.dat table (id 4) of 290-byte records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if everything came'),
         ('ler_cidades', 'db_files_read_cities', '0x54d8a0', 'Db_files.cpp', 'cdecl',
          'lê a tabela city.dat (id 21) de registros de 56 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada de 1 a 2 (com sinal) (outra: caixa e saída); 1 se veio tudo',
          'reads the city.dat table (id 21) of 56-byte records in one go, through zread (save) or fread (Data), with the entry version 1 to 2 (other: box and exit); 1 if everything came')])
CAMPOS = [('index.dat entrada +0x37: quantidade de registros; valor; lê (as três)', 'index.dat entry +0x37: record count; value; reads (all three)'),
          ('index.dat entrada +0x3f: versão da tabela; valor; lê (as três; fora da versão aceita, a original abre a caixa)', 'index.dat entry +0x3f: table version; value; reads (all three; outside the accepted version the original opens the box)'),
          ('TContinent (198 bytes, [0xae23a4]): +0x0 id; +0x4 nome (26); +0x1f sigla; +0x23 continentalidade; +0x3d federação (101); +0xa3 federação curta; +0xbe força regional (double); registro inteiro; grava (ler_continentes)', 'TContinent (198 bytes, [0xae23a4]): +0x0 id; +0x4 name (26); +0x1f three letters; +0x23 continentality; +0x3d federation (101); +0xa3 short federation; +0xbe regional strength (double); whole record; writes (ler_continentes)'),
          ('TNation (290 bytes, [0xae23a8]): +0x0 id; +0x4 nome (51, a chave por nome); +0x57 nacionalidade; +0x71 continente; +0x7a capital; +0x80 estádio; +0x8e reputação; +0x90..+0xa4 cores; +0xa8..+0x108 coeficientes FIFA/UEFA (double); +0x110..+0x118 rivais (ids que viram ponteiro no resolve); registro inteiro; grava (ler_nacoes)', 'TNation (290 bytes, [0xae23a8]): +0x0 id; +0x4 name (51, the name key); +0x57 nationality; +0x71 continent; +0x7a capital; +0x80 stadium; +0x8e reputation; +0x90..+0xa4 colours; +0xa8..+0x108 FIFA/UEFA coefficients (double); +0x110..+0x118 rivals (ids that become pointers in resolve); whole record; writes (ler_nacoes)'),
          ('TCities (56 bytes, [0xae23ac]): +0x0 id; +0x4 nome (26); +0x1f nação; +0x23 latitude e +0x2b longitude (double); +0x33 atração; +0x34 clima; registro inteiro; grava (ler_cidades)', 'TCities (56 bytes, [0xae23ac]): +0x0 id; +0x4 name (26); +0x1f nation; +0x23 latitude and +0x2b longitude (double); +0x33 attraction; +0x34 weather; whole record; writes (ler_cidades)')]
INTERESSA = ['SGE', 'CMRTE']
