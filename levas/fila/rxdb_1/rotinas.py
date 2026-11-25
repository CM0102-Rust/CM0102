# The whole database written into the saved game (Database.cpp; reader rxdb, round 6, 25/11/2026).
# Item SAVE. Same bytes in cm0102-1080.exe (the whole routine, 15.056 bytes).
# - 0x52dad0 save_database cdecl(folder, use_zip, zip_a, zip_b), returns 1/0; called only by save_database_for_game
#   (0x524d30, Rust: exe::save_database(save, 1, a4, kind)). Prologue sub esp, 0xa10 (6 bytes, no relative call).
#   Last ret 0x53159c; the failure tails 0x5313d7..0x5313ff.
# - Calls: langlib_sprintf 0x66f4e0 cdecl(text, "Saving database"); ScreenManager::send_network_status_message 0x802f80
#   thiscall(0xb74450, text, step, 0x18, -1) ret 0x10; zopen 0x9326e0 cdecl(folder, name, 1, zip_a, zip_b);
#   db_files_open_for_write 0x54d5b0 cdecl(folder, name); zwrite 0x932e10 / fwrite 0x945ce6 cdecl(buf, size, n, f);
#   zclose 0x932b00 cdecl(&z); fclose 0x945a09; ztell 0x933170 / ftell 0x946aae cdecl(f); the 17 writers
#   db_files_write_* (0x54d6e0..0x550950) cdecl(f, use_zip, table, n), 1 when all went out; resolve_database_indexes
#   0x5315a0 cdecl(lenient); utils_build_file_path 0x90bde0 cdecl("CM3_TEMP", ""); zadd_matching_files 0x933610
#   cdecl(folder, path, "*.lng", zip_a, zip_b); the box: sprintf(0xde1f64, "<X> table valid, but no entry in %s",
#   "index.dat") + the Database.cpp error box (database_error_box, the line of each table).
# - Pieces: database_save_pointers_to_ids (the 12 linked tables, one field list each), database_save_table (one per
#   table, from the DATABASE_SAVE_TABLES list: entry, box, file, writer), database_save_index (index.dat),
#   database_save_language_files (the *.lng into the container).
# Defects of the original (for DEFEITOS): the 460 bytes after the index.dat entries are stack never set (already
# listed; the copy writes zeros) and so are the entries of tables the loaded index lacks; the stadiums' box uses the
# continents' text; with the staff entry missing, the preferences, non-players and players compare their file name with
# entries[local_a0c] (unset or the last zip handle: a read out of the frame; the copy keeps the name); a failed write of
# the index.dat entries closes the file twice (zclose or fclose, 0x5313a6..0x5313d8; the copy closes once); the
# CM3_TEMP folder is strcpy'd into a 260-byte local (the copy cuts it at 259).
# Comparer: only the original (writes every table into the container). Proof: the save proof (prova_save.py --leva N:
# the .qst with and without the routine) and the whole routine emulated against the copy (see PRONTO.txt).
SAVE = True
ROTINAS = [('save_database', 0x52dad0, [0x81, 0xec, 0x10, 0x0a, 0x00, 0x00])]
_F = 'Database.cpp'
DOCS = ('a gravação do banco de dados inteiro no save',
        'the whole database written into the saved game',
        [('save_database', 'save_database', '0x52dad0', _F, 'cdecl',
          'grava o banco inteiro: no contêiner do save (use_zip ≠ 0) ou como arquivos .dat numa pasta. Com o banco ligado '
          '(G_DATABASE_SHARED_ALLOC_MODE ≠ 0), antes troca cada campo-ponteiro das competições de clube, clubes, nações, '
          'árbitros, estádios, preferências, fichas de não jogador, pessoas, competições de pessoa, histórico de pessoas, '
          'cidades e histórico de competições pelo id do registro apontado (−1 se nulo; nos nomes, o id em +0x33). Depois, '
          'com um passo da barra cada, as 22 tabelas numa ordem fixa: a entrada da tabela no index.dat carregado (pelo id '
          'da tabela) vai para a próxima vaga do índice novo; tabela carregada sem entrada mostra a caixa "<tabela> table '
          'valid, but no entry in index.dat" e não é gravada; com entrada, vai para o arquivo que a entrada diz, com a '
          'quantidade, o início (0) e a versão (1 ou 2) na entrada. Os clubes sem as seleções (gravadas como tabela '
          'própria), as competições de clube sem as de nação e sem as linhas de reserva (estas vão depois das de nação, no '
          'mesmo arquivo), o histórico de competições de clube sem as linhas das nações. Preferências, fichas de não '
          'jogador e de jogador, que no banco ficam dentro do arquivo das pessoas, ganham arquivo próprio '
          '("Preferences.dat", "nonplayer.dat", "player.dat") quando a entrada delas diz o arquivo das pessoas, com o '
          'início na posição nele, e as preferências passam ao id 22. Depois o index.dat (as quantidades das competições e '
          'técnicos de reserva, as 22 entradas, um byte 1, 460 bytes, o índice dos nomes ingleses dos clubes ou −1, a '
          'língua do banco), os ponteiros de volta (resolve_database_indexes(1)) e, no contêiner, os *.lng da pasta '
          'temporária. Devolve 1; uma falha para a gravação, volta os ponteiros com resolve_database_indexes(0) e devolve '
          '0. Defeitos do original: os 460 bytes depois das entradas são pilha sem valor (a cópia grava zeros), a caixa '
          'dos estádios diz "Continents", sem a entrada das pessoas as três tabelas comparam o nome com uma vaga lida de '
          'um local sem valor, a falha das entradas do index.dat fecha o arquivo duas vezes',
          'writes the whole database: into the saved game\'s container (use_zip ≠ 0) or as .dat files into a folder. With '
          'the database linked (G_DATABASE_SHARED_ALLOC_MODE ≠ 0), first turns every pointer field of the club '
          'competitions, clubs, nations, officials, stadiums, preferences, non-player sheets, people, staff competitions, '
          'people history, cities and competition history into the id of the record it points to (−1 when null; for the '
          'names, the id at +0x33). Then, with a progress step each, the 22 tables in a fixed order: the table\'s entry in '
          'the loaded index.dat (by table id) goes to the next slot of the new index; a loaded table without an entry '
          'shows the "<table> table valid, but no entry in index.dat" box and is not written; with one, it goes to the '
          'file its entry names, with the count, the start (0) and the version (1 or 2) set in the entry. The clubs '
          'without the national teams (written as their own table), the club competitions without the nation ones and the '
          'reserve rows (these follow the nation competitions in their file), the club competition history without the '
          'nations\' rows. Preferences, non-player and player sheets, kept inside the people file in the database, get '
          'their own file ("Preferences.dat", "nonplayer.dat", "player.dat") when their entry names the people file, '
          'with the start at the position in it, and the preferences take table id 22. Then index.dat (the reserve '
          'competition and manager counts, the 22 entries, a 1 byte, 460 bytes, the clubs\' English name index or −1, the '
          'database language), the pointers back (resolve_database_indexes(1)) and, into the container, the *.lng files '
          'of the temporary folder. Returns 1; a failure stops the save, resolves the pointers back with '
          'resolve_database_indexes(0) and returns 0. Defects of the original: the 460 bytes after the entries are stack '
          'never set (the copy writes zeros), the stadiums\' box says "Continents", without the people entry the three '
          'tables compare their name with a slot read from an unset local, a failed write of the index.dat entries closes '
          'the file twice')])
CAMPOS = [('index.dat entry (67 bytes, g_index_dat_entries 0xae1a80): +0 nome do arquivo (51), +0x33 id da tabela, +0x37 quantidade, +0x3b início no arquivo, +0x3f versão; escreve (save_database: na cópia local que vai para o index.dat)',
           'index.dat entry (67 bytes, g_index_dat_entries 0xae1a80): +0 file name (51), +0x33 table id, +0x37 count, +0x3b start in the file, +0x3f version; writes (save_database: in the local copy that goes to index.dat)')]
INTERESSA = ['SGE', 'CMRTE']
