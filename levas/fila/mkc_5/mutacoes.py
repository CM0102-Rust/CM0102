# Mutacoes (mkc_5), na carreira com as ligas norte-irlandesa e norueguesa (LIGAS): o emprestimo para clube
# norte-irlandes comeca no mes seguinte em vez de tres meses depois; os clubes de Dinamarca, Coreia, Irlanda do Norte e
# Pais de Gales nunca pegam emprestado; o clube noruegues (e o holandes) pega emprestado fora da janela.
MUTACOES = [
    ('mkc5_irlanda_do_norte_mes_seguinte',
     '.wrapping_add(3); // mkc5: Northern Ireland, three months ahead',
     '.wrapping_add(1); // mutacao: o mes seguinte',
     'mkc_5 northern ireland period'),
    ('mkc5_irlanda_do_norte_nunca_empresta',
     '    1 // mkc5: Northern Ireland, always',
     '    0 // mutacao: nunca',
     'mkc_5 northern ireland loan'),
    ('mkc5_noruega_sem_prazo',
     'G_CURRENT_DATE, 0) != 0 {\n        return 1; // mkc5: Norway, the window open',
     'G_CURRENT_DATE, 0) != 0 || true {\n        return 1; // mutacao: sem prazo',
     'mkc_5 norway deadline'),
]
