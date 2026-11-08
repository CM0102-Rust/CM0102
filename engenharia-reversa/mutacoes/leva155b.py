# Mutacoes refeitas da leva155 (a opcao 0 e a que o motor consulta, 0x6d5cd0). Uso: python mutacoes.py mutacoes\leva155b.py --carreira 2002-06-01
MUTACOES = [
    ('e_opcao0', "                    let v = exe::at_94739c(BUF) as u8;", "                    let v = (exe::at_94739c(BUF) as u8).wrapping_add((o == 0) as u8);"),
    ('e_opcao2', "                    let v = exe::at_94739c(BUF) as u8;", "                    let v = if o == 2 { 0 } else { exe::at_94739c(BUF) as u8 };"),
]
