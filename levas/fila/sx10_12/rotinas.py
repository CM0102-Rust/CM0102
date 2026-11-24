# A lista das fontes TrueType: a pasta FONTS, os modos de fonte e os nomes das fontes (o codigo de fontes que o mapa poe
# em fog_of_war.cpp; leitor sx10, rodada 5, 23/11/2026). Item de TELAS: as opcoes de fonte (Setup.cpp, game_config.cpp)
# e a varredura da pasta, que o FONT::FONT (sx10_13) faz antes do main.
# Convencoes conferidas na desmontagem: font_set_mode cdecl(modo); font_get_file_count cdecl() -> contagem (u16);
# font_get_mode_name cdecl(modo, buf, tamanho); font_scan_folder cdecl() -> 0/1; ttf_swap_* thiscall(ponteiro) -> 1;
# ttf_read_name_string cdecl(&FILE*, registro, id do nome, base, buf, tamanho); font_file_read_at cdecl(&FILE*,
# deslocamento, buf, tamanho). Chamadas: utils_build_file_path 0x90bde0, strncpy 0x9466d0, _makepath 0x945b13,
# _splitpath 0x944cff, stricmp 0x951c50, fopen/fseek/fread/fclose, _strdup 0x952f5b, file_find_first/next/close
# 0x4c0aa0/0x4c0b50/0x4c0c10, langlib_sprintf 0x66f4e0, operator new 0x944cf1.
# Quem chama: font_set_mode por 0x5e4a71 (a carga das fontes da interface); font_get_file_count por game_config.cpp
# (0x5d749e...) e Setup.cpp (0x8242c6); font_get_mode_name por game_config.cpp e Setup.cpp; a varredura pelo FONT::FONT
# e pelas tres acima; as auxiliares so pela varredura. Mesmos bytes no cm0102-1080.exe.
# O sx10_13 e o sx10_14 dependem deste (font_files, font_find_folder_once, font_is_font_extension).
TELAS = True
ROTINAS = [('font_set_mode', 0x5b1f30, [0xa1, 0x68, 0x2c, 0xae, 0x00]),
           ('font_get_file_count', 0x5b1fc0, [0xa1, 0x68, 0x2c, 0xae, 0x00]),
           ('font_get_mode_name', 0x5b2040, [0xa1, 0x68, 0x2c, 0xae, 0x00]),
           ('font_scan_folder', 0x5b2170, [0x81, 0xec, 0x48, 0x08, 0x00, 0x00]),
           ('ttf_swap_table_record', 0x5b25a0, [0x8b, 0x01, 0x56, 0x8b, 0xd0]),
           ('ttf_swap_name_table_header', 0x5b2650, [0x66, 0x8b, 0x01, 0x33, 0xd2]),
           ('ttf_swap_name_record', 0x5b2680, [0x66, 0x8b, 0x01, 0x33, 0xd2]),
           ('ttf_read_name_string', 0x5b26e0, [0x8b, 0x54, 0x24, 0x08, 0x33, 0xc0]),
           ('font_file_read_at', 0x5b2770, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('a lista das fontes TrueType: a pasta FONTS, os modos de fonte e os nomes das fontes',
        'the TrueType font list: the FONTS folder, the font modes and the font names',
        [('font_set_mode', 'font_set_mode', '0x5b1f30', 'fog_of_war.cpp', 'cdecl',
          'depois de achar a pasta FONTS (SI_DATA, senão CM3_DATA) e varrê-la uma vez, guarda o modo de fonte quando ele é menor que o número de arquivos; defeito: todo modo negativo passa, não só -2 e -1',
          'after finding the FONTS folder (SI_DATA, else CM3_DATA) and scanning it once, stores the font mode when it is below the number of files; defect: every negative mode passes, not only -2 and -1'),
         ('font_get_file_count', 'font_get_file_count', '0x5b1fc0', 'fog_of_war.cpp', 'cdecl',
          'devolve o número de arquivos de fonte achados na pasta FONTS (depois de achá-la e varrê-la uma vez)',
          'returns the number of font files found in the FONTS folder (after finding and scanning it once)'),
         ('font_get_mode_name', 'font_get_mode_name', '0x5b2040', 'fog_of_war.cpp', 'cdecl',
          'escreve o nome de um modo de fonte: -2 "Futuristic", -1 "Traditional" (textos do langlib), um índice de arquivo o nome completo da fonte ou, sem ele, o nome do arquivo; outro valor, vazio; sempre terminado no tamanho dado',
          'writes the name of a font mode: -2 "Futuristic", -1 "Traditional" (langlib texts), a file index the full font name or, without one, the file name; anything else empty; always terminated within the given size'),
         ('font_scan_folder', 'font_scan_folder', '0x5b2170', 'fog_of_war.cpp', 'cdecl',
          'lista até 100 arquivos .ttf/.ttc da pasta FONTS (molde *.tt?) que abram, com o nome do arquivo, o índice da face (sempre 0: as coleções não são percorridas) e o nome completo lido da tabela name; copia a lista para uma tabela nova e devolve 1, ou 0 sem arquivo; defeitos: a contagem não é zerada antes e o nome duplicado vaza nos arquivos recusados',
          'lists up to 100 .ttf/.ttc files of the FONTS folder (pattern *.tt?) that open, with the file name, the face index (always 0: collections are not walked) and the full name read from the name table; copies the list into a new table and returns 1, or 0 with no file; defects: the count is not reset first and the duplicated name leaks for refused files'),
         ('ttf_swap_table_record', 'ttf_swap_table_record', '0x5b25a0', 'fog_of_war.cpp', 'thiscall',
          'converte os quatro dwords big-endian de um registro do diretório de tabelas TrueType (marca, soma, deslocamento, tamanho); devolve 1',
          'converts the four big-endian dwords of a TrueType table directory record (tag, checksum, offset, length); returns 1'),
         ('ttf_swap_name_table_header', 'ttf_swap_name_table_header', '0x5b2650', 'fog_of_war.cpp', 'thiscall',
          'converte as três words big-endian do cabeçalho da tabela name (formato, contagem, deslocamento dos textos); devolve 1',
          'converts the three big-endian words of the name table header (format, count, string offset); returns 1'),
         ('ttf_swap_name_record', 'ttf_swap_name_record', '0x5b2680', 'fog_of_war.cpp', 'thiscall',
          'converte as seis words big-endian de um registro de nome TrueType (plataforma, codificação, idioma, id, tamanho, deslocamento); devolve 1',
          'converts the six big-endian words of a TrueType name record (platform, encoding, language, id, length, offset); returns 1'),
         ('ttf_read_name_string', 'ttf_read_name_string', '0x5b26e0', 'fog_of_war.cpp', 'cdecl',
          'lê o texto de um registro de nome quando ele tem o id pedido, limitado ao tamanho menos 1 e terminado; devolve 1 quando o texto não começa com zero (os nomes UTF-16 do Windows são recusados)',
          'reads the text of a name record when it has the wanted id, capped at the size minus 1 and terminated; returns 1 when the text does not start with a zero byte (the Windows UTF-16 names are refused)'),
         ('font_file_read_at', 'font_file_read_at', '0x5b2770', 'fog_of_war.cpp', 'cdecl',
          'posiciona o arquivo de fonte aberto num deslocamento e lê um bloco do tamanho dado; 1 quando leu',
          'seeks the open font file to an offset and reads one block of the given size; 1 when it was read')])
