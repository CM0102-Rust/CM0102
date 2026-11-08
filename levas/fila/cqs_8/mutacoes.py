# Mutacoes (cqs_8), nas eliminatorias da Eurocopa (as regras no construtor, 2001-000; o sorteio em 2002-067, os jogos
# so depois de junho de 2002): o sorteio duas semanas depois (24 de marco: os sorteios do dia mudam de lugar e os jogos
# do meio recebem outros numeros); um turno so (outras rodadas na tabela e na liga de cada grupo: acusadas em 2002-068);
# o grupo 1 com uma rodada a mais por turno (+0xa7): ESCAPA (06/11, carreira de 8 ligas ate 2002-06-01): a conta das
# rodadas do grupo 1 so pesa nos jogos de setembro de 2002 em diante. Antes dela, a situacao ate 16 tambem escapou (a
# situacao nao muda o sorteio nem nada ate junho).
MUTACOES = [
    ('cqs8_sorteio_duas_semanas_depois', 'construir(&mut data, 0xa, 2, cqs8_w(this + 0x40) as u16 as u32, 5); // cqs8: a data do sorteio', 'construir(&mut data, 0x18, 2, cqs8_w(this + 0x40) as u16 as u32, 5); // mutacao: 24 de março', 'cqs_8 euro rules'),
    ('cqs8_um_turno', 'core::ptr::write_unaligned((this + 0x3c) as *mut i16, 2); // cqs8: dois turnos', 'core::ptr::write_unaligned((this + 0x3c) as *mut i16, 1); // mutacao', 'cqs_8 euro rules'),
    ('cqs8_grupo_um_com_rodada_a_mais', 'core::ptr::write_unaligned((this + 0xa7) as *mut i16, n.wrapping_sub(1).wrapping_mul(cqs8_w(this + 0x3c))); // cqs8: as rodadas do grupo 1', 'core::ptr::write_unaligned((this + 0xa7) as *mut i16, n.wrapping_mul(cqs8_w(this + 0x3c))); // mutacao', 'cqs_8 euro group 1'),
]
