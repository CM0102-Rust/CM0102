# Mutacoes. A posicao prevista (+0x72 na estatistica da competicao) entra na importancia das partidas de liga e de copa
# (o motor le de novo a cada jogo): a ordenacao pela forca invertida (17 chamadas ate 2002-06-01: as posicoes previstas
# das copas e das ligas que usam a 0x4c08f0 saem trocadas), a posicao da rodada uma a mais na copa (12) e a posicao
# gravada uma a mais (todas as posicoes previstas, das copas e das ligas, no comeco de cada competicao). Os ramos ficam
# depois de todas as condicoes, colados no efeito.
MUTACOES = [
    ('cpt13_ordem_invertida', 'if *k.add(i) < *k.add(j) { // cpt13: a força maior sobe',
     'if *k.add(i) > *k.add(j) { // mutacao: a menor sobe',
     'cpt_13 clubs sorted'),
    ('cpt13_rodada_mais_um', 'a = b; // cpt13: a posição da rodada',
     'a = b.wrapping_add(1); // mutacao: uma a mais',
     'cpt_13 position by round'),
    ('cpt13_posicao_mais_um', 'core::ptr::write_volatile((e + 0x72) as *mut u8, pos as u8); // cpt13: a posição',
     'core::ptr::write_volatile((e + 0x72) as *mut u8, (pos as u8).wrapping_add(1)); // mutacao: uma a mais',
     'cpt_13 position recorded'),
]
