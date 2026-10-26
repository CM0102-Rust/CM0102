# Mutacoes da inicializacao dos jogadores (0x535a80). A prova e o .qst da prova do save: a criacao roda uma vez antes
# da gravacao e o banco inicializado (pessoas, jogadores, nao jogadores) vai para o save. As cinco mudam muitos
# registros: as habilidades e reputacoes de todos (x49), os vetores da segunda volta (/3), o sorteio do potencial
# (2000 -> 1999 em ~140 mil jogadores) e os 250 sorteados do molde (125: muda a sequencia de sorteios dali em diante).
MUTACOES = [
    ('dbv1_habilidade_x50', '            ww(j + 0xb, e20.wrapping_mul(50));', '            ww(j + 0xb, e20.wrapping_mul(49)); // mutacao'),
    ('dbv1_reputacao_x50', '            let a = w(nj + 0xc).wrapping_mul(50);', '            let a = w(nj + 0xc).wrapping_mul(49); // mutacao'),
    ('dbv1_metade', '                    por_dbv1(&mut v3, c, (x as i32 / 2) as i16);', '                    por_dbv1(&mut v3, c, (x as i32 / 3) as i16); // mutacao'),
    ('dbv1_sorteio_potencial', '            if sorteio(2000) == 0 {', '            if sorteio(1999) == 0 { // mutacao'),
    ('dbv1_molde_250', '                    cont -= 1; // os 250 sorteados', '                    cont -= 2; // mutacao: 125 sorteados'),
]
