# A tela do hall da fama: as entradas, a importacao, a exportacao e os nomes das ligas (hall_of_fame.cpp; leitor sx12b,
# rodada 5, 23/11/2026). Chamadas pela tela do hall da fama (o montador e o tratador depois de open_hall_of_fame_screen
# 0x825cc0, ja em Rust a abertura): get_entry, export_to_csv, try_load_quiet (passada ao dialogo de arquivo em
# 0x826a00), sort_league_order e get_table_name; import_file pela importacao (e por try_load_quiet, so conferindo);
# merge_entry pela importacao; operator= por get_entry e merge_entry; compare_league_names pelo qsort da ordem.
# Caminho no jogo para o Lucas: menu principal (ou o menu do jogo) -> Hall of Fame: o menu das ligas (ordenado pelo
# nome), as entradas de uma tabela, Exportar (grava o arquivo de texto) e Importar (escolher um hall_of_fame.bin:
# o dialogo confere o arquivo e a importacao junta as entradas).
# Convencoes conferidas na desmontagem:
# - 0x5ef130: thiscall(this, tabela, posicao, saida), ret 0xc (bytes com sinal; 0..27 e 0..99); operator= thiscall(saida,
#   this + (tabela x 100 + posicao) x 0x78); caixa 0xe7.
# - 0x5ef200: thiscall(this, origem), ret 4, eax this; nome e clube 50 bytes cada (laco de bytes), +0x64/+0x68/+0x6c,
#   Date::operator= 0x417c10 thiscall(this+0x70, temporaria, origem+0x70).
# - 0x5ef970: thiscall(this, tabela, entrada), ret 8; repe cmpsd 30 dwords nas 99 primeiras; pontuacao sem sinal (jbe);
#   operator= na 100a; qsort 0x945333 cdecl(tabela, 100, 0x78, 0x5f0620); caixa 0x18f.
# - 0x5efd50: thiscall(this, caminho, so_conferir), ret 8; fopen "rb", fread cdecl, unscramble 0x5f0680; com so_conferir,
#   sem caixa; mescla as entradas com tempo > 0 (com sinal) de cada tabela (merge_entry com o byte da tabela).
# - 0x5f0430: thiscall(this, caminho), ret 4; Date::Date 0x54a110 local; fopen "w"; langlib_sprintf; fprintf 0x9464f0
#   cdecl; localtime 0x946c40; Date::set 0x54a130 thiscall(d, dia, mes, ano + 1900, -1) ret 0x10;
#   Date::get_short_weekday_date_string 0x54ae10 thiscall(d, buffer, 0xc8, 0xff); utils_format_ordinal 0x90c310 cdecl(n,
#   -1); fclose. (O original guarda o time_t no mesmo local da data; Date::set o reescreve inteiro.)
# - 0x5f1090: cdecl(caminho), devolve ax 1/2; [0xb5ded8] = o hall da fama; import_file(caminho, 1).
# - 0x5f10c0: cdecl(forma), devolve 0xb5debc; a forma em 0xb5deb8; qsort(0xb5debc, 28, 1, 0x5f1100).
# - 0x5f1100: cdecl(a, b), qsort; get_table_name em buffers de 0xc8; utils_string_compare 0x90c1d0 cdecl(a, b, 1, 0).
# - 0x5f07c0: cdecl(tabela, saida, tamanho, curto); tabela de saltos em 0x5f1014 (28 casos, ate 0x5f1084): curto ->
#   langlib_sprintf(saida, menu, 0xad9c64) (o caso 0 sem o argumento), senao (saida, longo); caixa 0x3e6 (o default
#   0x496 e inalcancavel).
# Comparadores: lado a lado (entrada, tabela, buffer de importacao, ordem, texto); export_to_csv: o arquivo escrito pela
# original e reescrito pela copia, comparados; import_file: a original confere o arquivo antes (so_conferir) e, ruim, so
# a original roda.
TELAS = True
SAVE = False
ROTINAS = [('hall_of_fame_get_entry', 0x5ef130, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hall_of_fame_entry_assign', 0x5ef200, [0x83, 0xec, 0x08, 0x55, 0x8b, 0x6c, 0x24, 0x10]),
           ('hall_of_fame_merge_entry', 0x5ef970, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hall_of_fame_import_file', 0x5efd50, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00]),
           ('hall_of_fame_export_to_csv', 0x5f0430, [0x6a, 0xff, 0x68, 0x3b, 0xac, 0x95, 0x00]),
           ('hall_of_fame_try_load_quiet', 0x5f1090, [0x8b, 0x0d, 0xd8, 0xde, 0xb5, 0x00]),
           ('hall_of_fame_sort_league_order', 0x5f10c0, [0x32, 0xc0, 0xb9, 0xbc, 0xde, 0xb5, 0x00]),
           ('hall_of_fame_compare_league_names', 0x5f1100, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('hall_of_fame_get_table_name', 0x5f07c0, [0x8a, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tela do hall da fama',
        'the hall of fame screen',
        [('hall_of_fame_get_entry', 'HallOfFame::get_entry', '0x5ef130', 'hall_of_fame.cpp', 'thiscall, ret 0xc',
          'copia a entrada da tabela (0 a 27) e posição (0 a 99) pedidas para a saída e devolve 1; fora da faixa, a caixa "Error" e 0',
          'copies the entry of the given table (0 to 27) and rank (0 to 99) to the output and returns 1; out of range, the "Error" box and 0'),
         ('hall_of_fame_entry_assign', 'HallOfFameEntry::operator=', '0x5ef200', 'hall_of_fame.cpp', 'thiscall, ret 4',
          'copia uma entrada do hall da fama: o técnico, o clube ou a nação (50 bytes cada), o horário, a pontuação, a semente do jogo e a data',
          'copies a hall of fame entry: the manager, the club or nation (50 bytes each), the time stamp, the score, the game seed and the date'),
         ('hall_of_fame_merge_entry', 'HallOfFame::merge_entry', '0x5ef970', 'hall_of_fame.cpp', 'thiscall, ret 8',
          'junta uma entrada importada a uma tabela: se já há uma igual entre as 99 primeiras, ou se a pontuação é zero ou não passa a da última, nada (0); senão ela toma o lugar da última e a tabela é reordenada (1); tabela inválida ou entrada nula, a caixa',
          'merges an imported entry into a table: when an identical one is among the first 99, or the score is zero or not above the last one, nothing (0); otherwise it takes the last place and the table is sorted again (1); a bad table or a null entry, the box'),
         ('hall_of_fame_import_file', 'HallOfFame::import_file', '0x5efd50', 'hall_of_fame.cpp', 'thiscall, ret 8',
          'lê um arquivo do hall da fama para o buffer de importação e o confere (assinatura, versão, tamanho, soma depois de desembaralhar); arquivo ruim: 0, com a caixa só quando não é só conferência; bom e sem só conferir, junta cada entrada datada à sua tabela; 1',
          'reads a hall of fame file into the import buffer and checks it (magic, version, size, checksum after unscrambling); a bad file: 0, with the box only when not just checking; a good one, unless just checking, merges each dated entry into its table; 1'),
         ('hall_of_fame_export_to_csv', 'HallOfFame::export_to_csv', '0x5f0430', 'hall_of_fame.cpp', 'thiscall, ret 4',
          'grava as 28 tabelas num arquivo de texto: o título, o nome de cada tabela e uma linha por entrada com pontuação (posição por extenso, técnico, clube ou nação, data, pontuação), separadas por vírgula; sem arquivo, a caixa e 0',
          'writes the 28 tables to a text file: the heading, each table\'s name and one comma separated line per scored entry (ordinal rank, manager, club or nation, date, score); no file, the box and 0'),
         ('hall_of_fame_try_load_quiet', 'hall_of_fame_try_load_quiet', '0x5f1090', 'hall_of_fame.cpp', 'cdecl',
          'confere um arquivo para o diálogo de importação, sem caixa: 1 se o hall da fama existe e o arquivo é bom, senão 2',
          'checks a file for the import dialog, without a box: 1 when the hall of fame exists and the file is good, otherwise 2'),
         ('hall_of_fame_sort_league_order', 'hall_of_fame_sort_league_order', '0x5f10c0', 'hall_of_fame.cpp', 'cdecl',
          'monta a ordem das 28 tabelas para o menu, ordenada pelo nome (na forma pedida); o ano civil primeiro',
          'builds the order of the 28 tables for the menu, sorted by name (in the given form); the calendar year first'),
         ('hall_of_fame_compare_league_names', 'hall_of_fame_compare_league_names', '0x5f1100', 'hall_of_fame.cpp', 'cdecl',
          'ordena duas tabelas pelo nome: o ano civil antes das ligas, depois pela comparação de texto dos nomes',
          'orders two tables by name: the calendar year before the leagues, then by the text comparison of the names'),
         ('hall_of_fame_get_table_name', 'hall_of_fame_get_table_name', '0x5f07c0', 'hall_of_fame.cpp', 'cdecl',
          'escreve o nome de uma tabela: a forma do menu (o país da liga; "Jan - Dec") ou a longa ("English League"; "January - December"), traduzido; tabela inválida, saída nula ou tamanho abaixo de 50, a caixa e 0',
          'writes a table\'s name: the menu form (the league\'s country; "Jan - Dec") or the long one ("English League"; "January - December"), translated; a bad table, a null output or a size below 50, the box and 0')])
