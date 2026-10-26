# Mutacoes do olheiro (defensores, meias, atacantes): as tres invertem a resposta. As respostas so vao para os
# parametros da noticia do relatorio de um clube (run_scout_day 0x7b3e20, busca do tipo 4 com alvo): a carreira ate
# 2002-06-01 so acusa se algum olheiro fizer relatorio de clube nela e a noticia entrar na impressao. PODEM ESCAPAR
# da carreira (lacuna); os comparadores rodam no jogo real comparando quando um olheiro relata um clube.
MUTACOES = [
    ('dbo_defensores', '> 2) as u8 // zagueiros (dbo)', '<= 2) as u8 // mutacao: a resposta invertida (zagueiros)'),
    ('dbo_meias', '> 2) as u8 // meias (dbo)', '<= 2) as u8 // mutacao: a resposta invertida (meias)'),
    ('dbo_atacantes', '> 1) as u8 // atacantes (dbo)', '<= 1) as u8 // mutacao: a resposta invertida (atacantes)'),
]
