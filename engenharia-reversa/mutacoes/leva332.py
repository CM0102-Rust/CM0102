# Mutações da 332a leva (itens sx05_2, sx05_3, sx05_4, sx05_6). Uso: python mutacoes.py mutacoes\leva332.py --carreira 2002-06-01
MUTACOES = [
    ('sx05_2_cinco_partidas', '    if a.matches() < 3 {\n        return (b.matches() >= 3) as i32;', '    if a.matches() < 5 { // mutation: 5 matches\n        return (b.matches() >= 3) as i32;', 'sx05_2 year order by score'),
    ('sx05_2_sem_bonus_do_atacante', '            score *= 1.2f32 as f64;', '            score *= 1.0; // mutation: no attacker bonus', 'sx05_2 attacker bonus'),
    ('sx05_2_partidas_sem_dobro', '(n * 0.02 + n * 0.02)', '(n * 0.02) /* mutation: not doubled */', 'sx05_2 year order by score'),
    ('sx05_3_assistencias_valem_um', '(5 * e.goals() + 3 * e.assists() + e.rating_sum(2))', '(5 * e.goals() + e.assists() + e.rating_sum(2)) /* mutation: assists count 1 */', 'sx05_3 mode 3 by club reputation'),
    ('sx05_3_cartoes_ao_contrario', '        e.cards() as f64 / e.matches() as f64\n', '        -(e.cards() as f64) / e.matches() as f64 // mutation: fewest cards first\n', 'sx05_3 mode 4 cards'),
    ('sx05_3_sem_corte_de_carater', '            score *= 0.5;\n', '            score *= 1.0; // mutation: no character cut\n', 'sx05_3 mode 5 poor character'),
    ('sx05_4_passadas_de_dez', '        min_matches -= 5;\n', '        min_matches -= 10; // mutation: steps of 10\n', 'sx05_4 fewer matches pass'),
    ('sx05_4_idade_exclusiva', 'e.person().age() as i8 > max_age as i8', 'e.person().age() as i8 >= max_age as i8 /* mutation: the maximum age left out */', 'sx05_4 too old'),
    ('sx05_4_excluido_aceito', '                    covered!("sx05_4 excluded candidate");\n                    return false;', '                    covered!("sx05_4 excluded candidate");\n                    return true; // mutation: excluded accepted', 'sx05_4 excluded candidate'),
    ('sx05_6_oito_sem_dezesseis', '    if m & 8 != 0 { // sx05_6: 8 brings 0x10\n        m |= 0x10;', '    if m & 8 != 0 { // sx05_6: 8 brings 0x10\n        m |= 0; // mutation: 8 no longer gives 0x10', 'sx05_6 candidate'),
    ('sx05_6_sem_troca', '            core::ptr::copy(t.slot(k) as *const u8, t.slot(j) as *mut u8, TEAM_AWARD_SLOT_SIZE);\n            return place(k);', '            return t.complete() as u32; // mutation: no move', 'sx05_6 occupant moved'),
    ('sx05_6_reservas_trocados', 'const TEAM_AWARD_SUBSTITUTES: [u16; 9] = [1, 0x40, 4,', 'const TEAM_AWARD_SUBSTITUTES: [u16; 9] = [1, 4, 0x40, /* mutation */', 'sx05_6 candidate'),
]
