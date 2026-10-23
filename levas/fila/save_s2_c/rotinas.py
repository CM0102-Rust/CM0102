SAVE = True
ROTINAS = [('gravar_banco_no_save', 0x524d30, [0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x08]),
           ('carregar_banco_do_save', 0x524d10, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04]),
           ('liberar_dados_do_jogo', 0x82ff40, [0x53, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('o save: as ligações com o banco (gravar e carregar) e liberar os dados do jogo',
        'the saved game: the database links (save and load) and freeing the game data',
        [('gravar_banco_no_save', 'save_database_for_game', '0x524d30', 'Database.cpp', 'cdecl',
          'passo 1 do save_game_data: save_database(save, 1, a4, tipo), as tabelas do banco no contêiner',
          'save_game_data step 1: save_database(save, 1, a4, type), the database tables into the container'),
         ('carregar_banco_do_save', 'load_database_from_stream', '0x524d10', 'Database.cpp', 'cdecl',
          'passo 2 do load_game_data: load_database(save, compartilhado, 1), o banco lido do contêiner',
          'load_game_data step 2: load_database(save, shared, 1), the database read from the container'),
         ('liberar_dados_do_jogo', 'free_game_data', '0x82ff40', 'Setup.cpp', 'cdecl',
          'solta as telas, destrói os subsistemas guardados em globais (destrutor + delete, free, destrutor virtual) na ordem da original, apaga os *.tmp, libera as tabelas do banco, volta a data do jogo e zera três marcas; devolve 1',
          'releases the screens, destroys the subsystems held in globals (destructor + delete, free, virtual destructor) in the original order, deletes the *.tmp files, frees the database tables, resets the game date and clears three flags; returns 1')])
CAMPOS = [('[0xb67a50], [0xb6b7c8], [0xae2a60], [0xb67a44], [0xad9da8], [0xad9dac], [0xde1d08], [0xae28f0], [0xde1f44], [0xad9c60], [0xde1f54], [0xb63d28], [0xae2a3c], [0xad9c6c], [0xae2a30], [0xb7433c], [0xdd7ef4], [0xb74318], [0xde2f70], [0xde1f58], [0xae1000], [0xdd7f00], [0xb6b7c0], [0xb74350], [0xb5e134], [0xae2a58], [0xb63c98], [0xb74448], [0xb7431c], [0xae1050], [0xae2a38], [0xb7434c], [0xb74340], [0xdd7ee4], [0xde2f74], [0xb74430] subsistemas do jogo; ponteiro; lê, destrói e grava 0 (liberar_dados_do_jogo)', 'game subsystems; pointer; reads, destroys and writes 0 (liberar_dados_do_jogo)'),
          ('[0xae2c74] (amistosos), [0xb74434] objetos com destrutor virtual; ponteiro; lê, destrói e grava 0 (liberar_dados_do_jogo)', '[0xae2c74] (friendlies), [0xb74434] objects with a virtual destructor; pointer; reads, destroys and writes 0 (liberar_dados_do_jogo)'),
          ('[0xadadfc] (e [0xadadf8]), [0xae2d0c], [0xae2d14], [0xae2d1c], [0xae2d20], [0xae2d24], [0xae2338], [0xdd7ef0] vetores soltos; ponteiro; lê, libera e grava 0 (liberar_dados_do_jogo)', '[0xadadfc] (and [0xadadf8]), [0xae2d0c], [0xae2d14], [0xae2d1c], [0xae2d20], [0xae2d24], [0xae2338], [0xdd7ef0] loose arrays; pointer; reads, frees and writes 0 (liberar_dados_do_jogo)'),
          ('[0xae2c90] data do jogo; valor; grava (0x54a130 com 1, 0, 2001, -1) (liberar_dados_do_jogo)', '[0xae2c90] game date; value; writes (0x54a130 with 1, 0, 2001, -1) (liberar_dados_do_jogo)'),
          ('[0xae2c88] (byte), [0xdd711c], [0xdd7130] marcas do jogo; valor; grava 0 (liberar_dados_do_jogo)', '[0xae2c88] (byte), [0xdd711c], [0xdd7130] game flags; value; writes 0 (liberar_dados_do_jogo)')]
INTERESSA = ['SGE']
