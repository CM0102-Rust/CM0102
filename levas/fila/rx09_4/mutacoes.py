# Mutacoes (rx09_4), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL:
# - a final da repescagem com os vencedores (status 1) em vez dos perdedores (status 2): outros clubes jogam a final e
#   caem (junho de 2002, a virada muda);
# - o vencedor de um grupo do acesso com a linha marcada 0xfe em vez de 0: nao sobe na virada de 2002.
# O update_for_date fica sem mutacao propria: a unica decisao dele e chamar a fase seguinte quando a repescagem
# acaba, e sem ela a final nunca e criada (a liga fica parada); a mutacao da final so e acusada porque ele roda.
MUTACOES = [
    ('rx09_4_final_dos_vencedores',
     'if read_u8(e + cup_team_entry::STATUS) == 2 { // rx09_4: the losers of the relegation play-off',
     'if read_u8(e + cup_team_entry::STATUS) == 1 { // mutation: the winners',
     'rx09_4 segunda b relegation final'),
    ('rx09_4_vencedor_marcado_fe',
     "plan.row = Some((row, 0)); // rx09_4: the play-off group winner's row",
     'plan.row = Some((row, 0xfe)); // mutation: marked out',
     'rx09_4 segunda b row status'),
]
