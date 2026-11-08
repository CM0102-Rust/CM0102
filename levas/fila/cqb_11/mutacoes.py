# Mutacoes (cqb_11): duas na 0x51d2d0 (5 chamadas ate 2002-06-01) e uma na 0x51d9f0 (3 chamadas), pelo sorteio da
# rodada 0x51c410. Os confrontos sorteados decidem quem joga com quem e o mando: sem ordenar pela semente, os cabecas de
# chave caem em outros confrontos; com o lado sorteado sempre visitante, o mando muda; sem trocar o nivel pela conta da
# nacao, a ordem (e portanto o sorteio) muda. Podem escapar se a rodada sorteada so tiver times de uma semente igual
# (a ordem do qsort com chaves iguais pode coincidir), se nenhuma entrada achar as duas vagas livres na primeira volta
# (os confrontos ja tem time de um lado) ou se nenhum time da rodada separando as nacoes tiver nacao com mais de um
# time (os contadores mostram quantas vezes cada trecho rodou).
MUTACOES = [
    ('cqb11_sem_ordenar', '    ordenar(vetor, n as i32, 7, 0x51ebc0); // cqb11: pela semente, a maior primeiro',
     '    ordenar(vetor, 0, 7, 0x51ebc0); // mutacao: sem ordenar',
     'cqb_11 sorts by seed'),
    ('cqb11_lado_visitante', '                        lado = Some(true); // cqb11: o lado sorteado',
     '                        lado = Some(false); // mutacao: sempre visitante',
     'cqb_11 seed drawn side'),
    ('cqb11_sem_conta_da_nacao', '                    *((e + 6) as *mut u8) = cqb11_b(read_ptr(nacao).wrapping_add(contas)); // cqb11: o nível vira a conta da nação',
     '                    *((e + 6) as *mut u8) = cqb11_b(e + 6); // mutacao: fica a semente',
     'cqb_11 nation count'),
]
