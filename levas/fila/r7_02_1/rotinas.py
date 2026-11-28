# A carga do banco (Database.cpp; leitor r7_02, rodada 7, 28/11/2026): a 0x528540 load_database (21.904 bytes no mapa,
# 0x528540..0x52d97b com as 5 tabelas de saltos e índices 0x52d97c..0x52dacf).
# - cdecl(origem, ligado, do_save) -> 1 ou 0 (0x524d50 e 0x524d10 empurram 3 dwords e fazem add esp, 0xc). Prólogo
#   sub esp, 0x7d0 (6 bytes, sem relativo), igual no cm0102-1080.exe; o corpo difere no 1080 só no call do set_key_ids
#   (0x52d8fa: lá vai à entrada 0x603834), e a cópia chama o que o exe que roda chama (o deslocamento do call).
# - Por que estava fora (sobras.md, sx08b): "desviá-la põe os quadros da criação do jogo na pilha particular, e a
#   create_game lê resíduo de pilha". Conferido em 28/11: não vale mais. Quem chama a 0x524d50 (setup_start_new_game
#   0x81ce00, create_game_from_setup_file 0x827110 e create_game 0x828110) e a 0x524d10 (load_game_data, pela
#   load_database_from_stream já desviada) já são todas desviadas, então a carga já roda na pilha particular; e a
#   leitora conhecida do resíduo (0x697c40 weekly_board_confidence_update) já é Rust, com o N guardado de um clube ao
#   outro. O que sobra é o de sempre: o resíduo que a cópia deixa na pilha particular é outro, igual nas Rust x2 (o
#   detector de lixo da cadeia vigia).
# - Chamadas: as 18 leitoras do Db_files.cpp (já Rust, chamadas direto), db_files_open_for_read, zopen/zread/zclose,
#   buscar_no_conteiner (zseek), fread/fseek/fclose, malloc, GlobalAlloc/GlobalFree pela importação (0x9670f0/0x9670f4,
#   trocadas pelo alocador da DLL), database_free_all_tables (exe::, item r7_02_2 ou a original), langlib_sprintf, a
#   barra de progresso (0x802f80), makepath, file_find_*, utils_copy_file (0x90cc50, cdecl(origem, destino)),
#   zextract_matching_files, database_language_file_exists, Config::set_language, resolve_database_indexes (Rust, direto),
#   remove("duff.out"), database_format_staff_name, sprintf no buffer 0xde1f64, database_remove_staff_from_club,
#   database_club_has_reserve_squad, Date::Date (0x54a110 e 0x549f70), utils_random, database_get_national_team e
#   set_key_ids pelo call do exe. As caixas fatais (0x917, 0x94c..0xa22, 0xa7f, 0xa8c, 0xac6, 0xf9b, 0xfa2, 0xfaa) pela
#   database_fatal_error.
# - Na criação do jogo pela carreira (create_game_from_setup_file -> load_database_from_files(CM3_DATA, 1)): o modo
#   ligado, sem extensão no index.dat (o arquivo tem 1.482 bytes: o byte depois das entradas não existe), staff.dat de
#   versão 1 (169.342 pessoas de 157 bytes convertidas), sem preferências no índice, 16 linhas de reserva de técnicos.
#   Na carga do save (load_game_data -> load_database_from_stream, do_save = 1): o índice com extensão.
# Tipo: SAVE com CARGA. A prova do save cria o jogo do zero (a carga roda uma vez) e grava o .qst com e sem a rotina;
# com CARGA, também carrega o save de férias (do_save = 1). O comparador roda no jogo real comparando (criação).
# Comparador: a cópia carrega primeiro (ela mesma libera tudo antes), tira a foto; voltam os sorteios e o nome comum
# vazio; a original carrega por cima (libera as tabelas da cópia) e tira a foto; as fotos são comparadas byte a byte,
# os ponteiros pela tabela e deslocamento para onde apontam. Fica o banco da original. Os bytes que a original nunca
# escreve (as 127 linhas de competição de reserva e as linhas de reserva dos nomes, fora dos campos escritos, e o
# +0x6d das pessoas convertidas da versão 1) ficam fora da foto.
# Defeitos do original (cópia fiel): a conversão das pessoas da versão 1 não escreve o staff +0x6d (SQUAD_SELECTED_FOR,
# fica a sobra do heap); a falha ao abrir staff_history.dat ou staff_comp_history.dat fecha um fluxo nulo (fclose(NULL)
# ou zclose de 0); a entrada local das competições de reserva lidas do arquivo deixa o id e o início sem valor (a
# leitora não os lê; a cópia os zera); o tipo 6 de versão 1 no modo separado tira as linhas de reserva duas vezes.
SAVE = True
CARGA = True
ROTINAS = [('load_database', 0x528540, [0x81, 0xec, 0xd0, 0x07, 0x00, 0x00])]
DOCS = ('a carga do banco de dados: índice, tabelas, linhas de reserva e correções das pessoas',
        'loading the database: the index, the tables, the spare rows and the fixes of the people',
        [('load_database', 'load_database', '0x528540', 'Database.cpp', 'cdecl',
          'carrega o banco do index.dat e dos arquivos das tabelas, da pasta Data (fread) ou do contêiner do save (zread). '
          'Libera tudo antes; os arquivos de língua vão para a pasta temporária (do save, extraídos; da pasta, copiados; '
          'sem o da língua configurada, a língua volta a 0). O index.dat traz as duas contagens de reserva, as 22 entradas '
          'e, com a extensão, o índice dos nomes ingleses dos clubes e a língua dos nomes (sem ela, língua 0); faltando ou '
          'curto, devolve 0. No modo separado cada tabela tem o seu malloc; no ligado, poucos blocos (GlobalAlloc para o '
          'histórico de pessoas, o bloco comum e o histórico de competições de pessoas) com as tabelas dentro numa ordem '
          'fixa, 127 competições de reserva, 23.000 pessoas livres (classificação 6) com fichas de não jogador livres (só '
          'sem a extensão), 16 menos a reserva do índice linhas de reserva de pessoas, nomes, fichas e preferências, e '
          'folga de 0x52d pessoas e fichas de jogador. Lê cada arquivo pela leitora da sua tabela (a falta ou a leitura '
          'curta devolve 0; o histórico de pessoas e o de competições de pessoas são opcionais). Converte o staff.dat de '
          'versão 1 (157 para 110 bytes, as preferências numa tabela nova). No modo ligado troca os ids por ponteiros; '
          'no separado numera as competições de nação e o histórico delas. Depois corrige cada pessoa (clube extinto, '
          'reservas, gols sem jogos, entrada no clube antes de 1998 vira 1/8/1997, cargos 7 viram 6, técnico de seleção '
          'sem ficha, nação faltando por clube, seleção ou sorteio, cargos sem ficha, nomes faltando pelo nome comum '
          'vazio, cargo de seleção sem nação) e, sem a extensão, dá a cada uma das últimas seleções (uma por nação) a metade da reputação da seleção da sua nação '
          'e tira a divisão "Reserve" dos clubes. Por fim, as chaves fixas. Devolve 1. Defeitos do original: a conversão '
          'da versão 1 não escreve o +0x6d das pessoas; a falta do arquivo opcional fecha um fluxo nulo',
          'loads the database from index.dat and the table files, from the Data folder (fread) or from the saved game\'s '
          'container (zread). Frees everything first; the language files go to the temporary folder (from the save, '
          'extracted; from the folder, copied; without the configured language\'s file the language goes back to 0). '
          'index.dat has the two reserve counts, the 22 entries and, with the extension, the clubs\' English name index '
          'and the names language (without it, language 0); missing or short, returns 0. In the separate layout each '
          'table has its own malloc; in the linked one, a few blocks (GlobalAlloc for the people history, the shared block '
          'and the staff competition history) with the tables inside in a fixed order, 127 reserve competitions, 23,000 '
          'free people (classification 6) with free non-player sheets (only without the extension), 16 minus the index\'s '
          'reserve spare rows of people, names, sheets and preferences, and a slack of 0x52d people and player sheets. '
          'Reads each file with its table\'s reader (a missing file or a short read returns 0; the people history and the '
          'staff competition history are optional). Converts a version 1 staff.dat (157 to 110 bytes, the preferences '
          'into a new table). In the linked layout turns the ids into pointers; in the separate one numbers the nation '
          'competitions and their history. Then fixes each person (extinct club, reserves, goals without caps, joined '
          'the club before 1998 becomes 1/8/1997, jobs 7 become 6, national manager without a sheet, missing nation by '
          'club, national team or draw, jobs without sheets, missing names by the empty common name, national job without '
          'a nation) and, without the extension, gives each of the last national teams (one per nation) half the '
          'reputation of its nation\'s national team and '
          'takes the "Reserve" division off the clubs. Last, the fixed keys. Returns 1. Defects of the original: the '
          'version 1 conversion never writes the people\'s +0x6d; a missing optional file closes a null stream')])
CAMPOS = [('index.dat entry (67 bytes, g_index_dat_entries 0xae1a80): +0 nome do arquivo (51), +0x33 id da tabela, +0x37 quantidade, +0x3b início no arquivo, +0x3f versão; lê (load_database)',
           'index.dat entry (67 bytes, g_index_dat_entries 0xae1a80): +0 file name (51), +0x33 table id, +0x37 count, +0x3b start in the file, +0x3f version; reads (load_database)')]
INTERESSA = ['SGE', 'CMRTE']
