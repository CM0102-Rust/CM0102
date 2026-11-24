# Mutacoes (sx09c_4) pela prova do save (mutacoes.py --prova-save N), sem o 4o campo (a base da prova roda com as
# rotinas desligadas):
# - a gravacao poe a quantidade de suspensoes guardadas (emprestimo) mais um: o discipline.dat sai com 2 bytes
#   diferentes em qualquer save;
# - a carga apaga as quatro vagas de cada pessoa (0xff) depois de ler os registros: ninguem fica ligado a uma regra,
#   os suspensos do save de ferias voltam a jogar e os cartoes da temporada recomecam; so a carga do FeriasTesteSave
#   (CARGA) acusa (o save tem 85 suspensoes, docs/MAPA_DISCIPLINA.md).
MUTACOES = [
    ('sx09c_guardadas_mais_um',
     'let held = held_list.count() as i32; // the original stores the i16 in a dword and writes 2 bytes',
     'let held = held_list.count() as i32 + 1; // mutation'),
    ('sx09c_carga_sem_vagas',
     'discipline_create_rule_sets(this, z as *mut usize as u32); // sx09c_4: the rule sets read their entries',
     'for p in 0..read_u32(G_STAFF_COUNT) as usize { core::ptr::write_bytes((records + p * 0x1d) as *mut u8, 0xff, 0x14); } discipline_create_rule_sets(this, z as *mut usize as u32); // mutation'),
]
