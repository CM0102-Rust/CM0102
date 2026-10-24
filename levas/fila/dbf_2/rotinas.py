# Rodam na criação do jogo: load_database 0x528540 lê cada tabela do Data\*.dat uma vez (fread; no save,
# zread). A gravação do .qst da prova do save cria o jogo lendo o banco, e as tabelas lidas vão para o
# save (os históricos de pessoas e de competições de pessoal pelos .tmp dos subsistemas).
SAVE = True
ROTINAS = [('ler_estadios', 0x54da00, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_pessoas_157', 0x54e780, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_historico_de_pessoas', 0x54fd60, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a leitura dos .dat: estádios, pessoas 157, histórico de pessoas',
        'reading the .dat files: stadiums, staff 157, staff history',
        [('ler_estadios', 'db_files_read_stadiums', '0x54da00', 'Db_files.cpp', 'cdecl',
          'lê a tabela stadium.dat (id 5) de registros de 78 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the stadium.dat table (id 5) of 78-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came'),
         ('ler_pessoas_157', 'db_files_read_staff_v1', '0x54e780', 'Db_files.cpp', 'cdecl',
          'lê a tabela staff.dat (pessoas, versão de arquivo de 157 bytes) (id 6) de registros de 157 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the staff.dat (staff, 157-byte file version) table (id 6) of 157-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came'),
         ('ler_historico_de_pessoas', 'db_files_read_staff_history', '0x54fd60', 'Db_files.cpp', 'cdecl',
          'lê a tabela staff_history.dat (id 17) de registros de 17 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the staff_history.dat table (id 17) of 17-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came')])
CAMPOS = [('index.dat entrada +0x37: quantidade de registros; valor; lê (as três)', 'index.dat entry +0x37: record count; value; reads (all three)'),
          ('index.dat entrada +0x3f: versão da tabela; valor; lê (as três; fora da versão aceita, a original abre a caixa)', 'index.dat entry +0x3f: table version; value; reads (all three; outside the accepted version the original opens the box)'),
          ('TStadiums (78 bytes, [0xae23b0]): +0x0 id; +0x4 nome (51); +0x38 cidade; +0x3c capacidade; +0x40 sentados; +0x44 expansão; +0x48 estádio vizinho; +0x4c coberto; +0x4d aquecimento; registro inteiro; grava (ler_estadios)', 'TStadiums (78 bytes, [0xae23b0]): +0x0 id; +0x4 name (51); +0x38 city; +0x3c capacity; +0x40 seating; +0x44 expansion; +0x48 nearby stadium; +0x4c covered; +0x4d undersoil heating; whole record; writes (ler_estadios)'),
          ('TStaff v1 (157 bytes, [0xae2404 (temporária; convertida para 110 bytes em 0xae23bc)]): 157 bytes; os primeiros campos coincidem com o TStaff de 110 (nascimento +0x10, ano +0x18, contrato +0x3e/+0x46); o layout inteiro está no CM0102-Editor (P8); registro inteiro; grava (ler_pessoas_157)', 'TStaff v1 (157 bytes, [0xae2404 (temporária; convertida para 110 bytes em 0xae23bc)]): 157 bytes; the first fields match the 110-byte TStaff (birth +0x10, year +0x18, contract +0x3e/+0x46); the full layout is in CM0102-Editor (P8); whole record; writes (ler_pessoas_157)'),
          ('TStaffHistory (17 bytes, [0xae23cc]): +0x0 id; +0x4 pessoa; +0x8 ano; +0xa clube; +0xe emprestado; +0xf jogos; +0x10 gols; registro inteiro; grava (ler_historico_de_pessoas)', 'TStaffHistory (17 bytes, [0xae23cc]): +0x0 id; +0x4 staff; +0x8 year; +0xa club; +0xe on loan; +0xf apps; +0x10 goals; whole record; writes (ler_historico_de_pessoas)')]
INTERESSA = ['SGE', 'CMRTE']
