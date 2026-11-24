# Mutacoes (sx03_2). Os premios dao reputacao (e confianca da diretoria aos tecnicos): a carreira ENG ate 2002-06-01
# acusa pelo banco de dados.
# - o jovem do ano ate 23 em vez de 21 (5 de abril de 2002): outro vencedor;
# - sem o tecnico do ano (30 de maio de 2002);
# - sem o tecnico do mes (todo mes);
# - o jovem do mes sem limite de idade: o jogador do mes ganha duas vezes;
# - a equipe da semana so na Premier: as nomeacoes das outras divisoes mudam [duvida: pode escapar da carreira,
#   se a equipe da semana nao chega ao banco; o comparador e so a original].
MUTACOES = [
    ('sx03_2_jovem_do_ano_23',
     'nation_award(this, 1), [0x15, 0, 0x11, -1, -1, -1, -1]); // sx03_2: young player up to 21',
     'nation_award(this, 1), [0x17, 0, 0x11, -1, -1, -1, -1]); // mutation: young player up to 23',
     'sx03_2 england year awards'),
    ('sx03_2_sem_tecnico_do_ano',
     'award_manager_of_year(this, today, nation_award(this, 6 + 6 * k), *key); // sx03_2: manager of the year',
     'let _ = (k, key); // mutation: no manager of the year',
     'sx03_2 england managers of the year'),
    ('sx03_2_sem_tecnico_do_mes',
     'award_month_manager(nation_award(this, 3 + 6 * k), *key); // sx03_2: manager of the month',
     'let _ = (k, key); // mutation: no manager of the month',
     'sx03_2 england month players'),
    ('sx03_2_jovem_do_mes_sem_idade',
     'award_month_player(this, nation_award(this, 5 + 6 * k), 0x14); // sx03_2: young player of the month',
     'award_month_player(this, nation_award(this, 5 + 6 * k), -1); // mutation: no age limit',
     'sx03_2 england month players'),
    ('sx03_2_equipe_da_semana_so_premier',
     'let team_award = [2, 8, 14, 20].iter().any(|&k| nation_award(this, k) == award) // sx03_2: the division team awards',
     'let team_award = [2].iter().any(|&k| nation_award(this, k) == award) // mutation: only the Premier',
     'sx03_2 england team of the week'),
]
