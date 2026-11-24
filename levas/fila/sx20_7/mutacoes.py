# Mutacoes (sx20_7), item de SAVE com CARGA: a prova do save grava o .qst e carrega o save de ferias com e sem a leva:
# - o registro de tecnico lido sempre no formato antigo (0xce bytes: o resto do item sai deslocado; a carga acusa);
# - os registros de tecnico gravados com 0xce bytes (o .qst muda);
# - os grupos simulados lidos com um grupo a menos (a carga acusa).
MUTACOES = [
    ('sx20_7_registro_antigo',
     'if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < STAFF_RECORDS_SHORT_RECORD_VERSION { // sx20_7: the old record',
     'if true { // mutation: always the old record',
     'sx20_7 load staff records'),
    ('sx20_7_registros_curtos',
     '(read_usize(this + f::MANAGER_RECORDS), MANAGER_RECORD_SIZE as u32, records, 0x115d), // sx20_7: the manager records',
     '(read_usize(this + f::MANAGER_RECORDS), 0xce, records, 0x115d), // mutation: short manager records',
     'sx20_7 save staff records'),
    ('sx20_7_grupo_a_menos',
     'if exe::zread(read_usize(this + 4), SIMULATED_GROUP_SIZE as u32, read_u32(this), z) != read_u32(this) { // sx20_7: every group read',
     'if exe::zread(read_usize(this + 4), SIMULATED_GROUP_SIZE as u32, read_u32(this) - 1, z) != read_u32(this) - 1 { // mutation: one group less',
     'sx20_7 load simulated stats'),
]
