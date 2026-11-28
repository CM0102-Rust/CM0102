# Mutacoes pela prova do save (o .qst gravado pela original e pela Rust comparado item a item).
# - o premio de equipe sem gravar a data da atualizacao;
# - o premio do ano sem a lista curta;
# - as notas do ano sem as entradas.
MUTACOES = [
    ('sx05b_7_sem_data',
     '        (ta::UPDATE_DATE, 8, 0x96),',
     '        (ta::UPDATE_DATE, 0, 0x96), // mutation: the update date not saved (size 0: the array keeps its length)'),
    ('sx05b_7_sem_lista',
     '        save(shortlist, file);',
     '        let _ = (save, shortlist); // mutation: no shortlist'),
    ('sx05b_7_sem_entradas',
     '    if count != 0 && exe::zwrite(read_usize(this), yr::ENTRY_SIZE as u32, count, file) != read_u32(this + 4) {',
     '    if count != 0 && false { // mutation: no entries'),
]
