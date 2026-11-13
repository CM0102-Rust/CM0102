# Mutacoes (mwg_5), nas notas do ano e na virada do mes:
# - o vermelho conta tres vezes nos cartoes do ano (o comparador de add_player_rating pega);
# - o peso da competicao fica entre 1 e 15 (outras somas: o comparador pega; os premios de jogador do ano saem delas);
# - as notas do mes nao viram: o mes acumula com os anteriores (outro jogador do mes, outra reputacao: a carreira).
MUTACOES = [
    ('mwg5_vermelho_conta_tres',
     '+ 2 * read_u8(row + match_record_row::RED_CARD) as u16); // mwg5: a red card counts twice',
     '+ 3 * read_u8(row + match_record_row::RED_CARD) as u16); // mutation: a red card counts three times',
     'mwg_5 new player in the year'),
    ('mwg5_peso_ate_15',
     'weight = weight.clamp(1, 20); // mwg5: the competition weight within 1..20',
     'weight = weight.clamp(1, 15); // mutation: the competition weight within 1..15',
     'mwg_5 new player in the year'),
    ('mwg5_mes_acumula',
     "let table = read_usize(this); // mwg5: the old month's table",
     "let table = read_usize(this); if table != 0 { write_u8(this + month_ratings_fields::MONTH, month); write_i16(this + month_ratings_fields::YEAR, year); return; } // mutation: the month keeps the old ratings",
     'mwg_5 month reset'),
]
