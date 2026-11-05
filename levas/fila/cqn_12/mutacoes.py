# Mutacoes (cqn_12), nos inscritos das copas do Japao e dos EUA (ligas jogaveis na carreira de 8 ligas): na Copa da Liga
# japonesa, a classe dos clubes da J1 sorteada de 0 a 18 em vez de 0 a 19 (o qsort e o sorteio por sementes mudam); na
# Copa dos EUA, os 10 da segunda liga de baixo antes dos 10 da primeira (a ordem da lista muda os confrontos); na Copa do
# Imperador, a classe dos clubes da J1 invertida (0x20 - posicao: o qsort das 16 ultimas inverte as sementes). As tres
# rodam no construtor e na virada da temporada do Japao e dos EUA (dentro da janela). Os contadores ficam colados no
# efeito.
MUTACOES = [
    ('cqn12_semente_da_j1_ate_18',
     'let s = cqn12_sortear(0x14); // cqn12: a classe sorteada dos clubes da J1',
     'let s = cqn12_sortear(0x13); // mutacao: de 0 a 18',
     'cqn_12 semente da j1'),
    ('cqn12_copa_dos_eua_segunda_liga_antes',
     'for &c in va.iter().take(10).chain(vb.iter().take(10)) {',
     'for &c in vb.iter().take(10).chain(va.iter().take(10)) { // mutacao: a segunda liga antes',
     'cqn_12 inscritos da copa dos eua'),
    ('cqn12_copa_do_imperador_classe_invertida',
     'cqn12_inscrever(this, n, c, *((c + 0x5f) as *const u8)); // cqn12: a classe dos clubes da J1 na Copa do Imperador',
     'cqn12_inscrever(this, n, c, 0x20u8.wrapping_sub(*((c + 0x5f) as *const u8))); // mutacao: invertida',
     'cqn_12 classe da j1 na copa do imperador'),
]
