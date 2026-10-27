# Mutacoes das copias dos recordes do clube. Como as irmas da 216a (cr_recordista, cr_partida, cr_sequencia), os
# recordes do clube ficam fora da impressao da carreira: PODEM ESCAPAR da carreira; o comparando acusa na primeira copia.
MUTACOES = [
    ('cpf_nota_byte', '*((this + 0x1c) as *mut u8) = *((origem + 0x1c) as *const u8); // o byte do recorde de nota',
     '*((this + 0x1c) as *mut u8) = 0; // mutacao: o byte zerado'),
    ('cpf_melhor_vezes', '*(this as *mut u8) = *(origem as *const u8); // as vezes de melhor em campo',
     '*(this as *mut u8) = (*(origem as *const u8)).wrapping_add(1); // mutacao: uma vez a mais'),
    ('cpf_disciplina_vermelhos', '*((this + 1) as *mut u8) = *((origem + 1) as *const u8); // os vermelhos',
     '*((this + 1) as *mut u8) = *(origem as *const u8); // mutacao: os amarelos no lugar'),
]
