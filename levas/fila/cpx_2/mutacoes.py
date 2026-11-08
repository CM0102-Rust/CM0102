# Mutacoes (cpx_2), todas no desassossego pelo lance (0x8c1290, 44 chamadas ate 2002-06-01 na cobertura de 17/10,
# pela recusa, pela retirada no motivo 2 e pelo clube que recusa 0x8c36b0). O limit do sorteio em 2000 (e nao 4250)
# marca +0x2d (jogador desassossegado) em quase toda oferta de clube de injury_word_0c media; a marca muda a negociacao das
# ofertas e leva ao resto da conta, que sorteia de novo. O interesse minimo em 4,5 (e nao 5,5) leva mais jogadores ao
# sorteio(700) e a queixa. Sem o desconto do jogador listado (o sorteio(6000)), a fila do sorteio anda uma casa a menos
# e a nota muda. As tres mudam sorteios e o estado das ofertas.
MUTACOES = [
    ('cpx2_sorteio_2000', '    if (cpx2_w(comprador + 0x80) as i32) > r.wrapping_add(0x109a) { // cpx2: o sorteio do desassossego',
     '    if (cpx2_w(comprador + 0x80) as i32) > r.wrapping_add(2000) { // mutacao: o limit em 2000',
     'cpx_2 sorteio do desassossego'),
    ('cpx2_interesse_4_5', '    if !(x >= read_f64_at(0x96d298)) { // cpx2: o interesse mínimo',
     '    if !(x >= 4.5) { // mutacao: o interesse minimo em 4,5',
     'cpx_2 interesse avaliado'),
    ('cpx2_sem_desconto_do_listado', '        n = n.wrapping_add((-2000i32).wrapping_sub(random(0x1770)) as i16); // cpx2: o desconto do listado',
     '        // mutacao: sem o desconto do listado',
     'cpx_2 listado'),
]
