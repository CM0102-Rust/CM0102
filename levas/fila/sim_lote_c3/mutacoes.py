# Mutacoes da partida assistida, do apelido do humano e da copia da aposentadoria pendente. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('ha_ninguem_assiste', "    0 // ninguém assiste", "    1 // ninguém assiste"),
    ('ha_apelido', "    achou // apelido", "    achou ^ 1 // apelido"),
    ('ha_motivo', "*((this + 0x18) as *mut u8) = *((origem + 0x18) as *const u8); // motivo", "*((this + 0x18) as *mut u8) = *((origem + 0x18) as *const u8) ^ 1; // motivo"),
]
