# Mutacoes (cqb_3). As tres nas datas internacionais, montadas uma vez na criacao do jogo: em 2001 (de agosto a
# dezembro) os dias dos amistosos de selecoes vem das datas de ano +3 (2005, que a 0x5cc510 traz para 2001); a 0x5cbb00
# (cpy_4) marca os amistosos 12 semanas antes de cada data (sorteios, jogos novos, selecoes convocadas), e a carreira
# muda. Ano somado zerado: todas as datas em 2002, nenhuma no segundo semestre de 2001. Sem os 4 anos para tras: as
# datas de ano +3 ficam em 2005. O ano de hoje no lugar do da Copa: as datas de 2001 viram as de ano +0 (outros dias).
# Os contadores rodam uma vez (o da tabela, o do ano da Copa) ou uma vez por data de ano +3 (o dos 4 anos para tras).
MUTACOES = [
    ('cqb3_ano_zerado',
     '*((e + 0xb) as *mut u8) = ano; // cqb3: o ano a somar ao da Copa',
     '*((e + 0xb) as *mut u8) = 0 * ano; // mutacao: o ano a somar zerado',
     'cqb_3 fixture list built'),
    ('cqb3_sem_ano_para_tras',
     'ano = ano.wrapping_add(0xfffc); // cqb3: menos 4 se pass_ball de 3 anos à frente',
     'ano = ano.wrapping_add(0); // mutacao: sem os 4 anos para tras',
     'cqb_3 year backwards'),
    ('cqb3_ciclo_de_hoje',
     'ciclo = ciclo.wrapping_add(1); // cqb3: o ano da Copa',
     'break; // mutacao: o ano de hoje no lugar do da Copa',
     'cqb_3 cup year ahead'),
]
