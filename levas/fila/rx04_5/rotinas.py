# As formas norueguesas e alemãs das palavras de tipo de jogador (Langlib.cpp; leitor rx04, rodada 6, 25/11/2026).
# Só a 0x6701f0 langlib_apply_grammar_modifier as chama (0x671485, 0x6714d4; 0x674d54, 0x674db1), e só com o jogo em
# norueguês (8) ou alemão (3): não rodam na carreira de teste (em português).
# - 0x6820a0 langlib_inflect_norwegian_player_type e 0x682150 langlib_inflect_german_player_type (cdecl: texto, forma;
#   add esp, 8; quem chama não usa o eax). As tabelas 0x9eb328 e 0x9eb7ac: 41 linhas de 4 ponteiros (a forma de base e
#   as formas 1 a 3), terminadas por uma de base vazia; a maior forma tem 37 bytes.
# Iguais no cm0102-1080.exe. Prólogos: mov eax, [tabela] (5 bytes).
# Comparador: lado a lado, a cópia numa cópia do texto (0x100 bytes).
SAVE = False
ROTINAS = [('langlib_inflect_norwegian_player_type', 0x6820a0, [0xa1, 0x28, 0xb3, 0x9e, 0x00]),
           ('langlib_inflect_german_player_type', 0x682150, [0xa1, 0xac, 0xb7, 0x9e, 0x00])]
DOCS = ('as formas norueguesas e alemãs das palavras de tipo de jogador',
        'the Norwegian and German forms of the player type words',
        [('langlib_inflect_norwegian_player_type', 'langlib_inflect_norwegian_player_type', '0x6820a0', 'Langlib.cpp', 'cdecl',
          'regra: se o texto é uma das 41 palavras de tipo de jogador em norueguês ("offensive back"...), troca pela forma 1, 2 ou 3 (a definida, "offensive backen"...); outra forma ou outra palavra deixam o texto',
          'rule: when the text is one of the 41 Norwegian player type words ("offensive back"...), replaces it by form 1, 2 or 3 (the definite one, "offensive backen"...); another form or word leaves the text'),
         ('langlib_inflect_german_player_type', 'langlib_inflect_german_player_type', '0x682150', 'Langlib.cpp', 'cdecl',
          'regra: se o texto é uma das 41 palavras de tipo de jogador em alemão ("offensiver Außenverteidiger"...), troca pelo caso 1, 2 ou 3; outra forma ou outra palavra deixam o texto',
          'rule: when the text is one of the 41 German player type words ("offensiver Außenverteidiger"...), replaces it by case form 1, 2 or 3; another form or word leaves the text')])
INTERESSA = []
