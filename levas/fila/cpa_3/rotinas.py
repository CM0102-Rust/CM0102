# Bloco das competições, os auxiliares (comp_util.cpp): o tipo de confronto na liga (comp_util_get_league_clash_type
# 0x4b02e0, ~4 milhões de chamadas na carreira: a escalação automática 0x6dbc50, a mídia, as transferências, os
# recordes de clubes), a força do clube (comp_util_get_club_strength_score 0x4c0720, 1.117: a ordem por força
# 0x4c08f0, sorteios de chaves e ligas regionais) e o colocado mais recente de uma competição
# (comp_util_get_latest_placed_club 0x4afd80, 277: as vagas de copa e continentais; entra no lugar da 0x4b4420
# write_cup_win_or_defeat_news, que grava o texto num objeto de notícia por Text::set_string 0x4ae660 e
# Text::add_link 0x4ae8a0, que alocam e têm SEH: a comparação lado a lado duplicaria as alocações e os ponteiros
# nunca bateriam). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem
# SEH, sem sorteio próprio. Locais sem valor que a original passa adiante sem que sejam lidas: o número de jogadores
# da 0x4c0720 (dword com só o byte de baixo gravado; a 0x6e2000 só lê o byte) e a chave da 0x4afd80 (só +4 e +8
# gravados; a 0x540780 só lê esses). Caixa: só a da 0x4afd80 com lugar acima de 2 (fica com a original).
# Comparadores: 0x4b02e0 e 0x4c0720 pelo valor (só leem; a 0x4b02e0 em laço quentíssimo, sem regiões), 0x4afd80 pelo
# histórico de competições (ela o reordena). Em laço quente: a 0x4b02e0 (4 milhões, na escalação de cada partida);
# corre o risco da 0x547770 (233ª), de o desvio mudar o resíduo de pilha que uma original lê depois.
SAVE = False
ROTINAS = [('tipo_de_confronto_na_liga', 0x4b02e0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('forca_do_clube', 0x4c0720, [0x81, 0xec, 0xf0, 0x01, 0x00, 0x00]),
           ('clube_colocado_mais_recente', 0x4afd80, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00])]
DOCS = ('os auxiliares de competição: o tipo de confronto na liga, a força do clube e o colocado mais recente',
        'the competition helpers: the league clash type, the club strength score and the latest placed club',
        [('tipo_de_confronto_na_liga', 'comp_util_get_league_clash_type', '0x4b02e0', 'comp_util.cpp', 'cdecl',
          '1 = briga pelo título, 3 = briga na zona, 0 = nada, entre dois clubes da mesma liga, pelas notas ponderadas da estatística (0x4a4850) contra um terço dos clubes ou 0,2 × clubes + +0xbe (+ +0xbf)',
          '1 = title clash, 3 = zone clash, 0 = none, between two clubs of the same league, from the weighted statistics ratings (0x4a4850) against a third of the clubs or 0.2 × clubs + +0xbe (+ +0xbf)'),
         ('forca_do_clube', 'comp_util_get_club_strength_score', '0x4c0720', 'comp_util.cpp', 'cdecl',
          'dinheiro (+0x65 / 400 ou saldo / 300) + 16 × reputação + as notas fora da partida (0x6e6340) dos 15 escalados (0x6e2000) do elenco; clube nulo: −10.000.000',
          'money (+0x65 / 400 or balance / 300) + 16 × reputation + the off-match ratings (0x6e6340) of the 15 picked (0x6e2000) from the squad; null club: −10,000,000'),
         ('clube_colocado_mais_recente', 'comp_util_get_latest_placed_club', '0x4afd80', 'comp_util.cpp', 'cdecl',
          'ordena o histórico de competições e devolve o campeão, o vice ou o terceiro (+0xa, +0xe, +0x12) da primeira linha da competição; não achou: 0',
          'sorts the competition history and returns the winner, runner-up or third (+0xa, +0xe, +0x12) of the first row of the competition; not found: 0')])
CAMPOS = [('TClub +0x57 divisão; lê. Liga ([0xadadfc][id]) +8 estatística, +0x30 fases, +0x3e clubes, +0x43, +0xbe, +0xbf, virtual +0x98; lê. Entrada da estatística +4 jogos, +0x72..+0x75; lê (tipo_de_confronto_na_liga)',
           'TClub +0x57 division; reads. League ([0xadadfc][id]) +8 stats, +0x30 stages, +0x3e clubs, +0x43, +0xbe, +0xbf, virtual +0x98; reads. Stats entry +4 played, +0x72..+0x75; reads (tipo_de_confronto_na_liga)'),
          ('TClub +0x65 dinheiro, +0x80 reputação; [0xae2a38] finanças (saldo de 64 bits); lê (forca_do_clube)',
           'TClub +0x65 cash, +0x80 reputation; [0xae2a38] finances (64-bit balance); reads (forca_do_clube)'),
          ('[0xae23d4] / [0xae237c] histórico de competições (26 bytes); reordena e lê (clube_colocado_mais_recente)',
           '[0xae23d4] / [0xae237c] competition history (26 bytes); re-sorts and reads (clube_colocado_mais_recente)')]
INTERESSA = ['SGE', 'CMRTE']
