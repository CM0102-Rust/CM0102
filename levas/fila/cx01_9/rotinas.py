# A ordem dos clubes pela ultima divisao (comp_util.cpp, usada pela copa turca; leitor cx01, rodada 5, 24/11/2026).
# Competicoes com a Turquia (o grupo do mwd_27, TurCup::collect_entrants: qsort(lista, n, 4, 0x4c0a40) em 0x8fcd1a).
# - 0x4c0a40 compare_clubs_by_last_division cdecl(a, b) -> int: le so o clube de cada ponteiro, a divisao +0x5b
#   (reputacao +0x69 e id +0) e a posicao +0x5f.
# Igual no cm0102-1080.exe (inteira).
# Comparador: compare_order (o valor).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('compare_clubs_by_last_division', 0x4c0a40, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08])]
DOCS = ('a ordem dos clubes pela última divisão',
        'the order of clubs by their last division',
        [('compare_clubs_by_last_division', 'compare_clubs_by_last_division', '0x4c0a40', 'comp_util.cpp', 'cdecl',
          'ordena clubes pela última temporada: quem não tem última divisão vai depois; entre os que têm, a divisão de maior reputação primeiro, depois o maior id da competição, depois a melhor (menor) posição',
          'orders clubs by their last season: one without a last division goes after; between those with one, the division of higher reputation first, then the higher competition id, then the better (lower) position')])
CAMPOS = []
INTERESSA = []
