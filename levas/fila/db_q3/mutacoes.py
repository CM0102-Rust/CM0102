# Mutacoes da lista de transferencias e dos atributos de 1 a 20. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq3_listada', "*((v + 0x4e) as *const u8) & 9 != 0", "*((v + 0x4e) as *const u8) & 8 != 0"),
    ('dq3_arredonda', " + f64_em(0x968968); // arredonda", " + 0.0; // arredonda"),
    ('dq3_atributo_da_pessoa', "    calcular(pessoa, core::ptr::read_unaligned((pessoa + 0x61) as *const usize), atributo)\n", "    calcular(pessoa, core::ptr::read_unaligned((pessoa + 0x61) as *const usize), atributo).wrapping_add(1)\n"),
]
