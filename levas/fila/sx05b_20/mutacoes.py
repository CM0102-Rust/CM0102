# Mutacoes (sx05b_20), pela carreira ENG ate 2002-08-01 (os premios da Copa de 2002 em 13 de julho sobem reputacoes).
# - a chuteira de ouro pela estatistica 0x11 (as notas, nao os gols);
# - os premios em 20 de julho;
# - a Copa em anos de resto 1 (2002 fica sem premios).
MUTACOES = [
    ('sx05b_20_chuteira_por_nota',
     '    player(this, cup, today, nation_award(this, 0), -1, 0, 2);',
     '    player(this, cup, today, nation_award(this, 0), -1, 0, 0x11); // mutation',
     'sx05b_20 world cup awards day'),
    ('sx05b_20_dia_20',
     '    exe::date_date_549f70(&mut d, 0xd, 6, year as u32, 0);',
     '    exe::date_date_549f70(&mut d, 0x14, 6, year as u32, 0); // mutation'),
    ('sx05b_20_resto_um',
     '    if (year as i32 - 0x7ce) % 4 != 0 {',
     '    if (year as i32 - 0x7ce) % 4 != 1 { // mutation'),
]
