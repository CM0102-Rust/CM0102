# Mutacoes do registro do tecnico humano e do tecnico da busca. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('h_ferias', "        Some(r) => read_u32(r + human_manager_record::HOLIDAY_STAMP),", "        Some(r) => read_u32(r + 0x9a61),"),
    ('h_escalacao', "        Some(r) => r + human_manager_record::SAVED_LINEUP,", "        Some(r) => r + 0x9b2c,"),
    ('h_registro', "    Some((*(manager as *const usize) as isize + d as isize * 42735) as usize)", "    Some((*(manager as *const usize) as isize + d as isize * 42734) as usize)"),
]
