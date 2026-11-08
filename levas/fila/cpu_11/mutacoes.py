# Mutacoes. As formacoes das taticas da IA passam todas pela 0x5b3560 ao carregar (495 chamadas ate 2002-06-01) e o
# comparador lado a lado confere a formacao inteira a cada chamada: a coordenada gravada com o x par (as coordenadas e
# as distancias mudam), as posicoes com o bit 2 sem a distancia dobrada (a escolha das posicoes muda) e os empates
# fora da lista da fase (a posicao geral e a extra mudam quando a atual era um empate). Os ramos ficam depois de todas
# as condicoes, colados no efeito.
MUTACOES = [
    ('cpu11_x_par', 'core::ptr::write_unaligned(cel as *mut u32, coord); // cpu11: a coordenada',
     'core::ptr::write_unaligned(cel as *mut u32, coord & !1); // mutacao: x par',
     'cpu_11 coordinate recorded'),
    ('cpu11_sem_dobro', 'dist = dist.wrapping_add(dist); // cpu11: o dobro',
     '// mutacao: sem o dobro',
     'cpu_11 distance doubled'),
    ('cpu11_sem_empate', 'if dist <= melhor[p] { // cpu11: o empate entra na lista',
     'if dist < melhor[p] { // mutacao: sem empates',
     'cpu_11 tie in the list'),
]
