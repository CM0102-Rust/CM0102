# Mutacoes (mwg_8) (os comparadores de cada rotina pegam; na carreira, os recordes dos clubes e as clausulas):
# - "mais gols" passa a valer com o empate nos totais (>=);
# - "cerca de N semanas" arredonda para cima ((dias + 3) / 7);
# - a clausula de jogos do emprestimo pede 11 jogos.
MUTACOES = [
    ('mwg8_mais_gols_com_empate',
     '(a_total > b_total) as u32 // mwg8: more goals',
     '(a_total >= b_total) as u32 // mutation: a draw counts',
     'mwg_8 goals compared'),
    ('mwg8_semanas_para_cima',
     'write(out, ru::ABOUT_N_WEEKS, days / 7), // mwg8: about N weeks',
     'write(out, ru::ABOUT_N_WEEKS, (days + 3) / 7), // mutation: rounded up',
     'mwg_8 approximate duration'),
    ('mwg8_clausula_11_jogos',
     "write_u8(this + offer::APPEARANCE_GAMES, 10); // mwg8: the appearance clause's games",
     "write_u8(this + offer::APPEARANCE_GAMES, 11); // mutation: 11 games",
     'mwg_8 loan offer'),
]
