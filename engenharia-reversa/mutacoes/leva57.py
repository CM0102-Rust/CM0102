# Mutacoes da leva57 no jogo real (lateral, escanteio e penalti arrumam os 40 jogadores na grade e o
# escanteio consulta a tatica do clube). Uso: python mutacoes.py mutacoes\leva57.py --jogo 2001-08-15
MUTACOES = [
    ('b_conta', "    let conta = (this as isize + 54 * t as isize + 0xf5c8) as *mut u8;", "    let conta = (this as isize + 54 * t as isize + 0xf5c9) as *mut u8;"),
    ('b_lat_tab', "            let tabela = *((this + 0xf56e + 4 * (byte(j, 0x27) == 1) as usize) as *const usize);", "            let tabela = *((this + 0xf56e + 4 * (byte(j, 0x27) == 0) as usize) as *const usize);"),
    ('b_lat_dir', "    *((c + 0x19a) as *mut i16) = if *a7 != 0 { 270 } else { 90 };", "    *((c + 0x19a) as *mut i16) = if *a7 != 0 { 90 } else { 270 };"),
    ('b_esc_area', "                (true, true, 2) | (false, true, 3) => Some('c'),", "                (true, true, 2) => Some('c'),"),
    ('b_esc_desl', "                    let desl = if proprio == (l == 0) { 0x3b } else { 0x30 };", "                    let desl = if proprio { 0x3b } else { 0x30 };"),
    ('b_esc_band', "    let x = if byte(this, 0x8ea7) as i32 > 4 - r { 0 } else { 8 };", "    let x = if byte(this, 0x8ea7) as i32 >= 4 - r { 0 } else { 8 };"),
    ('b_pen_linha', "    let linha: i8 = if t != 0 { 1 } else { 10 };", "    let linha: i8 = if t != 0 { 2 } else { 10 };"),
    ('b_pen_9', "                } else if y > 9 {\n                    mover(j, byte(j, 0x102), 9);", "                } else if y > 10 {\n                    mover(j, byte(j, 0x102), 9);"),
]
