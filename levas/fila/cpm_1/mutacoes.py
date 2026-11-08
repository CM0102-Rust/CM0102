# Mutacoes (com o ramo de cada uma). O jogo de copa decidido na prorrogacao a favor do mandante dado ao visitante (o
# comparando acusa pelo lado e pelo codigo; a carreira pelo time que segue na copa). O confronto empatado no agregado
# dado ao primeiro lado, sem W.O., gols fora nem penalties (idem, nos confrontos de ida e volta das copas europeias e
# nacionais). A ordem dos cabecas de chave invertida (o comparando acusa pelo valor; a carreira pelo sorteio da rodada:
# poucas chamadas da 0x51d2d0/0x51d9f0 ate 2002-06-01).
MUTACOES = [
    ('cpm_jogo_prorrogacao', 'return Some(1); // o mandante na prorrogação',
     'return Some(2); // mutacao: o visitante',
     'cpm_1 match in extra time'),
    ('cpm_confronto_agregado', 'if tot1 > tot2 {',
     'if tot1 >= tot2 { // mutacao: o empate no agregado para o primeiro lado',
     'cpm_1 tie level on aggregate'),
    ('cpm_ordem_sorteio', 'cpm1_b(b + 6) as i32 - cpm1_b(a + 6) as i32 // o maior primeiro',
     'cpm1_b(a + 6) as i32 - cpm1_b(b + 6) as i32 // mutacao: o menor primeiro',
     'cpm_1 seeding order'),
]
