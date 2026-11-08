# Mutacoes (cpy_6). A 0x583b10 roda a cada jogo da Liga dos Campeoes (a virtual +4; a competicao roda com os clubes
# de todas as nacoes, inclusive os ingleses, de setembro de 2001 a marco de 2002 na fase de grupos). Na fase de
# grupos (+0x42 de 0 a 11) ela paga 200 mil ao vencedor e 100 mil a cada um no empate, no saldo do clube (0x5999a0),
# que a carreira le depois (verba de transferencias, propostas, salarios, diretoria). As tres mexem nesse dinheiro:
# a vitoria paga a metade; o mandante nao recebe no empate; o premio da vitoria vai para o perdedor. Os contadores
# ficam colados na soma, depois das condicoes (fase de grupos e o placar).
MUTACOES = [
    ('cpy6_vitoria_pela_metade',
     'european_cup_add_prize_money(european_cup_club_finance(vencedor), 0x30d40); // cpy6: o prêmio da vitória',
     'european_cup_add_prize_money(european_cup_club_finance(vencedor), 0x186a0); // mutacao: a vitoria paga a metade',
     'cpy_6 premio da vitoria'),
    ('cpy6_empate_sem_premio_do_mandante',
     'european_cup_add_prize_money(european_cup_club_finance(cpy6_d(jogo + 0x1c)), 0x186a0); // cpy6: o prêmio do empate do mandante',
     'let _ = jogo; // mutacao: o mandante nao recebe no empate',
     'cpy_6 premio do empate'),
    ('cpy6_premio_ao_perdedor',
     'european_cup_add_prize_money(european_cup_club_finance(vencedor), 0x30d40); // cpy6: o prêmio da vitória',
     'european_cup_add_prize_money(european_cup_club_finance(perdedor), 0x30d40); // mutacao: o premio vai para o perdedor',
     'cpy_6 premio da vitoria'),
]
