# Mutacoes (sbI_3), carreira ENG ate 2002-06-01: as selecoes ficam com o estadio do banco de dados; os jogos da
# Inglaterra (eliminatorias de setembro e outubro de 2001, amistosos) e de quem recebe selecoes passam a outro estadio
# (outra cidade, outro tempo e outro publico). A confirmar na prova: se escapar, o estadio de uma selecao nao entra em
# nada do que a carreira ENG le (ver PRONTO.txt).
MUTACOES = [
    ('sbI_3_selecoes_com_estadio',
     '            club.set_stadium(Stadium(0)); // sbI_3: a national team without a ground',
     '            let _ = club; // mutation',
     'sbI_3 new stadium object'),
]
