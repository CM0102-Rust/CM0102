# Mutacoes (cpv_6), nos eventos financeiros da semana (toda virada de semana, para todo clube de liga com injury_word_0c de
# 5000 ou nacao ativa): sem o sorteio(7000) da divida (um sorteio a menos por clube pobre ou de faixa 0 com saldo baixo,
# toda semana), o filtro da injecao de dinheiro em 7500 em vez de 4750 (os clubes de injury_word_0c 4750 a 7499 deixam de
# sortear a injecao toda semana) e sem o impulso das financas (0x59e050, que sorteia) na faixa 0 (a maioria dos clubes).
# As tres mudam a contagem dos sorteios e tudo o que sorteia depois.
MUTACOES = [
    ('cpv6_divida_sem_sorteio', '    let r = cpv6_sortear(7000); // cpv6: o sorteio da dívida',
     '    let r = 3500i32; // mutacao: sem o sorteio',
     'cpv_6 sorteio da divida'),
    ('cpv6_filtro_da_injecao', '    if cpv6_w(clube + 0x80) < 0x128e { // cpv6: o filtro da injeção',
     '    if cpv6_w(clube + 0x80) < 0x1d4c { // mutacao: o filtro em 7500',
     'cpv_6 injecao avaliada'),
    ('cpv6_sem_impulso', '            let ok = run_weekly_event(0x59e050, this, clube, fin) != 0; // cpv6: o impulso na faixa 0',
     '            let ok = false; // mutacao: sem o impulso',
     'cpv_6 impulso na faixa 0'),
]
