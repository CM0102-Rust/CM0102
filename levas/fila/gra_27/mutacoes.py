# Mutations (gra_27): the statistics are built once, when the career is created (2001-07); the languages they give are
# read by the game day (0x5d3020) and by the regens all season, so a different fluency changes the ENG career to
# 2002-06-01; the comparer (plan in a scratch object) catches all three at the creation.
# - the fluency divisor 50 -> 25 (native languages and the days abroad, shared helper);
# - the years abroad of a non-player capped at 3 instead of 10;
# - the days at the current club weighted 2.0 instead of 0.7 (foreign players at a club of another language).
MUTACOES = [
    ('gra27_fluencia_por_25',
     '    let v = (w as i16 as i32 - threshold as i16 as i32) / 50; // gra_27 the fluency divisor',
     '    let v = (w as i16 as i32 - threshold as i16 as i32) / 25; // gra_27 mutation: the fluency divisor',
     'gra_27 native language'),
    ('gra27_anos_fora_3',
     '        let years = if years > 10 { 10 } else { years }; // gra_27 the years abroad',
     '        let years = if years > 3 { 3 } else { years }; // gra_27 mutation: the years abroad',
     'gra_27 language of the club abroad'),
    ('gra27_dias_no_clube_x2',
     '                    let scaled = days as f64 * read_f64(rdata::F64_0_7); // gra_27 the days at the current club',
     '                    let scaled = days as f64 * 2.0; // gra_27 mutation: the days at the current club',
     'gra_27 days abroad from the history'),
]
