# Mutacoes refeitas da leva166 (c_k0, c_k5 e c_tipo5 so mexiam em limites exatos).
# Uso: python mutacoes.py mutacoes\leva166b.py --carreira 2002-06-01 --confirmar 2001-08-20
MUTACOES = [
    ('c_k0_b', "                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { low(-30) }", "                if v > 0x4b { 1 } else if v >= 0x28 { 3 } else if v >= 10 { 3 } else { low(-30) }"),
    ('c_k5_b', "            } else if ambicao > 5 && rep(0xb) > 0x4e2 {\n                5\n            } else {\n                6\n            }", "            } else if ambicao > 5 && rep(0xb) > 0x4e2 {\n                5\n            } else {\n                7\n            }"),
    ('c_tipo5_b', "    if c.contract_type() & 0x3f == 5 {\n        return 8;", "    if c.contract_type() & 0x3f == 5 {\n        return 7;"),
]
