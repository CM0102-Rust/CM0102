# Mutacoes do contrato de pessoal. A dos dias le a data de inicio (+0x25) no lugar do fim: deve acusar (renovacoes e
# saidas pelo fim do contrato). A das queixas zera so o byte baixo de +0x45 (as queixas 0x100 e 0x10000 ficam): deve
# acusar se alguma pessoa tiver essas queixas; senao o comparando acusa. As da retirada mudam a moral (+4 em vez de +5)
# e a felicidade minima (14): devem acusar (moral e felicidade decidem pedidos e renovacoes).
MUTACOES = [
    ('cph_dias_fim', 'dias(this + 0x2d, cph1_d(0xae2c90), cph1_d(0xae2c94)) // a data do fim do contrato',
     'dias(this + 0x25, cph1_d(0xae2c90), cph1_d(0xae2c94)) // mutacao: a data de inicio'),
    ('cph_queixas_zero', 'core::ptr::write_unaligned((this + 0x45) as *mut u32, 0); // as queixas',
     '*((this + 0x45) as *mut u8) = 0; // mutacao: so o byte baixo'),
    ('cph_retirada_moral', '*((this + 0x44) as *mut u8) = cph1_b(pessoa + 0x5b).wrapping_add(cph1_b(pessoa + 0x59)).wrapping_add(5);',
     '*((this + 0x44) as *mut u8) = cph1_b(pessoa + 0x5b).wrapping_add(cph1_b(pessoa + 0x59)).wrapping_add(4); // mutacao'),
    ('cph_retirada_felicidade', '*((this + 0x3d) as *mut u8) = 15; // a felicidade mínima',
     '*((this + 0x3d) as *mut u8) = 14; // mutacao: 14'),
]
