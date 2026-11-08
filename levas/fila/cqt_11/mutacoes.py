# Mutacoes (cqt_11), no Campeonato de Clubes da Oceania de janeiro de 2002: o campeao australiano e o neozelandes no
# grupo contrario ao sorteado; o par Fiji/Taiti idem (os grupos mudam); as semifinais 1o x 1o e 2o x 2o. Os contadores
# ficam colados no efeito.
MUTACOES = [
    ('cqt11_campeao_no_outro_grupo',
     'place_pot_pair(this, 0, campeao, nz.clubes[0], sorteio(2)); // cqt11: o campeão australiano e o neozelandês',
     'place_pot_pair(this, 0, campeao, nz.clubes[0], (sorteio(2) == 0) as i32); // mutacao: o outro grupo',
     'cqt_11 clubes da oceania'),
    ('cqt11_fiji_no_outro_grupo',
     'place_pot_pair(this, 1, fij.clubes[0], tai.clubes[0], sorteio(2)); // cqt11: Fiji e Taiti',
     'place_pot_pair(this, 1, fij.clubes[0], tai.clubes[0], (sorteio(2) == 0) as i32); // mutacao: o outro grupo',
     'cqt_11 clubes da oceania'),
    ('cqt11_semifinais_primeiro_contra_primeiro',
     '[a(0), b(1), a(1), b(0)] // cqt11: 1º A × 2º B, 2º A × 1º B',
     '[a(0), b(0), a(1), b(1)] // mutacao: 1o x 1o',
     'cqt_11 cruzamentos da oceania'),
]
