# Mutacoes (sx12_5) pela prova do save (mutacoes.py --prova-save N): o .qst gravado no modo rust com a mutacao contra a
# base (a mesma DLL sem as rotinas da leva). Sem o 4o campo (contador), como as dos itens de save publicados.
# - os registros fisicos gravados com 30 bytes: o injury.dat sai menor e deslocado (o zwrite devolve a quantidade, sem
#   caixa);
# - a senha gravada com 49 bytes: o human_manager.dat sai deslocado;
# - as competicoes sem gravar o temporario: os .tmp das competicoes ficam de fora ou velhos no .qst (pode escapar se o
#   temporario de cada competicao ja estiver em dia no momento do save).
MUTACOES = [
    ('sx12_registros_fisicos',
     'if exe::zwrite(read_usize(this + injury_fields::PHYSICAL_RECORDS), 0x1f, staff, z) != read_u32(G_STAFF_COUNT) { // sx12_5: the physical records',
     'if exe::zwrite(read_usize(this + injury_fields::PHYSICAL_RECORDS), 0x1e, staff, z) != read_u32(G_STAFF_COUNT) { // mutation: 30-byte records'),
    ('sx12_senha_curta',
     '        (0x9a6a, 1, 0x32), // the password\n',
     '        (0x9a6a, 1, 0x31), // mutation: a byte short\n'),
    ('sx12_competicoes_sem_temporario',
     '                f(comp, 1); // sx12_5: the competition\'s temporary file\n',
     '                let _ = f; // mutation: the temporary file is not written\n'),
]
