# A J1 (jap_j1.cpp; agente C da frente das competições, 05/11/2026): a fase seguinte, a final entre os campeões dos
# dois turnos e a virada da temporada do futebol japonês. Na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt), cada uma roda uma vez: a final da J1 de 2001 e a virada para 2002.
# - 0x6639c0 JapJ1::advance_stage: a fase seguinte; ao chegar a 2 monta a final.
# - 0x663a50 JapJ1::create_championship_final_stage: a final dos campeões dos turnos.
# - 0x6636d0 JapJ1::start_new_season: a virada (acesso e rebaixamento, a J1 refeita, a virada da J2).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - a final, da fase seguinte: call direto com o ecx da liga, sem push -> thiscall, ret 0;
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl; 0x944cf1 (operator new): push 0xb2; call; add esp, 4 -> cdecl;
# - a virtual +0x3c (create_round_table): push 0; push &extra; push &rodadas; push 2; mov ecx, esi -> thiscall, ret 0x10;
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 pushes; mov ecx, eax -> thiscall, ret 0x3c (n = 2, os bytes 2 e 1, o 13º
#   argumento 0, sementes nulas; a cópia passa as words limpas);
# - 0x9452ca (free): push lista; call; push t; call; add esp, 8 -> cdecl (e push [esi+0xc]; call na virada);
# - 0x51c800 (Cup::set_up_round): push 0; mov ecx, [[+0xc]+8] -> thiscall, ret 4;
# - a virtual +0xb0 (acesso e rebaixamento): push 1; mov ecx, esi -> thiscall, ret 4;
# - 0x687970 (free_table): push 0; mov ecx, esi -> thiscall, ret 4; 0x4a1c50: push 1; mov ecx, [esi+8] -> thiscall, ret 4;
# - o destrutor das fases (virtual 0): push 1; mov ecx, fase -> thiscall, ret 4;
# - 0x5cc7d0, 0x5a8f60: call sem push -> cdecl(void); 0x945934 (exit): push −1; call -> cdecl;
# - as virtuais +0x8c, +0x5c e +8 (a virada da J2): mov ecx, liga, sem push -> thiscall, ret 0;
# - 0x6634e0 (create_table), 0x6835c0 (create_fixtures), 0x68aa80 (expectativas): mov ecx, esi; call -> thiscall, ret 0;
#   0x6827d0 (sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x79cee0 (renew_officials): push [esi+4]; mov ecx, [0xb74340] -> thiscall, ret 4;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original, com o plano (a fase seguinte: +0x30 + 1; a final: os dois campeões na lista da fase,
# +0x3e = 2, +4 e +0x30 = 2; a virada: o ano + 1 e +0x2c = 3).
# Determinismo: a lista vem das tabelas; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqq9_fase_seguinte_da_j1', 0x6639c0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqq9_final_da_j1', 0x663a50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqq9_virada_da_j1', 0x6636d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a J1: a fase seguinte, a final entre os campeões dos dois turnos e a virada da temporada',
        'the J1: the next stage, the final between the two stage winners and the season turnover',
        [('cqq9_fase_seguinte_da_j1', 'JapJ1::advance_stage', '0x6639c0', 'jap_j1.cpp', 'thiscall, ret 0',
          'A J1 tem dois turnos (as fases 0 e 1, cada um uma liga de pontos corridos) e a final (fase 2): a fase seguinte sobe a fase atual e, no fim do segundo turno, monta a final; depois da última fase é erro (a caixa)',
          'The J1 has two stages (0 and 1, each a round-robin league) and the final (stage 2): the next stage moves the current stage on and, at the end of the second stage, builds the final; past the last stage it is an error (the box)'),
         ('cqq9_final_da_j1', 'JapJ1::create_championship_final_stage', '0x663a50', 'jap_j1.cpp', 'thiscall, ret 0',
          'A final da J1: o campeão do primeiro turno contra o do segundo, em dois jogos [inferência: ida e volta], sem sementes; ao contrário dos estaduais brasileiros, não confere se é o mesmo clube (a final é montada mesmo assim)',
          'The J1 championship final: the first-stage winner against the second-stage winner, over two games [inference: home and away], with no seeds; unlike the Brazilian state leagues, it does not check for the same club (the final is built anyway)'),
         ('cqq9_virada_da_j1', 'JapJ1::start_new_season', '0x6636d0', 'jap_j1.cpp', 'thiscall, ret 0',
          'A virada do Japão, comandada pela J1: primeiro o acesso e o rebaixamento com a J2; depois a J1 é refeita com três fases (os dois turnos e a final), ano seguinte, regras, a tabela e os jogos; então vira a J2, e vêm as posições previstas, as expectativas das diretorias e a renovação dos árbitros',
          'The Japanese turnover, led by the J1: first promotion and relegation with the J2; then the J1 is rebuilt with three stages (the two halves and the final), next year, rules, the table and the fixtures; then the J2 turns over, followed by the predicted positions, the board expectations and the referee renewal')])
CAMPOS = [('liga +0x2c: o número de fases; valor; lê (fase seguinte), grava 3 (virada)', 'league +0x2c: the number of stages; value; reads (next stage), writes 3 (turnover)'),
          ('liga +0x30: a fase atual; valor; lê e grava', 'league +0x30: the current stage; value; reads and writes'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; lê os dois turnos, grava a final; libera e realoca (virada)', 'league +0xc: the stages (pointer array); pointer; reads the two halves, writes the final; frees and reallocates (turnover)'),
          ('fase de liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê a primeira linha', 'league stage +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads the first row'),
          ('liga +0x40: o ano da temporada; valor; lê, grava + 1 (virada)', 'league +0x40: the season year; value; reads, writes + 1 (turnover)'),
          ('liga +0x4c: [inferência] marca da temporada; valor; grava 0 (virada)', 'league +0x4c: [inference] season flag; value; writes 0 (turnover)'),
          ('liga +0xba: lista auxiliar; ponteiro; libera e zera (virada)', 'league +0xba: auxiliary list; pointer; frees and clears (turnover)'),
          ('liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê (virada)', 'league +8: the statistics block (Comp_Stats); pointer; reads (turnover)'),
          ('[0x9cf768] id da J2; id; lê', '[0x9cf768] J2 id; id; reads'),
          ('[0xb74340] o gerente dos árbitros; ponteiro; lê', '[0xb74340] the officials manager; pointer; reads'),
          ('fase de copa +0xa7: a lista dos times (6 bytes, +0 o clube); ponteiro; lê (comparador)', 'cup stage +0xa7: the team list (6 bytes, +0 the club); pointer; reads (comparator)')]
INTERESSA = []
