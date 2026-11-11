# Mutacoes (mkb_1), no comprador da IA revendo o lance depois da contraproposta (0x8c0810; 13 chamadas na carreira
# automatica ate 2002-06-01, docs/cobertura_simulacao.txt, 17/10):
# - o comprador aceita pagar ate o dobro do valor dele (antes, o valor): contrapropostas que eram retiradas viram
#   lances refeitos, quem assina e quanto se paga mudam (contador "mkb_1 buyer walks away");
# - a verba de transferencias tem de cobrir o dobro do total (antes, o total): lances refeitos viram retiradas
#   (contador "mkb_1 over budget");
# - o lance que sempre deixa o jogador inquieto passa de 17,5 milhoes para 1,75 milhao: mais contratos com a marca
#   0x800000, o jogador pede para sair (contador "mkb_1 player hears of the bid").
MUTACOES = [
    ('mkb1_paga_o_dobro',
     '    if w > value {',
     '    if w > value.wrapping_mul(2) { // mutation: up to twice the value',
     'mkb_1 buyer walks away'),
    ('mkb1_inquieto_com_menos',
     'let unsettle = w > 17_500_000 // mkb1: the fee that always unsettles',
     'let unsettle = w > 1_750_000 // mutation: 1.75 million',
     'mkb_1 player hears of the bid'),
]
