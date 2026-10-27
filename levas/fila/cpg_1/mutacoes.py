# Mutacoes das copias da noticia e das transferencias. A da noticia deixa a ultima ficha sem copiar: so muda algo nas
# noticias com 50 fichas, e as noticias ficam fora da impressao da carreira: PODE ESCAPAR (o comparando acusa). A da
# transferencia passada perde a ultima dupla (+0x18) e a da futura o indice da oferta: devem acusar se a copia decidir
# algo da carreira (a futura leva a oferta ao dia da transferencia); senao o comparando acusa.
MUTACOES = [
    ('cpg_noticia_fichas', 'for k in 0..50 {\n        d(5 + 4 * k); // as fichas da notícia',
     'for k in 0..49 {\n        d(5 + 4 * k); // mutacao: sem a ultima ficha'),
    ('cpg_passada_duplas', 'for k in 0..7 {\n        core::ptr::write_unaligned((this + 4 * k) as *mut u32, core::ptr::read_unaligned((origem + 4 * k) as *const u32)); // as duplas da transferência',
     'for k in 0..6 {\n        core::ptr::write_unaligned((this + 4 * k) as *mut u32, core::ptr::read_unaligned((origem + 4 * k) as *const u32)); // mutacao: sem a ultima'),
    ('cpg_futura_oferta', 'core::ptr::write_unaligned((this + 8) as *mut u32, core::ptr::read_unaligned((origem + 8) as *const u32)); // o índice da oferta',
     'core::ptr::write_unaligned((this + 8) as *mut u32, 0); // mutacao: a primeira oferta'),
]
