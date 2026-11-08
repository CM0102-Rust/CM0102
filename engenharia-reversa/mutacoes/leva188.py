# Mutacoes dos acessores do treino. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('o_lista', "    core::ptr::read_unaligned((gerente + 0x856) as *const usize)", "    core::ptr::read_unaligned((gerente + 0x852) as *const usize)"),
    ('e_treino', "    (tabela as isize + id as isize * 6) as usize", "    (tabela as isize + id as isize * 6 + 2) as usize"),
    ('c_carga', "    *training_load_byte(lesoes, id) = valor;", "    *training_load_byte(lesoes, id) = valor.wrapping_add(1);"),
]
