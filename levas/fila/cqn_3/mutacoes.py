# Mutações (cqn_3):
# - a inscrição europeia da etapa 3 (2002) feita com a marca 1 (os inscritos ficam com 1 em vez de 2 e os que saem com
#   4 em vez de 0; a marca da pessoa é do banco, o hash diário pega) (contador "cqn_3 inscreve o clube");
# - os estádios de até 50 mil lugares fora da lista dos campos neutros nas nações com competições (as finais e
#   semifinais em campo neutro da carreira, como as semifinais da Copa da Inglaterra, vão para outros estádios: outro
#   público e outra renda) (contador "cqn_3 estadio pequeno");
# - a marca do estádio nacional trocada (a busca do campo neutro 0x85c800 a usa) (contador "cqn_3 entra na lista").
MUTACOES = [
    ('cqn3_inscreve_com_marca_1', 'inscrever(cqn3_d(0xdd7ef0) as usize, c, etapa, 0); // cqn3: inscreve o clube', 'inscrever(cqn3_d(0xdd7ef0) as usize, c, etapa, 1); // mutacao', 'cqn_3 inscreve o clube'),
    ('cqn3_estadios_ate_50_mil_fora', '(cqn3_d(e + 0x3c) as i32) < 0x1388 { // cqn3: menos de 5000 lugares', '(cqn3_d(e + 0x3c) as i32) < 0xc350 { // mutacao', 'cqn_3 estadio pequeno'),
    ('cqn3_nacional_trocado', '*((r + 8) as *mut u32) = (cqn3_d(nacao + 0x80) as usize == e) as u32; // cqn3: o estádio nacional', '*((r + 8) as *mut u32) = (cqn3_d(nacao + 0x80) as usize != e) as u32; // mutacao', 'cqn_3 entra na lista'),
]
