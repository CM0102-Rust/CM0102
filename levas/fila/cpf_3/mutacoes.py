# Mutacoes das copias dos recordes de valor e de gols numa partida. Recordes do clube fora da impressao da carreira:
# PODEM ESCAPAR da carreira; a de gols (1 chamada) so o comparando acusa se a chamada vier no trecho comparado.
MUTACOES = [
    ('cpf_valor_troca', '*((this + 0x28) as *mut u8) = *((origem + 0x28) as *const u8); // a marca de troca',
     '*((this + 0x28) as *mut u8) = 1; // mutacao: sempre troca'),
    ('cpf_gols', '*(this as *mut u8) = *(origem as *const u8); // os gols',
     '*(this as *mut u8) = (*(origem as *const u8)).wrapping_add(1); // mutacao: um gol a mais'),
]
