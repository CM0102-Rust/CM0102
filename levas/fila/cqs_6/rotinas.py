# O Troféu da Liga inglesa (LDV Vans Trophy): a final e a classificação final nas estatísticas (eng_auto_cup.cpp, em
# cups\). Frente das competições (Agente D, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x56bce0 (EngAutoCup::create_final_stage, 501 bytes): a final entre os campeões das duas chaves (2002-070).
# - 0x56c010 (EngAutoCup::rank_teams, 558 bytes): a classificação final (Comp_Stats) pela rodada em que cada clube caiu
#   (2001-000; passageira: só as estatísticas da copa).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Convenções conferidas (lição da 258a): as duas thiscall, ret 0. Dentro: 0x944e46 / 0x944cf1 / 0x9452ca (push; add
# esp, 4 -> cdecl); o método +0x3c (push 0; push &extra; push &rodadas; push 1; mov ecx -> thiscall, ret 0x10);
# Cup_Stage::Cup_Stage 0x522e00 (15 pushes; mov ecx, nova -> thiscall, ret 0x3c); Cup::draw_round 0x51c410 (push 0; mov
# ecx, fase -> thiscall, ret 4); qsort 0x945333 (4 pushes; add esp, 0x20 junto -> cdecl) com 0x4b07d0;
# Comp_Stats::set_club_final_position 0x4a2540 (push posição; push clube; mov ecx, [esi+8] -> thiscall, ret 8: a posição
# vai no dword do byte local com lixo em cima; a 0x4a2540 só lê o byte); a caixa (aviso_em).
# Comparadores: só a original (a final aloca e sorteia; a classificação acha ou acrescenta entradas nas estatísticas), a
# final conferindo a fase em [+0xc][1].
# Determinismo: nada lê pilha, heap ou endereço como valor (o lixo do dword da posição não é lido).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs6_trofeu_final', 0x56bce0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqs6_trofeu_classificacao', 0x56c010, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('o Troféu da Liga inglesa: a final entre os campeões das chaves e a classificação final',
        'the English Football League Trophy: the final between the section winners and the final ranking',
        [('cqs6_trofeu_final', 'EngAutoCup::create_final_stage', '0x56bce0', 'eng_auto_cup.cpp', 'thiscall, ret 0',
          'regra: quando as chaves acabam, a final do Troféu é um jogo (Cup_Stage de 2, fase 1) entre o que segue vivo na lista da copa (situação 1) e o que segue vivo na fase das chaves; a rodada é sorteada na hora (o mando)',
          'rule: when the sections end, the Trophy final is a single tie (2-team Cup_Stage, stage 1) between the one still alive in the cup list (status 1) and the one still alive in the section stage; the round is drawn at once (home side)'),
         ('cqs6_trofeu_classificacao', 'EngAutoCup::rank_teams', '0x56c010', 'eng_auto_cup.cpp', 'thiscall, ret 0',
          'regra: a classificação final do Troféu nas estatísticas: os dois de maior reputação de cada metade disputam o 1º e o 2º (o de reputação maior fica em 1º); os outros, rodada a rodada da última para a primeira, ficam com a mesma posição os que caíram na mesma rodada (3, depois 3 + 2 × quantos caíram antes...)',
          'rule: the Trophy final ranking in the statistics: the highest-reputation club of each half takes 1st and 2nd (the one with the higher reputation is 1st); the others, round by round from the last to the first, share one position per round (3, then 3 + 2 × those already placed...)')])
CAMPOS = [('copa +0xa7 lista dos inscritos (6 bytes: clube, +5 situação); ponteiro; lê', 'cup +0xa7 entrants list (6 bytes: club, +5 status); pointer; reads'),
          ('copa +0xc fases ([0] as chaves, [1] a final); ponteiro; lê e grava', 'cup +0xc stages ([0] sections, [1] final); pointer; reads and writes'),
          ('copa +0xa3 tabela das rodadas (0x68 bytes por rodada, +0x1a os clubes da rodada), +0x3c rodadas; lê', 'cup +0xa3 round table (0x68 bytes per round, +0x1a the round\'s clubs), +0x3c rounds; reads'),
          ('copa +8 estatísticas (Comp_Stats); ponteiro; grava a posição final de cada clube', 'cup +8 statistics (Comp_Stats); pointer; writes each club\'s final position'),
          ('clube +0x80 reputação (word); valor; lê', 'club +0x80 reputation (word); value; reads')]
INTERESSA = []
