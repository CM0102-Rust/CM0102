# Mutacoes dos vetores ordenados. A dos interesses nao reordena depois de tirar (o bsearch seguinte erra): deve acusar
# no comparando na primeira remocao com dois ou mais; na carreira, se a midia decidir pelos interesses. A do pedido nao
# copia a data dos seguintes e a do apelido deixa o ultimo com id 0 em vez de -1: o comparando acusa na primeira
# remocao com seguintes; a carreira PODE NAO ACUSAR (pedidos de observacao e apelidos quase nao decidem nada).
MUTACOES = [
    ('cph_interesse_ordem', 'if *n != 0 {\n        media_crt_qsort(cph4_d(this + 0xc) as usize, *n, 0x19, 0x757ea0);',
     'if *n < 0 {\n        media_crt_qsort(cph4_d(this + 0xc) as usize, *n, 0x19, 0x757ea0); // mutacao: sem reordenar'),
    ('cph_pedido_data', 'copiar_data(r + 8, &mut local, r + 0x19);',
     'copiar_data(r + 8, &mut local, r + 8); // mutacao: a data fica'),
    ('cph_apelido_fim', 'core::ptr::write_unaligned((fim - 0x1e) as *mut u32, u32::MAX);',
     'core::ptr::write_unaligned((fim - 0x1e) as *mut u32, 0); // mutacao: id 0'),
]
