# A NSL australiana (aus_nsl.cpp; agente C da frente das competições, 06/11/2026): a fase seguinte, a série final dos
# seis primeiros e a grande final. Na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt), a fase
# seguinte roda duas vezes e cada fase uma vez: o fim da NSL de 2001/02.
# - 0x411f70 AusNsl::advance_stage: a fase seguinte; ao chegar a 0 monta a série final, a 1, a grande final.
# - 0x412010 AusNsl::create_finals_series_stage: a série final (uma liga curta dos seis primeiros da tabela).
# - 0x4121c0 AusNsl::create_grand_final_stage: a grande final (os dois primeiros da série final).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - a fase seguinte, virtual (sem chamada relativa): mov ecx, liga -> thiscall, ret 0 (a rotina termina em ret);
# - a série final e a grande final, da fase seguinte: call direto com o ecx da liga, sem push -> thiscall, ret 0;
# - a virtual +0x3c (create_round_table): push 0; push &extra; push &rodadas; push 0 ou 1; mov ecx, esi -> thiscall,
#   ret 0x10;
# - 0x944cf1 (operator new): push 0xee ou 0xb2; call; add esp, 4 -> cdecl; 0x944e46 (malloc): push 8; add esp, 4 -> cdecl;
# - 0x68b100 (League_Stage::League_Stage): 22 pushes; mov ecx, eax -> thiscall, ret 0x58 (a liga, 6, o vetor, 1, +4, a
#   tabela, as rodadas, +0xc2, +0xc3, +0xc4, &+0xc5, &{2,0,0,0}, o ano, 0, o extra, +0x51, 1, 0, +0xd9, −1, 0, 2; a
#   original empurra os words e bytes com lixo nos bytes altos e o extra com a palavra alta da pilha sem valor, que o
#   construtor não lê: a cópia passa os valores limpos);
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 pushes; mov ecx, eax -> thiscall, ret 0x3c (a liga, 2, a lista, as rodadas, +4,
#   a tabela, o ano, 1, 1, o extra, 0x14, 0, 0, 0, sementes nulas; words limpas);
# - 0x9452ca (free): push; call; add esp, 4 -> cdecl;
# - 0x684230 (a notícia do sorteio): mov ecx, fase; call, sem push -> thiscall, ret 0;
# - 0x51c800 (Cup::set_up_round): push 0; mov ecx, [[+0xc]+4] -> thiscall, ret 4;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original, com o plano (a fase seguinte: +0x30 + 1; a série final: os seis primeiros na tabela da
# fase 0, +0x3e = 6, +0x30 = 0, devolve 1; a grande final: os dois primeiros da série final na lista da fase 1).
# Determinismo: as listas vêm das tabelas; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr3_fase_seguinte_da_nsl', 0x411f70, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqr3_serie_final_da_nsl', 0x412010, [0x6a, 0xff, 0x68, 0x1e, 0x3a, 0x95, 0x00]),
           ('cqr3_grande_final_da_nsl', 0x4121c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a NSL australiana: a fase seguinte, a série final dos seis primeiros e a grande final',
        'the Australian NSL: the next stage, the top-six finals series and the grand final',
        [('cqr3_fase_seguinte_da_nsl', 'AusNsl::advance_stage', '0x411f70', 'aus_nsl.cpp', 'thiscall, ret 0',
          'A NSL tem a temporada regular (pontos corridos), a série final (fase 0) e a grande final (fase 1): a fase seguinte sobe a fase atual e monta a série final no fim da temporada regular e a grande final no fim da série final; depois da última fase é erro (a caixa)',
          'The NSL has the regular season (round robin), the finals series (stage 0) and the grand final (stage 1): the next stage moves the current stage on and builds the finals series at the end of the regular season and the grand final at the end of the finals series; past the last stage it is an error (the box)'),
         ('cqr3_serie_final_da_nsl', 'AusNsl::create_finals_series_stage', '0x412010', 'aus_nsl.cpp', 'thiscall, ret 0',
          'A série final da NSL: os seis primeiros da temporada regular jogam uma fase de pontos corridos própria (com os pontos e os critérios de desempate da liga) e os dois primeiros dela vão à grande final; manda a notícia do sorteio. [inferência] Na NSL real era um mata-mata com dupla chance; aqui é uma liga curta dos seis',
          'The NSL finals series: the top six of the regular season play their own round-robin stage (with the league points and tie-breakers) and its top two go to the grand final; sends the draw news. [inference] The real NSL used a double-chance knockout; here it is a short league of the six'),
         ('cqr3_grande_final_da_nsl', 'AusNsl::create_grand_final_stage', '0x4121c0', 'aus_nsl.cpp', 'thiscall, ret 0',
          'A grande final da NSL: o primeiro contra o segundo da série final, numa fase de copa de dois clubes [inferência: jogo único, o primeiro como mandante]; o vencedor é o campeão australiano',
          'The NSL grand final: the top two of the finals series in a two-club cup stage [inference: a single game, the first as home side]; the winner is the Australian champion')])
CAMPOS = [('liga +0x30 / +0x2c: a fase atual / a quantidade de fases; valor; lê e grava (fase seguinte, série final)', 'league +0x30 / +0x2c: the current stage / the stage count; value; reads and writes (next stage, finals series)'),
          ('liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê os seis primeiros', 'league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads the top six'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; grava as vagas 0 e 1, lê a tabela da fase 0', "league +0xc: the stages (pointer array); pointer; writes slots 0 and 1, reads stage 0's table"),
          ('liga +0xc2 / +0xc3 / +0xc4 / +0xc5: pontos por vitória e empate, categoria dos jogos, critérios de desempate; valor; lê', 'league +0xc2 / +0xc3 / +0xc4 / +0xc5: points for a win and a draw, match category, tie-breakers; value; reads'),
          ('liga +0x40 / +0x51 / +0xd9 / +4: o ano, a importância, os bits de regra, o registro da competição; valor; lê', 'league +0x40 / +0x51 / +0xd9 / +4: the year, the importance, the rule bits, the competition record; value; reads'),
          ('fase de copa +0xa7 / +0x3e: os times (6 bytes) / quantos; lê (comparador)', 'cup stage +0xa7 / +0x3e: the teams (6 bytes) / how many; reads (comparator)')]
INTERESSA = []
