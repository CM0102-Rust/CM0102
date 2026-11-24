# Mutacoes (sbA_4): os pontos do ranking mudam a posicao das selecoes (e a faixa de forca dos jogos seguintes); o
# comparador do resultado (o plano contra a entrada que a original acrescenta) as acusa na comparacao mesmo que a
# carreira ENG ate 2002-06-01 nao mude.
MUTACOES = [
    ('sbA_4_gol_vale_mais',
     '    let goals = |g: i8, base: f32| (g.min(5) as f64 * read_f64(rdata::F64_0_2) * base as f64 * read_f64(rdata::F64_0_3) + base as f64) as f32;',
     '    let goals = |g: i8, base: f32| (g.min(5) as f64 * read_f64(rdata::F64_0_2) * base as f64 + base as f64) as f32; // mutation'),
    ('sbA_4_amistoso_pesado',
     '        read_f64(rdata::F64_1)\n    } else {\n        return Err(());',
     '        read_f64(rdata::F64_1_5) // mutation\n    } else {\n        return Err(());'),
    ('sbA_4_empate_igual',
     '        (0, core::cmp::Ordering::Equal) => (1.0, 1.0),',
     '        (0, core::cmp::Ordering::Equal) => (0.0, 2.0), // mutation'),
]
