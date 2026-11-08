# Mutacoes da leva56 no jogo real (corner e falta perto do gol consultam a tatica do clube, que
# so existe com o banco carregado). Uso: python mutacoes.py mutacoes\leva56_jogo.py --sem-base --jogo 2001-08-15
# Primeira rodada: as do pick_restart_taker designado (ordem 0/1, lado da area, sorteio) passaram: os clubes do
# computador quase nunca tem pick_restart_taker designado, e o corner cai na pontuacao. Estas atacam a pontuacao.
MUTACOES = [
    ('cj_89', "                    let s = match_pl_ftol(f(j, match_player::CORNERS) - f(j, match_player::JUMPING) * read_f32_at(0x968f18) as f64) as i16;", "                    let s = match_pl_ftol(f(j, match_player::CROSSING) - f(j, match_player::JUMPING) * read_f32_at(0x968f18) as f64) as i16;"),
    ('cj_40', "                    if table_a(this, j) & 0x40 == 0 {", "                    if table_a(this, j) & 0x20 == 0 {"),
    ('cj_075', "                        match_pl_ftol(s as f64 * read_f64_at(0x969040)) as i16 // 0x969040 = 0.75\n                    } else {\n                        match_pl_ftol(s as f64 * read_f64_at(0x968928)) as i16", "                        match_pl_ftol(s as f64 * read_f64_at(0x968928)) as i16\n                    } else {\n                        match_pl_ftol(s as f64 * read_f64_at(0x968928)) as i16"),
    ('cj_falta', "let mut s = (byte(pd, player_sheet::FREE_KICKS) as i16).wrapping_mul(5)", "let mut s = (byte(pd, player_sheet::FREE_KICKS) as i16).wrapping_mul(1)"),
]
