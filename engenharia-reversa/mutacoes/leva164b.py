# Mutacoes refeitas da leva164 (g_meio e g_volta so mudavam um caso exato que a carreira nao reaches).
# Uso: python mutacoes.py mutacoes\leva164b.py --carreira 2002-06-01 --confirmar 2001-08-20
MUTACOES = [
    ('g_meio_b', "        return if p == n / 2 { n - 1 } else if p == n { n / 2 } else { p };", "        return if p == n / 2 { n - 1 } else if p == n { n / 2 } else { p + 1 };"),
    ('g_volta_b', "    let volta = |p: i32| if p < n { p } else { p - n };", "    let volta = |p: i32| if p < n { p } else { p - n + 1 };"),
]
