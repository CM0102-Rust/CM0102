# A virada das copas: a Supercopa japonesa, a Copa dos EUA e o Jogo das Estrelas da MLS, com os dois times das estrelas
# (jap_super.cpp, usa_open_cup.cpp, usa_mls_all_stars.cpp, em cups\). Frente das competições (Agente D, 06/11/2026),
# carreira de 8 ligas (começa em janeiro de 2001).
# - 0x6664e0 (JapSuper::start_new_season, 425 bytes), 0x90b790 (UsaOpenCup::start_new_season, 425 bytes) e 0x90a760
#   (UsaMlsAllStars::start_new_season, 409 bytes: o mesmo molde sem as posições previstas): o slot 3 (+0x08), conferido
#   instrução a instrução; um ajudante comum (cqr6_virada).
# - 0x90a540 (UsaMlsAllStars::set_up_teams, 535 bytes): os dois times das estrelas, o mandante pelo ano e o estádio
#   sorteado entre os clubes da conferência do mandante.
# Iguais no cm0102-1080.exe (as quatro inteiras).
# Convenções conferidas (lição da 258a): as quatro thiscall, ret 0. Dentro: 0x9452ca e 0x944e46 (push; add esp, 4 ->
# cdecl); 0x4a1c50 (push 1; mov ecx, [esi+8] -> thiscall, ret 4); os inscritos 0x666160 / 0x90b530 / 0x90a540 (mov ecx,
# esi; call -> thiscall, ret 0; devolução com sinal); as virtuais +0x8c, +0x94, +0x5c (thiscall sem argumentos); o
# sorteio 0x90cfc0 (push 6; add esp, 4 -> cdecl); usa_mls_get_club_division 0x909ec0 (push clube; add esp, 4 -> cdecl,
# movsx ax); a caixa (aviso_em).
# Comparadores: as viradas só a original (liberam e refazem as tabelas), conferindo o ano +1; os times das estrelas lado a
# lado a partir do mesmo estado (o sorteador, +0x3e, os dois clubes das estrelas e [0xb67a34]), a lista da cópia
# comparada nos 12 bytes e liberada; o sorteio da cópia sem contar.
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr6_virada_supercopa_japonesa', 0x6664e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqr6_virada_copa_dos_eua', 0x90b790, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqr6_virada_jogo_das_estrelas', 0x90a760, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqr6_times_do_jogo_das_estrelas', 0x90a540, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a virada da Supercopa japonesa, da Copa dos EUA e do Jogo das Estrelas da MLS, e os dois times das estrelas',
        'the season turn of the Japanese Super Cup, the US Open Cup and the MLS All-Star Game, and the two all-star teams',
        [('cqr6_virada_supercopa_japonesa', 'JapSuper::start_new_season', '0x6664e0', 'jap_super.cpp', 'thiscall, ret 0',
          'regra: na virada da temporada a Supercopa japonesa é refeita: as tabelas da edição anterior liberadas, o histórico arquivado, o ano +1, os dois clubes (JapSuper::select_teams: o campeão da J1 e o da Copa do Imperador [inferência]), as rodadas, a árvore e as posições previstas',
          'rule: at the season turn the Japanese Super Cup is rebuilt: last edition\'s tables freed, history archived, year +1, the two clubs (JapSuper::select_teams: the J1 champion and the Emperor\'s Cup winner [inference]), the rounds, the tree and the predicted positions'),
         ('cqr6_virada_copa_dos_eua', 'UsaOpenCup::start_new_season', '0x90b790', 'usa_open_cup.cpp', 'thiscall, ret 0',
          'regra: a mesma virada na Copa dos EUA (US Open Cup): as tabelas liberadas, o histórico arquivado, o ano +1, os inscritos (UsaOpenCup::collect_entrants), as rodadas, a árvore e as posições previstas',
          'rule: the same turn in the US Open Cup: tables freed, history archived, year +1, the entrants (UsaOpenCup::collect_entrants), the rounds, the tree and the predicted positions'),
         ('cqr6_virada_jogo_das_estrelas', 'UsaMlsAllStars::start_new_season', '0x90a760', 'usa_mls_all_stars.cpp', 'thiscall, ret 0',
          'regra: a virada do Jogo das Estrelas da MLS: as tabelas liberadas, o histórico arquivado, o ano +1, os dois times das estrelas, as rodadas e a árvore (sem posições previstas)',
          'rule: the MLS All-Star Game turn: tables freed, history archived, year +1, the two all-star teams, the rounds and the tree (no predicted positions)'),
         ('cqr6_times_do_jogo_das_estrelas', 'UsaMlsAllStars::set_up_teams', '0x90a540', 'usa_mls_all_stars.cpp', 'thiscall, ret 0',
          'regra: o Jogo das Estrelas da MLS opõe as duas seleções das conferências ([0x9d00a4] e [0x9d00a8], clubes da tabela); no ano ímpar a de [0x9d00a4] manda o jogo, no par a de [0x9d00a8]; o estádio é o de um clube da MLS sorteado (1 em 6, pela ordem da tabela) entre os da conferência do mandante (0x413 no ano ímpar, 0x414 no par); os dois times ficam com a nação dos EUA [inferência]',
          'rule: the MLS All-Star Game pits the two conference all-star teams ([0x9d00a4] and [0x9d00a8], clubs in the table); in odd years the [0x9d00a4] team is at home, in even years the [0x9d00a8] one; the stadium is that of an MLS club drawn (1 in 6, in table order) among the home conference\'s clubs (0x413 in odd years, 0x414 in even ones); both teams get the USA nation [inference]')])
CAMPOS = [('copa +0x40 ano (word); valor; lê e grava +1 (viradas), lê (times das estrelas: o mandante pelo ano par ou ímpar)',
           'cup +0x40 year (word); value; reads and writes +1 (season turns), reads (all-star teams: home side by odd/even year)'),
          ('copa +0x4c, +0xab, +0xb1 (byte); valor; grava 0 (viradas)', 'cup +0x4c, +0xab, +0xb1 (byte); value; writes 0 (season turns)'),
          ('copa +0xa7 inscritos (6 bytes: clube, +4, +5), +0xa3 rodadas, +0xad confrontos por rodada; ponteiro; libera e grava (viradas, times das estrelas)',
           'cup +0xa7 entrants (6 bytes: club, +4, +5), +0xa3 rounds, +0xad fixtures per round; pointer; frees and writes (season turns, all-star teams)'),
          ('[0x9d00a4], [0x9d00a8] os dois clubes das estrelas; índice na tabela dos clubes; lê', '[0x9d00a4], [0x9d00a8] the two all-star clubs; index into the club table; reads'),
          ('clube +0x57 divisão (competição); ponteiro; lê. clube +0x69 estádio; ponteiro; lê e grava (o mandante das estrelas); clube +0x6d byte; grava 0; clube +0x53 nação; ponteiro; grava',
           'club +0x57 division (competition); pointer; reads. club +0x69 stadium; pointer; reads and writes (the all-star home team); club +0x6d byte; writes 0; club +0x53 nation; pointer; writes'),
          ('[0x9cf590] a MLS; id; lê. [0x9cf4f8] os EUA [inferência]; índice na tabela das nações [0xae23a8] (0x122 bytes); lê',
           '[0x9cf590] the MLS; id; reads. [0x9cf4f8] the USA [inference]; index into the nation table [0xae23a8] (0x122 bytes); reads')]
INTERESSA = ['CMRTE']
