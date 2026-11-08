# Mutacoes das regras da nacao e do pow. O texto pela vaga 1 da noticia: so o modo Rust e a carreira (o comparando roda
# a original): PODE ESCAPAR (texto de noticia fica fora da impressao). O emprestimo sem conferir a nacao do dono: o
# comparando acusa quando o dono e de outra nacao e as regras dizem 0. O pow com o expoente dobrado: acusa na primeira
# chamada.
MUTACOES = [
    ('cpi_prazo_vaga', 'let regras = transfer_rules_at(this, vaga(noticia, 0) as usize); // as regras da vaga 0',
     'let regras = transfer_rules_at(this, vaga(noticia, 1) as usize); // mutacao: a vaga 1'),
    ('cpi_emprestimo_nacao', '.find(|&r| cpi4_d(r + 0xa) == nacao) // a nação das regras',
     '.find(|&r| cpi4_d(r + 0xa) != nacao) // mutacao: outra nacao'),
    ('cpi_pow', '    game_pow(base, expoente)\n}',
     '    game_pow(base, expoente * 2.0) // mutacao: o expoente dobrado\n}'),
]
