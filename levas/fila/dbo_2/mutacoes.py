# Mutacoes do olheiro (esforcados, goleiro, meias que roubam): as tres invertem a resposta. Mesma lacuna do dbo_1:
# so o relatorio de um clube (run_scout_day 0x7b3e20, busca do tipo 4 com alvo) as chama, e a resposta so vai para a
# noticia; PODEM ESCAPAR da carreira ate 2002-06-01; os comparadores rodam no jogo real comparando.
MUTACOES = [
    ('dbo_esforcados', '    (n > 4) as u8 // esforcados (dbo)', '    (n <= 4) as u8 // mutacao: a resposta invertida (esforcados)'),
    ('dbo_goleiro', '            return 1; // goleiro em condicoes (dbo)', '            return 0; // mutacao: nunca acha o goleiro'),
    ('dbo_meias_que_roubam', '    (n > 1) as u8 // meias que roubam (dbo)', '    (n <= 1) as u8 // mutacao: a resposta invertida (meias que roubam)'),
]
