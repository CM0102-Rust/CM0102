# Mutacoes (mka_1), cada uma com o ramo:
# - o presidente: m = Negocios + 1 no lugar de + 6 (a reserva da diretoria cai: verbas maiores, outros lances e outras
#   contratacoes da IA);
# - o teto suave: acima de Z fica a metade do excesso, nao 10% (verbas dos clubes ricos maiores: outros lances);
# - o preco pedido pelo jogador do clube humano: o piso (o valor guardado) dobra (os lances da IA pelos jogadores do
#   clube do treinador de ferias mudam ou somem).
MUTACOES = [
    ('mka1_presidente',
     'let mut v = (chairman.non_player().business() as i8).wrapping_add(6);',
     'let mut v = (chairman.non_player().business() as i8).wrapping_add(1); // mutation: Business + 1',
     'mka_1 board reserve'),
    ('mka1_teto_suave',
     'budget = ftol_i64(budget.wrapping_sub(cap as i64) as f64 * read_f64(rdata::F64_0_1) + cap as f64);',
     'budget = ftol_i64(budget.wrapping_sub(cap as i64) as f64 * read_f64(rdata::F64_0_5) + cap as f64); // mutation: half the excess',
     'mka_1 soft cap'),
    ('mka1_piso_humano',
     'let mut stored = p.value() as i32;',
     'let mut stored = (p.value() as i32).wrapping_mul(2); // mutation: twice the stored value',
     'mka_1 human asking price'),
]
