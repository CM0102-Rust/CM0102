# The written history files (.his) (History.cpp; reader tel42, screens round 2, 22/11/2026): reading one into the
# history screen, existence, category, title, the -verifyhist check and the order of the written history list.
# Uses HISTORY_CPP, HISTORY_FILE_TEXT, HISTORY_EMPTY, HISTORY_RECORD_SIZE, HISTORY_PICTURE_NAME_SIZE, tel26_copy_text and
# tel35_c_string_len of the batch 318 (already in motor.rs).
# Proven in the disassembly (prologue to the last ret):
# - 0x5f1e50 history_load_his_file(name, last): cdecl -> 1/0 (called by history_screen_open 0x5f1472/0x5f149a, Rust).
# - 0x5f2500 history_his_file_exists(name): cdecl (club/staff/competition menus 0x4825d1, 0x49d23c, 0x88b9a1).
# - 0x5f2580 history_get_his_file_category(name): cdecl -> 0..4 or -1 (verify 0x5f2963, written history 0x5f2f1e).
# - 0x5f2700 history_get_his_file_title(name): cdecl -> 0xb5e008 or 0 (written history 0x5f2f33).
# - 0x5f2840 verify_history_files(): cdecl, with the jump table at 0x5f2b64 (5 cases: 0/1 -> clubs, 2/3 ->
#   competitions, 4 -> staff); called only by create_game (0x828327) with "-verifyhist" on the command line.
# - 0x5f3410 history_compare_entries_by_title(a, b): cdecl (qsort of the written history list, 0x5f2fa2).
# - Calls: fopen 0x945b00, fclose 0x945a09, fgetc 0x9469e0, fscanf 0x9470b8, stricmp 0x951c50, realloc 0x945667,
#   sprintf, utils_build_file_path, ScreenManager get/set_screen_param (set with 1 copies the text), makepath,
#   file_find_first/next/close, utils_string_compare 0x90c1d0 (a, b, 1, 0), club/comp_get_english_name, Date::get_month
#   and get_day_of_month (thiscall, no stack argument: the year and month+1 pushed before them are the sprintf
#   arguments), send_network_status_message, gui_show_message_box.
# Same in the cm0102-1080 exe (all six whole).
# How to reach them in the game: Find -> History (the written history list: category, title, order) and a club's or
# competition's menu -> History (exists, then the history screen reads the file); verify_history_files: start a new game
# with -verifyhist on the command line (one box per .his file with no match).
# Comparers: load side by side by content (the copy's digest, its blocks freed, the parameters put back, then the
# original); exists/category/title side by side over the static buffers and the pushed-back character; verify only the
# original (boxes and network messages); the order through compare_order!.
# Defects of the original kept: a NATION .his file is checked against the club names only (its message says "nation
# or club"); the error returns of load leave the file open and the sections already allocated; the character right
# after a '}' (or before the first section) is lost when the text starts there; a section name has no bound (0x64 bytes);
# the word buffer of fscanf is 32 bytes; a file with no category compares the empty text with the category names.
TELAS = True
ROTINAS = [('history_load_his_file', 0x5f1e50, [0x81, 0xec, 0x38, 0x02, 0x00, 0x00]),
           ('history_his_file_exists', 0x5f2500, [0xa1, 0x38, 0x31, 0xae, 0x00]),
           ('history_get_his_file_category', 0x5f2580, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x20]),
           ('history_get_his_file_title', 0x5f2700, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x20]),
           ('verify_history_files', 0x5f2840, [0x81, 0xec, 0x30, 0x03, 0x00, 0x00]),
           ('history_compare_entries_by_title', 0x5f3410, [0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9])]
DOCS = ('os arquivos de história escrita (.his): ler um para a tela de história, existência, categoria, título, a conferência -verifyhist e a ordem por título',
        'the written history files (.his): reading one into the history screen, existence, category, title, the -verifyhist check and the order by title',
        [('history_load_his_file', 'history_load_his_file', '0x5f1e50', 'History.cpp', 'cdecl',
          'lê o .his: {title:} o título (parâmetro 1), {section:} uma seção nova, {pic:} uma foto que passa a valer para os parágrafos seguintes; o texto vira parágrafos (uma quebra de linha = espaço, duas = fim do parágrafo); grava seções, fotos e contagens nos parâmetros 2 a 6',
          'reads the .his: {title:} the title (parameter 1), {section:} a new section, {pic:} a picture used by the following paragraphs; the text becomes paragraphs (one line break = a space, two = end of paragraph); stores sections, pictures and counts in parameters 2 to 6'),
         ('history_his_file_exists', 'history_his_file_exists', '0x5f2500', 'History.cpp', 'cdecl',
          'diz se cm3_history\\<nome>.his existe, só em jogo de um jogador na tela local',
          'tells whether cm3_history\\<name>.his exists, only in a single-player game on the local screen'),
         ('history_get_his_file_category', 'history_get_his_file_category', '0x5f2580', 'History.cpp', 'cdecl',
          'lê a marca {cat:} ou {category:} e devolve o índice entre CLUB, NATION, COMP, LEAGUE e PLAYER, ou -1',
          'reads the {cat:} or {category:} tag and returns its index among CLUB, NATION, COMP, LEAGUE and PLAYER, or -1'),
         ('history_get_his_file_title', 'history_get_his_file_title', '0x5f2700', 'History.cpp', 'cdecl',
          'lê a marca {title:} num texto fixo e o devolve, ou 0',
          'reads the {title:} tag into a static text and returns it, or 0'),
         ('verify_history_files', 'verify_history_files', '0x5f2840', 'History.cpp', 'cdecl',
          'com -verifyhist, confere cada .his contra o banco pela categoria (clube e nação contra os nomes dos clubes, competição e liga contra as competições, jogador contra "nome sobrenome dia.mês.ano") e mostra uma caixa para cada um sem par',
          'with -verifyhist, checks each .his against the database by category (club and nation against the club names, competition and league against the competitions, player against "first second day.month.year") and shows a box for each one with no match'),
         ('history_compare_entries_by_title', 'history_compare_entries_by_title', '0x5f3410', 'History.cpp', 'cdecl',
          'ordem da lista de história escrita pelo título, entrada nula por último',
          'order of the written history list by title, a null entry last')])
