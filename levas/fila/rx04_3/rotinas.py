# A carga de uma língua do language.ldb, a preparação e o fim da biblioteca de línguas (Langlib.cpp; leitor rx04,
# rodada 6, 25/11/2026). Rodam na abertura do programa (a preparação, 0x5d7304, e a língua do game.cfg, 0x5d7689) e na
# saída (0x5d7bb4, salto final para 0x66e9c0).
# - 0x66eb40 langlib_change_language (2.432 bytes; cdecl: língua, progresso; devolve 1/0; chamada por 0x5d7689,
#   0x5d823e, 0x5d858f, pela preparação e por ela mesma). Lida instrução a instrução: o marcador 0xf1fffff1
#   (utils_read_file_format_marker 0x90c140) pede os dwords invertidos; versão 1 ou 2 (na 2 as línguas 0 e 1 não
#   contam); a tabela por operator new (n*0x34+4) e o vector constructor iterator 0x9463af com 0x66f4c0/0x66ea10; o
#   progresso por ScreenManager::send_network_status_message 0x802f80 (thiscall em 0xb74450: msg, i+1, n, -1) a cada
#   250; qsort com 0x682060.
# - 0x66e790 langlib_setup (cdecl: caminho, flag; o jogo passa 1). GetKeyboardLayout pela importação [0x9671d8]; a
#   língua pela tabela de saltos 0x66e960/0x66e9a0 (lida do exe).
# - 0x66e9c0 langlib_shutdown (cdecl; devolve o eax do operator delete, ou 0, que quem salta para ela repassa).
# Iguais no cm0102-1080.exe (inteiras). Prólogos sem relativo (0x66eb40: mov eax, fs:[0], 6 bytes).
# O language.ldb do jogo: versão 1, 9.807 textos, 10 línguas (0, 2..10); a carga do português lê os 204 MB inteiros.
# Comparadores: um depois do outro (a carga não volta atrás): a cópia carrega, a língua de antes volta, a original
# carrega (destruindo a tabela da cópia); o resultado e o estado inteiro (língua, posição, disponíveis, contagens, erro,
# caminho e cada entrada da tabela, num hash) iguais. O fim: a cópia (sem tabela e com a língua -1), depois a original.
SAVE = False
ROTINAS = [('langlib_change_language', 0x66eb40, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('langlib_setup', 0x66e790, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('langlib_shutdown', 0x66e9c0, [0xa1, 0xf0, 0x79, 0xb6, 0x00])]
DOCS = ('a carga de uma língua do language.ldb, a preparação e o fim da biblioteca de línguas',
        'loading a language from language.ldb, the setup and the shutdown of the language library',
        [('langlib_change_language', 'langlib_change_language', '0x66eb40', 'Langlib.cpp', 'cdecl',
          'regra: carrega uma língua do language.ldb: antes da preparação, a caixa da linha 0x1ce; a tabela antiga é destruída; o arquivo dá a versão (1 ou 2), o número de textos e de línguas e as línguas disponíveis; língua que não está no arquivo: carrega a primeira disponível; inglês (0) ou a biblioteca que lê o arquivo a cada busca param aí; senão, uma entrada por texto (a chave inglesa sem chaves, sem espaços e com "\\n" como quebra de linha, o formato dos argumentos, a tradução e a ordem dos pedaços dessa língua) e a tabela ordenada pela chave; com progresso, a mensagem "Changing Language" a cada 250 textos',
          'rule: loads a language from language.ldb: before the setup, the box of line 0x1ce; the old table is destroyed; the file gives the version (1 or 2), the number of texts and of languages and the available languages; a language not in the file: the first available one is loaded; English (0) or a library that reads the file at each lookup stop there; otherwise one entry per text (the English key without braces, without spaces and with "\\n" as a line break, the argument format, that language\'s translation and piece order) and the table sorted by key; with progress, the "Changing Language" message every 250 texts'),
         ('langlib_setup', 'langlib_setup', '0x66e790', 'Langlib.cpp', 'cdecl',
          'regra: guarda o caminho do language.ldb e se a língua vai toda para a memória, e carrega a língua do teclado do Windows: inglês dos EUA 1, outro inglês 0, francês 2, alemão 3, italiano 4, espanhol 5 (mexicano 18), dinamarquês 6, holandês 7, norueguês 8, português 9 (do Brasil 17), sueco 10, japonês 11, coreano 12, polonês 13, russo 14, os outros 0',
          'rule: keeps the path of language.ldb and whether the whole language goes into memory, and loads the language of the Windows keyboard: US English 1, other English 0, French 2, German 3, Italian 4, Spanish 5 (Mexican 18), Danish 6, Dutch 7, Norwegian 8, Portuguese 9 (Brazilian 17), Swedish 10, Japanese 11, Korean 12, Polish 13, Russian 14, any other 0'),
         ('langlib_shutdown', 'langlib_shutdown', '0x66e9c0', 'Langlib.cpp', 'cdecl',
          'regra: destrói a tabela de textos e põe a língua em -1',
          'rule: destroys the text table and sets the language to -1')])
INTERESSA = []
