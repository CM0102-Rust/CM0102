# Mutações (rx07_1), com a Itália e a Espanha até 2002-08-01. As duas mexem no plano que o comparador também usa, então
# a comparação acusa mesmo quando a carreira não muda:
# - os contratos começados no mesmo ano da data de hoje nunca disparam a cláusula de não acesso (as contratações de
#   janeiro a abril de 2002 nos clubes que não subiram);
# - a cláusula de rebaixamento pendente não é apagada quando o clube sobe (e pode disparar depois).
MUTACOES = [
    ('rx07_1_mesmo_ano_nao_dispara',
     '    year > due_year || (year == due_year && day >= due_day)',
     '    year > due_year // mutation',
     'rx07_1 non-promotion clause'),
    ('rx07_1_rebaixamento_fica',
     '    main_contracts_of_club_and_b_team(this, club).into_iter().map(|c| (c, clear(c.relegation_rc()), clear(c.non_promotion_rc()))).collect()',
     '    main_contracts_of_club_and_b_team(this, club).into_iter().map(|c| (c, c.relegation_rc(), clear(c.non_promotion_rc()))).collect() // mutation',
     'rx07_1 division clauses reset'),
]
