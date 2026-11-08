# Mutacoes da forma do adjetivo, do atributo de treinador da pessoa e da nota com a habilidade reduzida. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq13_adjetivo', "(indice as u8 as i8 as i32).wrapping_sub(1) as u32;", "(indice as u8 as i8 as i32).wrapping_sub(2) as u32;"),
    ('dq13_atributo', "    calcular(pessoa, nao_jogador, atributo)\n", "    calcular(pessoa, nao_jogador, atributo) ^ 1\n"),
    ('dq13_reduzida', "habilidade * read_f64_at(0x967880) + (v + v)", "habilidade * read_f64_at(0x967880) * 2.0 + (v + v)"),
]
