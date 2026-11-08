# Mutações da 218a leva (itens save_s1_a, save_s1_b, save_s1_c). Uso: python mutacoes.py mutacoes\leva218.py --prova-save 218
MUTACOES = [
    ('sv_padrao_letra', '            if exe::utils_char_to_lower(c as u32) != n {', '            if exe::utils_char_to_lower(c as u32 + 1) != n {'),
    ('sv_padrao_ponto', "                if *name.add(j) == b'.' && exe::at_946540(name.add(j + 1), b'.' as i32) == 0 {", "                if *name.add(j) == b'.' && exe::at_946540(name.add(j + 1), b'.' as i32) != 0 {"),
    ('sv_padrao_fim', '        if *default.add(i) == 0 {\n            return 1;', '        if *default.add(i) == 0 {\n            return 0;'),
    ('sv_emp_bloco', '            (exe::zwrite(z + 0x41705, (1) as u32, (n) as u32, z) as i32);', '            (exe::zwrite(z + 0x41705, (1) as u32, (n - 1) as u32, z) as i32);'),
    ('sv_emp_resto', '        let mut n = (exe::crt_fread(z + 0x41705, 1, 0x30d40, f) as i32);\n        while n > 0 {', '        let mut n = (exe::crt_fread(z + 0x41705, 1, 0x30d40, f) as i32);\n        while n > 0x1000 {'),
    ('sv_emp_so_o_primeiro', '        if exe::file_find_next((found_p) as *mut u8) != 0 {', '        if exe::file_find_next((found_p) as *mut u8) != 0 || true {'),
    ('sv_rle_teto', '                if n.wrapping_add(0x80) < 0xff {', '                if n.wrapping_add(0x80) < 0x100 {'),
    ('sv_rle_dois', '            } else if n <= 1 && c <= 0x80 {', '            } else if n <= 2 && c <= 0x80 {'),
    ('sv_rle_fecho', '                let mark = (n as u8).wrapping_add(0x80);', '                let mark = (n as u8).wrapping_add(0x81);'),
]
