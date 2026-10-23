# Mutacoes da idade, da transferencia livre e da UE. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq2_aniversario', "        anos.wrapping_sub(1) // ainda não fez aniversário", "        anos.wrapping_sub(2) // ainda não fez aniversário"),
    ('dq2_europa', "    (g(continente) == g(0x9cfa10)) as u32 // Europa", "    (g(continente) != g(0x9cfa10)) as u32 // Europa"),
    ('dq2_ue', "    if *((n + 0x7f) as *const u8) == 2 { return 1; } // nacionalidade da UE", "    if *((n + 0x7f) as *const u8) == 3 { return 1; } // nacionalidade da UE"),
]
