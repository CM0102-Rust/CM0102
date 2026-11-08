# Mutacoes da leva88 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva88.py --jogo 2001-08-20
MUTACOES = [
    ('b_6', "    (if side_of_player(this) == 1 { y >= 6 } else { y < 6 }) as u8", "    (if side_of_player(this) == 1 { y >= 5 } else { y < 6 }) as u8"),
    ('m_11', "    if random(11) != 0 {\n        return;\n    }\n    let ff", "    if random(10) != 0 {\n        return;\n    }\n    let ff"),
    ('m_metade', "        *f(match_player::STAMINA_BOOST) = (*f(match_player::STAMINA_BOOST) as f64 * read_f64_at(0x968968)) as f32;", "        *f(match_player::STAMINA_BOOST) = (*f(match_player::STAMINA_BOOST) as f64 * read_f64_at(0x969040)) as f32;"),
    ('m_50', "    if b1 > 50 {", "    if b1 > 60 {"),
    ('m_teto', "    } else if *f(match_player::STAMINA) as f64 > read_f64_at(0x968910) { // 0x968910 = 10000\n        *f(match_player::STAMINA) = 10000.0;", "    } else if *f(match_player::STAMINA) as f64 > read_f64_at(0x968910) {\n        *f(match_player::STAMINA) = 9000.0;"),
    ('i_amistoso', "            if byte(t, staff::CLASSIFICATION) == 1 && u(t + staff::CLUB_JOB) != 0 && exe::comp_includes_club(object(), u(t + staff::CLUB_JOB)) != 0 {", "            if byte(t, staff::CLASSIFICATION) == 1 && u(t + staff::CLUB_JOB) != 0 && exe::comp_includes_club(object(), u(t + staff::CLUB_JOB)) == 0 {"),
    ('i_principal', "        if exe::comp_includes_club(object(), exe::database_get_national_team(u(t + staff::NATION))) != 0 {", "        if exe::comp_includes_club(object(), u(t + staff::NATION)) != 0 {"),
    ('i_16', "    for s in 0..16 {\n        let t = manager(s);\n        if byte(t, staff::CLASSIFICATION) != 1 {", "    for s in 0..15 {\n        let t = manager(s);\n        if byte(t, staff::CLASSIFICATION) != 1 {"),
]
