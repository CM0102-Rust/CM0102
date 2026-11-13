# Mutacoes (mwd_14): na Segunda Divisao turca sobem 2 em vez de 3: as situacoes da tabela e o acesso de 2002 mudam.
MUTACOES = [
    ('mwd14_turca_sobem_2',
     'turkish_league_rules(3, read_usize(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) as u32, read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B) as u32) // mwd14: three go up',
     'turkish_league_rules(2, read_usize(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) as u32, read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B) as u32) // mutation: two go up',
     'mwd_14 turkish second division rules'),
]
