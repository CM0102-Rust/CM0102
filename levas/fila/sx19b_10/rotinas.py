# O motor de busca da tela de procura: a construcao, os clubes e as comparacoes de texto (search_eng.cpp; leitor sx19b,
# rodada 5, 24/11/2026). Rotinas inteiras: 0x804f00..0x804fca (ret), 0x804fd0..0x804fde (ret), 0x804fe0..0x80530d
# (ret 8), 0x806260..0x80644b (ret), 0x805f10..0x805fa8 (ret), 0x805fb0..0x80602a (ret), 0x806030..0x80607c (ret) e
# 0x8061f0..0x80625c (ret).
# O sx19b_11 (as pessoas) usa search_text_contains, c_string_len e SEARCH_ENG_*: integra junto ou depois.
# Quem chama: a tela de procura (Procurar, no menu: 0x5a3173.. o construtor, 0x5a4ed5.. o destrutor, 0x5a3280 e 0x5a3736
# os clubes); a comparacao de texto tambem pela busca das noticias (0x7887f6, 0x788826, 0x78897e, 0x7889a3) e pelas
# outras comparacoes. A carreira automatica nao passa.
# TELAS: o caminho no jogo: Procurar (o botao de busca da barra), clubes e selecoes: um texto vazio, um nome de clube,
# um nome de nacao, de divisao e de continente; nas noticias, a busca por texto.
# Convencoes conferidas na desmontagem:
# - 0x804f00 thiscall(motor) -> motor; quadro SEH (o estado 0 marca a base ja construida); 0x944e46 malloc([0xae2364] x 4);
# - 0x804fd0 thiscall(motor): 0x9452ca free(+0x34) (o ponteiro fica);
# - 0x804fe0 thiscall(motor, texto, selecao) ret 8: 0x53b900 database_is_national_team cdecl(clube); 0x945333
#   qsort(+0x34, +0x38, 4, 0x540080 database_compare_clubs_by_short_name, ja Rust); os clubes contados sao [0xae235c] -
#   [0xae2350] (o total de clubes menos o de nacoes, como o original faz);
# - 0x806260 cdecl(campo, texto, separar): 0x90c1d0 utils_string_compare cdecl(a, b, 0, n) -> 0 igual; a busca por
#   palavras troca o espaco por 0 durante a comparacao e o devolve;
# - 0x805f10 / 0x805fb0 / 0x806030 / 0x8061f0 cdecl(registro, texto): os campos de nome do continente (+4, +0x1f,
#   +0x23, +0x3d, +0xa3), da nacao (+4, +0x38, +0x53, +0x57), do clube (+4, +0x38) e da divisao (+4, +0x38, +0x53).
# Comparadores: as comparacoes de texto lado a lado; find_clubs lado a lado sobre o vetor (o da original e guardado,
# a copia o refaz, compara e o da original volta); construtor e destrutor so a original.
TELAS = True
ROTINAS = [('new_search_engine', 0x804f00, [0x6a, 0xff, 0x68, 0x5b, 0x11, 0x96, 0x00]),
           ('search_engine_destructor', 0x804fd0, [0x8b, 0x41, 0x34, 0x85, 0xc0]),
           ('find_clubs', 0x804fe0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('search_text_contains', 0x806260, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('search_text_matches_continent', 0x805f10, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('search_text_matches_nation', 0x805fb0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('search_text_matches_club', 0x806030, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('search_text_matches_division', 0x8061f0, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('o motor de busca da tela de procura: construção, clubes e comparações de texto',
        'the find screen search engine: construction, clubs and text matching',
        [('new_search_engine', 'SearchEngine::SearchEngine', '0x804f00', 'search_eng.cpp', 'thiscall',
          'monta o motor de busca: esvazia o texto de erro, tipo 2, um vetor de um ponteiro por pessoa (sem memória: a caixa) e nenhum resultado',
          'builds the search engine: empties the error text, kind 2, an array of one pointer per person (no memory: the box) and no results'),
         ('search_engine_destructor', 'SearchEngine::~SearchEngine', '0x804fd0', 'search_eng.cpp', 'thiscall',
          'libera o vetor de resultados, se houver',
          'frees the result array, if any'),
         ('find_clubs', 'SearchEngine::find_clubs', '0x804fe0', 'search_eng.cpp', 'thiscall, ret 8',
          'enche o vetor com os clubes (ou seleções) cujo nome, divisão, nação ou continente contém o texto (texto vazio: todos), só os de nação com continente, e ordena pelo nome curto',
          'fills the array with the clubs (or national teams) whose name, division, nation or continent contains the text (empty text: all), only those whose nation has a continent, and sorts them by short name'),
         ('search_text_contains', 'search_text_contains', '0x806260', 'search_eng.cpp', 'cdecl',
          'diz se o texto aparece em algum ponto do campo (comparação do jogo, sem diferenciar maiúsculas); com "separar", se alguma palavra dele aparece; texto maior que o campo: 0',
          'tells whether the text occurs anywhere in the field (the game\'s comparison, case-insensitive); with "split", whether any word of it occurs; a text longer than the field: 0'),
         ('search_text_matches_continent', 'search_text_matches_continent', '0x805f10', 'search_eng.cpp', 'cdecl',
          'diz se o texto aparece em algum dos cinco nomes do continente',
          'tells whether the text occurs in any of the continent\'s five names'),
         ('search_text_matches_nation', 'search_text_matches_nation', '0x805fb0', 'search_eng.cpp', 'cdecl',
          'diz se o texto aparece em algum dos quatro nomes da nação',
          'tells whether the text occurs in any of the nation\'s four names'),
         ('search_text_matches_club', 'search_text_matches_club', '0x806030', 'search_eng.cpp', 'cdecl',
          'diz se o texto aparece no nome longo ou curto do clube',
          'tells whether the text occurs in the club\'s long or short name'),
         ('search_text_matches_division', 'search_text_matches_division', '0x8061f0', 'search_eng.cpp', 'cdecl',
          'diz se o texto aparece em algum dos três nomes da divisão',
          'tells whether the text occurs in any of the division\'s three names')])
CAMPOS = [('motor de busca da tela de procura: +0x33 o tipo (2); +0x34 o vetor de resultados (um ponteiro por pessoa); +0x38 quantos',
           'find screen search engine: +0x33 the kind (2); +0x34 the result array (one pointer per person); +0x38 how many')]
INTERESSA = []
