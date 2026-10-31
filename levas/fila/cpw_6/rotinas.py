# O prêmio do mês (month_award.cpp, award_manager.cpp): MONTH_AWARD::award_best_rated_player 0x767150 (na virada do mês,
# pelos prêmios das nações; os da Inglaterra em 0x579a37..0x579adc: o jogador e o jovem do mês de cada divisão),
# MONTH_AWARD::set_winner 0x7671c0 (pela 0x767150 e direto pelos prêmios de técnico do mês) e
# award_raise_winner_reputation 0x412f00 (pela 0x7671c0 e pelos prêmios do ano). A reputação do premiado fica no registro
# do jogador (+0x61) ou do não jogador (+0x69) e na confiança da diretoria, lidos pela carreira. Família fora das do
# brief (finance, club, stadium, game e human_manager esgotadas para simulação): month_award.cpp e award_manager.cpp,
# que nenhum outro agente usa. Iguais no cm0102-1080.exe; sem SEH; x87 só na 0x412f00 (fild × constante e _ftol
# 0x945640, precisão dupla). Comparadores: a 0x7671c0 e a 0x412f00 em ensaio (a cópia sem a notícia 0x767330, o
# registro do prêmio 0x86c8c0 e as listas dos clubes 0x842c00, que alocam) e a original a partir do mesmo estado, pela
# vaga do mês e pelas reputações; a 0x767150 só a original (só chama as duas e o melhor do mês 0x768130).
SAVE = False
ROTINAS = [('premiar_o_melhor_do_mes', 0x767150, [0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x09]),
           ('gravar_vencedor_do_mes', 0x7671c0, [0x51, 0x53, 0x8b, 0x5c, 0x24, 0x14]),
           ('reputacao_do_premiado', 0x412f00, [0x53, 0x55, 0x8b, 0x4c, 0x24, 0x1c])]
DOCS = ('o prêmio do mês: o melhor das notas, a vaga do vencedor e a reputação do premiado',
        'the month award: the best of the ratings, the winner slot and the reputation of the winner',
        [('premiar_o_melhor_do_mes', 'MONTH_AWARD::award_best_rated_player', '0x767150', 'month_award.cpp', 'thiscall, ret 8',
          'o melhor das notas do mês (0x768130) na liga do prêmio e com a idade pedida vira o vencedor do mês, com o clube dele',
          'the best of the month ratings (0x768130) in the award league and with the requested age becomes the month winner, with his club'),
         ('gravar_vencedor_do_mes', 'MONTH_AWARD::set_winner', '0x7671c0', 'month_award.cpp', 'thiscall, ret 0x10',
          'acha a vaga do mês, grava a pessoa e o clube, sobe a reputação do premiado, manda a notícia e registra o prêmio no histórico',
          'finds the month slot, stores the person and the club, raises the winner reputation, sends the news and records the award in the history'),
         ('reputacao_do_premiado', 'award_raise_winner_reputation', '0x412f00', 'award_manager.cpp', 'cdecl',
          'jogador: as reputações mundial, nacional e atual sobem uma fração das do prêmio, até 10000 e pelo menos elas; não jogador: sobem até elas e a confiança da diretoria ganha 1000 (entre 6500 e 10000)',
          'player: the world, home and current reputations rise by a fraction of the award ones, up to 10000 and at least them; non-player: they rise up to them and the board confidence gains 1000 (between 6500 and 10000)')])
CAMPOS = [('prêmio do mês (MONTH_AWARD) +5 prêmio, +9 liga, +0x15 reputação nacional, +0x17 mundial, +0x19 de jogador, 12 vagas de 11 bytes (+0x1d pessoa, +0x21 clube, +0x25 mês, +0x26 ano); ponteiros/ids/valores; lê e grava as vagas (gravar_vencedor_do_mes)',
           'month award (MONTH_AWARD) +5 award, +9 league, +0x15 home reputation, +0x17 world, +0x19 player flag, 12 slots of 11 bytes (+0x1d person, +0x21 club, +0x25 month, +0x26 year); pointers/ids/values; reads and writes the slots (gravar_vencedor_do_mes)'),
          ('jogador (pessoa +0x61) +9 reputação nacional, +0xb atual, +0xd mundial; valores; lê e grava (reputacao_do_premiado)',
           'player (staff +0x61) +9 home reputation, +0xb current, +0xd world; values; reads and writes (reputacao_do_premiado)'),
          ('não jogador (pessoa +0x69) +8, +0xa, +0xc reputações; valores; lê e grava (reputacao_do_premiado)',
           'non-player (staff +0x69) +8, +0xa, +0xc reputations; values; reads and writes (reputacao_do_premiado)'),
          ('pessoa +0x1a nação, +0x39 clube; clube +0x53 nação; ids/ponteiros; lê (reputacao_do_premiado, premiar_o_melhor_do_mes)',
           'staff +0x1a nation, +0x39 club; club +0x53 nation; ids/pointers; reads (reputacao_do_premiado, premiar_o_melhor_do_mes)'),
          ('[[0xb67a44]] linhas de 73 bytes da diretoria por clube, +6 confiança; valor; lê e grava (reputacao_do_premiado)',
           '[[0xb67a44]] 73-byte board rows per club, +6 confidence; value; reads and writes (reputacao_do_premiado)'),
          ('[[0xde1f54] + 0x856] gerente das listas dos clubes; ponteiro; lê (reputacao_do_premiado)',
           '[[0xde1f54] + 0x856] club shortlist manager; pointer; reads (reputacao_do_premiado)')]
INTERESSA = ['CMRTE']
