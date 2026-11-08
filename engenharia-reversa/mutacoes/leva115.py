# Mutacoes da leva115 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva115.py --jogo 2001-08-20
MUTACOES = [
    ('t_onze', "    if n == 11 {\n        request_buffers(", "    if n == 10 {\n        request_buffers("),
    ('t_goleiro', "        if byte(j, match_player::POSITION) < 0 || tabela(j) == 1 {\n            continue;", "        if byte(j, match_player::POSITION) < 0 || tabela(j) == 2 {\n            continue;"),
    ('t_modo', "    let mut mode = 0x8800u32;", "    let mut mode = 0x8801u32;"),
    ('t_pos', "        if let Some(k) = (0..n).find(|&k| place[k] == f) {\n            *((j + match_player::POSITION) as *mut i8) = pos[k];", "        if let Some(k) = (0..n).find(|&k| place[k] == f) {\n            *((j + match_player::POSITION) as *mut i8) = pos[(k + 1) % n];"),
    ('t_marca', "        mark[n] = marks(u(match_manager::TACTICS), byte(j, match_player::POSITION));", "        mark[n] = marks(u(match_manager::TACTICS), byte(j, match_player::POSITION)) ^ 1;"),
]
