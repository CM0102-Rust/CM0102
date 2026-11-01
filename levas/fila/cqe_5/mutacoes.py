# Mutacoes (cqe_5), nas regras inglesas de emprestimo (0x579d60 pela 0x8d0040; 0x579fd0 pelos emprestimos concluidos):
# - "dois ao mesmo tempo" vira "um ao mesmo tempo": todo clube com um jogador emprestado de outro passa a ser recusado
#   (512+ recusas ate 2001-09-01 so com dois): menos emprestimos na carreira;
# - a recusa da mesma divisao vale para qualquer divisao inglesa (sem conferir a [0x9cf5bc]): os pedidos entre clubes
#   da mesma divisao abaixo da primeira passam a ser recusados (64+ ate 2001-09-01 so na primeira);
# - a conta da temporada soma 3 por emprestimo: com dois emprestimos o clube ja passa de cinco e e recusado (16+
#   emprestimos contados ate 2001-09-01).
MUTACOES = [
    ('cqe5_um_ao_mesmo_tempo',
     'if n > 1 { // cqe5: dois ao mesmo tempo',
     'if n > 0 { // mutacao: um ao mesmo tempo',
     'cqe_5 dois ao mesmo tempo'),
    ('cqe5_mesma_divisao_qualquer',
     'if comp == cqe5_d(dono + 0x57) && comp != 0 && cqe5_d(comp) == cqe5_d(0x9cf5bc) {',
     'if comp == cqe5_d(dono + 0x57) && comp != 0 { // mutacao: qualquer divisao',
     'cqe_5 mesma divisao'),
    ('cqe5_conta_tres',
     '*(p as *mut u8) = cqe5_b(p).wrapping_add(1); // cqe5: a conta da temporada',
     '*(p as *mut u8) = cqe5_b(p).wrapping_add(3); // mutacao: soma 3',
     'cqe_5 emprestimo contado'),
]
