# Lote 4b da carreira ampla (22/10/2026, de dia, intercalado com as levas): o lote 3 e as escapadas
# por dados das levas 206, 210 a 213. Uso: python mutacoes.py mutacoes\lote_ampla_4b.py --carreira 2002-03-31 --ligas "ENG ESP ITA FRA BRA JPN AUS USA" --limit 7200
MUTACOES = [
    ('f_fase', '        0xf => (o.response_round() >= 3) as u32,', '        0xf => (o.response_round() >= 2) as u32,'),
    ('m_alguma', '    any_news_for_player(average, *id, |_| true) as u8', '    any_news_for_player(average, *id, |r| *((r + 0x10) as *const u8) != 0) as u8'),
    ('d_sem_regra', '    if k == -1 {\n        return;\n    }\n    let rules', '    if k <= 0 {\n        return;\n    }\n    let rules'),
    ('d_43', '    for k in 0..43usize {', '    for k in 0..42usize {'),
    ('t_desmarca', '    *staff_flags_ptr(manager, id) &= 0xbf;', '    *staff_flags_ptr(manager, id) &= 0x3f;'),
    ('t_vaga', '    if k < *((table + transfer_manager_fields::OFFER_CAPACITY) as *const i32) {', '    if k <= *((table + transfer_manager_fields::OFFER_CAPACITY) as *const i32) {'),
    ('r_ids', '    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {', '    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e] {'),
    ('s_empate', '        if best == 0 || injury_word_0c(c) > injury_word_0c(Staff(best)) {', '        if best == 0 || injury_word_0c(c) >= injury_word_0c(Staff(best)) {'),
    ('c_clube', '    *club = (*(G_CLUB_TABLE as *const usize) as isize + (contract.club() as i32) as isize * 581) as usize;', '    *club = (*(G_CLUB_TABLE as *const usize) as isize + (contract.club() as i32) as isize * 580) as usize;'),
]
