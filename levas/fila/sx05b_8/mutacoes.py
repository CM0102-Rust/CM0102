# Mutacoes pela prova do save com a carga (CARGA: o FeriasTesteSave carregado com e sem as rotinas).
# - os incidentes de forma nunca lidos (o resto do awol.dat fica sem uso; a terceira lista vazia muda a impressao se
#   o save tiver incidentes);
# - a lista das ausencias sem os registros;
# - a tabela de marcas sem ler (fica o que o malloc deu).
MUTACOES = [
    ('sx05b_8_sem_incidentes',
     '            if version <= 0x3e005 {',
     '            if version <= 0x7fffffff { // mutation: never read'),
    ('sx05b_8_ausencias_vazias',
     '            exe::llist_add_tail(read_usize(this + off), record.as_ptr() as usize, size);',
     '            if k != 0 { exe::llist_add_tail(read_usize(this + off), record.as_ptr() as usize, size); } // mutation'),
    ('sx05b_8_marcas_sem_ler',
     '    if flags == 0 || exe::zread(flags, 1, n, z) != n || !awol_read(np, 4, z) {',
     '    if flags == 0 || exe::zread(flags, 1, 0, z) != 0 || !awol_read(np, 4, z) { // mutation'),
]
