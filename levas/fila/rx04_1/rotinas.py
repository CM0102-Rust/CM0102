# O estado da biblioteca de línguas (Langlib.cpp) e a busca da tradução de um texto (leitor rx04, rodada 6, 25/11/2026).
# Rodam em todo texto de notícia e de tela (langlib_sprintf chama a busca; a língua é lida por várias rotinas).
# - 0x66f4d0 langlib_get_language (cdecl, sem argumentos; 6 bytes: mov eax, [0x9eb2dc]; ret).
# - 0x682060 langlib_compare_string_ptrs (cdecl, 2 ponteiros; a ordem da tabela, passada por endereço ao qsort 0x945333
#   e ao bsearch 0x946464, push 0x682060 em 0x66f479 e 0x66fcc0; o strcmp em linha, sem sinal, -1/0/1).
# - 0x66fbb0 langlib_lookup_translation (cdecl: entrada, texto, índice ou 0; chamada só de 0x66f570, add esp, 0xc;
#   devolve 1/0). Lida instrução a instrução: a chave sem chaves, sem espaços, com "\n" e '\"'; com [0xb647b0] == 1,
#   bsearch na tabela (a chave numa entrada de busca local, o 2o ponteiro dela) e a cópia das 3 strings (operator
#   delete da antiga, operator new strlen+1, strcpy) e dos 10 dwords; senão o language.ldb lido registro a registro
#   (fseek do cabeçalho, fread 0x800 da chave, 0x1e do formato, 0x800 + 10 dwords por língua; fseek SEEK_CUR
#   0x828n - 0x80a). O laço que tira espaços do fim da chave (0x66fc45) nunca age (os espaços já saíram): omitido.
# Iguais no cm0102-1080.exe (inteiras). Prólogos sem call/jmp relativo.
# Defeitos do original: a busca pelo arquivo não tira as chaves nem o '\"' da chave lida (texto com eles nunca é
# achado assim) e não inverte os números de um arquivo big-endian; o jogo só usa a tabela (setup sempre com 1).
# Comparadores: get_language e a ordem, valor contra valor; a busca lado a lado (a original na entrada de quem chama,
# a cópia numa entrada própria, liberada depois; resultado, posição, strings e números iguais), só com a tabela.
# Determinismo: só a tabela e o texto.
SAVE = False
ROTINAS = [('langlib_get_language', 0x66f4d0, [0xa1, 0xdc, 0xb2, 0x9e, 0x00]),
           ('langlib_compare_string_ptrs', 0x682060, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04]),
           ('langlib_lookup_translation', 0x66fbb0, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a língua do jogo e a busca da tradução de um texto',
        "the game language and the lookup of a text's translation",
        [('langlib_get_language', 'langlib_get_language', '0x66f4d0', 'Langlib.cpp', 'cdecl',
          'regra: devolve a língua atual do jogo (-1 antes da preparação e depois do fim, 0 inglês, 9 português...)',
          'rule: returns the current game language (-1 before the setup and after the shutdown, 0 English, 9 Portuguese...)'),
         ('langlib_compare_string_ptrs', 'langlib_compare_string_ptrs', '0x682060', 'Langlib.cpp', 'cdecl',
          'regra: compara os textos para onde dois ponteiros apontam, como o strcmp (-1, 0 ou 1, byte sem sinal): a ordem da tabela de textos',
          'rule: compares the strings two pointers point to, as strcmp (-1, 0 or 1, unsigned bytes): the order of the text table'),
         ('langlib_lookup_translation', 'langlib_lookup_translation', '0x66fbb0', 'Langlib.cpp', 'cdecl',
          'regra: acha a tradução de um texto inglês na língua atual: a chave é o texto sem chaves e sem espaços, com "\\n" virando quebra de linha; com a língua na memória (o que o jogo usa), busca binária na tabela; senão, o language.ldb lido texto a texto; achada, copia a chave, o formato dos argumentos, a tradução e a ordem dos pedaços para a entrada de quem chama e devolve 1 (e a posição na tabela, se pedida); sem o arquivo, a caixa da linha 0xda6',
          "rule: finds the translation of an English text in the current language: the key is the text without braces and spaces, with \"\\n\" as a line break; with the language in memory (what the game uses), a binary search of the table; otherwise language.ldb read text by text; when found, copies the key, the argument format, the translation and the order of the pieces into the caller's entry and returns 1 (and the table position, when asked); without the file, the box of line 0xda6")])
CAMPOS = [('entrada da tabela de línguas (0x34 bytes, tabela 0xb679f0): +0 chave inglesa, +4 formato dos argumentos (vsprintf, "~" entre os pedaços), +8 tradução (0x08 = pedaço), +0xc 10 números dos pedaços na ordem da tradução; lê e grava',
           'language table entry (0x34 bytes, table 0xb679f0): +0 English key, +4 argument format (vsprintf, "~" between pieces), +8 translation (0x08 = a piece), +0xc 10 piece numbers in translation order; reads and writes')]
INTERESSA = []
