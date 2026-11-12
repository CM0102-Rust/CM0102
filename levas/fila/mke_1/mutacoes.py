# Mutacoes (mke_1), em quem entra na lista de compras de um clube da IA quando um jogador lhe e oferecido (0x842e10;
# 12,6 milhoes de chamadas ate 2002-06-01): todas mudam QUEM entra na lista, entao quem recebe lance e quem assina:
# - o interesse minimo sobe de 3,15 para 4,15: jogadores que entravam ficam de fora (contador "mke_1 not interested");
# - o teto elevado vai ate 0,4 x a verba em vez de 0,8: alvos caros deixam de caber (contador "mke_1 ceiling raised");
# - o salario desejado so cabe ate 0,6 x o teto (antes 1,2 x) (contador "mke_1 wage too high").
MUTACOES = [
    ('mke1_interesse_maior',
     'if exe::shortlist_manager_staff_interest_in_joining_club(this, player, club, 1, 0, st, 0) < read_f64(rdata::F64_3_15) { // mke1: the interest a player needs',
     'if exe::shortlist_manager_staff_interest_in_joining_club(this, player, club, 1, 0, st, 0) < read_f64(rdata::F64_3_15) + 1.0 { // mutation: interest 4.15',
     'mke_1 not interested'),
    ('mke1_teto_metade',
     'let cap = b * read_f64(rdata::F64_0_8); // mke1: the raised ceiling within 0.8 × the budget',
     'let cap = b * read_f64(rdata::F64_0_8) * 0.5; // mutation: 0.4 × the budget',
     'mke_1 ceiling raised'),
    ('mke1_salario_curto',
     'if (cap as f64 * read_f64(rdata::F64_1_2)) < wanted_wage { // mke1: the wage within 1.2 × the cap',
     'if (cap as f64 * read_f64(rdata::F64_1_2) * 0.5) < wanted_wage { // mutation: 0.6 × the cap',
     'mke_1 wage too high'),
]
