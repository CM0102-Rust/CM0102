# Mutacoes dos auxiliares de competicao. A do tipo de confronto troca a briga pelo titulo (1) pela da zona (3): a
# escalacao automatica de cada partida le o tipo; a carreira ate 2002-06-01 deve acusar. A da forca do clube pesa a
# injury_word_0c por 8 em vez de 16: so muda a ordem por forca (0x4c08f0, 17 chamadas) se dois clubes trocarem de lugar:
# PODE ESCAPAR. A do colocado mais recente troca campeao e vice: muda as vagas de copa quando a competicao pede o
# campeao de outra; PODE ESCAPAR se nenhuma vaga dessas for decidida ate junho (o comparando acusa na primeira chamada).
MUTACOES = [
    ('cpa_confronto_titulo', '            return 1; // briga pelo título',
     '            return 3; // mutacao: a zona no lugar do titulo'),
    ('cpa_forca_reputacao', '    forca = forca.wrapping_add((core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) << 4); // a reputação',
     '    forca = forca.wrapping_add((core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) << 3); // mutacao: metade'),
    ('cpa_colocado_lugar', '    cpa3_d(p + [0xa, 0xe, 0x12][lugar as usize]) // campeão, vice, terceiro',
     '    cpa3_d(p + [0xe, 0xa, 0x12][lugar as usize]) // mutacao: campeao e vice trocados'),
]
