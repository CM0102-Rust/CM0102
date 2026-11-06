# Mutacoes (cqs_8), nas eliminatorias da Eurocopa (as regras no construtor, 2001-000; o sorteio em 2002-067, os jogos
# so depois de junho de 2002): o sorteio duas semanas depois (24 de marco: os sorteios do dia mudam de lugar e os jogos
# do meio recebem outros numeros); um turno so (outras rodadas na tabela e na liga de cada grupo); a situacao do clube
# ate 16 (o sorteio le a situacao das selecoes [inferencia]).
MUTACOES = [
    ('cqs8_sorteio_duas_semanas_depois', 'construir(&mut data, 0xa, 2, cqs8_w(this + 0x40) as u16 as u32, 5); // cqs8: a data do sorteio', 'construir(&mut data, 0x18, 2, cqs8_w(this + 0x40) as u16 as u32, 5); // mutacao: 24 de março', 'cqs_8 regras da eurocopa'),
    ('cqs8_um_turno', 'core::ptr::write_unaligned((this + 0x3c) as *mut i16, 2); // cqs8: dois turnos', 'core::ptr::write_unaligned((this + 0x3c) as *mut i16, 1); // mutacao', 'cqs_8 regras da eurocopa'),
    ('cqs8_situacao_ate_16', '((((x as u8 as i8) < 0xe) as i32) - 3) as i16 // cqs8: a situação pela estatística 0x12', '((((x as u8 as i8) < 0x10) as i32) - 3) as i16 // mutacao', 'cqs_8 grupo 1 da eurocopa'),
]
