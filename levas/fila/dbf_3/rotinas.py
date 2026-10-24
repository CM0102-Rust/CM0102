# Rodam na criação do jogo: load_database 0x528540 lê cada tabela do Data\*.dat uma vez (fread; no save,
# zread). A gravação do .qst da prova do save cria o jogo lendo o banco, e as tabelas lidas vão para o
# save (os históricos de pessoas e de competições de pessoal pelos .tmp dos subsistemas).
SAVE = True
ROTINAS = [('ler_historicos_de_competicao', 0x550230, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_competicoes_de_pessoal', 0x550380, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ler_historico_de_competicoes_de_pessoal', 0x5504d0, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a leitura dos .dat: históricos de competição, competições de pessoal, histórico de competições de pessoal',
        'reading the .dat files: competition histories, staff competitions, staff competition history',
        [('ler_historicos_de_competicao', 'db_files_read_comp_history', '0x550230', 'Db_files.cpp', 'cdecl',
          'lê a tabela club_comp_history.dat e nation_comp_history.dat (id 19, 20) de registros de 26 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the club_comp_history.dat and nation_comp_history.dat table (id 19, 20) of 26-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came'),
         ('ler_competicoes_de_pessoal', 'db_files_read_staff_comps', '0x550380', 'Db_files.cpp', 'cdecl',
          'lê a tabela staff_comp.dat (id 11) de registros de 101 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 2 (outra: caixa e saída); 1 se veio tudo',
          'reads the staff_comp.dat table (id 11) of 101-byte records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if everything came'),
         ('ler_historico_de_competicoes_de_pessoal', 'db_files_read_staff_comp_history', '0x5504d0', 'Db_files.cpp', 'cdecl',
          'lê a tabela staff_comp_history.dat (id 18) de registros de 58 bytes de uma vez, por zread (save) ou fread (Data), com a versão da entrada igual a 1 (outra: caixa e saída); 1 se veio tudo',
          'reads the staff_comp_history.dat table (id 18) of 58-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came')])
CAMPOS = [('index.dat entrada +0x37: quantidade de registros; valor; lê (as três)', 'index.dat entry +0x37: record count; value; reads (all three)'),
          ('index.dat entrada +0x3f: versão da tabela; valor; lê (as três; fora da versão aceita, a original abre a caixa)', 'index.dat entry +0x3f: table version; value; reads (all three; outside the accepted version the original opens the box)'),
          ('TCompHistory (26 bytes, [0xae23d4 / 0xae23dc]): +0x0 id; +0x4 competição; +0x8 ano; +0xa campeão; +0xe vice; +0x12 terceiro; +0x16 sede; registro inteiro; grava (ler_historicos_de_competicao)', 'TCompHistory (26 bytes, [0xae23d4 / 0xae23dc]): +0x0 id; +0x4 competition; +0x8 year; +0xa winners; +0xe runners-up; +0x12 third place; +0x16 host; whole record; writes (ler_historicos_de_competicao)'),
          ('TStaffComp (101 bytes, [0xae23e0]): +0x0 id; +0x4 nome (51, a chave por nome); +0x38 nome curto; +0x53 continente; +0x57 nação; +0x5b/+0x5f cores; +0x63 reputação; registro inteiro; grava (ler_competicoes_de_pessoal)', 'TStaffComp (101 bytes, [0xae23e0]): +0x0 id; +0x4 name (51, the name key); +0x38 short name; +0x53 continent; +0x57 nation; +0x5b/+0x5f colours; +0x63 reputation; whole record; writes (ler_competicoes_de_pessoal)'),
          ('TStaffCompHistory (58 bytes, [0xae23e4]): +0x0 id; +0x4 competição; +0x8 ano; +0xa..+0x36 campeão, vice e terceiro (primeiro nome, sobrenome, pessoa, info); registro inteiro; grava (ler_historico_de_competicoes_de_pessoal)', 'TStaffCompHistory (58 bytes, [0xae23e4]): +0x0 id; +0x4 competition; +0x8 year; +0xa..+0x36 winner, runner-up and third (first name, second name, staff, info); whole record; writes (ler_historico_de_competicoes_de_pessoal)')]
INTERESSA = ['SGE', 'CMRTE']
