# Mutacoes (cpy_12). A noticia do ultimato (0x6ac700, 4 chamadas na cobertura de 17/10; "cpq_1 ultimatum given" 2 na
# base) sorteia sempre (sorteio(5) no campo 7): sem esse sorteio a fila anda uma casa a menos e todos os sorteios
# seguintes mudam. No veto da diretoria (0x696b10, 1098 chamadas, pelas ofertas sem troca), comprando para um clube
# humano (o de ferias conta: 0x600a50 com 1), a verba acima de 1,2 x veta (0x800, a oferta cai); sem esse veto a oferta
# segue. Os fisioterapeutas (comprando jogador com lesao > 20) sorteiam varias vezes: com a gravidade contra 200 fixo o
# ramo da gravidade nao sorteia mais e a fila muda (escapa se o ramo nao rodar: o contador diz).
MUTACOES = [
    ('cpy12_ultimato_sem_sorteio', '    gravar(nt, 7, sorteio(5) as u32); // cpy12: o sorteio do ultimato',
     '    gravar(nt, 7, 0); // mutacao: sem o sorteio do ultimato',
     'cpy_12 ultimatum news'),
    ('cpy12_verba_sem_veto', '    if valor as i64 > orcamento.wrapping_mul(12) / 10 { // cpy12: a verba da diretoria',
     '    if false && valor as i64 > orcamento.wrapping_mul(12) / 10 { // mutacao: a verba não veta',
     'cpy_12 board without funds'),
    ('cpy12_fisio_sem_sorteio', '                let r = sorteio(200); // cpy12: o sorteio da gravidade',
     '                let r: i32 = 200; // mutacao: sem o sorteio da gravidade',
     'cpy_12 physio evaluates'),
]
