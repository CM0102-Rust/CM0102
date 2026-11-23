# As escapadas da 324a leva (textos de noticias, itens gra; ENG ate 2002-06-01).
# A confirmacao da cadeia no modo comparando vai so ate 2001-08-20; a comparacao longa (ate 2002-06-01, bancadas 4 e
# 5: levas/324/conf_escapadas_N.txt) acusou gra1_sorteio_vinculo e gra18_vinculo_do_clube e mediu as outras duas.

def _par(pt, en):
    return (pt, en)


_CONF_PT = ('Nada da simulacao le o texto das noticias; o comparador de texto acusa, mas na comparacao ate 2001-08-20 o '
            'ramo nao rodou no modo comparando (a carreira comparando sorteia diferente da Rust). Medido: comparacao ate 2002-06-01 (levas/324/conf_escapadas_5.txt) com 0 divergencias em 10^10 comparacoes do motor.')
_CONF_EN = ('Nothing in the simulation reads the news text; the text comparer catches it, but in the comparison to '
            '2001-08-20 the branch did not run in comparison mode (the comparing career draws differently from the Rust '
            'one). Measured: comparison to 2002-06-01 (levas/324/conf_escapadas_5.txt) with 0 divergences in 10^10 engine comparisons.')

ESCAPADAS = {
    'gra12_acerto_comum': _par(
        'LACUNA MEDIDA. O corpo da noticia de acerto do jogador usa o modelo de outro motivo (o texto ganha uma frase). '
        + _CONF_PT,
        'MEASURED GAP. The body of the player\'s agreed-terms news uses another reason\'s template (the text gains a '
        'sentence). ' + _CONF_EN),
    'gra14_proposta_por_taxa': _par(
        'LACUNA MEDIDA. O corpo da noticia de proposta por taxa usa o modelo da bandeira 2. ' + _CONF_PT,
        'MEASURED GAP. The body of the offer-for-a-fee news uses the flag 2 template. ' + _CONF_EN),
    'gra16_vinculo_do_clube': _par(
        'LACUNA EXPLICADA. Na noticia de suspensao com jogo, o vinculo do clube aponta para o adversario. O comparador da '
        'noticia de suspensao ignora de proposito os vinculos 0x7d5 e 0x7d3 (a copia nao reproduz o dado deles), e nada '
        'da simulacao le os vinculos: so a tela da noticia, ao clicar no nome.',
        'EXPLAINED GAP. In the ban-with-a-match news, the club\'s link points to the opponent. The ban news comparer '
        'deliberately ignores the 0x7d5 and 0x7d3 links (the copy does not reproduce their data), and nothing in the '
        'simulation reads the links: only the news screen, when the name is clicked.'),
}  # nome: (explicacao pt, explicacao en)
