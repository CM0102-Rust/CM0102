# Mutacoes (cx08_12), com a Argentina ate 2002-08-01:
# - o mais forte e previsto em 2o (o comparador das previstas acusa na criacao; as expectativas mudam);
# - nas zonas os codigos multiplicam por 2 (o comparador do progresso acusa).
MUTACOES = [
    ('cx08_12_previsto_segundo',
     'const ARG_SECOND_PREDICTED: [u32; 25] = [1, 2, 2, 2, 4,',
     'const ARG_SECOND_PREDICTED: [u32; 25] = [2 /* mutation */, 2, 2, 2, 4,',
     'cx08_12 argentine second predicted positions'),
    ('cx08_12_zonas_vezes_dois',
     '(c, b, w) = (c.wrapping_mul(3), b.wrapping_mul(3), w.wrapping_mul(3));',
     '(c, b, w) = (c.wrapping_mul(2), b.wrapping_mul(3), w.wrapping_mul(3)); // mutation',
     'cx08_12 argentine second phase progress'),
]
