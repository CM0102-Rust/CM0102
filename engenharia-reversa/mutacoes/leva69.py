# Mutacoes da leva69 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva69.py
MUTACOES = [
    ('c_limite', "    if read_i32(club) >= limit {\n        let continent", "    if read_i32(club) > limit {\n        let continent"),
    ('c_cont0', "        if continent != 0 && (nation == 0 || d(nation, 0x71) != continent) {", "        if continent != 0 && (nation != 0 && d(nation, 0x71) != continent) {"),
    ('c_divisao', "        if nation == d(base, 0x5d) && d(club, 0x57) != 0 && d(club, 0x57) == base {", "        if nation == d(base, 0x5d) && d(club, 0x57) != 0 {"),
    ('c_outro', "        if d(nation, 0x71) != d(base, 0x59) {\n            return 0;", "        if d(nation, 0x71) == d(base, 0x59) {\n            return 0;"),
    ('c_grupo', "        if read_i8(stage + league::COMP_TYPE) == 2 {\n            grupo = stage;", "        if read_i8(stage + league::COMP_TYPE) == 1 {\n            grupo = stage;"),
    ('c_passo', "if grupo != 0 { (grupo, 0xb1, 0x3b) }", "if grupo != 0 { (grupo, 0xb1, 0x3a) }"),
    ('c_ate', "    while i <= n {", "    while i < n {"),
    ('e_lista', "[KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]\n        .iter()", "[KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES]\n        .iter()"),
]
