# As escapadas da 330a leva. A confirmacao da cadeia no modo comparando vai ate 2001-08-20 (e a carga do save de
# ferias em portugues); a comparacao longa (ate 2002-06-01, bancada 4: levas/330/conf_escapadas_4.txt) decide, e o
# resultado entra num commit seguinte, como na 324a, 328a e 329a.

_PT = ' A comparacao longa decide.'
_EN = ' The long comparison decides.'

ESCAPADAS = {
    'sx01_em_casa': (
        'EM CONFIRMACAO. A historia da midia do time nunca e "em casa": so muda o texto da historia, que so e escrito '
        'quando ela chega ao clube humano, e o modelo de casa so pesa nas historias de resultado.' + _PT,
        'BEING CONFIRMED. The team media story is never "at home": it only changes the story text, written only when it '
        'reaches the human club, and the home template only matters in the result stories.' + _EN),
    'sx04_2_jogador_dobro_dos_jogos': (
        'EM CONFIRMACAO. A equipe da temporada exige o dobro dos jogos como titular: a equipe sai no fim da temporada, '
        'depois da confirmacao curta.' + _PT,
        'BEING CONFIRMED. The team of the season requires twice the starts: the team comes out at the season\'s end, '
        'after the short confirmation.' + _EN),
    'sx02b_3_revenda_ao_contrario': (
        'EM CONFIRMACAO. A ordem dos percentuais de revenda por pessoa invertida: o comparador da ordem acusa, mas so '
        'roda com dois ou mais percentuais de revenda para ordenar, raros no comeco da carreira.' + _PT,
        'BEING CONFIRMED. The order of the sell-on percentages by person inverted: the order comparer catches it, but it '
        'only runs with two or more sell-on percentages to sort, rare early in the career.' + _EN),
    'sx01c_confianca_da_comissao': (
        'EM CONFIRMACAO. A confianca da comissao tecnica muda no sentido contrario: so muda a linha da diretoria, que a '
        'carreira le nas decisoes de demissao e de contratacao de comissao, mais adiante na temporada.' + _PT,
        'BEING CONFIRMED. The staff confidence changes the other way: it only changes the board row, which the career '
        'reads in the sacking and staff hiring decisions, later in the season.' + _EN),
}  # nome: (explicacao pt, explicacao en)
