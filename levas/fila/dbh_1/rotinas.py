# Rodam na gravação do save: save_database 0x52dad0 (use_zip = 1) grava cada tabela no contêiner por zwrite
# (com use_zip = 0, a base na pasta, por fwrite). A prova do save grava o .qst com e sem as rotinas.
# O histórico de pessoas só é gravado com a tabela presente ([0xae23cc] ≠ 0): no jogo ela já virou .tmp do
# subsistema e o save_database pula (a mutação dela escapa da prova do save; ver mutacoes.py).
SAVE = True
ROTINAS = [('gravar_historico_de_pessoas', 0x54fe50, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_competicoes', 0x5501d0, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_historicos_de_competicao', 0x550320, [0x8b, 0x44, 0x24, 0x08, 0x56])]
DOCS = ('a gravação dos .dat: histórico de pessoas, competições, históricos de competição',
        'writing the .dat files: staff history, competitions, competition histories',
        [('gravar_historico_de_pessoas', 'db_files_write_staff_history', '0x54fe50', 'Db_files.cpp', 'cdecl',
          'grava de uma vez n registros de 17 bytes (staff_history.dat, id 17) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo',
          'writes n 17-byte records (staff_history.dat, id 17) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went'),
         ('gravar_competicoes', 'db_files_write_comps', '0x5501d0', 'Db_files.cpp', 'cdecl',
          'grava de uma vez n registros de 107 bytes (club_comp.dat, em duas chamadas: as competições e as 127 de reserva; nation_comp.dat) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo',
          'writes n 107-byte records (club_comp.dat, in two calls: the competitions and the 127 reserve ones; nation_comp.dat) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went'),
         ('gravar_historicos_de_competicao', 'db_files_write_comp_history', '0x550320', 'Db_files.cpp', 'cdecl',
          'grava de uma vez n registros de 26 bytes (club_comp_history.dat e nation_comp_history.dat, ids 19 e 20) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo',
          'writes n 26-byte records (club_comp_history.dat and nation_comp_history.dat, ids 19 and 20) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went')])
CAMPOS = [('TStaffHistory (17 bytes, [0xae23cc]): registro inteiro; lê (gravar_historico_de_pessoas)', 'TStaffHistory (17 bytes, [0xae23cc]): whole record; reads (gravar_historico_de_pessoas)'),
          ('TComp (107 bytes, [0xae23d0 / 0xae23d8]): registro inteiro; lê (gravar_competicoes)', 'TComp (107 bytes, [0xae23d0 / 0xae23d8]): whole record; reads (gravar_competicoes)'),
          ('TCompHistory (26 bytes, [0xae23d4 / 0xae23dc]): registro inteiro; lê (gravar_historicos_de_competicao)', 'TCompHistory (26 bytes, [0xae23d4 / 0xae23dc]): whole record; reads (gravar_historicos_de_competicao)')]
INTERESSA = ['SGE', 'CMRTE']
