# Mutacoes da leva123 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva123.py --jogo 2001-10-01
MUTACOES = [
    ('u_amarelo', "            2 => 0x1ff9,", "            2 => 0x1ff8,"),
    ('u_sem_cartao', "                if read_i8(e + engine::PENDING_INCIDENT_SEVERITY) as i32 > r && j.position() >= 0 { 0x210a } else { -1 }", "                if read_i8(e + engine::PENDING_INCIDENT_SEVERITY) as i32 >= r && j.position() >= 0 { 0x210a } else { -1 }"),
    ('u_tabela', "        let skips = v <= 0x3c && matches!(v, 0 | 5..=13 | 0x3b | 0x3c);", "        let skips = v <= 0x3c && matches!(v, 0 | 5..=12 | 0x3b | 0x3c);"),
    ('u_fora', "                    (weight as f64 * 0.16666666666666666 + 0.83) as f32", "                    (weight as f64 * 0.16666666666666666 + 0.84) as f32"),
    ('u_campo', "                    ((weight as f64 + 1.0) * 0.5) as f32", "                    ((weight as f64 + 1.0) * 0.6) as f32"),
    ('u_cansar', "                call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j.addr(), [1, 0, 0, 0, 0]);", "                call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j.addr(), [0; 5]);"),
    ('u_nove', "            if random(9) == 0 {", "            if random(8) == 0 {"),
    ('u_fim', "    *((m + engine::PUNISHMENT_STATE) as *mut u8) = 2;", "    *((m + engine::PUNISHMENT_STATE) as *mut u8) = 3;"),
]
