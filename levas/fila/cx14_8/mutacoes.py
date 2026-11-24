# Mutacoes (cx14_8), com a Suecia ate 2002-08-01:
# - o vencedor do mata-mata com o status 0 em vez de 1 (o mata-mata de novembro de 2001 marca o perdedor como vencedor:
#   muda quem sobe para a Allsvenskan de 2002; acusada tambem pelo comparador dos registros);
# - o progresso do mata-mata com 2 x codigo em vez de 2 x codigo - 1 (acusada pelo comparador, lado a lado).
# O texto da noticia fica sem mutacao (so com clube humano no mata-mata): so o comparador o prova, se rodar.
MUTACOES = [
    ('cx14_8_vencedor_status_0',
     "winner: 1, // cx14_8: the play-off winner's status",
     'winner: 0, // mutation: status 0',
     'cx14_8 superettan records'),
    ('cx14_8_progresso_dobro',
     'let twice = |v: u8| v.wrapping_mul(2).wrapping_sub(1); // cx14_8: 2 x code - 1',
     'let twice = |v: u8| v.wrapping_mul(2); // mutation: 2 x code',
     'cx14_8 superettan play-off progress'),
]
