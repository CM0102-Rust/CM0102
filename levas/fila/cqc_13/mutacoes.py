# Mutacoes (cqc_13): as tres na 0x7f8c00 (5160 chamadas ate 2002-06-01; a 0x7b5be0 roda 50390 vezes, 10 por chamada:
# ha um tecnico humano com +0x5f = 1). A ordem dos observadores e sorteada ate achar um nao marcado (perto de 29
# sorteios por partida): com 9 observadores, sem o sorteio (a ordem fixa) ou aceitando o repetido, a contagem de
# sorteios da carreira muda na primeira partida (e os olheiros observam outra coisa). Escapariam so se nenhuma partida
# passasse pelo humano (o contador "cqc_13 olheiros do humano" mostra quantas).
MUTACOES = [
    ('cqc13_nove_olheiros', '        for _ in 0..10 { // cqc13: os 10 observadores',
     '        for _ in 0..9 { // mutacao: 9 observadores',
     'cqc_13 olheiros do humano'),
    ('cqc13_sem_sortear', '                let q = sortear(10) as i8; // cqc13: o observador sorteado',
     '                let q = (0..10usize).find(|&x| cqc13_d(marcas + x * 4) == 0).unwrap_or(0) as i8; // mutacao: sem sortear',
     'cqc_13 olheiros do humano'),
    ('cqc13_aceita_repetido', '                if cqc13_d(marcas.wrapping_add((q as isize * 4) as usize)) == 0 {',
     '                if true { // mutacao: aceita o repetido',
     'cqc_13 olheiros do humano'),
]
