# Rodam na gravação do jogo: save_database 0x52dad0 (a única chamadora) grava cada tabela do banco no
# .qst por zwrite 0x932e10 (já em Rust, chamado pelo endereço). Prólogo de 5 bytes (mov eax,[esp+8];
# push esi): o test/je curto seguinte fica fora. Iguais no cm0102-1080.exe; sem SEH, quadro nem caixa.
SAVE = True
ROTINAS = [('gravar_nao_jogadores', 0x54efd0, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_jogadores', 0x54fbb0, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('gravar_preferencias', 0x54fd00, [0x8b, 0x44, 0x24, 0x08, 0x56])]
DOCS = ('a gravação dos .dat: não jogadores, jogadores, preferências',
        'writing the .dat files: non-players, players, preferences',
        [('gravar_nao_jogadores', 'db_files_write_players', '0x54efd0', 'Db_files.cpp', 'cdecl', 'grava a tabela dos não jogadores do staff.dat (id 9; o mapa da comunidade a chama de players) de registros de 68 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo', 'writes the staff.dat non-player (id 9; the community map calls it players) table of 68-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out'),
         ('gravar_jogadores', 'db_files_write_non_players', '0x54fbb0', 'Db_files.cpp', 'cdecl', 'grava a tabela dos jogadores do staff.dat (id 10; o mapa da comunidade a chama de non_players) de registros de 70 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo', 'writes the staff.dat player (id 10; the community map calls it non_players) table of 70-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out'),
         ('gravar_preferencias', 'db_files_write_staff_extra', '0x54fd00', 'Db_files.cpp', 'cdecl', 'grava a tabela das preferências das pessoas (id 22) de registros de 52 bytes de uma vez, por zwrite (save) ou fwrite (Data); 1 se foi tudo', 'writes the staff preferences (id 22) table of 52-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out')])
CAMPOS = [('TNonPlayer (68 bytes, [0xae23c0]): registro inteiro; lê (gravar_nao_jogadores)', 'TNonPlayer (68 bytes, [0xae23c0]): whole record; reads (gravar_nao_jogadores)'),
          ('TPlayer (70 bytes, [0xae23c4]): registro inteiro; lê (gravar_jogadores)', 'TPlayer (70 bytes, [0xae23c4]): whole record; reads (gravar_jogadores)'),
          ('TPreferences (52 bytes, [0xae23c8]): registro inteiro; lê (gravar_preferencias)', 'TPreferences (52 bytes, [0xae23c8]): whole record; reads (gravar_preferencias)')]
INTERESSA = ['SGE', 'CMRTE']
