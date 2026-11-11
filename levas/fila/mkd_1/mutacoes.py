# Mutacoes (mkd_1). A rotina roda na satisfacao com o contrato (0x866620, passe diario das pessoas com random(50) = 0)
# e na decisao sobre as ofertas de contrato (0x4ce4e0); a satisfacao (1 a 3) muda a moral e os pedidos de
# transferencia, e a decisao muda quem assina e por quanto:
# - o atacante sem os bonus (o valor do contrato cai: outra satisfacao, outro pedido de saida);
# - sem a luva espalhada pelos dias (+8): outro valor para todos os contratos com luva;
# - o fator de duracao com um ano a mais no expoente (contratos de igual termino deixam de valer o mesmo).
MUTACOES = [
    ('mkd1_atacante_sem_bonus',
     'st = bonus(st, rdata::F64_0_8, rdata::F64_0_5); // mkd1: the attacker\'s bonuses',
     'st = st; // mutation: the attacker without the bonuses',
     'mkd_1 contract value'),
    ('mkd1_sem_luva',
     'st = st + spread as f64 / (days as i32 as f64 * read_f32(F32_0_2222222239) as f64); // mkd1: the spread fee',
     'st = st + 0.0 * (spread as f64); // mutation: without the spread fee',
     'mkd_1 contract value'),
    ('mkd1_duracao_mais_um_ano',
     'let mut q = game_pow(f as f64, (years as f64).abs()); // mkd1: the length factor',
     'let mut q = game_pow(f as f64, (years as f64).abs() + 1.0); // mutation: one more year in the length factor',
     'mkd_1 contract value'),
]
