# Mutacoes (sx21b_1). Rodam nas ofertas da carreira ENG; caem pelos comparadores lado a lado (e pela carreira quando
# mudam a avaliacao de uma oferta).
MUTACOES = [
    ('sx21b_1_teto_0_8',
     'if chance > read_f64(rdata::F64_0_75) { // sx21b_1: the chance is capped at 0.75',
     'if chance > read_f64(rdata::F64_0_8) { // mutation: capped at 0.8',
     'sx21b_1 international clause weighed'),
    ('sx21b_1_vendedor_pobre',
     '(offer_factor, clause_factor) = (0.1, 0.5); // sx21b_1: the poorest seller',
     '(offer_factor, clause_factor) = (0.2, 0.5); // mutation: 0.2',
     'sx21b_1 the poorest seller'),
    ('sx21b_1_venda_25',
     'o.set_sell_on_percent(0x14); // sx21b_1: a 20% sell-on share',
     'o.set_sell_on_percent(0x19); // mutation: 25%',
     'sx21b_1 sell-on share instead of fee'),
]
