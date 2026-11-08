# Mutacoes da leva43 (ver docs/ROTINAS.md): k_ap0 e k_ap1 sao equivalentes (y ja limitado). Uso: python mutacoes.py mutacoes\leva43.py
MUTACOES = [
    ('k_sai', "if !(0..=1).contains(&sai) {", "if !(0..=2).contains(&sai) {"),
    ('k_8ea8', "*((this + engine::BALL_Y) as *mut i8) = if sai == 0 { 5 } else { 6 };", "*((this + engine::BALL_Y) as *mut i8) = if sai == 0 { 6 } else { 5 };"),
    ('k_y6', "                if y < 6 {" + chr(10) + "                    y = 6;", "                if y < 5 {" + chr(10) + "                    y = 6;"),
    ('k_y5', "} else if y >= 6 {" + chr(10) + "                y = 5;", "} else if y > 6 {" + chr(10) + "                y = 5;"),
    ('k_ap0', "apontar_para(p, x, y, 4, if y < 9 { y + 3 } else { 11 });", "apontar_para(p, x, y, 4, if y < 8 { y + 3 } else { 11 });"),
    ('k_ap1', "apontar_para(p, x, y, 4, if y > 3 { y - 3 } else { 0 });", "apontar_para(p, x, y, 4, if y >= 3 { y - 3 } else { 0 });"),
    ('k_melhor', "byte(this, 0x8ea8));" + chr(10) + "        if d < melhor {", "byte(this, 0x8ea8));" + chr(10) + "        if d <= melhor {"),
    ('k_pos10', "        if pos > 10 {" + chr(10) + "            *((p + match_player::POSITION) as *mut i8) = -1;", "        if pos > 11 {" + chr(10) + "            *((p + match_player::POSITION) as *mut i8) = -1;"),
    ('k_nada', "*(this as *mut u8) = 1;" + chr(10) + "        return;", "*(this as *mut u8) = 2;" + chr(10) + "        return;"),
    ('k_dy', "let dy = if sai == 0 { 11i8.wrapping_sub(y) }", "let dy = if sai == 0 { 10i8.wrapping_sub(y) }"),
    ('k_8ed8', "*((this + engine::BALL_DIRECTION) as *const u16)" + chr(10) + "    } else {", "*((this + engine::BALL_DIRECTION) as *const u16) >> 8" + chr(10) + "    } else {"),
    ('k_campo', "{ (0xf572, 0xa6) } else { (0xf56e, 0x9e) };", "{ (0xf572, 0xa6) } else { (0xf56e, 0x9d) };"),
    ('k_sx', "*((chosen + match_player::FACING) as *mut i16) = v as u8 as i8 as i16;", "*((chosen + match_player::FACING) as *mut i16) = v as i16;"),
]
