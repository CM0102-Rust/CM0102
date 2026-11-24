# Mutacoes pela prova do save (o .qst gravado com e sem a rotina comparado item a item, o awol.dat).
# - a tabela de marcas com um byte a menos;
# - as contas das listas dobradas;
# - os incidentes de forma com 13 bytes.
MUTACOES = [
    ('sx05b_9_marca_a_menos',
     '    if exe::zwrite(read_usize(this + awol_fields::PERSON_FLAGS), 1, staff, z) != staff {',
     '    if exe::zwrite(read_usize(this + awol_fields::PERSON_FLAGS), 1, staff - 1, z) != staff - 1 { // mutation'),
    ('sx05b_9_conta_dobrada',
     '        let count = read_i16(list + llist::COUNT) as i32;\n        if exe::zwrite(&count',
     '        let count = read_i16(list + llist::COUNT) as i32 * 2; // mutation\n        if exe::zwrite(&count'),
    ('sx05b_9_incidente_curto',
     '            if exe::zwrite(record, size, 1, z) != 1 {',
     '            if exe::zwrite(record, size.min(0xd), 1, z) != 1 { // mutation'),
]
