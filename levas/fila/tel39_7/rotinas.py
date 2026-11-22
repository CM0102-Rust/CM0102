# A classe Text (2): o construtor, o destruidor e o esvaziar (comp_text.cpp; leitor tel39, rodada 2 das telas,
# 22/11/2026).
# Convencoes conferidas na desmontagem: Text::Text(texto) thiscall(this, origem) ret 4 -> this; Text::~Text
# thiscall(this) ret; Text::clear thiscall(this) ret -> eax 1. Chamadas: Text::set_string (Rust, tel39_6),
# operator delete 0x944c94, LLIST::~LLIST 0x68d040.
# Quando rodam: em todo texto de noticia e de tela (0x4ae590 em 0x42544a... 0x7828ea, 0x788250; 0x4ae630 em dezenas
# de funcoes e funclets; 0x4afa40 em 0x44971b, 0x6b2ffe, 0x77bd30, 0x788806...): na carreira o tempo todo.
# Comparadores: o construtor lado a lado (a copia num Text local); o destruidor so a original; o esvaziar so a
# original, conferindo depois os dois ponteiros nulos e o retorno 1.
# O construtor de copia (0x4ae5c0) foi para o tel39_2: so a caixa de confirmacao com Text (file_screens.cpp,
# 0x591a1b) o chama, nas telas.
# Depende de: tel39_6 (text_set_string, TEXT_*, comp_text_free), text_contents (compare/media.rs).
ROTINAS = [('text_construct', 0x4ae590, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('text_destruct', 0x4ae630, [0x56, 0x8b, 0xf1, 0x8b, 0x06]),
           ('text_clear', 0x4afa40, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x06])]
DOCS = ('a classe Text (2): o construtor, o destruidor e o esvaziar',
        'the Text class (2): the constructor, the destructor and clearing',
        [('text_construct', 'Text::Text', '0x4ae590', 'comp_text.cpp', 'thiscall',
          'constrói um Text sem texto e sem vínculos e copia o texto inicial quando há um',
          'constructs a Text with no string and no links and copies the initial string when there is one'),
         ('text_destruct', 'Text::~Text', '0x4ae630', 'comp_text.cpp', 'thiscall',
          'destrói um Text: libera o texto e a lista de vínculos (os ponteiros ficam como estavam)',
          'destroys a Text: frees the string and the link list (the pointers stay as they were)'),
         ('text_clear', 'Text::clear', '0x4afa40', 'comp_text.cpp', 'thiscall',
          'esvazia um Text para reuso: libera o texto e a lista e zera os dois ponteiros; devolve 1',
          'empties a Text for reuse: frees the string and the list and nulls both pointers; returns 1')])
