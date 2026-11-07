# A Liga dos Campeões asiática: a fase seguinte, os dois grupos e as semifinais (asia_club_champ.cpp, em eurocomp\).
# Frente das competições (Agente E, 06/11/2026), carreira de 8 ligas (o Japão jogável; a edição de 2001 fica dentro da
# janela).
# - 0x40c070 (AsiaClubChamp::advance_stage): a fase seguinte (passageira).
# - 0x40c110 (AsiaClubChamp::create_group_stages, 700 bytes): os 8 vivos dos mata-matas das duas zonas em dois
#   grupos. Duas mutações (as linhas de cada grupo de trás para a frente).
# - 0x40c3d0 (AsiaClubChamp::create_semi_final_stage): as semifinais cruzadas. Uma mutação (1º × 1º, 2º × 2º).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): a fase seguinte é a virtual +0x28 (thiscall, ret 0); os grupos e as
# semifinais thiscall, ret 0 (call direto da fase seguinte). Dentro: a virtual +0x3c (4 pushes; mov ecx -> thiscall,
# ret 0x10), 0x68b100 (22 pushes, dois ponteiros para os bytes de regras locais; mov ecx, novo -> thiscall, ret
# 0x58), 0x684230 (mov ecx -> thiscall, ret 0), 0x522e00 (15 pushes; mov ecx, novo -> thiscall, ret 0x3c), 0x51c410
# (push 0; mov ecx, fase -> thiscall, ret 4), 0x944e46, 0x944cf1, 0x9452ca e a caixa.
# Comparadores: só a original (alocam, sorteiam e mandam notícia), com o plano: a fase seguinte confere +0x30 (os
# grupos gravam 2); os grupos conferem as duas fases com os 4 vivos de cada e +0x30; as semifinais a fase nova com os 4
# cruzados (lidos antes).
# Determinismo: nada lê pilha, heap ou endereço como valor (os 8 vivos numa lista zerada; a original a deixa na pilha
# e só lê as vagas gravadas).
# Provas (06/11/2026, cópia fora do projeto em %TEMP%: motor.rs da principal com a 280a publicada e a 281a integrada,
# 1.986 rotinas, + os 12 itens cqr_10..13, cqs_10..13, cqt_10..13, 36 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 só com os 4 cqr (parou em 2001-273) e até
#   2002-06-01 com os 12 itens (parou em 2002-151; as copas de janeiro e fevereiro de 2002 e a Libertadores de 2002
#   rodam depois de outubro): 0 linhas DIVERGENCIA, 0 EXCECAO, nenhuma caixa (data, motor e aleatorio: 0 divergências);
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a (a mesma da base sem os itens: as cópias são
#   fiéis); todos os contadores das mutações rodaram;
# - mutações (mutacoes.py --carreira 2002-06-01 com as 8 ligas, base f29c85195c2a534f): as três ACUSADAS (os grupos 1 e
#   2 de trás para a frente em 2002-079, as semifinais 1º × 1º em 2002-143: a Liga dos Campeões asiática desta janela é
#   a de 2002, que acaba em maio).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqt12_asia_fase_seguinte', 0x40c070, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqt12_asia_grupos', 0x40c110, [0x6a, 0xff, 0x68, 0xcc, 0x37, 0x95, 0x00]),
           ('cqt12_asia_semifinais', 0x40c3d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a Liga dos Campeões asiática: a fase seguinte, os dois grupos e as semifinais',
        'the Asian Club Championship: the next stage, the two groups and the semi-finals',
        [('cqt12_asia_fase_seguinte', 'AsiaClubChamp::advance_stage', '0x40c070', 'asia_club_champ.cpp', 'thiscall, ret 0', 'regra: a Liga dos Campeões asiática avança uma fase quando a atual acaba; depois dos mata-matas das duas zonas (ao chegar a 1) monta os dois grupos; depois dos grupos (ao chegar a 3) as semifinais; passar da última fase é erro', "rule: the Asian Club Championship moves on one stage when the current one ends; after the two zones' knockouts (on reaching 1) it builds the two groups; after the groups (on reaching 3) the semi-finals; going past the last stage is an error"),
         ('cqt12_asia_grupos', 'AsiaClubChamp::create_group_stages', '0x40c110', 'asia_club_champ.cpp', 'thiscall, ret 0', 'regra: os 4 vivos do mata-mata da primeira zona formam o grupo 1 e os 4 da segunda zona o grupo 2 (fases de liga 1 e 2, turno único [inferência], as regras e o ano da copa); os técnicos humanos recebem a notícia', "rule: the 4 survivors of the first zone's knockout make group 1 and the 4 of the second zone group 2 (league stages 1 and 2, single round [inference], the cup's rules and year); human managers get the news"),
         ('cqt12_asia_semifinais', 'AsiaClubChamp::create_semi_final_stage', '0x40c3d0', 'asia_club_champ.cpp', 'thiscall, ret 0', 'regra: as semifinais da Liga dos Campeões asiática cruzam os grupos: 1º do grupo 1 × 2º do grupo 2 e 1º do grupo 2 × 2º do grupo 1, numa fase de copa (a fase 3) com a primeira rodada sorteada na hora', 'rule: the Asian Club Championship semi-finals cross the groups: group 1 winner × group 2 runner-up and group 2 winner × group 1 runner-up, in a cup stage (stage 3) with the first round drawn at once')])
CAMPOS = [('copa +0xa7 tabela do mata-mata da zona 1 (6 bytes: clube, +4 classe, +5 situação: 1 = vivo), +0x3e times; ponteiro/valor; lê (grupos)', 'cup +0xa7 zone 1 knockout table (6 bytes: club, +4 class, +5 status: 1 = alive), +0x3e teams; pointer/value; reads (groups)'),
          ('copa +0xc fases ([0] o mata-mata da zona 2, [1] e [2] os grupos, [3] as semifinais); ponteiro; lê e grava', 'cup +0xc stages ([0] zone 2 knockout, [1] and [2] the groups, [3] the semi-finals); pointer; reads and writes'),
          ('copa +0x2c fases, +0x30 fase atual; valor; lê e grava (fase seguinte, grupos)', 'cup +0x2c stages, +0x30 current stage; value; reads and writes (next stage, groups)')]
INTERESSA = ['CMRTE']
