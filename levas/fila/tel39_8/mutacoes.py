# Mutacoes (tel39_8).
# - a lista de comentaristas le so a primeira linha do arquivo (o comparador do construtor acusa no inicio da
#   carreira; na carreira Rust, as materias sorteiam de uma lista menor);
# - a suspensao por gesto obsceno comeca 21 dias depois em vez de 14 (so acusada se houver um gesto obsceno de quem
#   tem o bit 0x40 ate 2002-06-01: pode escapar, dito no PRONTO).
MUTACOES = [
    ('tel39_8_uma_linha',
     'while pundit_list_read_pundit_line(this, file, line.as_mut_ptr() as usize) != 0 {}',
     'if pundit_list_read_pundit_line(this, file, line.as_mut_ptr() as usize) != 0 {}',
     'tel39_8 pundit list'),
    ('tel39_8_tres_semanas',
     'const OBSCENE_BAN_DELAY_DAYS: u32 = 14;',
     'const OBSCENE_BAN_DELAY_DAYS: u32 = 21;',
     'tel39_8 obscene gesture ban'),
]
