# A ordem do menu das nacoes-chave e a contagem de partes do menu de nacoes (key_nation.cpp; leitor sx12, rodada 5,
# 23/11/2026). Rodam na carreira de ferias: a barra de menus (Menubar 0x75db00) e montada a cada dia (897 chamadas na
# cobertura do mundo), alem das telas do tecnico, da busca, da configuracao e das transferencias.
# Convencoes conferidas na desmontagem:
# - key_nation_sorted_order (0x66a0c0..0x66a0f5): cdecl(flag), devolve 0xb63d38; o flag vai inteiro para o dword
#   0xb63d5c; qsort 0x945333 cdecl(0xb63d38, 0x23, 1, 0x66a100).
# - key_nation_compare_entries (0x66a100..0x66a27b): cdecl(a, b) do qsort (indices de um byte); 0x90c1d0
#   utils_string_compare cdecl(a, b, 1, 0) sobre +4 do continente, +4 ou +0x38 da nacao.
# - NATION_MENU_SPLIT::part_count (0x66a350..0x66a359): thiscall, o chamador le o al (0x6af55f "cmp al, 1"); o resto do
#   eax e o que sobrou (inc eax).
SAVE = False
ROTINAS = [('key_nation_sorted_order', 0x66a0c0, [0x32, 0xc0, 0xb9, 0x38, 0x3d, 0xb6, 0x00]),
           ('key_nation_compare_entries', 0x66a100, [0x8b, 0x4c, 0x24, 0x04, 0x53]),
           ('nation_menu_split_part_count', 0x66a350, [0x80, 0x79, 0x09, 0x10, 0x0f, 0x9f, 0xc0])]
DOCS = ('a ordem do menu das nacoes-chave e as partes do menu de nacoes',
        'the key nation menu order and the nation menu parts',
        [('key_nation_sorted_order', 'key_nation_sorted_order', '0x66a0c0', 'key_nation.cpp', 'cdecl',
          'poe os indices 0 a 34 das linhas da tabela das nacoes-chave na ordem dos menus (o qsort com a comparacao abaixo) e devolve o vetor estatico; o argumento diz se as nacoes se comparam pelo nome curto',
          'puts the indices 0 to 34 of the key nation table rows in menu order (qsort with the comparison below) and returns the static array; the argument says whether the nations compare by their short name'),
         ('key_nation_compare_entries', 'key_nation_compare_entries', '0x66a100', 'key_nation.cpp', 'cdecl',
          'compara duas linhas da tabela das nacoes-chave: primeiro as tres linhas do mundo (sem nacao nem continente) pelo indice, depois as dos continentes pelo nome do continente, depois as nacoes pelo nome (ou nome curto, pelo indicador); ponteiro nulo: o a nulo vai depois, o b nulo antes',
          'compares two key nation table rows: first the three world rows (no nation nor continent) by index, then the continent rows by the continent name, then the nations by name (or short name, by the flag); a null pointer: a null a goes after, a null b before'),
         ('nation_menu_split_part_count', 'NATION_MENU_SPLIT::part_count', '0x66a350', 'key_nation.cpp', 'thiscall',
          'devolve 2 quando o menu de nacoes tem mais de 16 nacoes e se divide em duas partes, senao 1',
          'returns 2 when the nation menu holds more than 16 nations and is split in two parts, else 1')])
