# Mutacoes (sx05b_1). A equipe da temporada inglesa sobe as reputacoes dos titulares (a carreira acusa) e a semana
# avisa os clubes humanos (o comparador lado a lado acusa os ids de clube).
# - os reservas tambem creditados (doze em vez de onze);
# - a noticia sem apagar o clube das vagas avisadas (o clube e avisado de novo);
# - o periodo com um ano a menos.
MUTACOES = [
    ('sx05b_1_doze_titulares',
     '    for k in 0..11 {\n        let id = t.slot_id(k);',
     '    for k in 0..12 { // mutation: a substitute credited\n        let id = t.slot_id(k);',
     'sx05b_1 starter credited'),
    ('sx05b_1_clube_nao_apagado',
     '                    count = count.wrapping_add(1);\n                    write_i32(t.slot(j) + 0x10, -1);',
     '                    count = count.wrapping_add(1); // mutation: club kept',
     'sx05b_1 human club told'),
    ('sx05b_1_ano_a_menos',
     '    let start = (year as i32).wrapping_sub(read_i8(this + ta::SEASON_SPAN) as i32) as u32;',
     '    let start = (year as i32).wrapping_sub(read_i8(this + ta::SEASON_SPAN) as i32 + 1) as u32; // mutation'),
]
