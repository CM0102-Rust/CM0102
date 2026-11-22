# printouts.cpp's static dates and the pundit list reader (printouts.cpp; reader tel36, the screens round, 21/11/2026).
# Conventions checked in the disassembly: the static initialisers cdecl(void) (Date::Date 0x54a110 on the static,
# atexit 0x94627a); the destructors a bare `ret` (1 byte; the detour's jump lands in the alignment padding);
# PunditList::read_pundit_line thiscall(list, file, line) ret 8 -> al; parse_media_type_name thiscall(list, name)
# ret 4 -> al and PunditList::parse_quoted_token thiscall(list, src, out) ret 8 -> eax (the caller loads ecx = the
# list; neither reads it). Calls: utils_read_config_line 0x90cba0 cdecl(file, line); sscanf 0x946fd8 and strncpy
# 0x9466d0 cdecl; strncmp 0x946fa0 cdecl; LLIST::add_tail 0x68d0d0 thiscall(list, &record, 0xd4) ret 8.
# parse_quoted_token and parse_media_type_name checked against the original by emulating it (unicorn): 3000 random
# lines and the seven media names plus three others.
# When they run: the static routines at every program start and exit (before WinMain); the pundit reader from the
# pundit list loader 0x7da190, called at a new game and at a load (0x535848, 0x829f95, 0x82cceb): it also runs at the
# start of the automatic career (one call per line of the pundit file), so the career comparison already sees it.
# Defects of the original: the pundit record's bytes 0x66..0xc9 are never written (stack garbage goes into the list);
# the copy writes 0 there and the comparer skips them.
# Equal in cm0102-1080.exe.
# Comparers: the pundit reader runs the original only (it reads the file) and checks the record it added (the tail of
# the list that grew, the serial) against the copy's parsing of the line it read; the static routines only the original.
TELAS = True
ROTINAS = [('printouts_static_init_date_b74360', 0x7d4400, [0xb9, 0x60, 0x43, 0xb7, 0x00]),
           ('printouts_static_destroy_date_b74360', 0x7d4420, [0xc3, 0x90, 0x90, 0x90, 0x90]),
           ('printouts_static_init_date_b74358', 0x7d4430, [0xb9, 0x58, 0x43, 0xb7, 0x00]),
           ('printouts_static_destroy_date_b74358', 0x7d4450, [0xc3, 0x90, 0x90, 0x90, 0x90]),
           ('pundit_list_read_pundit_line', 0x7da6e0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0xcc, 0x02, 0x00, 0x00]),
           ('parse_media_type_name', 0x7daa70, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('pundit_list_parse_quoted_token', 0x7dab30, [0x55, 0x8b, 0x6c, 0x24, 0x0c])]
DOCS = ('as datas estáticas do printouts.cpp e a leitura da lista de comentaristas',
        "printouts.cpp's static dates and the pundit list reader",
        [('printouts_static_init_date_b74360', 'printouts_static_init_date_b74360', '0x7d4400', 'printouts.cpp', 'cdecl',
          'a data estática 0xb74360 nasce vazia na partida do programa, com o destruidor no atexit',
          'the static date 0xb74360 is built empty at program start, with its destructor registered with atexit'),
         ('printouts_static_destroy_date_b74360', 'printouts_static_destroy_date_b74360', '0x7d4420', 'printouts.cpp', 'cdecl',
          'o destruidor da data 0xb74360: nada',
          'the destructor of the date 0xb74360: nothing'),
         ('printouts_static_init_date_b74358', 'printouts_static_init_date_b74358', '0x7d4430', 'printouts.cpp', 'cdecl',
          'a data estática 0xb74358 nasce vazia na partida do programa, com o destruidor no atexit',
          'the static date 0xb74358 is built empty at program start, with its destructor registered with atexit'),
         ('printouts_static_destroy_date_b74358', 'printouts_static_destroy_date_b74358', '0x7d4450', 'printouts.cpp', 'cdecl',
          'o destruidor da data 0xb74358: nada',
          'the destructor of the date 0xb74358: nothing'),
         ('pundit_list_read_pundit_line', 'PunditList::read_pundit_line', '0x7da6e0', 'printouts.cpp', 'thiscall, ret 8',
          'lê uma linha do arquivo de comentaristas: o número, o nome, o tipo de mídia e, nas mídias locais (jornal e rádio locais), a cidade ou o clube (a cidade do estádio dele); depois a nação (Inglaterra quando vazia ou desconhecida); o registro vai para a lista local ou a nacional com o número de série corrente; o comentarista local de lugar desconhecido é pulado; 0 no fim do arquivo ou sem número',
          'reads one line of the pundit file: the number, the name, the media type and, for the local media (local daily and radio), the city or the club (its stadium\'s city); then the nation (England when empty or unknown); the record goes to the local or the national list with the running serial; a local pundit with an unknown place is skipped; 0 at the end of the file or with no number'),
         ('parse_media_type_name', 'parse_media_type_name', '0x7daa70', 'printouts.cpp', 'thiscall, ret 4',
          'o nome do tipo de mídia dá o código: NATIONAL_DAILY 1, NATIONAL_SUNDAY 7, WEBSITE 0, RADIO 3, MAGAZINE 5, LOCAL_RADIO 4, LOCAL_DAILY 2; outro, -1',
          'the media type name gives its code: NATIONAL_DAILY 1, NATIONAL_SUNDAY 7, WEBSITE 0, RADIO 3, MAGAZINE 5, LOCAL_RADIO 4, LOCAL_DAILY 2; any other -1'),
         ('pundit_list_parse_quoted_token', 'PunditList::parse_quoted_token', '0x7dab30', 'printouts.cpp', 'thiscall, ret 8',
          'tira o próximo texto entre aspas da linha (até 98 caracteres; brancos antes dele pulados) e devolve o ponto de continuar (um depois do caractere seguinte ao fim)',
          'takes the next quoted text of the line (up to 98 characters; blanks before it skipped) and returns where to continue (one past the character after the end)')])
