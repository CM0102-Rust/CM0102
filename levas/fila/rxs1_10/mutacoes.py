# Mutação (rxs1_10): o último byte do registro sem copiar. Acusada pelo comparador (lado a lado dos bytes) na primeira
# cópia; a carreira de teste só a roda se um clube visto mudar os jogadores virtuais de bloco (explicado no PRONTO.txt).
MUTACOES = [
    ('rxs1_10_registro',
     'core::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, staff::SIZE); // rxs1_10: the whole record',
     'core::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, staff::SIZE - 1); // mutation',
     'rxs1_10 staff record copied'),
]
