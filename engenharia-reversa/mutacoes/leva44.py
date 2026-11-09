# Mutacoes da leva44 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva44.py
MUTACOES = [
    ('r_dono', "if this.ball_holder() != 0 || this.set_piece_taker() != 0 || this.ball_roll() == 0 {", "if this.set_piece_taker() != 0 || this.ball_roll() == 0 {"),
    ('r_parado', "        this.set_ball_roll(0);" + chr(10) + "        this.set_ball_height(ball_height::ON_GROUND);", "        this.set_ball_roll(0);"),
    ('r_v2', "    let anda = if v >= 2 {", "    let anda = if v >= 3 {"),
    ('r_v0', "    } else if v <= 0 {" + chr(10) + "        false", "    } else if v < 0 {" + chr(10) + "        false"),
    ('r_sorte', "        a >= random(2)", "        a > random(2)"),
    ('r_8ea9', "        if this.ball_height() != ball_height::ON_GROUND {", "        if this.ball_height() > ball_height::ON_GROUND {"),
    ('r_220', "if dir > 0xdc && dir < 0x140 {", "if dir >= 0xdc && dir < 0x140 {"),
    ('r_140', "} else if dir > 0x28 && dir < 0x8c {", "} else if dir > 0x28 && dir <= 0x8c {"),
    ('r_130', "} else if dir > 0x82 && dir < 0xe6 {", "} else if dir > 0x83 && dir < 0xe6 {"),
    ('r_50', "} else if dir < 0x32 || dir > 0x136 {", "} else if dir <= 0x32 || dir > 0x136 {"),
    ('r_x9', "if x < 0 || y < 0 || x >= 9 || y >= 12 {", "if x < 0 || y < 0 || x >= 10 || y >= 12 {"),
    ('r_lado', "ball_out_with(this, this.last_possession_side() as u8, call);", "ball_out_with(this, this.possession_side() as u8, call);"),
]
