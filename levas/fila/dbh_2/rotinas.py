# Rodam na gravação do save: save_database 0x52dad0 (use_zip = 1) grava cada tabela no contêiner por zwrite
# (com use_zip = 0, a base na pasta, por fwrite). A prova do save grava o .qst com e sem as rotinas.
# gravar_registros_58 serve às cores e ao histórico de competições de pessoal; este só com a tabela
# presente ([0xae23e4] ≠ 0), e no jogo ela já virou .tmp do subsistema: no save, só as cores.
SAVE = True
ROTINAS = [('gravar_competicoes_de_pessoal', 0x550470, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_arbitros', 0x5506b0, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_registros_58', 0x550800, [0x8b, 0x44, 0x24, 0x08, 0x56])]
DOCS = ('a gravação dos .dat: competições de pessoal, árbitros, cores',
        'writing the .dat files: staff competitions, officials, colours',
        [('gravar_competicoes_de_pessoal', 'db_files_write_staff_comps', '0x550470', 'Db_files.cpp', 'cdecl',
          'grava de uma vez n registros de 101 bytes (staff_comp.dat, id 11) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo',
          'writes n 101-byte records (staff_comp.dat, id 11) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went'),
         ('gravar_arbitros', 'db_files_write_officials', '0x5506b0', 'Db_files.cpp', 'cdecl',
          'grava de uma vez n registros de 43 bytes (officials.dat, id 7) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo',
          'writes n 43-byte records (officials.dat, id 7) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went'),
         ('gravar_registros_58', 'db_files_write_records_0x3a', '0x550800', 'Db_files.cpp', 'cdecl',
          'grava de uma vez n registros de 58 bytes (colour.dat, id 2, e staff_comp_history.dat, id 18: a mesma rotina para os dois) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo',
          'writes n 58-byte records (colour.dat, id 2, and staff_comp_history.dat, id 18: the same routine for both) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went')])
CAMPOS = [('TStaffComp (101 bytes, [0xae23e0]): registro inteiro; lê (gravar_competicoes_de_pessoal)', 'TStaffComp (101 bytes, [0xae23e0]): whole record; reads (gravar_competicoes_de_pessoal)'),
          ('TOfficials (43 bytes, [0xae23e8]): registro inteiro; lê (gravar_arbitros)', 'TOfficials (43 bytes, [0xae23e8]): whole record; reads (gravar_arbitros)'),
          ('TColour (58 bytes, [0xae23ec]) e TStaffCompHistory (58 bytes, [0xae23e4]): registro inteiro; lê (gravar_registros_58)', 'TColour (58 bytes, [0xae23ec]) and TStaffCompHistory (58 bytes, [0xae23e4]): whole record; reads (gravar_registros_58)')]
INTERESSA = ['SGE', 'CMRTE']
