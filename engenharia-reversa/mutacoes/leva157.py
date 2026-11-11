# Mutacoes da leva157 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva157.py --carreira 2002-06-01
MUTACOES = [
    ('n_volume', "    if b(commentary_slot::CURRENT_PART) as i8 <= 0 && *((nar + commentary_queue::COMMENTARY_SPEED) as *const u8) >= 0x65 {", "    if b(commentary_slot::CURRENT_PART) as i8 <= 0 && *((nar + commentary_queue::COMMENTARY_SPEED) as *const u8) >= 0x64 {"),
    ('n_duracao', "(weight.wrapping_mul(volume) / 100) as i16 as i32", "(weight.wrapping_mul(volume) / 50) as i16 as i32"),
]
