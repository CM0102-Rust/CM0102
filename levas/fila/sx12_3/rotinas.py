# O menu de nacoes partido em dois: a montagem, o cabecalho de cada parte e o lado de uma nacao (key_nation.cpp; leitor
# sx12, rodada 5, 23/11/2026). Rodam na carreira de ferias pela barra de menus (Menubar 0x75db00: 897 montagens,
# 1.794 cabecalhos na cobertura do mundo) e nas telas do tecnico (0x6ae1a0, 0x6b0a10), da configuracao e das
# transferencias.
# Convencoes conferidas na desmontagem:
# - NATION_MENU_SPLIT::NATION_MENU_SPLIT (0x66a280..0x66a34f): thiscall(so_escolhidas, ordem, nome_curto), ret 0xc,
#   devolve this; objeto de 0xe bytes (+0 letra, +1 ordem, +5 so_escolhidas, +9 contagem, +0xa nome_curto). O ebp do
#   segundo laco nunca muda (o indice e o edi, que avanca): [edi+ebp] e [edi] sao o mesmo byte.
# - NATION_MENU_SPLIT::format_header (0x66a360..0x66a42b): thiscall(texto, tamanho, segunda), ret 0xc; so o byte baixo
#   de "segunda" e lido; tamanho < 0x14: caixa "Error" (linha 0x57f); 0x66f4e0 langlib_sprintf cdecl(texto, modelo,
#   letra movsx). O eax de volta nao e lido (0x6af5a1).
# - NATION_MENU_SPLIT::in_second_part (0x66a430..0x66a453): thiscall(nacao), ret 4; o chamador le o al.
SAVE = False
# O cabecalho e o lado da nacao so rodam com mais de 16 nacoes escolhidas (o menu partido): a carreira ENG tem uma so.
# 18 nacoes; a barra de menus e montada todo dia, entao um mes basta.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA POL TUR GRE HRV RUS SCO BEL HOL GER DEN'
ATE = '2001-08-15'
ROTINAS = [('nation_menu_split_construct', 0x66a280, [0x51, 0x8b, 0x44, 0x24, 0x10]),
           ('nation_menu_split_format_header', 0x66a360, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nation_menu_split_in_second_part', 0x66a430, [0x8b, 0x41, 0x0a, 0x85, 0xc0])]
DOCS = ('o menu de nacoes partido em duas partes',
        'the nation menu split in two parts',
        [('nation_menu_split_construct', 'NATION_MENU_SPLIT::NATION_MENU_SPLIT', '0x66a280', 'key_nation.cpp', 'thiscall, ret 0xc',
          'monta a divisao do menu de nacoes: conta as nacoes da tabela das nacoes-chave escolhidas para jogar (so as marcadas com o bit 2, se pedido) e, na ordem dada, a nacao do meio ((contagem + 1) / 2) da a letra onde comeca a segunda parte: a primeira letra do nome curto ou do nome',
          'builds the nation menu split: counts the key nation table nations chosen to be played (only those with bit 2, if asked) and, in the given order, the middle nation ((count + 1) / 2) gives the letter where the second part starts: the first letter of its short name or name'),
         ('nation_menu_split_format_header', 'NATION_MENU_SPLIT::format_header', '0x66a360', 'key_nation.cpp', 'thiscall, ret 0xc',
          'escreve o cabecalho de uma parte do menu: "Nacoes A-{letra}" na primeira, "Nacoes {letra seguinte}-Z" na segunda; texto de menos de 20 bytes: a caixa "Error" e nada',
          'writes the header of one menu part: "Nations A-{letter}" for the first, "Nations {next letter}-Z" for the second; a text under 20 bytes: the "Error" box and nothing'),
         ('nation_menu_split_in_second_part', 'NATION_MENU_SPLIT::in_second_part', '0x66a430', 'key_nation.cpp', 'thiscall, ret 4',
          'diz se a nacao vai para a segunda parte: a primeira letra do nome curto (ou do nome) depois da letra da divisao',
          "tells whether the nation goes in the second part: the first letter of its short name (or name) after the split letter")])
