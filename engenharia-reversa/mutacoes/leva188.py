# Mutacoes dos acessores do treino. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('o_lista', "    core::ptr::read_unaligned((manager + 0x856) as *const usize)", "    core::ptr::read_unaligned((manager + 0x852) as *const usize)"),
    ('e_treino', "    (table as isize + id as isize * 6) as usize", "    (table as isize + id as isize * 6 + 2) as usize"),
    ('c_carga', "    *training_load_byte(injuries, id) = value;", "    *training_load_byte(injuries, id) = value.wrapping_add(1);"),
]
