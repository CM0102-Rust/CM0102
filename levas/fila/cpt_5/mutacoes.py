# Mutacoes (cpt_5), todas no tempo das cidades (0x922ee0, todo dia pelo laco do dia do jogo, 3 ou 4 sorteios por
# cidade): duas medidas em vez de tres (um sorteio a menos por cidade por dia), o vento novo duas vezes em dez (mais
# sorteios de vento) e o vento novo sem o sorteio (um sorteio a menos por vento novo). As tres mudam a contagem de
# sorteios do dia, e tudo o que sorteia depois muda.
MUTACOES = [
    ('cpt5_duas_medidas', '        for k in 0..3usize { // cpt5: as três medidas',
     '        for k in 0..2usize { // mutacao: duas medidas',
     'cpt_5 weather of the day'),
    ('cpt5_vento_mais_vezes', '        if cpt5_sortear(10) == 0 { // cpt5: um em dez muda o vento',
     '        if cpt5_sortear(10) <= 1 { // mutacao: dois em dez',
     'cpt_5 weather of the day'),
    ('cpt5_vento_sem_sorteio', '            let r = cpt5_sortear(3); // cpt5: o vento novo',
     '            let r = 1; // mutacao: sem o sorteio',
     'cpt_5 new wind'),
]
