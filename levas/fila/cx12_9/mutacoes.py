# Mutacoes (cx12_9), com a Irlanda (do Norte) ate 2002-08-01 (a repescagem em maio de 2002 e a virada no verao):
# - o clube da Premier que perde a repescagem fica (situacao 0xfe no lugar de 3): ninguem cai e a virada de 2002 monta
#   outra Premier e outra Primeira; acusada tambem pelo comparador dos registros (a situacao da linha);
# - o vencedor da repescagem que vem da Primeira recebe o texto de rebaixado: o texto nao volta a simulacao; acusa o
#   comparador do texto, com o modo CM_TEXTO_NOTICIAS e um tecnico humano como destinatario;
# - o clube da Primeira na repescagem sem dobrar os codigos: a carreira quase nao le a previsao; acusa o comparador do
#   progresso, lado a lado nos 3 bytes.
MUTACOES = [
    ('cx12_9_perdedor_da_premier_fica',
     'plan(FirstDivisionRecord::Relegation(record), row, Some(3), premier_id) // cx12_9: the Premier club goes down',
     'plan(FirstDivisionRecord::Relegation(record), row, Some(0xfe), premier_id) // mutation: it stays',
     'cx12_9 relegation record'),
    ('cx12_9_promovido_rebaixado',
     '(1, 0, false) => Some(PLAY_OFF_WIN_PROMOTION_TEXT), // cx12_9: the First Division club goes up',
     '(1, 0, false) => Some(PLAY_OFF_RELEGATED_TEXT), // mutation: the relegated text',
     'cx12_9 play-off fate text'),
    ('cx12_9_codigos_sem_dobrar',
     '(c, b, w) = (c << 1, b << 1, w << 1); // cx12_9: the First Division club doubles its codes',
     '(c, b, w) = (c, b, w); // mutation: not doubled',
     'cx12_9 first division club play-off progress'),
]
