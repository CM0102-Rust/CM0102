# A virada do futebol brasileiro (bra_nat_first.cpp, bra_nat_second.cpp; agente C da frente das competições,
# 05/11/2026): a virada da Série A, que comanda a do Brasil inteiro, o acesso e o rebaixamento em cadeia e a virada da
# Série B. Cada uma roda uma vez na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt), na virada
# do Brasil para 2002.
# - 0x42a640 BraNatFirst::start_new_season: a virada (promoção em cadeia, Copa do Brasil, Série C, estaduais, a Série A
#   refeita, as viradas das outras catorze);
# - 0x42ae90 BraNatFirst::process_promotion_relegation_chain: as posições finais e as trocas A-B, B-C e B-fases da C;
# - 0x42cc10 BraNatSecond::start_new_season: a Série B refeita.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - a virtual +0xb0 (a cadeia): push 1; mov ecx, esi -> thiscall, ret 4 (0x42ae90 termina em ret 4);
# - 0x42b760, 0x42b520, 0x42b130: mov ecx, esi; call, sem push nem add esp -> thiscall, ret 0 (o mapa as diz cdecl(void),
#   sem argumentos de pilha: o mesmo efeito);
# - 0x68a980 (League::limit_club_division_tier): push 1; push −3; push 3; push 1; mov ecx, liga -> thiscall, ret 0x10;
# - 0x687970 (free_table): push 0; mov ecx, esi -> thiscall, ret 4; 0x9452ca (free): push; call; add esp, 4 -> cdecl;
# - 0x4a1c50 (archive_season_and_reset): push 1; mov ecx, [esi+8] -> thiscall, ret 4;
# - o destrutor das fases (virtual 0): push 1; mov ecx, fase -> thiscall, ret 4;
# - as virtuais +0x8c, +0x5c, +8 (start_new_season das outras) e +0xa4 (record_final_positions): mov ecx, liga, sem
#   push -> thiscall, ret 0;
# - as tabelas 0x42a480 e 0x42ca70, 0x6835c0 (create_fixtures), 0x68aa80 (set_club_season_expectations): mov ecx, esi;
#   call, sem push -> thiscall, ret 0; 0x6827d0 (sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x79cee0 (renew_officials): push [esi+4]; mov ecx, [0xb74340] -> thiscall, ret 4;
# - 0x689c80 (league_promote_and_relegate): push −1; push −1; push 1; push 1; push de baixo; push de cima; mov ecx, esi
#   -> thiscall, ret 0x18 (a rotina guarda o ecx na entrada: mov [esp+0x1c], ecx; o mapa a diz stdcall);
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: as viradas, só a original conferindo o ano + 1; a cadeia, só a original.
# Determinismo: nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqq8_virada_da_serie_a', 0x42a640, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqq8_acesso_e_rebaixamento', 0x42ae90, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('cqq8_virada_da_serie_b', 0x42cc10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada do futebol brasileiro: a Série A, o acesso e o rebaixamento em cadeia e a Série B',
        'the Brazilian football turnover: Series A, the promotion and relegation chain and Series B',
        [('cqq8_virada_da_serie_a', 'BraNatFirst::start_new_season', '0x42a640', 'bra_nat_first.cpp', 'thiscall, ret 0',
          'A virada do Brasil, comandada pela Série A: primeiro o acesso e o rebaixamento em cadeia; depois os inscritos da Copa do Brasil, a entrada dos 16 melhores clubes sem divisão nacional na Série C e as trocas dos estaduais com as suas divisões de baixo; o nível de divisão dos clubes da A e da B é limitado; a Série A é refeita (ano seguinte, regras, 28 clubes, jogos); e então viram a Série B, a Série C e os doze estaduais (Baiano, Centro-Oeste, Gaúcho, Goiano, Mineiro, Norte, Nordeste, Paranaense, Pernambucano, Carioca, Catarinense e Paulista), com as expectativas das diretorias e a renovação dos árbitros',
          'The Brazilian turnover, led by Series A: first the promotion and relegation chain; then the Brazilian Cup entrants, the 16 best clubs without a national division into Series C and the swaps between the state leagues and their lower divisions; the division tier of the A and B clubs is capped; Series A is rebuilt (next year, rules, 28 clubs, fixtures); then Series B, Series C and the twelve state leagues turn over (Bahia, Centre-West, Rio Grande do Sul, Goias, Minas Gerais, North, Northeast, Parana, Pernambuco, Rio, Santa Catarina and Sao Paulo), with the board expectations and the referee renewal'),
         ('cqq8_acesso_e_rebaixamento', 'BraNatFirst::process_promotion_relegation_chain', '0x42ae90', 'bra_nat_first.cpp', 'thiscall, ret 4',
          'O acesso e o rebaixamento no Brasil: grava as posições finais da Série A, da Série B, da Série C e das sete fases da C; depois troca os clubes marcados entre A e B, entre B e C e entre a B e cada fase da C (os que sobem e os que descem saem das marcas do fim da temporada, sem número fixo aqui)',
          'Promotion and relegation in Brazil: records the final positions of Series A, Series B, Series C and the seven Series C stages; then swaps the marked clubs between A and B, between B and C and between B and each C stage (who goes up and down comes from the end-of-season marks, with no fixed number here)'),
         ('cqq8_virada_da_serie_b', 'BraNatSecond::start_new_season', '0x42cc10', 'bra_nat_second.cpp', 'thiscall, ret 0',
          'A virada da Série B: guarda a temporada nas estatísticas, apaga a tabela e as fases, passa ao ano seguinte e refaz as regras, a tabela com os 28 clubes da Série B, os jogos e a ordem; depois as posições previstas, as expectativas das diretorias e a renovação dos árbitros',
          'The Series B turnover: archives the season in the statistics, clears the table and stages, moves to the next year and rebuilds the rules, the table with the 28 Series B clubs, the fixtures and the order; then the predicted positions, the board expectations and the referee renewal')])
CAMPOS = [('liga +0x4c: [inferência] marca da temporada; valor; grava 0 (viradas)', 'league +0x4c: [inference] season flag; value; writes 0 (turnovers)'),
          ('liga +0xba: lista auxiliar; ponteiro; libera e zera (viradas)', 'league +0xba: auxiliary list; pointer; frees and clears (turnovers)'),
          ('liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê', 'league +8: the statistics block (Comp_Stats); pointer; reads'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; lê e destrói (viradas), lê as sete da Série C (cadeia)', 'league +0xc: the stages (pointer array); pointer; reads and destroys (turnovers), reads the seven of Series C (chain)'),
          ('liga +0x30: a fase atual; valor; lê e grava −1 (viradas)', 'league +0x30: the current stage; value; reads and writes −1 (turnovers)'),
          ('liga +0x40: o ano da temporada; valor; lê e grava + 1 (viradas)', 'league +0x40: the season year; value; reads and writes + 1 (turnovers)'),
          ('liga +4: o registro da competição no banco; ponteiro; lê (árbitros)', 'league +4: the competition record in the database; pointer; reads (referees)'),
          ('[0xadadfc] a tabela das competições; ponteiro; lê', '[0xadadfc] the competition table; pointer; reads'),
          ('[0x9cf9ec] / [0x9cf9e0] / [0x9cf9ac]: ids das Séries A, B e C; id; lê', '[0x9cf9ec] / [0x9cf9e0] / [0x9cf9ac]: Series A, B and C ids; id; reads'),
          ('[0x9cf9c0], [0x9cf998], [0x9cf9c8], [0x9cf9b4], [0x9cf9d4], [0x9cf994], [0x9cf9a4], [0x9cf9d0], [0x9cf9bc], [0x9cf9dc], [0x9cf9b0], [0x9cf9e4]: ids dos doze estaduais; id; lê', '[0x9cf9c0], [0x9cf998], [0x9cf9c8], [0x9cf9b4], [0x9cf9d4], [0x9cf994], [0x9cf9a4], [0x9cf9d0], [0x9cf9bc], [0x9cf9dc], [0x9cf9b0], [0x9cf9e4]: the twelve state league ids; id; reads'),
          ('[0xb74340] o gerente dos árbitros; ponteiro; lê', '[0xb74340] the officials manager; pointer; reads')]
INTERESSA = []
