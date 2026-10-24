# Rodam na gravação do jogo: save_database 0x52dad0 (a única chamadora) grava cada tabela do banco no
# .qst por zwrite 0x932e10 (já em Rust, chamado pelo endereço). Prólogo de 5 bytes (mov eax,[esp+8];
# push esi): o test/je curto seguinte fica fora. Iguais no cm0102-1080.exe; sem SEH, quadro nem caixa.
SAVE = True
ROTINAS = [('gravar_continentes', 0x54d6e0, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_nacoes', 0x54d840, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_cidades', 0x54d9a0, [0x8b, 0x44, 0x24, 0x08, 0x56])]
DOCS = ('a gravação dos .dat: continentes, nações, cidades',
        'writing the .dat files: continents, nations, cities',
        [('gravar_continentes', 'db_files_write_continents', '0x54d6e0', 'Db_files.cpp', 'cdecl', 'grava a tabela continent.dat (id 3) de registros de 198 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo', 'writes the continent.dat (id 3) table of 198-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out'),
         ('gravar_nacoes', 'db_files_write_nations', '0x54d840', 'Db_files.cpp', 'cdecl', 'grava a tabela nation.dat (id 4) de registros de 290 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo', 'writes the nation.dat (id 4) table of 290-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out'),
         ('gravar_cidades', 'db_files_write_cities', '0x54d9a0', 'Db_files.cpp', 'cdecl', 'grava a tabela city.dat (id 21) de registros de 56 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo', 'writes the city.dat (id 21) table of 56-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out')])
CAMPOS = [('TContinent (198 bytes, [0xae23a4]): registro inteiro (+0xbe força regional, double); lê (gravar_continentes: vai para o arquivo)', 'TContinent (198 bytes, [0xae23a4]): whole record (+0xbe regional strength, double); reads (gravar_continentes: goes to the file)'),
          ('TNation (290 bytes, [0xae23a8]): registro inteiro (os rivais já trocados por índices pelo save_database); lê (gravar_nacoes)', 'TNation (290 bytes, [0xae23a8]): whole record (rivals already turned into indexes by save_database); reads (gravar_nacoes)'),
          ('TCities (56 bytes, [0xae23ac]): registro inteiro; lê (gravar_cidades)', 'TCities (56 bytes, [0xae23ac]): whole record; reads (gravar_cidades)')]
INTERESSA = ['SGE', 'CMRTE']
