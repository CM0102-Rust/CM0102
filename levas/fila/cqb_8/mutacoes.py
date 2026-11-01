# Mutacoes (cqb_8), todas na 0x755340 (9 chamadas ate 2002-06-01 na cobertura de 17/10, pelo boato de transferencia;
# 57 pessoas por chamada, cada uma com dois a cinco sorteios condicionais). Os sorteios a mais ou a menos mudam a
# contagem de sorteios da carreira; as desafeicoes (preferencias), as rejeicoes do clube e as marcas e queixas do
# contrato mudam o banco e as transferencias.
# - A chance inicial passa de 10 a 20: sorteio(chance) contra sorteio(nota) em cada pessoa, e o desafeto e os
#   sorteios do contrato que vem depois dele mudam.
# - A nota do jogador do elenco perde 5 (+0x5d + 5 em vez de + 10): mais pessoas passam a nao gostar do jogador.
# - O desafeto sai tambem no empate dos sorteios (>= em vez de >): mais desafeicoes e mais sorteios do contrato.
MUTACOES = [
    ('cqb8_chance_inicial_20', '    let mut chance: i32 = 0xa; // cqb8: a chance inicial',
     '    let mut chance: i32 = 0x14; // mutacao: a chance inicial', 'cqb_8 reacao do elenco'),
    ('cqb8_nota_do_jogador_menos_5', '            nota = cqb8_b(p + 0x5d).wrapping_add(10); // cqb8: a nota do jogador do elenco',
     '            nota = cqb8_b(p + 0x5d).wrapping_add(5); // mutacao: a nota do jogador do elenco', 'cqb_8 nota do jogador'),
    ('cqb8_desafeto_no_empate', '        if r1 > cqb8_sortear(nota as i32) { // cqb8: o desafeto sorteado',
     '        if r1 >= cqb8_sortear(nota as i32) { // mutacao: o desafeto sorteado', 'cqb_8 desafeto sorteado'),
]
