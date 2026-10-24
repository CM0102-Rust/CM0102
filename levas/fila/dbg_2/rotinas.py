# Rodam na gravação do jogo: save_database 0x52dad0 (a única chamadora) grava cada tabela do banco no
# .qst por zwrite 0x932e10 (já em Rust, chamado pelo endereço). Prólogo de 5 bytes (mov eax,[esp+8];
# push esi): o test/je curto seguinte fica fora. Iguais no cm0102-1080.exe; sem SEH, quadro nem caixa.
SAVE = True
ROTINAS = [('gravar_estadios', 0x54daf0, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_clubes', 0x54e5d0, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_pessoas_110', 0x54e720, [0x8b, 0x44, 0x24, 0x08, 0x56])]
DOCS = ('a gravação dos .dat: estádios, clubes, pessoas 110',
        'writing the .dat files: stadiums, clubs, staff 110',
        [('gravar_estadios', 'db_files_write_stadiums', '0x54daf0', 'Db_files.cpp', 'cdecl', 'grava a tabela stadium.dat de registros de 78 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo', 'writes the stadium.dat table of 78-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out'),
         ('gravar_clubes', 'db_files_write_clubs', '0x54e5d0', 'Db_files.cpp', 'cdecl', 'grava a tabela club.dat e nat_club.dat (ids 0, 1) de registros de 581 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo', 'writes the club.dat and nat_club.dat (ids 0, 1) table of 581-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out'),
         ('gravar_pessoas_110', 'db_files_write_staff', '0x54e720', 'Db_files.cpp', 'cdecl', 'grava a tabela staff.dat (pessoas, 110 bytes; id 6) de registros de 110 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo', 'writes the staff.dat (staff, 110 bytes; id 6) table of 110-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out')])
CAMPOS = [('TStadium (78 bytes): registro inteiro (+0x3c capacidade); lê (gravar_estadios)', 'TStadium (78 bytes): whole record (+0x3c capacity); reads (gravar_estadios)'),
          ('TClub (581 bytes, [0xae23b4 / 0xae23b8]): registro inteiro (ponteiros já trocados por índices); lê (gravar_clubes)', 'TClub (581 bytes, [0xae23b4 / 0xae23b8]): whole record (pointers already turned into indexes); reads (gravar_clubes)'),
          ('TStaff (110 bytes, [0xae23bc]): registro inteiro (ponteiros já trocados por índices); lê (gravar_pessoas_110)', 'TStaff (110 bytes, [0xae23bc]): whole record (pointers already turned into indexes); reads (gravar_pessoas_110)')]
INTERESSA = ['SGE', 'CMRTE']
