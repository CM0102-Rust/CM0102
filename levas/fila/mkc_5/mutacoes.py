# Mutacoes (mkc_5), na carreira com as ligas norte-irlandesa e norueguesa (LIGAS): o emprestimo para clube
# norte-irlandes comeca no mes seguinte em vez de tres meses depois; os clubes de Dinamarca, Coreia, Irlanda do Norte e
# Pais de Gales nunca pegam emprestado. A mutacao da Noruega saiu (ver PRONTO.txt): a janela norueguesa vai de 1 de
# setembro a 31 de agosto, o ano todo, e o ramo do prazo nunca roda.
MUTACOES = [
    ('mkc5_irlanda_do_norte_mes_seguinte',
     '.wrapping_add(3); // mkc5: Northern Ireland, three months ahead',
     '.wrapping_add(1); // mutacao: o mes seguinte',
     'mkc_5 northern ireland period'),
    ('mkc5_irlanda_do_norte_nunca_empresta',
     '    1 // mkc5: Northern Ireland, always',
     '    0 // mutacao: nunca',
     'mkc_5 northern ireland loan'),
]
