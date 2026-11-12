# Mutacoes (mke_4), na mudanca de clube de uma pessoa (0x8c9720; 750 chamadas ate 2002-06-01: toda transferencia
# concluida, compra, emprestimo e troca):
# - a inquietacao depois da mudanca (o +0xa do registro de 79 bytes) usa 2,5 em vez de 1,5: os recem-chegados ficam mais
#   inquietos (moral, rendimento, vontade de sair) (contador "mke_4 moved");
# - o piso de reputacao pela nota da lista de compras usa 2/9 x 4: os contratados pela lista sobem mais de reputacao
#   (valor, salarios, interesse dos outros clubes).
MUTACOES = [
    ('mke4_inquieto',
     'let mut add = ftol((read_f64(F64_1_5) - fl as f64 * read_f64(rdata::F64_0_1)) * u as f64) as u8; // mke4: the unsettlement after a move',
     'let mut add = ftol((read_f64(F64_1_5) + 1.0 - fl as f64 * read_f64(rdata::F64_0_1)) * u as f64) as u8; // mutation: 2.5',
     'mke_4 moved'),
    ('mke4_piso_da_lista',
     'let z = ftol((q as f64 + r as f64 * read_f64(F64_2_9)) * read_f64(F64_1_150)); // mke4: the shortlist rating lifts the floor',
     'let z = ftol((q as f64 + r as f64 * read_f64(F64_2_9) * 4.0) * read_f64(F64_1_150)); // mutation: 4 x the rating',
     'mke_4 moved'),
]
