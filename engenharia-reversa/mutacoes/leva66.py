# Mutacoes da leva66 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva66.py --jogo 2002-01-01
MUTACOES = [
    ('p_limpa', "    core::ptr::write_bytes((m + 0x1af) as *mut u8, 0xff, 22);", "    core::ptr::write_bytes((m + 0x1af) as *mut u8, 0xff, 21);"),
    ('p_marca', "                *((m as isize + 0x8eda + 20 * l as isize + e as isize) as *mut u8) = 1;", "                *((m as isize + 0x8eda + 20 * l as isize + e as isize) as *mut u8) = 2;"),
    ('p_tipo5', "                let c = cobrador(m, lado, 5, m + 0x8eda);", "                let c = cobrador(m, lado, 5, 0);"),
    ('p_5f', "                && byte(tecnico, 0x5f) != 0\n", "\n"),
    ('p_11', "                *((m as isize + 0x1af + 11 * l as isize + vez as isize) as *mut i8) = e;", "                *((m as isize + 0x1af + 11 * l as isize + vez as isize) as *mut i8) = e.wrapping_add(1);"),
]
