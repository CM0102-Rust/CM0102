# As escapadas da 333a leva (sx09_4: o genero do nome do periodo do dia). A confirmacao da cadeia no modo comparando
# vai ate 2001-08-20 (e a carga do save de ferias em portugues); a comparacao longa (ate 2002-06-01, bancada 4:
# levas/333/conf_escapadas_4.txt) mediu as duas.

_PT = (' O genero do periodo do dia so pesa no texto das noticias que citam a hora de um jogo, e so nas linguas em que ele '
       'muda a concordancia. O jogo de teste e em portugues, onde o genero do periodo do dia nao muda o texto. Medido: comparacao ate 2002-06-01 (levas/333/conf_escapadas_4.txt) com 0 divergencias em 10^10 comparacoes do motor.')
_EN = (' The gender of the time of day only matters in the text of the news items that name a match time, and only in the '
       'languages where it changes the agreement. The test game is in Portuguese, where the gender of the time of day does not change the text. Measured: comparison to 2002-06-01 (levas/333/conf_escapadas_4.txt) with 0 divergences in 10^10 engine comparisons.')

ESCAPADAS = {
    'sx09_sem_genero': (
        'LACUNA MEDIDA. Toda lingua ganha um genero para o periodo do dia (o original so o da a lingua 3).' + _PT,
        'MEASURED GAP. Every language gets a gender for the time of day (the original only gives it to language 3).' + _EN),
    'sx09_lingua_3': (
        'LACUNA MEDIDA. O teste da lingua 3 fica invertido: todas as linguas menos a 3 ganham o genero.' + _PT,
        'MEASURED GAP. The language 3 test is inverted: every language but 3 gets the gender.' + _EN),
}  # nome: (explicacao pt, explicacao en)
