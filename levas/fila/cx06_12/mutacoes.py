# Mutacoes (cx06_12), com as 8 ligas e ATE 2004-11-01:
# - a selecao classificada nos grupos nao entra na fase final (sem EuroChamp::add_qualified_team): a Eurocopa de 2004 tem
#   outra selecao no lugar (o sorteio e os jogos de jun/2004 mudam);
# - quem cai na repescagem ganha o texto "qualify for": o comparador de texto acusa (com o modo CM_TEXTO_NOTICIAS);
# - a mudanca de situacao usa 13 no lugar de 14: o comparador lado a lado acusa.
MUTACOES = [
    ('cx06_12_sem_fase_final',
     '    add_qualified_team(finals, club);\n}',
     '    let _ = (finals, add_qualified_team); // mutation\n}',
     'cx06_12 qualified from the groups'),
    ('cx06_12_texto_da_repescagem',
     'euro_qual_finals_text(text, club_id, if d == 1 { 0x9c470c } else { 0x9c46b8 }); // "qualify for" / "out of"',
     'euro_qual_finals_text(text, club_id, 0x9c470c); // mutation',
     'cx06_12 play-off fate text'),
    ('cx06_12_limite_13',
     '    if ((now as u8) as i8) < 0xe { r.wrapping_sub(1) as i32 } else { r as i32 }',
     '    if ((now as u8) as i8) < 0xd { r.wrapping_sub(1) as i32 } else { r as i32 } // mutation',
     'cx06_12 status change'),
]
