# Mutacoes da partida da noticia, do resultado da eliminatoria e do dia da rodada da copa. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('cu_clube', "    d(0x1c, ponteiro(0xc, 0xae23b4, 0x245)); // clube da casa", "    d(0x1c, ponteiro(0x10, 0xae23b4, 0x245)); // clube da casa"),
    ('cu_sem_destaque', "    4 // sem destaque", "    3 // sem destaque"),
    ('cu_semana', "    *((e + 0x27) as *mut u8) = semana as u8; // dia da semana", "    *((e + 0x27) as *mut u8) = (semana as u8).wrapping_add(1); // dia da semana"),
]
