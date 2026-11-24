# Mutacoes (sx05b_3). Os premios do ano da Inglaterra no fim da temporada sobem reputacoes (a carreira acusa); a
# noticia e ensaiada pelo comparador (o sorteio do modelo).
# - o segundo com a reputacao inteira;
# - o terceiro sem o premio;
# - a noticia sem o sorteio do modelo.
MUTACOES = [
    ('sx05b_3_segundo_inteiro',
     '                2 => (ftol(national as f64 * read_f64(rdata::F64_0_75)) as u32, ftol(world as f64 * read_f64(rdata::F64_0_75)) as u32),',
     '                2 => (national as u16 as u32, world as u16 as u32), // mutation',
     'sx05b_3 winner credited'),
    ('sx05b_3_sem_terceiro',
     '    for place in 1..=3u32 {',
     '    for place in 1..=2u32 { // mutation: no third',
     'sx05b_3 winner credited'),
    ('sx05b_3_sem_sorteio',
     '    slot(0x13, exe::utils_random(3) as u32);',
     '    slot(0x13, 0); // mutation: no draw'),
]
