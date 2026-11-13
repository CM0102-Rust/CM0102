# Mutacoes (mwg_2), na contagem e na completacao dos elencos com jogadores virtuais (toda partida, os dois lados):
# - o lesionado grave conta como disponivel: os grupos com lesionados deixam de ganhar virtuais (outras escalacoes);
# - a semente dos virtuais muda (10001 em vez de 10000): outros jogadores virtuais (outras notas, outros resultados);
# - os clubes de liga selecionada ganham virtuais adultos, sem a perda de capacidade dos jovens.
MUTACOES = [
    ('mwg2_lesionado_disponivel',
     'continue; // mwg2: the injured are not available',
     '// mutation: the injured count as available',
     'mwg_2 injured player'),
    ('mwg2_outra_semente',
     'let seed_g = (before as i8 as i32 + 10000)',
     'let seed_g = (before as i8 as i32 + 10001)',
     'mwg_2 group below its minimum'),
    ('mwg2_virtuais_adultos',
     "youngsters = 1; // mwg2: a selected league's club gets young virtual players",
     "youngsters = 0; // mutation: a selected league's club gets grown-up virtual players",
     'mwg_2 selected league club'),
]
