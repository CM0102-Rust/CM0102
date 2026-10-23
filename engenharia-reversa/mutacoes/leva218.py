# Mutações da 218a leva (itens save_s1_a, save_s1_b, save_s1_c). Uso: python mutacoes.py mutacoes\leva218.py --prova-save 218
MUTACOES = [
    ('sv_padrao_letra', '            if maiuscula(c as u32) != n {', '            if maiuscula(c as u32 + 1) != n {'),
    ('sv_padrao_ponto', "                if *nome.add(j) == b'.' && achar(nome.add(j + 1), b'.' as i32) == 0 {", "                if *nome.add(j) == b'.' && achar(nome.add(j + 1), b'.' as i32) != 0 {"),
    ('sv_padrao_fim', '        if *padrao.add(i) == 0 {\n            return 1;', '        if *padrao.add(i) == 0 {\n            return 0;'),
    ('sv_emp_bloco', '            gravar(z + 0x41705, 1, n, z);', '            gravar(z + 0x41705, 1, n - 1, z);'),
    ('sv_emp_resto', '        let mut n = ler(z + 0x41705, 1, 0x30d40, f);\n        while n > 0 {', '        let mut n = ler(z + 0x41705, 1, 0x30d40, f);\n        while n > 0x1000 {'),
    ('sv_emp_so_o_primeiro', '        if proximo(achado_p) != 0 {', '        if proximo(achado_p) != 0 || true {'),
    ('sv_rle_teto', '                if n.wrapping_add(0x80) < 0xff {', '                if n.wrapping_add(0x80) < 0x100 {'),
    ('sv_rle_dois', '            } else if n <= 1 && c <= 0x80 {', '            } else if n <= 2 && c <= 0x80 {'),
    ('sv_rle_fecho', '                let marca = (n as u8).wrapping_add(0x80);', '                let marca = (n as u8).wrapping_add(0x81);'),
]
