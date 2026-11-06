# A MLS (usa_mls.cpp; agente C da frente das competições, 06/11/2026): a virada da temporada, as posições previstas e a
# lista dos doze clubes. Na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt): a virada roda uma vez
# (a MLS de 2002), as posições previstas duas (na criação da carreira e na virada) e a lista uma.
# - 0x908e00 UsaMls::start_new_season: a virada.
# - 0x909bb0 UsaMls::set_predicted_positions: as posições previstas das três divisões.
# - 0x909a40 UsaMls::get_club_array: a lista dos doze clubes (as três tabelas).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - a virada e a lista, virtuais (sem chamada relativa): mov ecx, liga -> thiscall; a virada termina em ret, a lista em
#   ret 8 (lê dois ponteiros da pilha: a lista e a conta);
# - as posições previstas, do construtor (0x9071c3: mov ecx, esi; call) e da virada (a virtual +0x5c, sem push) ->
#   thiscall, ret 0;
# - 0x687970 (free_table): push 0; mov ecx, esi -> thiscall, ret 4; 0x9452ca (free): push; call; add esp -> cdecl;
# - o destrutor das fases (virtual 0): push 1; mov ecx, fase -> thiscall, ret 4;
# - 0x4a1c50 (archive_season_and_reset): push 1; mov ecx, [esi+8] -> thiscall, ret 4;
# - as virtuais +0x8c e +0x5c: mov ecx, esi, sem push -> thiscall, ret 0;
# - 0x908560 (create_table), 0x9086e0 (create_season_fixtures), 0x68aa80 (expectativas): mov ecx, esi; call -> thiscall,
#   ret 0; 0x6827d0 (sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x79cee0 (renew_officials): push [esi+4]; mov ecx, [0xb74340] -> thiscall, ret 4;
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl;
# - 0x945333 (qsort): push 0x4b07d0; push 4; push n; push base; call; add esp, 0x30 depois das três -> cdecl;
# - 0x4a2540 (Comp_Stats::set_club_final_position): push posição; push clube; mov ecx, [esi+8] -> thiscall, ret 8;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original, com o plano (a virada: o ano + 1 e as três divisões montadas; as posições previstas: os
# doze clubes ordenados pela mesma qsort numa cópia e a posição +0x72 do registro de cada um; a lista: n = 12 e os
# clubes das três tabelas na ordem).
# Determinismo: as listas vêm das tabelas; a qsort compara reputações; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs2_virada_da_mls', 0x908e00, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqs2_posicoes_previstas_da_mls', 0x909bb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqs2_clubes_da_mls', 0x909a40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a MLS: a virada da temporada, as posições previstas e a lista dos doze clubes',
        'the MLS: the season turnover, the predicted positions and the list of the twelve clubs',
        [('cqs2_virada_da_mls', 'UsaMls::start_new_season', '0x908e00', 'usa_mls.cpp', 'thiscall, ret 0',
          'A virada da MLS: apaga a tabela, o calendário e as fases (as outras duas divisões, o play-off e a final), guarda a temporada nas estatísticas, passa ao ano seguinte e refaz as regras, a primeira divisão, os jogos da temporada (com as fases das outras duas divisões) e a ordem; depois as posições previstas, as expectativas das diretorias e a renovação dos árbitros',
          'The MLS turnover: clears the table, the schedule and the stages (the other two divisions, the play-off and the final), archives the season in the statistics, moves to the next year and rebuilds the rules, the first division, the season fixtures (with the other two division stages) and the order; then the predicted positions, the board expectations and the referee renewal'),
         ('cqs2_posicoes_previstas_da_mls', 'UsaMls::set_predicted_positions', '0x909bb0', 'usa_mls.cpp', 'thiscall, ret 0',
          'As posições previstas da MLS: em cada divisão os quatro clubes são ordenados pela reputação e ganham as posições 1, 4, 7 e 10 (o favorito de cada divisão é previsto em 1º, o segundo em 4º, o terceiro em 7º e o último em 10º), uma escala comum às três divisões',
          'The MLS predicted positions: in each division the four clubs are ordered by reputation and get positions 1, 4, 7 and 10 (each division favourite is predicted 1st, the second 4th, the third 7th and the last 10th), one scale shared by the three divisions'),
         ('cqs2_clubes_da_mls', 'UsaMls::get_club_array', '0x909a40', 'usa_mls.cpp', 'thiscall, ret 8',
          'A lista dos doze clubes da MLS: os quatro da primeira divisão, depois os quatro da segunda e os quatro da terceira, na ordem das tabelas [inferência: usada pelo jogo das estrelas da MLS]',
          'The list of the twelve MLS clubs: the four of the first division, then the four of the second and the four of the third, in table order [inference: used by the MLS All-Star game]')])
CAMPOS = [('liga +0x4c: [inferência] marca da temporada; valor; grava 0', 'league +0x4c: [inference] season flag; value; writes 0'),
          ('liga +0xba: o calendário; ponteiro; libera e zera', 'league +0xba: the schedule; pointer; frees and clears'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; destrói as fases (virada), lê as vagas 0 e 1', 'league +0xc: the stages (pointer array); pointer; destroys the stages (turnover), reads slots 0 and 1'),
          ('liga +0x30: a fase atual; valor; lê e grava −1', 'league +0x30: the current stage; value; reads and writes −1'),
          ('liga +0x40: o ano da temporada; valor; lê e grava + 1', 'league +0x40: the season year; value; reads and writes + 1'),
          ('liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê', 'league +8: the statistics block (Comp_Stats); pointer; reads'),
          ('estatística, registro do clube +0x72: a posição prevista; valor; grava (posições previstas)', 'statistics, club record +0x72: the predicted position; value; writes (predicted positions)'),
          ('liga +0xb1 / +0x3e: a tabela / quantos clubes; lê', 'league +0xb1 / +0x3e: the table / how many clubs; reads'),
          ('clube +reputação: lida pela 0x4b07d0 (qsort); lê', 'club reputation: read by 0x4b07d0 (qsort); reads'),
          ('[0xb74340] o gerente dos árbitros; ponteiro; lê', '[0xb74340] the officials manager; pointer; reads')]
INTERESSA = ['CMRTE']
