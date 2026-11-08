# Mutacoes (cpv_5): sem o sorteio do merchandising (um sorteio a menos por clube de liga, toda semana), sem o segundo
# sorteio da loja nos ingressos da temporada (um a menos por clube que ainda vende ingressos, toda semana da
# pre-temporada) e a receita dos ingressos com o aplicar invertido (milhoes no saldo dos clubes: a band das financas
# 0x5986b0 muda e com ela os sorteios da divida 0x59dd40 e do clima 0x923630). As duas primeiras mudam a contagem dos
# sorteios; a terceira pode tardar a aparecer se a band de nenhum clube mudar (o saldo e lido pelas transferencias).
MUTACOES = [
    ('cpv5_merchandising_sem_sorteio', '    let r = cpv5_sortear(0xb); // cpv5: o sorteio do merchandising',
     '    let r = 5i32; // mutacao: sem o sorteio',
     'cpv_5 merchandising'),
    ('cpv5_loja_sem_sorteio', '    let r2 = cpv5_sortear(rep); // cpv5: o segundo sorteio da loja',
     '    let r2 = rep / 2; // mutacao: sem o sorteio',
     'cpv_5 tickets sold'),
    ('cpv5_receita_invertida', '    if aplicar != 0 { // cpv5: a receita dos ingressos',
     '    if aplicar == 0 { // mutacao: a receita invertida',
     'cpv_5 tickets sold'),
]
