# Mutacoes da leva102 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva102.py --jogo 2001-08-20
MUTACOES = [
    ('f_1000', "    if v > 1000 {\n        factor = read_f64_at(0x968dc0)", "    if v > 999 {\n        factor = read_f64_at(0x968dc0)"),
    ('f_campo', "            Some((ritmo + 15) * 10)\n        } else if byte(j, match_player::POSITION) < 0 {", "            Some((ritmo + 16) * 10)\n        } else if byte(j, match_player::POSITION) < 0 {"),
    ('f_banco', "            Some(ritmo * 12 + 0x145)", "            Some(ritmo * 12 + 0x146)"),
    ('f_fora', "            Some((ritmo + 15) * 10)\n        };", "            Some((ritmo + 14) * 10)\n        };"),
    ('f_k', "        let k = ritmo * 12 + 0x55;", "        let k = ritmo * 12 + 0x56;"),
    ('f_marca', "            let v = if mark > 0x2d && mark < 0x41 {", "            let v = if mark > 0x2e && mark < 0x41 {"),
    ('f_copia', "            *f(match_player::STAMINA_CEILING) = *f(match_player::STAMINA);", "            *f(match_player::STAMINA_CEILING) = *f(match_player::STAMINA) + 1.0;"),
    ('f_6500', "                    *f(match_player::STAMINA_CEILING) = 6500.0;", "                    *f(match_player::STAMINA_CEILING) = 6400.0;"),
    ('f_59', "    let x = *f(match_player::STAMINA_BOOST) as f64 + *f(match_player::STAMINA_CEILING) as f64;", "    let x = *f(match_player::STAMINA_CEILING) as f64;"),
]
