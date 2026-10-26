# Mutacoes do olheiro (faltam atacantes e zagueiros pelo alto, zagueiros rapidos): as tres invertem a resposta.
# Mesma lacuna do dbo_1: so o relatorio de um clube (run_scout_day 0x7b3e20) as chama e a resposta so vai para a
# noticia; PODEM ESCAPAR da carreira ate 2002-06-01; os comparadores rodam no jogo real comparando.
MUTACOES = [
    ('dbo_atacantes_pelo_alto', '<= 1) as u8 // atacantes pelo alto (dbo)', '> 1) as u8 // mutacao: a resposta invertida (atacantes pelo alto)'),
    ('dbo_zagueiros_pelo_alto', '<= 1) as u8 // zagueiros pelo alto (dbo)', '> 1) as u8 // mutacao: a resposta invertida (zagueiros pelo alto)'),
    ('dbo_zagueiros_rapidos', '<= 1) as u8 // zagueiros rapidos (dbo)', '> 1) as u8 // mutacao: a resposta invertida (zagueiros rapidos)'),
]
