# As semifinais dos estaduais e regionais: Pernambucano, Catarinense e Paulista (o slot que monta o mata-mata depois dos pontos corridos, em bra_reg_pern.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp;
# agente C da frente das competições, 04/11/2026). O mesmo molde nas sete semifinais (Baiano, Norte, Nordeste,
# Pernambucano, Catarinense, Paulista e o mata-mata do Gaúcho, 4 times) e nos dois mata-matas de 8 (Centro-Oeste e
# Goiano): só mudam o fonte, as linhas das caixas e o arranjo; conferido instrução a instrução (c_diff) contra a
# 0x430050 e a 0x4312d0. Rodam da fase seguinte (advance_stage, call direto com o ecx da liga -> thiscall, ret 0)
# quando a fase passa de −1 a 0; na carreira de 8 ligas até 2002-06-01, duas vezes cada (docs/cobertura_competicoes.txt).
# - 0x439710 BraRegPern::create_semi_final_stage: Pernambucano, 1º × 4º, 2º × 3º.
# - 0x43bf50 BraRegSanta::create_semi_final_stage: Catarinense, 1º × 4º, 2º × 3º.
# - 0x43cf80 BraRegSp::create_semi_final_stage: Paulista, 1º × 4º, 2º × 3º.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl; 0x944cf1 (operator new): push 0xb2; call; add esp, 4 -> cdecl;
# - a virtual +0x3c (create_round_table): push 0; push &extra; push &rodadas; push 0; mov ecx, esi -> thiscall, ret 0x10;
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 pushes; mov ecx, eax -> thiscall, ret 0x3c (conferido no mapa e na cpz_1:
#   lê n, as rodadas, o ano e o extra como words; a cópia passa os valores limpos);
# - 0x51c800 (Cup::set_up_round): push 0; mov ecx, [[+0xc]] -> thiscall, ret 4;
# - 0x9452ca (free): push t; call; push lista; call; add esp, 8 -> cdecl;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (aloca, monta a rodada com sorteio), conferindo o plano feito antes: os clubes das linhas
# na lista da fase (como conjunto), +0x3e, +4 da fase e +0x30 = 0 (o modelo da cpz_1).
# Determinismo: as sementes e a lista vêm da tabela; nada de pilha, heap ou endereço como valor (as sementes ficam
# num arranjo local, como na original, que o construtor copia).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqn8_mata_mata_do_pernambucano', 0x439710, [0x6a, 0xff, 0x68, 0xee, 0x49, 0x95, 0x00]),
           ('cqn8_mata_mata_do_catarinense', 0x43bf50, [0x6a, 0xff, 0x68, 0xee, 0x4a, 0x95, 0x00]),
           ('cqn8_mata_mata_do_paulista', 0x43cf80, [0x6a, 0xff, 0x68, 0x5e, 0x4b, 0x95, 0x00])]
DOCS = ('as semifinais dos estaduais e regionais: Pernambucano, Catarinense e Paulista',
        'the semi-finals of the state and regional leagues: Pernambuco, Santa Catarina and Sao Paulo',
        [('cqn8_mata_mata_do_pernambucano', 'BraRegPern::create_semi_final_stage', '0x439710', 'bra_reg_pern.cpp', 'thiscall, ret 0',
          'O Pernambucano: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado])',
          'Pernambuco: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])'),
         ('cqn8_mata_mata_do_catarinense', 'BraRegSanta::create_semi_final_stage', '0x43bf50', 'bra_reg_santa.cpp', 'thiscall, ret 0',
          'O Catarinense: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado])',
          'Santa Catarina: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])'),
         ('cqn8_mata_mata_do_paulista', 'BraRegSp::create_semi_final_stage', '0x43cf80', 'bra_reg_sp.cpp', 'thiscall, ret 0',
          'O Paulista: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado])',
          'Sao Paulo: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])')])
CAMPOS = [('liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê', 'league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; grava a primeira', 'league +0xc: the stages (pointer array); pointer; writes the first'),
          ('liga +0x30: a fase atual; valor; grava 0', 'league +0x30: the current stage; value; writes 0'),
          ('liga +0x40: o ano da temporada; valor; lê', 'league +0x40: the season year; value; reads'),
          ('fase de copa +0xa7: a lista dos times (6 bytes, +0 o clube); ponteiro; lê (comparador)', 'cup stage +0xa7: the team list (6 bytes, +0 the club); pointer; reads (comparator)')]
INTERESSA = []
