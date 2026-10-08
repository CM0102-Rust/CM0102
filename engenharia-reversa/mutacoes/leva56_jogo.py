# Mutacoes da leva56 no jogo real (escanteio e falta perto do gol consultam a tatica do clube, que
# so existe com o banco carregado). Uso: python mutacoes.py mutacoes\leva56_jogo.py --sem-base --jogo 2001-08-15
# Primeira rodada: as do cobrador designado (ordem 0/1, lado da area, sorteio) passaram: os clubes do
# computador quase nunca tem cobrador designado, e o escanteio cai na pontuacao. Estas atacam a pontuacao.
MUTACOES = [
    ('cj_89', "                    let s = ftol(f(j, 0x89) - f(j, 0xad) * f32_em(0x968f18) as f64) as i16;", "                    let s = ftol(f(j, 0x8d) - f(j, 0xad) * f32_em(0x968f18) as f64) as i16;"),
    ('cj_40', "                    if tabela_a(this, j) & 0x40 == 0 {", "                    if tabela_a(this, j) & 0x20 == 0 {"),
    ('cj_075', "                        ftol(s as f64 * f64_em(0x969040)) as i16\n                    } else {\n                        ftol(s as f64 * f64_em(0x968928)) as i16", "                        ftol(s as f64 * f64_em(0x968928)) as i16\n                    } else {\n                        ftol(s as f64 * f64_em(0x968928)) as i16"),
    ('cj_falta', "let mut s = (byte(pd, 0x29) as i16).wrapping_mul(5)", "let mut s = (byte(pd, 0x29) as i16).wrapping_mul(1)"),
]
