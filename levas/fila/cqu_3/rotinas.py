# O resultado de cada jogo do Mineiro, do Paranaense, do Carioca e da J1 (o slot add_result de
# bra_reg_minas_gerais.cpp, bra_reg_parana.cpp, bra_reg_rio.cpp e jap_j1.cpp; agente C da frente das competições,
# 06/11/2026). O mesmo molde de 128 bytes nos quatro (conferido instrução a instrução: só mudam a rodada em que começa o
# segundo turno, 11 nos estaduais e 15 na J1, e a rotina do turno, cqr_1). Na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt) rodam 260, 260, 258 e 360 vezes (os jogos dos estaduais de 2002 e da J1 de 2001/02).
# - 0x435130 BraRegMinasGerais::add_result, 0x4386e0 BraRegParana::add_result, 0x43af20 BraRegRio::add_result,
#   0x663940 JapJ1::add_result.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas:
# - os quatro são o slot +4 da vtable (chamados por ponteiro, como o da MLS, cqk_9): a rotina lê o jogo em [esp+4] e o
#   estat em [esp+8] da entrada e termina em ret 8 -> thiscall(this, jogo, estat), ret 8;
# - 0x685d30 (League::add_result): push estat; push jogo; ecx = this (o mesmo da entrada) -> thiscall, ret 8;
# - a rotina do turno (0x4351b0, 0x438760, 0x43afa0, 0x663c20; cqr_1): push edx (dl = +0x30 + 1, sobra de registrador
#   em cima); mov ecx, esi -> thiscall, ret 4 (o turno passa o k ao construtor da fase, 0x68b100, que lê só o byte
#   para +0x44: a cópia passa o byte limpo);
# - o slot +4 da fase (League_Stage): push estat; push jogo; mov ecx, fase; call [vtable + 4] -> thiscall, ret 8.
# Comparador: só a original (o jogo entra nas tabelas e o turno pode ser montado), com o plano: a devolução 1, a marca
# +0x42 do jogo como estava, o bloco +8 da liga igual e, se o jogo não tinha fase, a fase atual já no turno do jogo,
# com a fase dele montada.
# Determinismo: só os campos do jogo e da liga.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqu3_resultado_do_mineiro', 0x435130, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
           ('cqu3_resultado_do_paranaense', 0x4386e0, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
           ('cqu3_resultado_do_carioca', 0x43af20, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
           ('cqu3_resultado_da_j1', 0x663940, [0x53, 0x8b, 0x5c, 0x24, 0x0c])]
_R = ('O resultado de cada jogo {liga}: o jogo entra na tabela geral e, se ainda não tem fase, ganha a do seu turno (o primeiro até a {r}ª rodada, o segundo a partir da {s}ª); o turno é montado quando o primeiro jogo dele chega, e o jogo entra também na tabela do turno, sem contar de novo nas estatísticas da liga; o campeão de cada turno vai à final',
      'Each game result of {liga_en}: the game goes into the overall table and, if it has no stage yet, gets its half\'s (the first up to round {r}, the second from round {s}); the half is built when its first game arrives, and the game also goes into the half table, without counting again in the league statistics; each half winner goes to the final')
DOCS = ('o resultado de cada jogo do Mineiro, do Paranaense, do Carioca e da J1: a tabela geral e a do turno',
        'each game result of the Minas Gerais, Paraná and Rio state leagues and J1: the overall and the half table',
        [('cqu3_resultado_do_mineiro', 'BraRegMinasGerais::add_result', '0x435130', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 8',
          _R[0].format(liga='do Mineiro', r=10, s=11), _R[1].format(liga_en='the Minas Gerais state league', r=10, s=11)),
         ('cqu3_resultado_do_paranaense', 'BraRegParana::add_result', '0x4386e0', 'bra_reg_parana.cpp', 'thiscall, ret 8',
          _R[0].format(liga='do Paranaense', r=10, s=11), _R[1].format(liga_en='the Paraná state league', r=10, s=11)),
         ('cqu3_resultado_do_carioca', 'BraRegRio::add_result', '0x43af20', 'bra_reg_rio.cpp', 'thiscall, ret 8',
          _R[0].format(liga='do Carioca', r=10, s=11), _R[1].format(liga_en='the Rio state league', r=10, s=11)),
         ('cqu3_resultado_da_j1', 'JapJ1::add_result', '0x663940', 'jap_j1.cpp', 'thiscall, ret 8',
          _R[0].format(liga='da J1', r=14, s=15), _R[1].format(liga_en='J1', r=14, s=15))])
CAMPOS = [('jogo +0x42: a fase do jogo na liga (0xff: nenhuma); valor; grava o turno e devolve 0xff', 'fixture +0x42: the game stage in the league (0xff: none); value; writes the half and restores 0xff'),
          ('jogo +0x34: a rodada do jogo; valor; lê', 'fixture +0x34: the game round; value; reads'),
          ('liga +0x30: a fase atual; valor; lê', 'league +0x30: the current stage; value; reads'),
          ('liga +8: o bloco de estatísticas; ponteiro; zera durante a tabela do turno e devolve', 'league +8: the statistics block; pointer; zeroes during the half table and restores')]
INTERESSA = ['CMRTE']
