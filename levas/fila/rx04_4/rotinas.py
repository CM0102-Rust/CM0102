# Se uma língua está no language.ldb, e o construtor e o destrutor da entrada da tabela de línguas (Langlib.cpp; leitor
# rx04, rodada 6, 25/11/2026).
# - 0x66ea50 langlib_is_language_available (cdecl: id; devolve 1/0; chamada por 0x5d7673, 0x5d8036, 0x823a3d,
#   0x823e27, 0x827705: a configuração na abertura e a tela de opções).
# - 0x66f4c0 LANGLIB_ENTRY::LANGLIB_ENTRY (thiscall, sem argumentos, devolve this; passado por endereço ao vector
#   constructor iterator em 0x66efdd). Zera só as 3 strings.
# - 0x66ea10 LANGLIB_ENTRY::~LANGLIB_ENTRY (thiscall; passado por endereço aos iteradores em 0x66e9d1, 0x66ec37, 0x66efd8,
#   0x66f0a1, e nos desenroladores 0x95c8d6/0x95c8f6).
# Iguais no cm0102-1080.exe. Prólogos sem relativo (0x66f4c0: mov eax, ecx; xor ecx, ecx; mov [eax], ecx, 6 bytes;
# 0x66ea10: push esi; mov esi, ecx; mov eax, [esi], 5 bytes).
# Comparadores: a disponível lado a lado (só lê; antes da preparação só a original, por causa da caixa); o construtor
# lado a lado (a cópia numa entrada própria cheia de 0xcd); o destrutor não roda duas vezes nas mesmas strings: a cópia
# destrói uma entrada própria com cópias das 3 strings, depois a original a de quem chama (nada a comparar: só libera;
# a comparação passa por ele ~9.800 vezes a cada carga de língua comparada, quando a original destrói a tabela da cópia).
SAVE = False
ROTINAS = [('langlib_is_language_available', 0x66ea50, [0xa1, 0xdc, 0xb2, 0x9e, 0x00]),
           ('langlib_entry_new', 0x66f4c0, [0x8b, 0xc1, 0x33, 0xc9, 0x89, 0x08]),
           ('langlib_entry_drop', 0x66ea10, [0x56, 0x8b, 0xf1, 0x8b, 0x06])]
DOCS = ('se uma língua está no language.ldb, e a entrada da tabela de línguas',
        'whether a language is in language.ldb, and the language table entry',
        [('langlib_is_language_available', 'langlib_is_language_available', '0x66ea50', 'Langlib.cpp', 'cdecl',
          'regra: devolve 1 se a língua está entre as do language.ldb; antes da preparação, a caixa da linha 0x1a6 e 0',
          "rule: returns 1 when the language is one of language.ldb's; before the setup, the box of line 0x1a6 and 0"),
         ('langlib_entry_new', 'LANGLIB_ENTRY::LANGLIB_ENTRY', '0x66f4c0', 'Langlib.cpp', 'thiscall',
          'regra: a entrada nova com as três strings (chave, formato, tradução) nulas',
          'rule: the new entry with its three strings (key, format, translation) null'),
         ('langlib_entry_drop', 'LANGLIB_ENTRY::~LANGLIB_ENTRY', '0x66ea10', 'Langlib.cpp', 'thiscall',
          'regra: libera a chave, o formato e a tradução da entrada (os que não são nulos), nessa ordem',
          "rule: frees the entry's key, format and translation (the ones not null), in that order")])
INTERESSA = []
