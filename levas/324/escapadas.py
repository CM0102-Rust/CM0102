# As escapadas da 324a leva (textos de noticias, itens gra; ENG ate 2002-06-01).
# A confirmacao da cadeia no modo comparando vai so ate 2001-08-20; as que nao foram acusadas la seguem para a
# comparacao longa (ate 2002-06-01, bancadas 4 e 5: levas/324/conf_escapadas_N.txt), e o resultado entra num commit
# seguinte, como na 320a.

def _par(pt, en):
    return (pt, en)


_CONF_PT = ('Nada da simulacao le o texto das noticias; o comparador de texto acusa, mas na comparacao ate 2001-08-20 o '
            'ramo nao rodou no modo comparando (a carreira comparando sorteia diferente da Rust). A comparacao longa decide.')
_CONF_EN = ('Nothing in the simulation reads the news text; the text comparer catches it, but in the comparison to '
            '2001-08-20 the branch did not run in comparison mode (the comparing career draws differently from the Rust '
            'one). The long comparison decides.')

ESCAPADAS = {
    'gra12_acerto_comum': _par(
        'EM CONFIRMACAO. O corpo da noticia de acerto do jogador usa o modelo de outro motivo (o texto ganha uma frase). '
        + _CONF_PT,
        'BEING CONFIRMED. The body of the player\'s agreed-terms news uses another reason\'s template (the text gains a '
        'sentence). ' + _CONF_EN),
    'gra14_proposta_por_taxa': _par(
        'EM CONFIRMACAO. O corpo da noticia de proposta por taxa usa o modelo da bandeira 2. ' + _CONF_PT,
        'BEING CONFIRMED. The body of the offer-for-a-fee news uses the flag 2 template. ' + _CONF_EN),
    'gra1_sorteio_vinculo': _par(
        'EM CONFIRMACAO. No corpo da noticia de sorteio de copa, o vinculo do clube sorteado aponta para o adversario. '
        'Os sorteios de copa da carreira comecam depois de agosto. ' + _CONF_PT,
        'BEING CONFIRMED. In the cup draw news body, the drawn club\'s link points to the opponent. The career\'s cup '
        'draws start after August. ' + _CONF_EN),
    'gra16_vinculo_do_clube': _par(
        'LACUNA EXPLICADA. Na noticia de suspensao com jogo, o vinculo do clube aponta para o adversario. O comparador da '
        'noticia de suspensao ignora de proposito os vinculos 0x7d5 e 0x7d3 (a copia nao reproduz o dado deles), e nada '
        'da simulacao le os vinculos: so a tela da noticia, ao clicar no nome.',
        'EXPLAINED GAP. In the ban-with-a-match news, the club\'s link points to the opponent. The ban news comparer '
        'deliberately ignores the 0x7d5 and 0x7d3 links (the copy does not reproduce their data), and nothing in the '
        'simulation reads the links: only the news screen, when the name is clicked.'),
    'gra18_vinculo_do_clube': _par(
        'EM CONFIRMACAO. Na noticia de recorde do clube, o vinculo do clube ganha outro tipo. As noticias de recorde saem '
        'no fim de cada competicao. ' + _CONF_PT,
        'BEING CONFIRMED. In the club record news, the club\'s link gets another type. Record news comes out at each '
        'competition\'s end. ' + _CONF_EN),
}  # nome: (explicacao pt, explicacao en)
