# Mutacoes (cx10_7): textos, acusados pela comparacao com CM_TEXTOS=1 (o texto nao volta para a simulacao). Nenhuma abre
# caixa. Saem so quando a noticia sai ate a data (clube em crise, aperto acima de 50, obras canceladas no fim da
# temporada do clube): podem escapar se nenhuma dessas noticias sair na carreira ENG ate 2002-06-01.
MUTACOES = [
    ('cx10_7_crise_sem_ligacao',
     '        let club = club_at(id as i32 as isize);\n        exe::text_add_link(out, (club.addr() + club::NAME) as *const u8, 0x7d5, club);',
     '        let _ = club_at(id as i32 as isize); // mutation'),
    ('cx10_7_obras_motivo_trocado',
     '            fmt(buf, if (0..=1).contains(&reason) { 0x9c93f0 } else { 0x9c9370 });',
     '            fmt(buf, if (0..=1).contains(&reason) { 0x9c9370 } else { 0x9c93f0 }); // mutation'),
    ('cx10_7_leste_sem_lugares',
     '                fmt(buf, 0x9c91cc, s.capacity().wrapping_sub(seats)); // the east terrace, capacity down to <n>',
     '                fmt(buf, 0x9c91cc, s.capacity()); // mutation'),
]
