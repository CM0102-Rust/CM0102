# Mutacoes (sx12_3). O menu de nacoes nao volta para a simulacao: acusadas pelos comparadores da rotina, que rodam a
# cada montagem da barra de menus na carreira de ferias.
# - a nacao depois da do meio (outra letra de divisao);
# - o cabecalho da segunda parte sem o "+ 1" da letra;
# - a nacao igual a letra da divisao tambem na segunda parte.
MUTACOES = [
    ('sx12_nacao_do_meio',
     'let middle = (count as i8 as i32 + 1) / 2; // sx12_3: the middle nation',
     'let middle = (count as i8 as i32 + 3) / 2; // mutation: the nation after the middle one',
     'sx12_3 split letter'),
    ('sx12_cabecalho_segunda',
     'story_lang!(text, 0x9eb0b0, letter + 1); // sx12_3: "Nations <%c - Character>-Z"',
     'story_lang!(text, 0x9eb0b0, letter); // mutation: the split letter itself',
     'sx12_3 second part header'),
    ('sx12_lado_da_nacao',
     '(first as i8 > read_i8(this)) as u8 // sx12_3: after the split letter',
     '(first as i8 >= read_i8(this)) as u8 // mutation: from the split letter on',
     'sx12_3 second part'),
]
