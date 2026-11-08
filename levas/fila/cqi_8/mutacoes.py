# Mutacoes (cqi_8): as regras dos estaduais, gravadas na montagem da carreira e [inferencia] na virada da temporada
# de ano civil; os estaduais jogam no primeiro semestre, antes de 2002-06-01. A tabela muda (pontos, posicoes,
# destinos) e a memoria do jogo (impressao diaria) com ela.
# - Centro-Oeste: a vitoria vale 2 pontos.
# - Gaucho (e Nordeste, Paulista): o empate vale 0.
# - Catarinense (e Baiano): a vitoria vale 2. (O desempate confronto direto -> saldo escapou na carreira de 8 ligas
#   ate 2002-06-01, 04/11: nos dois estaduais os empatados em pontos ficaram na mesma ordem pelos dois criterios.)
MUTACOES = [
    ('cqi8_vitoria_centro', 'bra_reg_central_set_state_league_rules(o, 1, 8, 3); // cqi8: o Centro-Oeste',
     'bra_reg_central_set_state_league_rules(o, 1, 8, 3); cqi8_b(o, 0xc2, 2); // mutacao: o Centro-Oeste', 'cqi_8 regras do centro-oeste'),
    ('cqi8_empate_gaucho', 'bra_reg_central_set_state_league_rules(o, 1, 4, 2); // cqi8: o Gaúcho',
     'bra_reg_central_set_state_league_rules(o, 1, 4, 2); cqi8_b(o, 0xc3, 0); // mutacao: o Gaúcho', 'cqi_8 regras do gaucho'),
    ('cqi8_vitoria_catarinense', 'bra_reg_central_set_state_league_rules(o, 2, 4, 1); // cqi8: o Catarinense',
     'bra_reg_central_set_state_league_rules(o, 2, 4, 1); cqi8_b(o, 0xc2, 2); // mutacao: o Catarinense', 'cqi_8 regras do catarinense'),
]
