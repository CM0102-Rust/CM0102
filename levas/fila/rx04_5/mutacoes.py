# Mutacoes (rx04_5). So rodam com o jogo em noruegues ou alemao (o jogo de teste esta em portugues): acusada pelo
# comparador lado a lado numa sessao nessas linguas, nao pela cadeia normal.
# - a forma pedida trocada pela seguinte (1 -> 2, 2 -> 3, 3 -> 1).
MUTACOES = [
    ('rx045_forma_seguinte',
     '                copy_c_string(text, read_usize(row + 4 * form as usize));',
     '                copy_c_string(text, read_usize(row + 4 * (form as usize % 3 + 1))); // mutation',
     'rx04_5 norwegian player type'),
]
