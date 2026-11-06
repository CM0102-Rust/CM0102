# Os turnos do Carioca, do Mineiro, do Paranaense e da J1 (o slot create_half_season_stage de bra_reg_rio.cpp,
# bra_reg_minas_gerais.cpp, bra_reg_parana.cpp e jap_j1.cpp; agente C da frente das competições, 06/11/2026). O mesmo
# molde de 400 bytes nos quatro (conferido instrução a instrução: só mudam o fonte, a linha da caixa, 0x3d5 ou 0x278, e
# o 20º argumento do construtor, 0xb nos estaduais e 0xf na J1). Na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt) o turno do Carioca, do Mineiro e do Paranaense roda 4 vezes (os dois turnos de 2001
# e de 2002) e o da J1, 3.
# - 0x43afa0 BraRegRio::create_half_season_stage, 0x4351b0 BraRegMinasGerais::create_half_season_stage,
#   0x438760 BraRegParana::create_half_stage, 0x663c20 JapJ1::create_half_season_stage.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - os quatro, do resultado de cada jogo (0x43af20 e as irmãs): push edx (dl = +0x30 + 1); mov ecx, esi; call ->
#   thiscall, ret 4 (a rotina usa o byte baixo e empurra o dword inteiro ao construtor: a cópia faz o mesmo);
# - 0x944e46 (malloc): push 4 × +0x3e; call; add esp, 4 -> cdecl; 0x944cf1 (operator new): push 0xee; call; add esp, 4
#   -> cdecl;
# - 0x68b100 (League_Stage::League_Stage): 22 pushes; mov ecx, eax -> thiscall, ret 0x58 (a liga, +0x3e, o vetor, 1,
#   +4, 0, 0, +0xc2, +0xc3, +0xc4, &+0xc5, &{1,0,0,0}, o ano, k, 0x439/0x43a, 0, 1, 0, 8, 0xb ou 0xf, 0, 2; a original
#   empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores limpos);
# - 0x9452ca (free): push vetor; call; add esp, 4 -> cdecl;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (aloca o vetor e a fase), com o plano: a vaga k tem a fase, +0x30 = k e a fase tem os
# +0x3e clubes da liga.
# Determinismo: o vetor vem da tabela; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr1_turno_do_carioca', 0x43afa0, [0x6a, 0xff, 0x68, 0x7e, 0x4a, 0x95, 0x00]),
           ('cqr1_turno_do_mineiro', 0x4351b0, [0x6a, 0xff, 0x68, 0x0e, 0x48, 0x95, 0x00]),
           ('cqr1_turno_do_paranaense', 0x438760, [0x6a, 0xff, 0x68, 0x7e, 0x49, 0x95, 0x00]),
           ('cqr1_turno_da_j1', 0x663c20, [0x6a, 0xff, 0x68, 0x5e, 0xc1, 0x95, 0x00])]
_TURNO = ('Um turno {liga}: quando o primeiro jogo de um turno é processado, monta a fase de pontos corridos dele (uma liga de pontos corridos com todos os {n} clubes da tabela, os mesmos pontos por vitória e por empate e os mesmos critérios de desempate da liga, e uma vaga de cima, a do campeão do turno); o primeiro turno vai para a fase 0 e o segundo (a partir da {r}ª rodada, pelo +0x34 do jogo) para a fase 1; o campeão de cada turno (o primeiro da tabela da fase) vai à final',
          'One half of {liga_en}: when the first game of a half is processed, it builds that half\'s round-robin stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as the league, and one top place, the half winner\'s); the first half goes to stage 0 and the second (from round {r}, by the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final')
def _t(liga, liga_en, n, r):
    return (_TURNO[0].format(liga=liga, n=n, r=r), _TURNO[1].format(liga_en=liga_en, r=r))
DOCS = ('os turnos do Carioca, do Mineiro, do Paranaense e da J1: a fase de pontos corridos de cada turno',
        'the halves of the Rio, Minas Gerais and Parana state leagues and of the J1: each half\'s round-robin stage',
        [('cqr1_turno_do_carioca', 'BraRegRio::create_half_season_stage', '0x43afa0', 'bra_reg_rio.cpp', 'thiscall, ret 4')
         + _t('do Carioca', 'the Rio state league', 'doze', '12'),
         ('cqr1_turno_do_mineiro', 'BraRegMinasGerais::create_half_season_stage', '0x4351b0', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 4')
         + _t('do Mineiro', 'the Minas Gerais state league', 'doze', '12'),
         ('cqr1_turno_do_paranaense', 'BraRegParana::create_half_stage', '0x438760', 'bra_reg_parana.cpp', 'thiscall, ret 4')
         + _t('do Paranaense', 'the Parana state league', 'doze', '12'),
         ('cqr1_turno_da_j1', 'JapJ1::create_half_season_stage', '0x663c20', 'jap_j1.cpp', 'thiscall, ret 4')
         + _t('da J1', 'the J1', '[inferência: dezesseis]', '16')])
CAMPOS = [('liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê', 'league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads'),
          ('liga +0x3e: a quantidade de clubes; valor; lê', 'league +0x3e: the club count; value; reads'),
          ('liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; lê', 'league +0xc2 / +0xc3: points for a win / a draw; value; reads'),
          ('liga +0xc4: a categoria dos jogos; valor; lê', 'league +0xc4: the match category; value; reads'),
          ('liga +0xc5: os critérios de desempate; ponteiro (o endereço); lê', 'league +0xc5: the tie-breakers; pointer (the address); reads'),
          ('liga +0x40: o ano da temporada; valor; lê', 'league +0x40: the season year; value; reads'),
          ('liga +4: o registro da competição no banco; ponteiro; lê', 'league +4: the competition record in the database; pointer; reads'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; grava a vaga k', 'league +0xc: the stages (pointer array); pointer; writes slot k'),
          ('liga +0x30: a fase atual; valor; grava k', 'league +0x30: the current stage; value; writes k')]
INTERESSA = []
