# Mutacoes do clube basco e das listas de transferencias e de emprestimos. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq5_basco', "    CLUBES_BASCOS.iter().any(|&a| id == g(a)) as u32", "    (!CLUBES_BASCOS.iter().any(|&a| id == g(a))) as u32"),
    ('dq5_listada', "*((v + 0x4e) as *const u8) & 1 == 0", "*((v + 0x4e) as *const u8) & 2 == 0"),
    ('dq5_emprestimo', "*((v + 0x4e) as *const u8) & 2 != 0) as u32", "*((v + 0x4e) as *const u8) & 1 != 0) as u32"),
]
