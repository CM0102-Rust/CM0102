# Mutacoes (cx17_2), ENG ate 2005-12-31 (a campanha sul-americana de 2004, que acaba em 15 de novembro de 2005):
# - cinco vagas em vez de seis (a selecao em 5o lugar sai das vagas): acusada pelos dois comparadores lado a lado;
# - a classificada nao entra na Copa do Mundo: a Copa de 2006 fica com uma selecao a menos no sorteio de dezembro de 2005
#   (no comparador os registros so rodam na original: a carreira acusa) [inferencia: a data pelo
#   world_cup_fields::SEEDING_DAY, 15/12 do ano anterior a Copa];
# - a eliminada segue convocada (o molde da cqj10): a carreira acusa.
MUTACOES = [
    ('cx17_2_cinco_vagas',
     '6 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i32 // cx17_2: six places less the exempt',
     '5 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i32 // mutation: five places',
     'cx17_2 south american qualification status'),
    ('cx17_2_classificada_fora_da_copa',
     'exe::worldcup_add_qualified_team(world_cup, club); // cx17_2: the qualified nation enters the World Cup',
     'let _ = world_cup; // mutation: the qualified nation stays out of the World Cup',
     'cx17_2 south american to the world cup'),
    ('cx17_2_eliminada_segue_convocada',
     'clear_national_team_competition(Club(club)); // cx17_2: the eliminated nation ends its campaign',
     'let _ = club; // mutation: the eliminated nation stays in the campaign',
     'cx17_2 south american out'),
]
