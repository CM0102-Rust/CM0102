# Os construtores das eliminatórias da Copa na Ásia, na Oceania e na América do Sul (wc_asia_league.cpp,
# wc_oceania_league.cpp, wc_south_american_league.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x911bf0 (WcAsiaLeague::WcAsiaLeague), 0x91df90 (WcOceaniaLeague::WcOceaniaLeague),
#   0x9202b0 (WcSouthAmericanLeague::WcSouthAmericanLeague).
# O ano da campanha: um ano depois do ciclo de 4 a partir de 2000 volta um (2001 -> 2000) e sobe até o ciclo; Ásia e
# América do Sul nunca antes de 2004 (a campanha de 2002 já estava em curso), a Oceania sem mínimo (num jogo de 2001,
# a de 2000, que joga em 2001). As seções se encadeiam: acima/abaixo (+0x1c/+0x20) os ids das seções vizinhas, lidos
# das chaves (CONCACAF/Oceania na Ásia; Ásia/América do Sul na Oceania; Oceania/África na América do Sul).
# Diferenças conferidas: a Ásia grava +0x49 = 7, +0x4a = 3 e +0xf7 = 0, 13 fases; a Oceania cria as estatísticas antes
# das previsões e das regras, depois a tabela do grupo; a América do Sul não grava a importância (+0x51), usa +0xd9 =
# 0x24, nenhuma fase, e chama a carga da campanha real de 2002 (0x9216d0) quando o ano do jogo não é anterior à edição.
# Convenções conferidas: thiscall, ret 8; as rotinas das seções por mov ecx -> thiscall, ret 0 (o retorno de 0x913f60
# e de 0x91fa80 não é lido; o de 0x920eb0 é testado no eax inteiro); 0x687b10 (push 1 -> ret 4); 0x944e46/0x944cf1
# (cdecl); 0x49ee70 (push this; mov ecx -> thiscall, ret 4).
# Comparadores: só a original (alocam e leem o save), com o plano: this, vtable, registro, ano e as seções vizinhas.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_wc_asia_league', 0x911bf0, [0x6a, 0xff, 0x68, 0x09, 0x57, 0x96, 0x00]),
           ('construct_wc_oceania_league', 0x91df90, [0x6a, 0xff, 0x68, 0xb9, 0x5a, 0x96, 0x00]),
           ('construct_wc_south_american_league', 0x9202b0, [0x6a, 0xff, 0x68, 0xa9, 0x5b, 0x96, 0x00])]
DOCS = ('os construtores das eliminatórias da Copa na Ásia, na Oceania e na América do Sul',
        'the constructors of the Asian, Oceania and South American World Cup qualifying',
        [('construct_wc_asia_league', 'WcAsiaLeague::WcAsiaLeague', '0x911bf0', 'wc_asia_league.cpp', 'thiscall, ret 8', 'regra: as eliminatórias asiáticas começam no ano do ciclo de quatro anos a partir de 2000 (o ano seguinte ao do ciclo volta um), nunca antes de 2004; entre a seção da CONCACAF e a da Oceania; 13 fases, a lista de inscritos, as regras e as estatísticas', 'rule: the Asian qualifying starts in the year of the four-year cycle from 2000 (the year after the cycle goes back one), never before 2004; between the CONCACAF and the Oceania sections; 13 stages, the entrant list, the rules and the statistics'),
         ('construct_wc_oceania_league', 'WcOceaniaLeague::WcOceaniaLeague', '0x91df90', 'wc_oceania_league.cpp', 'thiscall, ret 8', 'regra: as eliminatórias da Oceania começam no ano do ciclo (num jogo de 2001, a campanha de 2000, que joga em 2001); entre a seção asiática e a sul-americana; 3 fases, os inscritos, as estatísticas, as previsões, as regras e a tabela do grupo', 'rule: the Oceania qualifying starts in the cycle year (in a 2001 game, the 2000 campaign, played in 2001); between the Asian and the South American sections; 3 stages, the entrants, the statistics, the predictions, the rules and the group table'),
         ('construct_wc_south_american_league', 'WcSouthAmericanLeague::WcSouthAmericanLeague', '0x9202b0', 'wc_south_american_league.cpp', 'thiscall, ret 8', 'regra: as eliminatórias sul-americanas começam no ano do ciclo, nunca antes de 2004; entre a seção da Oceania e a africana; sem fases extras, os isentos, as regras, as vagas e as estatísticas; quando o ano do jogo já é o da edição, carrega a campanha real de 2002', 'rule: the South American qualifying starts in the cycle year, never before 2004; between the Oceania and the African sections; no extra stages, the exempt nations, the rules, the slots and the statistics; when the game year is already the edition year, it loads the real 2002 campaign')])
CAMPOS = [('competição +0x40 ano, +4 registro, +0x1c/+0x20 seções vizinhas, +0xc/+0x2c fases, +8 estatísticas, +0x45, +0x49, +0x4a, +0x51, +0xd9, +0xf7 (Ásia); valor/ponteiro; grava', 'competition +0x40 year, +4 record, +0x1c/+0x20 neighbour sections, +0xc/+0x2c stages, +8 statistics, +0x45, +0x49, +0x4a, +0x51, +0xd9, +0xf7 (Asia); value/pointer; writes')]
