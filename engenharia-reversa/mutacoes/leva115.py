# Mutacoes da leva115 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva115.py --jogo 2001-08-20
MUTACOES = [
    ('t_onze', "    if n == 11 {\n        request_buffers(", "    if n == 10 {\n        request_buffers("),
    ('t_goleiro', "        if byte(j, 0x19) < 0 || tabela(j) == 1 {\n            continue;", "        if byte(j, 0x19) < 0 || tabela(j) == 2 {\n            continue;"),
    ('t_modo', "    let mut modo = 0x8800u32;", "    let mut modo = 0x8801u32;"),
    ('t_pos', "        if let Some(k) = (0..n).find(|&k| lugar[k] == f) {\n            *((j + 0x19) as *mut i8) = pos[k];", "        if let Some(k) = (0..n).find(|&k| lugar[k] == f) {\n            *((j + 0x19) as *mut i8) = pos[(k + 1) % n];"),
    ('t_marca', "        marca[n] = marcas(u(0x2b), byte(j, 0x19));", "        marca[n] = marcas(u(0x2b), byte(j, 0x19)) ^ 1;"),
]
