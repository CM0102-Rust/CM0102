# Mutacoes (rx10_12), com as 8 ligas ate 2003-08-15. Nenhuma abre caixa.
# - o id da nacao ao contrario no desempate (o maior primeiro): o comparador da comparacao acusa (lado a lado) quando
#   dois inscritos tem continente e reputacao da nacao iguais; a carreira, se a ordem dos potes mudar;
# - o primeiro grupo sem a ordenacao: os potes saem da ordem dos inscritos (a carreira: outros clubes no grupo da fase -1
#   e nos das outras fases; o comparador do grupo nao, ele le a lista como ficou).
# Os registros ficam sem mutacao propria: o que eles gravam e historico (nada lido antes da data volta a simulacao).
MUTACOES = [
    ('rx10_12_id_invertido',
     '    let id = (read_u32(na) as i32).wrapping_sub(read_u32(nb) as i32); // rx10_12: the lower nation id first',
     '    let id = (read_u32(nb) as i32).wrapping_sub(read_u32(na) as i32); // mutation',
     'rx10_12 club world championship first group'),
    ('rx10_12_sem_ordenar',
     '    exe::crt_qsort(read_usize(this + 0xee), read_u32(this + 0xf2), 4, 0x92b400usize); // rx10_12: seeded by continent strength',
     '    // mutation: no seeding',
     'rx10_12 club world championship first group'),
]
