# Mutacoes (cx17_5), ENG ate 2002-08-01 (a Copa do Mundo de 2002, de 31 de maio a 30 de junho, e a virada dela):
# - quem cai nos grupos da Copa segue convocado (o molde da cqj10): a carreira acusa (os registros so rodam na original
#   no comparador);
# - o terceiro de um grupo com o codigo 25 no lugar de 17: o comparador lado a lado acusa.
# A virada (start_new_season) fica sem mutacao propria: o que ela monta (as seis secoes, a lista e as regras de 2006) so e
# usado nas edicoes seguintes, depois da data da cadeia; o comparador confere o plano (o ano + 4, a fase, a tabela).
MUTACOES = [
    ('cx17_5_eliminado_nos_grupos_segue_convocado',
     'clear_national_team_competition(Club(club)); // cx17_5: the group exit ends the campaign',
     'let _ = club; // mutation: the group exit stays in the campaign',
     'cx17_5 world cup group exit'),
    ('cx17_5_terceiro_25',
     '3 => 0x11, // cx17_5: the third of a group',
     '3 => 0x19, // mutation: the third as the fourth',
     'cx17_5 world cup progress'),
]
