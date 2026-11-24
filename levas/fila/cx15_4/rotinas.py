# As ordens das sedes neutras por nacao e por continente (stadium.cpp; leitor cx15, rodada 5, 24/11/2026). Carreira ENG
# (sem LIGAS): a ordem por nacao roda na criacao do jogo (build_venue_list 0x85e231, o qsort da lista das sedes) e as
# duas no find_available_venue 0x85c800 (0x85c824 e 0x85c83b por nacao, 0x85c8a4 e 0x85c8be por continente), ja em Rust,
# quando um jogo precisa de campo neutro (finais, amistosos de selecoes).
# - 0x85e2f0 stadium_compare_venues_by_nation cdecl(a, b) -> i32: +0 (a - b); iguais, com os dois estadios (+0xc) a
#   capacidade +0x3c de b - a de a, senao 0. Prologo mov ecx, [esp+4]; mov edx, [esp+8] (8 bytes).
# - 0x85e320 stadium_compare_venues_by_continent cdecl(a, b) -> i32: o mesmo com +4. Prologo igual (8 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: as duas no mesmo par, resultados iguais.
SAVE = False
ROTINAS = [('stadium_compare_venues_by_nation', 0x85e2f0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('stadium_compare_venues_by_continent', 0x85e320, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('as ordens das sedes neutras por nação e por continente',
        'the neutral venue orders by nation and by continent',
        [('stadium_compare_venues_by_nation', 'stadium_compare_venues_by_nation', '0x85e2f0', 'stadium.cpp', 'cdecl',
          'ordena as sedes neutras pela nação e, na mesma nação, pela capacidade do estádio, a maior primeiro',
          'sorts the neutral venues by nation and, in the same nation, by stadium capacity, the largest first'),
         ('stadium_compare_venues_by_continent', 'stadium_compare_venues_by_continent', '0x85e320', 'stadium.cpp', 'cdecl',
          'ordena as sedes neutras pelo continente e, no mesmo continente, pela capacidade do estádio, a maior primeiro',
          'sorts the neutral venues by continent and, in the same continent, by stadium capacity, the largest first')])
CAMPOS = []
INTERESSA = []
