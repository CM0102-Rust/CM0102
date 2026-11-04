# A MLS: o destino de cada clube no fim das fases, o resultado nas tabelas das duas conferências e a conferência do
# Jogo das Estrelas (frente das competições, 03/11/2026, agente C).
# - 0x908200 UsaMls::update_staff_records_for_status (337 bytes), slot 18 da MLS (vtable 0x970844). 17 chamadas.
# - 0x9099d0 UsaMls::add_result (100 bytes), slot 2 da MLS. 276 chamadas.
# - 0x90a000 usa_mls_get_club_all_star_conference (304 bytes com o caso de erro), chamada pelo Jogo das Estrelas
#   (0x90a980). 8 chamadas.
# (contagens da carreira de 8 ligas até 2002-06-01, docs/cobertura_competicoes.txt)
# Iguais no cm0102-1080.exe (0x400 bytes a partir de cada uma).
# Convenções conferidas:
#   0x908200: thiscall (`mov edi, ecx`), ret 0x18 (6 argumentos: clube, situação, fase, _, jogo, _), devolve 0 ou -1.
#   0x9099d0: thiscall (`mov esi, ecx`), ret 8 (jogo, estatísticas), devolve 1.
#   0x90a000: cdecl (o chamador 0x90aabf / 0x90acb8 faz `add esp, 4`), 1 argumento (o clube), devolve ax (os
#   chamadores só comparam ax com 0x413 / 0x414).
#   0x86bdd0 add_qualified_for_round_record: thiscall (`mov ecx, [0xdd7f00]`), 5 argumentos, ret 0x14.
#   0x86c1d0 add_failed_to_qualify_record: thiscall ([0xdd7f00]), 4 argumentos (clube, comp, fase, 0x28), ret 0x10.
#   0x86c000 add_knocked_out_record: thiscall ([0xdd7f00]), 5 argumentos, ret 0x14.
#   0x868c50 add_league_champions_record: thiscall ([0xdd7f00]), 2 argumentos, ret 8.
#   0x4bf850 comp_util_scale_by_round: cdecl (`add esp, 0x10`), 4 argumentos (0x32, 0x46, palavra +0x34 do jogo, 2).
#   A original empurra registradores inteiros com lixo acima das palavras do jogo (eax do chamador, a metade de cima
#   do this, o eax da escala); a 0x4bf850, a 0x86bdd0 e a 0x86c000 só usam as palavras: a cópia passa as palavras.
#   0x685d30 League::add_result: thiscall (`mov ecx, esi`), 2 argumentos (jogo, estatísticas), ret 8.
#   o slot 2 (+4) dos estágios 0 e 1 ([[this+0xc]], [[this+0xc]+4]): thiscall (`mov ecx, [eax]`), 2 argumentos.
# Comparadores: 0x908200 só a original (os registros alocam) com a devolução do ensaio; 0x9099d0 só a original (o
# resultado entra nas tabelas) com o plano: devolução 1, a marca +0x42 do jogo e o bloco +8 devolvido; 0x90a000 pela
# devolução (só lê; o caso de erro fica com a original).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cqk9_destino_na_mls', 0x908200, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('cqk9_resultado_na_mls', 0x9099d0, [0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('cqk9_conferencia_das_estrelas', 0x90a000, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a MLS: o destino de cada clube no fim das fases, o resultado nas tabelas das duas conferências e a conferência do Jogo das Estrelas',
        'the MLS: the fate of each club at the end of the stages, the result in the two conference tables and the All-Star conference',
        [('cqk9_destino_na_mls', 'UsaMls::update_staff_records_for_status', '0x908200', 'usa_mls.cpp', 'thiscall',
          'MLS: na temporada e nas duas primeiras rodadas dos play-offs, quem avança ganha o registro de classificação '
          '(fase 0xa0, rodada 0x78, peso 0x28) e quem fica de fora o de não classificado; na final, o vencedor ganha o '
          'registro da final (peso 0x50) e os outros o da rodada, com peso de 50 a 70; depois, o campeão ganha o de campeão',
          'MLS: in the season and the first two play-off rounds, those who go through get the qualification record (stage '
          '0xa0, round 0x78, weight 0x28) and those left out the failed-to-qualify one; in the final, the winner gets the '
          'final record (weight 0x50) and the others the round one, weighted 50 to 70; then the champion gets the '
          'champion record'),
         ('cqk9_resultado_na_mls', 'UsaMls::add_result', '0x9099d0', 'usa_mls.cpp', 'thiscall',
          'MLS: cada jogo vai para a tabela geral e, na temporada regular, também para as tabelas das duas conferências '
          '(Leste e Oeste [inferência]), sem contar duas vezes as estatísticas',
          'MLS: each game goes to the overall table and, in the regular season, also to the two conference tables (East '
          'and West [inference]), without counting the statistics twice'),
         ('cqk9_conferencia_das_estrelas', 'usa_mls_get_club_all_star_conference', '0x90a000', 'usa_mls.cpp', 'cdecl',
          'Jogo das Estrelas da MLS: Leste (0x413) para New England, MetroStars, Tampa Bay, D.C. United, Columbus e Miami; '
          'Oeste (0x414) para Colorado, Dallas, Kansas City, Los Angeles, San Jose e Chicago; os de fora da MLS, nenhuma',
          'MLS All-Star game: East (0x413) for New England, MetroStars, Tampa Bay, D.C. United, Columbus and Miami; West '
          '(0x414) for Colorado, Dallas, Kansas City, Los Angeles, San Jose and Chicago; clubs outside the MLS, none')])
CAMPOS = [('liga +4: registro da competição no banco; ponteiro; lê (vai nos registros de carreira)', 'league +4: competition record in the database; pointer; reads (goes into the career records)'),
          ('liga +8: bloco de estatísticas (Comp_Stats); ponteiro; lê, grava 0 e devolve (add_result)', 'league +8: statistics block (Comp_Stats); pointer; reads, writes 0 and restores (add_result)'),
          ('liga +0xc: vetor de estágios (as duas conferências); ponteiro; lê', 'league +0xc: stage vector (the two conferences); pointer; reads'),
          ('liga +0x30: estágio atual (1 = a temporada regular da MLS [inferência]); valor; lê', 'league +0x30: current stage (1 = the MLS regular season [inference]); value; reads'),
          ('jogo +0x42: marca da conferência (-1 geral, 0 e 1 as conferências) [inferência]; valor; lê e grava', 'match +0x42: conference flag (-1 overall, 0 and 1 the conferences) [inference]; value; reads and writes'),
          ('jogo +0x30 / +0x32 / +0x34: rodada, fase e a rodada da escala; valor (palavra); lê', 'match +0x30 / +0x32 / +0x34: round, stage and the scale round; value (word); reads'),
          ('clube +0: id; clube +0x57: a liga do clube (club_comp); ponteiro; lê', 'club +0: id; club +0x57: the club league (club_comp); pointer; reads'),
          ('[0x9cf590] chave da MLS; [0x9d037c..0x9d03a8] chaves dos doze clubes; id; lê', '[0x9cf590] MLS key; [0x9d037c..0x9d03a8] the twelve club keys; id; reads'),
          ('[0xdd7f00]: Staff_Records; ponteiro; lê', '[0xdd7f00]: Staff_Records; pointer; reads')]
INTERESSA = ['CMRTE']
