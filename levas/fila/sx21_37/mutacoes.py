# Mutacoes (sx21_37). Caem pelos comparadores (valor) quando as rotinas rodam; a da comparacao muda a ordem das listas.
MUTACOES = [
    ('sx21_37_maiuscula_vira_minuscula',
     'read_u8(CHARACTER_TABLE + b as usize * 3 + 1) as u32 // sx21_37: the upper-case column',
     'read_u8(CHARACTER_TABLE + b as usize * 3 + 2) as u32 // mutation: the lower-case column',
     'sx21_37 upper case'),
    ('sx21_37_minuscula_vira_chave',
     'read_u8(CHARACTER_TABLE + b as usize * 3 + 2) as u32 // sx21_37: the lower-case column',
     'read_u8(CHARACTER_TABLE + b as usize * 3) as u32 // mutation: the sort key',
     'sx21_37 lower case'),
    ('sx21_37_ordem_invertida',
     'return if x < y { -1 } else { 1 }; // sx21_37: the first difference',
     'return if x < y { 1 } else { -1 }; // mutation: reversed',
     'sx21_37 strings differ'),
]
