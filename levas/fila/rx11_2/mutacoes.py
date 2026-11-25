# Mutacoes (rx11_2), ENG ate 2005-12-31 (a campanha de 2006); os registros so rodam na original no comparador, entao a
# carreira acusa (o molde do cx17_2):
# - a eliminada de um grupo africano segue convocada (as convocacoes mudam);
# - o vencedor da repescagem africana nao entra na Copa (a Copa de 2006 fica com uma selecao a menos no sorteio de
#   dezembro de 2005);
# - o classificado do grupo final da CONCACAF nao entra na Copa (idem).
# As honras da temporada ficam sem mutacao propria: trocar a competicao deixa seleções presas a uma campanha encerrada,
# o que so aparece nas convocacoes da edicao seguinte (depois da data).
MUTACOES = [
    ('rx11_2_africana_eliminada_segue',
     "clear_national_team_competition(Club(club)); // rx11_2: the African group's eliminated nation ends its campaign",
     "let _ = club; // mutation: the eliminated nation stays in the campaign",
     'rx11_2 african group records'),
    ('rx11_2_repescagem_africana_fora_da_copa',
     'qualify_nation_for_world_cup(club, out, 0x37); // rx11_2: the African play-off winner to the World Cup',
     'european_cup_add_qualified_record(club, comp, 0, 0, 0x37); // mutation: no World Cup place',
     'rx11_2 african play-off records'),
    ('rx11_2_concacaf_fora_da_copa',
     'qualify_nation_for_world_cup(club, out, 0x64); // rx11_2: the CONCACAF final group qualifier',
     'european_cup_add_qualified_record(club, comp, 0, 0, 0x64); // mutation: no World Cup place',
     'rx11_2 concacaf final group records'),
]
