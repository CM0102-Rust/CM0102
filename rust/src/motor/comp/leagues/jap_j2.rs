use super::*;

/// J2: four round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; 2 go up to J1 (the
/// rule word stays the base one, 3: champion and promotion), no relegation, no league below; 5 bench substitutes, 3
/// substitutions. Then the schedule
/// Original: `JapJ2::set_up_rules` (0x665140, jap_j2.cpp, thiscall).
///
/// [pt] { 0x665140 } (thiscall, jap_j2.cpp): `JapJ2::set_up_rules`, as regras da J2 e o calendário; devolve 1.
pub(crate) unsafe extern "thiscall" fn jap_j2_set_up_rules(this: usize) -> u32 {
    j2_rules(this);
    crate::comparison::branch("cqj_7 regras da j2");
    create_main_schedule(this);
    1
}

/// J2 has 12 clubs, those of the division in database order; those beyond 12 lose their division; no prize fund nor TV
/// money
/// Original: `JapJ2::create_table` (0x665030, jap_j2.cpp, thiscall, ret 0).
///
/// [pt] { 0x665030 } (thiscall, ret 0, jap_j2.cpp): `JapJ2::create_table()`: a J2 tem 12 clubes (+0x57 = [0x9cf768]; os que
/// sobram perdem a divisão), sem fundo de prêmios nem TV. Fonte 0x9eaedc, linha 0x13a. Devolve 1 se encheu.
pub(crate) unsafe extern "thiscall" fn jap_j2_create_table(this: usize) -> u32 {
    if alloc_league_table(this, 0xc, 0x9eaedc, 0x13a) == 0 {
        return 0;
    }
    fill_japanese_table(this, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, 0, "cqm_9 j2 cheia")
}
