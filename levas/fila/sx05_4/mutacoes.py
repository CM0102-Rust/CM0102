# Mutacoes (sx05_4). Os coletores escolhem os premiados do ano pelas notas (fim da temporada, ate 2002-08-01): acusadas
# pelos comparadores lado a lado (a saida e a conta) e, mudando o premiado, pela reputacao dele na carreira.
# - as passadas descendo de 10 em 10 partidas em vez de 5;
# - a idade maxima exclusiva (quem tem a idade maxima fica de fora);
# - o candidato da lista excluida aceito.
MUTACOES = [
    ('sx05_4_passadas_de_dez',
     '        min_matches -= 5;\n',
     '        min_matches -= 10; // mutation: steps of 10\n',
     'sx05_4 fewer matches pass'),
    ('sx05_4_idade_exclusiva',
     'e.person().age() as i8 > max_age as i8',
     'e.person().age() as i8 >= max_age as i8 /* mutation: the maximum age left out */',
     'sx05_4 too old'),
    ('sx05_4_excluido_aceito',
     '                    covered!("sx05_4 excluded candidate");\n                    return false;',
     '                    covered!("sx05_4 excluded candidate");\n                    return true; // mutation: excluded accepted',
     'sx05_4 excluded candidate'),
]
