# A Série C (bra_nat_third.cpp; agente C da frente das competições, 06/11/2026): a final, a virada da temporada e as
# posições previstas. Na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt): a final roda uma vez (a
# Série C de 2001), a virada uma (a de 2002, chamada pela virada da Série A, cqq_8) e as posições previstas duas (no
# construtor e na virada).
# - 0x42e980 BraNatThird::create_final_stage: a final dos campeões dos dois grupos da terceira fase.
# - 0x42def0 BraNatThird::start_new_season: a virada.
# - 0x42f0a0 BraNatThird::set_predicted_positions: as posições previstas dos 56.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - a final, da fase seguinte da Série C (0x42e40c): mov ecx, liga; call, sem push -> thiscall, ret 0;
# - a virada, virtual (a +8, da virada da Série A): mov ecx, liga, sem push -> thiscall, ret 0;
# - as posições previstas, do construtor (0x42d4e1: mov ecx, esi; call) e da virada (a virtual +0x5c) -> thiscall, ret 0;
# - a virtual +0x3c (create_round_table): push 0; push &extra; push &rodadas; push 10; mov ecx, esi -> thiscall, ret 0x10;
# - 0x944cf1 (operator new): push 0xb2; add esp, 4 -> cdecl; 0x944e46 (malloc): push n; add esp, 4 -> cdecl;
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 pushes; mov ecx, eax -> thiscall, ret 0x3c (a liga, 2, a lista, as rodadas, +4,
#   a tabela, o ano, 10, 1, o extra, 0x14, 0, 0, 0, sementes nulas; words limpas);
# - 0x687970 (free_table): push 0; mov ecx, esi -> thiscall, ret 4; 0x9452ca (free): push; call; add esp -> cdecl;
# - 0x4a1c50 (archive_season_and_reset): push 1; mov ecx, [esi+8] -> thiscall, ret 4;
# - o destrutor das fases (virtual 0): push 1; mov ecx, fase -> thiscall, ret 4;
# - as virtuais +0x8c e +0x5c: mov ecx, esi, sem push -> thiscall, ret 0;
# - 0x42dc90 (create_table), 0x6835c0 (create_fixtures), 0x68aa80 (expectativas): mov ecx, esi; call -> thiscall, ret 0;
#   0x6827d0 (sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x79cee0 (renew_officials): push [esi+4]; mov ecx, [0xb74340] -> thiscall, ret 4;
# - 0x42ee80 (create_group_stage): push g; mov ecx, esi -> thiscall, ret 4 (cqs_3);
# - 0x945333 (qsort): push 0x4b07d0; push 4; push 0x38; push lista; call; add esp, 0x10 -> cdecl;
# - 0x4a2540 (Comp_Stats::set_club_final_position): push posição; push clube; mov ecx, [esi+8] -> thiscall, ret 8;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original, com o plano (a final: os campeões dos grupos na lista da fase 10, +0x30 = 10; a virada:
# o ano + 1, os sete grupos montados e a lista dos 56 zerada; as posições previstas: a lista ordenada pela mesma qsort
# numa cópia e a posição +0x72 do registro de cada clube).
# Determinismo: as listas vêm das tabelas e da lista dos 56; a qsort compara reputações; nada de pilha, heap ou endereço
# como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs4_final_da_serie_c', 0x42e980, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqs4_virada_da_serie_c', 0x42def0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqs4_posicoes_previstas_da_serie_c', 0x42f0a0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a Série C: a final, a virada da temporada e as posições previstas',
        'Brazilian Series C: the final, the season turnover and the predicted positions',
        [('cqs4_final_da_serie_c', 'BraNatThird::create_final_stage', '0x42e980', 'bra_nat_third.cpp', 'thiscall, ret 0',
          'A final da Série C: o campeão do grupo A da terceira fase contra o do grupo B, numa fase de copa de dois [inferência: ida e volta]; o vencedor é o campeão da Série C',
          'The Series C final: the winner of third-round group A against the winner of group B, in a two-club cup stage [inference: home and away]; the winner is the Series C champion'),
         ('cqs4_virada_da_serie_c', 'BraNatThird::start_new_season', '0x42def0', 'bra_nat_third.cpp', 'thiscall, ret 0',
          'A virada da Série C: guarda a temporada, apaga a tabela, o calendário e as fases, passa ao ano seguinte e refaz as regras, a lista dos 56 clubes e a tabela, os jogos e a ordem; depois as expectativas das diretorias, a renovação dos árbitros, os sete grupos da primeira fase e as posições previstas; a lista dos 56 é descartada no fim',
          'The Series C turnover: archives the season, clears the table, the schedule and the stages, moves to the next year and rebuilds the rules, the 56-club list and the table, the fixtures and the order; then the board expectations, the referee renewal, the seven first-round groups and the predicted positions; the 56-club list is discarded at the end'),
         ('cqs4_posicoes_previstas_da_serie_c', 'BraNatThird::set_predicted_positions', '0x42f0a0', 'bra_nat_third.cpp', 'thiscall, ret 0',
          'As posições previstas da Série C: os 56 ordenados pela reputação; o favorito em 1º, o segundo em 2º, os dois seguintes em 3º (as semifinais), dois em 5º e dois em 7º, oito em 9º (a segunda fase) e os outros quarenta em faixas de oito: 16º, 24º, 32º, 40º e 48º',
          'The Series C predicted positions: the 56 ordered by reputation; the favourite 1st, the second 2nd, the next two 3rd (the semi-finals), two 5th and two 7th, eight 9th (the second round) and the other forty in bands of eight: 16th, 24th, 32nd, 40th and 48th')])
CAMPOS = [('liga +0xc: as fases (vetor de ponteiros); ponteiro; lê as vagas 8 e 9, grava a 10 (final); destrói, libera e realoca (virada)', 'league +0xc: the stages (pointer array); pointer; reads slots 8 and 9, writes 10 (final); destroys, frees and reallocates (turnover)'),
          ('liga +0x30: a fase atual; valor; grava 10 (final), −1 (virada)', 'league +0x30: the current stage; value; writes 10 (final), −1 (turnover)'),
          ('liga +0x40: o ano da temporada; valor; lê e grava + 1', 'league +0x40: the season year; value; reads and writes + 1'),
          ('liga +0xee: a lista dos 56 clubes; ponteiro; ordena (posições previstas), libera e zera (virada)', 'league +0xee: the 56-club list; pointer; sorts (predicted positions), frees and clears (turnover)'),
          ('liga +0x2c: a quantidade de fases; valor; lê', 'league +0x2c: the stage count; value; reads'),
          ('liga +0x4c, +0xba: [inferência] marca da temporada, o calendário; grava 0, libera', 'league +0x4c, +0xba: [inference] season flag, the schedule; writes 0, frees'),
          ('estatística, registro do clube +0x72: a posição prevista; valor; grava', 'statistics, club record +0x72: the predicted position; value; writes'),
          ('[0xb74340] o gerente dos árbitros; ponteiro; lê', '[0xb74340] the officials manager; pointer; reads')]
INTERESSA = ['CMRTE']
