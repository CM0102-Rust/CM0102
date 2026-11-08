# Mutacoes (cqd_7), na arvore dos confrontos da copa (0x518790, 22 chamadas ate 2002-06-01, na criacao das copas):
# - a moeda do lado na rodada seguinte invertida: o vencedor de cada confronto vai ao outro lado da vaga (mandante e
#   visitante trocados nas rodadas seguintes das copas);
# - a moeda do lado na primeira rodada invertida: os times das copas entram no outro lado (os mandos da primeira
#   rodada mudam);
# - a vaga inicial na rodada seguinte uma adiante (pelo par ou sorteada): outros cruzamentos nas rodadas seguintes.
# Os contadores ficam colados no efeito: a vaga gravada na rodada seguinte e o time gravado no confronto, depois de
# todas as condicoes.
MUTACOES = [
    ('cqd7_moeda_da_rodada_seguinte',
     'let primeiro_a = moeda != 0; // cqd7: a moeda do lado na rodada seguinte',
     'let primeiro_a = moeda == 0; // mutacao: a moeda invertida',
     'cqd_7 place in the next round'),
    ('cqd7_moeda_da_primeira_rodada',
     'let primeiro_0 = moeda != 0; // cqd7: a moeda do lado na primeira rodada',
     'let primeiro_0 = moeda == 0; // mutacao: a moeda invertida',
     'cqd_7 team in the first round'),
    ('cqd7_vaga_inicial_adiante',
     'let (mut s, inicio) = (s, inicio); // cqd7: a vaga inicial na rodada seguinte',
     'let (mut s, inicio) = { let n = (cqd7_w(comp_cup_round(this, r) + 0x82) as i32).max(1); let x = (s as i16 as i32 + 1) % n; (x, x) }; // mutacao: a vaga seguinte',
     'cqd_7 place in the next round'),
]
