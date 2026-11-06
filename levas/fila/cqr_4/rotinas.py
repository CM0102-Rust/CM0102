# A MLS (usa_mls.cpp; agente C da frente das competições, 06/11/2026): a fase seguinte, o play-off dos oito e a final
# da MLS Cup. Na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt), a fase seguinte roda duas vezes e
# o play-off e a final uma vez cada: o fim da MLS de 2001.
# - 0x909400 UsaMls::advance_stage: a fase seguinte; da fase 1 à 2 (o play-off) e da 2 à 3 (a final).
# - 0x9094c0 UsaMls::create_playoff_stage: os três campeões de divisão e os cinco melhores dos outros.
# - 0x9097d0 UsaMls::create_final_stage: os dois vencedores do play-off.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - a fase seguinte, virtual (sem chamada relativa): mov ecx, liga -> thiscall, ret 0 (a rotina termina em ret);
# - o play-off e a final, da fase seguinte: call direto com o ecx da liga, sem push -> thiscall, ret 0;
# - 0x944e46 (malloc): push 0x20 ou 8; call; add esp, 4 -> cdecl; 0x944cf1 (operator new): push 0xb2; add esp, 4 -> cdecl;
# - 0x945333 (qsort): push 0x90a130; push 6; push 3 ou 9; push base; call; add esp, 0x10 -> cdecl (0x90a130, o
#   critério: os pontos, word em +4, do maior para o menor; sem sorteio);
# - a virtual +0x3c (create_round_table): push 0; push &extra; push &rodadas; push 2 ou 3; mov ecx, esi -> thiscall,
#   ret 0x10;
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 pushes; mov ecx, eax -> thiscall, ret 0x3c (o play-off: a liga, 8, a lista, as
#   rodadas, +4, a tabela, o ano, 2, 1, o extra, 0x12, 1, 0, 0, sementes nulas; a final: 2, ..., 3, 1, o extra, 0x14,
#   0, 0, 0, sementes nulas; a original empurra as words com lixo nos bytes altos: a cópia passa as words limpas);
# - 0x9452ca (free): push; call; add esp, 4 -> cdecl;
# - 0x51c800 (Cup::set_up_round): push 0; mov ecx, fase -> thiscall, ret 4;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original, com o plano (a fase seguinte: +0x30 de 1 a 2 e de 2 a 3; o play-off: os oito da
# escolha, feita antes com os mesmos registros e o mesmo qsort, na lista da fase 2; a final: os dois primeiros
# vencedores do play-off na lista da fase 3).
# Determinismo: as listas vêm das tabelas; o qsort só compara pontos; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr4_fase_seguinte_da_mls', 0x909400, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqr4_play_off_da_mls', 0x9094c0, [0x6a, 0xff, 0x68, 0x1e, 0x54, 0x96, 0x00]),
           ('cqr4_final_da_mls', 0x9097d0, [0x6a, 0xff, 0x68, 0x3e, 0x54, 0x96, 0x00])]
DOCS = ('a MLS: a fase seguinte, o play-off dos oito e a final da MLS Cup',
        'the MLS: the next stage, the eight-club play-off and the MLS Cup final',
        [('cqr4_fase_seguinte_da_mls', 'UsaMls::advance_stage', '0x909400', 'usa_mls.cpp', 'thiscall, ret 0',
          'A MLS tem três divisões de quatro (a liga e as fases 0 e 1, que correm juntas na temporada regular), o play-off (fase 2) e a final (fase 3): a fase seguinte vai da fase 1 ao play-off e do play-off à final, e monta cada um; depois da última fase é erro (a caixa)',
          'The MLS has three divisions of four (the league and stages 0 and 1, played together in the regular season), the play-off (stage 2) and the final (stage 3): the next stage goes from stage 1 to the play-off and from the play-off to the final, building each; past the last stage it is an error (the box)'),
         ('cqr4_play_off_da_mls', 'UsaMls::create_playoff_stage', '0x9094c0', 'usa_mls.cpp', 'thiscall, ret 0',
          'O play-off da MLS: oito clubes. Os três campeões de divisão, ordenados pelos pontos, e os cinco melhores dos outros (do segundo ao quarto de cada divisão, pelos pontos; empate sem critério próprio). Os pares: o melhor campeão contra o quinto melhor dos outros, o segundo campeão contra o quarto, o terceiro campeão contra o terceiro e o melhor dos outros contra o segundo [inferência: os pares seguem a lista]; os vencedores vão à final',
          'The MLS play-off: eight clubs. The three division winners, ordered by points, and the five best of the rest (second to fourth of each division, by points; ties with no own rule). The pairs: the best winner against the fifth best of the rest, the second winner against the fourth, the third winner against the third and the best of the rest against the second [inference: pairs follow the list]; the winners go to the final'),
         ('cqr4_final_da_mls', 'UsaMls::create_final_stage', '0x9097d0', 'usa_mls.cpp', 'thiscall, ret 0',
          'A final da MLS Cup: os dois vencedores do play-off (os dois primeiros com a marca de vencedor, na ordem da lista do play-off) numa fase de copa de dois [inferência: jogo único]; o vencedor é o campeão da MLS',
          'The MLS Cup final: the two play-off winners (the first two with the winner mark, in play-off list order) in a two-club cup stage [inference: a single game]; the winner is the MLS champion')])
CAMPOS = [('liga +0x30 / +0x2c: a fase atual / a quantidade de fases; valor; lê e grava (fase seguinte)', 'league +0x30 / +0x2c: the current stage / the stage count; value; reads and writes (next stage)'),
          ('liga +0xb1: a tabela (linhas de 0x3b bytes: +0 o clube, +0x19 os pontos); ponteiro; lê as linhas 0 a 3 da liga e das fases 0 e 1', 'league +0xb1: the table (0x3b-byte rows: +0 the club, +0x19 the points); pointer; reads rows 0 to 3 of the league and of stages 0 and 1'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; lê as vagas 0, 1 e 2, grava a 2 e a 3', 'league +0xc: the stages (pointer array); pointer; reads slots 0, 1 and 2, writes 2 and 3'),
          ('fase de copa +0xa7: os times (registros de 6 bytes: +0 o clube, +5 a marca de vencedor); ponteiro; lê (final)', 'cup stage +0xa7: the teams (6-byte records: +0 the club, +5 the winner mark); pointer; reads (final)'),
          ('fase de copa +0x3e: a quantidade de times; valor; lê', 'cup stage +0x3e: the team count; value; reads'),
          ('liga +0x40 / +4: o ano / o registro da competição; valor; lê', 'league +0x40 / +4: the year / the competition record; value; reads')]
INTERESSA = []
