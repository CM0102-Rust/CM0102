# Mutacoes (cqi_9): as regras dos estaduais, gravadas na montagem da carreira e [inferencia] na virada da temporada
# de ano civil; os estaduais jogam no primeiro semestre, antes de 2002-06-01. A tabela muda (pontos, posicoes,
# destinos) e a memoria do jogo (impressao diaria) com ela.
# - Paranaense: a vitoria vale 2 pontos.
# - Pernambucano (e Norte): o empate vale 0.
# - Carioca (e Mineiro): o primeiro desempate passa do confronto direto (3) para o saldo (1).
MUTACOES = [
    ('cqi9_vitoria_parana', 'bra_reg_parana_set_state_league_rules(o, 0, 4); // cqi9: o Paranaense',
     'bra_reg_parana_set_state_league_rules(o, 0, 4); cqi9_b(o, 0xc2, 2); // mutacao: o Paranaense', 'cqi_9 paranaense rules'),
    ('cqi9_empate_pernambucano', 'bra_reg_parana_set_state_league_rules(o, 4, 2); // cqi9: o Pernambucano',
     'bra_reg_parana_set_state_league_rules(o, 4, 2); cqi9_b(o, 0xc3, 0); // mutacao: o Pernambucano', 'cqi_9 pernambucano rules'),
    ('cqi9_desempate_carioca', 'bra_reg_parana_set_state_league_rules(o, 0, 2); // cqi9: o Carioca',
     'bra_reg_parana_set_state_league_rules(o, 0, 2); cqi9_b(o, 0xc5, 1); // mutacao: o Carioca', 'cqi_9 carioca rules'),
]
