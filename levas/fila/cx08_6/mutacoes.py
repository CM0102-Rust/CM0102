# Mutacoes (cx08_6), com a Argentina ate 2002-08-01:
# - o clube da Segunda marcado 5 nao sobe (so o campeao): as divisoes mudam na virada (a carreira);
# - so tres clubes descem da Segunda para a Terceira (e tres sobem): as divisoes mudam na virada (a carreira);
# - a noticia do rebaixado por promedio usa o texto comum de rebaixamento (0x99b540): o comparador de texto acusa com o
#   modo CM_TEXTO_NOTICIAS.
MUTACOES = [
    ('cx08_6_status_5_nao_sobe',
     '            0 | 5 => exe::league_move_club_to_division(this, club, this.competition().addr(), 0),',
     '            0 => exe::league_move_club_to_division(this, club, this.competition().addr(), 0), // mutation',
     'cx08_6 argentine chain'),
    ('cx08_6_so_tres_descem',
     '    while k < second.club_count() as i32 && moved < 4 {',
     '    while k < second.club_count() as i32 && moved < 3 { // mutation',
     'cx08_6 argentine second and third swap'),
    ('cx08_6_texto_comum',
     '    relegated_by_average_text(this, club_id, 0x987420, text);',
     '    relegated_by_average_text(this, club_id, 0x99b540, text); // mutation',
     'cx08_6 argentine relegated by average text'),
]
