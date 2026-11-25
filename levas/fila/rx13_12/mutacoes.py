# Mutacoes (rx13_12), pela carreira ENG SCO WAL ate 2002-08-01 (a Liga de Gales joga a temporada 2001/02 com esta
# tabela; o fundo de premios paga os clubes no fim):
# - a tabela com a ordem dos clubes girada uma linha (outros clubes em cada linha: outra tabela de jogos);
# - sem fundo de premios proprio.
MUTACOES = [
    ('rx13_12_ordem_girada',
     'Some(10000), None, 0, "rx13_12 league of wales table") // rx13_12: 18 clubs',
     'Some(10000), None, 1, "rx13_12 league of wales table") // mutation: rotated',
     'rx13_12 league of wales table'),
    ('rx13_12_sem_fundo',
     'KEY_CLUB_COMP_WELSH_PREMIER_DIVISION, Some(10000), None, 0,',
     'KEY_CLUB_COMP_WELSH_PREMIER_DIVISION, None, None, 0, // mutation: no prize fund\n       ',
     'rx13_12 league of wales table'),
]
