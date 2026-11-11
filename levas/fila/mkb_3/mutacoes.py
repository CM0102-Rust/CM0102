# Mutacoes (mkb_3), na escolha do alvo de uma vaga da lista de compras da IA (0x83f3a0; chamada pela
# allocate_transfer_budget 0x83b6e0 a cada revisao da lista de um clube da IA, 30 + random(70) dias por clube, e pela
# update_staff_status 0x843e10 quando a vaga perde o alvo): todas mudam QUEM entra na
# lista de compras, entao quem recebe lance e quem assina:
# - a vontade minima do alvo sobe de 3,15 para 4,15: alvos que entravam ficam de fora (contador "mkb_3 new best");
# - o divisor do conhecimento cai a um quarto (k / 40 em vez de k / 10): o clube conhece mais jogadores, entram
#   outros (contador "mkb_3 known by the divisor");
# - o estrangeiro sem convocacao precisa de reputacao vista 1250 + sorteio(2000) em vez de 3250: mais estrangeiros
#   (contador "mkb_3 foreigner seen");
# - o salario desejado so cabe ate 0,6 x o teto (antes 1,2 x): alvos caros ficam de fora (contador "mkb_3 wage too
#   high").
MUTACOES = [
    ('mkb3_vontade_maior',
     'if interest >= read_f64(rdata::F64_3_15) { // mkb3: the willingness a target needs',
     'if interest >= read_f64(rdata::F64_3_15) + 1.0 { // mutation: willingness 4.15',
     'mkb_3 new best'),
    ('mkb3_conhece_mais',
     '    k /= 10; // mkb3: the knowledge divisor',
     '    k /= 40; // mutation: a quarter of the divisor',
     'mkb_3 known by the divisor'),
    ('mkb3_estrangeiro_facil',
     'return seen >= exe::utils_random(2000) + 3250; // mkb3: the reputation a foreigner needs',
     'return seen >= exe::utils_random(2000) + 1250; // mutation: 1250',
     'mkb_3 foreigner seen'),
    ('mkb3_salario_curto',
     'if (cap as f64 * read_f64(rdata::F64_1_2)) < wanted_wage { // mkb3: the wage within 1.2 × the cap',
     'if (cap as f64 * read_f64(rdata::F64_1_2) * 0.5) < wanted_wage { // mutation: 0.6 × the cap',
     'mkb_3 wage too high'),
]
