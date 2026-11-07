# Os construtores da Série C, da J2 e da MLS (bra_nat_third.cpp, jap_j2.cpp, usa_mls.cpp; agente C da frente das
# competições, 06/11/2026). Rodam uma vez cada, na criação da carreira de 8 ligas (docs/cobertura_competicoes.txt). O
# molde dos construtores dos estaduais (cqt_1..4) e das ligas nacionais (cqu_1), conferido instrução a instrução, com as
# variações de cada uma:
# - 0x42d2c0 BraNatThird::BraNatThird (608 bytes): vtable 0x967f00, disciplina 7, sem importância, 11 fases além da
#   liga, regras 0x42de50, tabela 0x42dc90; +0xee (a lista dos 56) zerado no começo; no fim, os sete grupos (0x42ee80,
#   cqs_3), as posições previstas da Série C (0x42f0a0, cqs_4) e a lista liberada e zerada.
# - 0x6644c0 JapJ2::JapJ2 (528 bytes): vtable 0x96caf4, disciplina 0x13, sem importância, sem fases (+0x2c = 0, +0xc =
#   0, sem malloc), regras 0x665140, tabela 0x665030, posições previstas da League (0x68a850).
# - 0x906f80 UsaMls::UsaMls (624 bytes): vtable 0x970844, disciplina 0x1e, importância 0xa antes da carga, 4 fases,
#   as regras gravadas em linha (sem rotina própria), tabela 0x908560 (cqs_1), os jogos pela 0x9086e0
#   (create_season_fixtures) no lugar da 0x6835c0, posições previstas 0x909bb0 (cqs_2).
# Iguais no cm0102-1080.exe (inteiros). Convenções conferidas nos pontos de chamada da original:
# - os construtores, da criação das competições (0x832bde, 0x835b32, 0x8382a1): push registro; push ano; mov ecx, novo
#   -> thiscall, ret 8 (lê o ano como word);
# - 0x682200 (League::League): mov ecx, esi; call -> thiscall, ret 0;
# - 0x687b10 (League::load_from_temp_file): push 1; mov ecx, esi -> thiscall, ret 4;
# - 0x944e46 (malloc): push 0x2c ou 0x10; call; add esp, 4 -> cdecl;
# - as regras, a tabela, os jogos (0x6835c0 ou 0x9086e0) e as posições previstas (0x42f0a0, 0x68a850, 0x909bb0): mov
#   ecx, esi; call -> thiscall, ret 0; 0x6827d0 (sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x944cf1 (operator new): push 0x5ce; call; add esp, 4 -> cdecl; 0x49ee70 (Comp_Stats::Comp_Stats): push esi; mov
#   ecx, novo -> thiscall, ret 4;
# - 0x42ee80 (o grupo g): push ecx (o dword com g no byte de baixo e os bytes altos do ponteiro do new em cima); mov
#   ecx, esi -> thiscall, ret 4 (a cópia passa g limpo: a rotina e as que ela chama só usam o g);
# - 0x9452ca (free): push +0xee; call; add esp, 4 -> cdecl;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (aloca e sorteia os jogos), conferindo a devolução, a vtable, o ano, o registro, a
# disciplina e (na MLS) a importância; na liga nova, as fases, o vetor delas (nulo na J2), a importância, as regras
# dos pontos da MLS (+0xbe..+0xc8; a ordem reescreve +0xa7) e, na Série C, os sete grupos e a lista dos 56 zerada.
# Determinismo: só constantes e os argumentos.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqu2_construir_serie_c', 0x42d2c0, [0x6a, 0xff, 0x68, 0x09, 0x45, 0x95, 0x00]),
           ('cqu2_construir_j2', 0x6644c0, [0x6a, 0xff, 0x68, 0x89, 0xc1, 0x95, 0x00]),
           ('cqu2_construir_mls', 0x906f80, [0x6a, 0xff, 0x68, 0x79, 0x53, 0x96, 0x00])]
DOCS = ('os construtores da Série C, da J2 e da MLS',
        'the constructors of the Brazilian third division, J2 and MLS',
        [('cqu2_construir_serie_c', 'BraNatThird::BraNatThird', '0x42d2c0', 'bra_nat_third.cpp', 'thiscall, ret 8',
          'A Série C na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, onze fases além da liga (os sete grupos de sete da primeira fase, a segunda e a terceira fases e a final), as regras, a tabela, os jogos, a ordem, as estatísticas, os sete grupos da primeira fase com os 56 clubes, as posições previstas (pela reputação) e a lista dos 56 liberada',
          'The Brazilian third division at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, eleven stages besides the league (the seven first-round groups of seven, the second and third rounds and the final), the rules, the table, the fixtures, the order, the statistics, the seven first-round groups with the 56 clubs, the predicted positions (by reputation) and the list of 56 freed'),
         ('cqu2_construir_j2', 'JapJ2::JapJ2', '0x6644c0', 'jap_j2.cpp', 'thiscall, ret 8',
          'A J2 na criação da carreira: o ano e a competição do banco, a regra de disciplina 0x13 e, sem estado salvo, só pontos corridos (nenhuma fase além da liga), as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas pela força dos clubes',
          'J2 at career creation: the year and the database competition, disciplinary rule 0x13 and, with no saved state, a plain round robin (no stage besides the league), the rules, the table, the fixtures, the order, the statistics and the predicted positions by club strength'),
         ('cqu2_construir_mls', 'UsaMls::UsaMls', '0x906f80', 'usa_mls.cpp', 'thiscall, ret 8',
          'A MLS na criação da carreira: o ano e a competição do banco, a regra de disciplina 0x1e e a importância 10 e, sem estado salvo, quatro fases além da liga, as regras (3 pontos por vitória e 1 por empate, os critérios de desempate e a categoria dos jogos), a tabela das três divisões, os jogos da temporada, a ordem, as estatísticas e as posições previstas',
          'MLS at career creation: the year and the database competition, disciplinary rule 0x1e and importance 10 and, with no saved state, four stages besides the league, the rules (3 points for a win and 1 for a draw, the tie-breakers and the match category), the table of the three divisions, the season fixtures, the order, the statistics and the predicted positions')])
CAMPOS = [('liga +0x50: a regra de disciplina (índice na tabela dos órgãos disciplinares); valor; grava 7, 0x13 ou 0x1e', 'league +0x50: the disciplinary rule (index into the disciplinary bodies table); value; writes 7, 0x13 or 0x1e'),
          ('liga +0x51: [inferência] a importância; valor; grava 0xa (MLS)', 'league +0x51: [inference] the importance; value; writes 0xa (MLS)'),
          ('liga +0x2c / +0x30 / +0xc: as fases / a atual / o vetor; grava', 'league +0x2c / +0x30 / +0xc: the stages / the current one / the array; writes'),
          ('liga +0xc2 / +0xc3: os pontos por vitória / por empate; valor; grava 3 / 1 (MLS)', 'league +0xc2 / +0xc3: the points for a win / a draw; value; writes 3 / 1 (MLS)'),
          ('liga +0xee: a lista dos 56 clubes da Série C; ponteiro; zera e libera', 'league +0xee: the list of the 56 third division clubs; pointer; zeroes and frees'),
          ('liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; grava', 'league +8: the statistics block (Comp_Stats); pointer; writes')]
INTERESSA = ['CMRTE']
