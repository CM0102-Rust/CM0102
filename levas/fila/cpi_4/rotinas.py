# Regras de transferência da nação (transfer_manager.cpp) e o pow do game.cpp: o texto da notícia do fim do prazo
# (TRANSFER_MANAGER::write_deadline_news_text 0x8e05a0, chamada pela 0x77bc80 das notícias), se o empréstimo pode
# acabar (TRANSFER_MANAGER::can_end_loan 0x8d0530; shortlist e telas de transferência) e o pow_double 0x5d5a20
# (chamado pela 0x5ce2d0). As duas primeiras só escolhem as regras da nação (28 em +0x8ac) e chamam um método delas
# (+0x30, +0x24). Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem
# sorteio próprio, sem locais. Comparadores: o texto só pela original (o método das regras escreve); o empréstimo
# pelo valor quando nenhuma regra responde; o pow bit a bit.
SAVE = False
ROTINAS = [('texto_do_fim_do_prazo', 0x8e05a0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('pode_encerrar_emprestimo', 0x8d0530, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
           ('pow_do_game', 0x5d5a20, [0xdd, 0x44, 0x24, 0x04, 0xdd, 0x44, 0x24, 0x0c])]
DOCS = ('as regras de transferência da nação: o texto da notícia do fim do prazo e o fim do empréstimo; o pow do game.cpp',
        'the nation transfer rules: the deadline news text and the loan ending; the game.cpp pow',
        [('texto_do_fim_do_prazo', 'TRANSFER_MANAGER::write_deadline_news_text', '0x8e05a0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o texto da notícia do fim do prazo pelas regras da nação da vaga 0 da notícia (método +0x30)',
          'the deadline news text through the rules of the nation in news slot 0 (method +0x30)'),
         ('pode_encerrar_emprestimo', 'TRANSFER_MANAGER::can_end_loan', '0x8d0530', 'transfer_manager.cpp', 'thiscall, ret 0x14',
          'com o clube dono do jogador na mesma nação do clube, as regras dessa nação respondem (método +0x24); senão 1',
          'with the player parent club in the same nation as the club, that nation rules answer (method +0x24); else 1'),
         ('pow_do_game', 'pow_double', '0x5d5a20', 'game.cpp', 'cdecl',
          'o pow da CRT do jogo (0x945ff0) de dois double, resultado em st0',
          'the game CRT pow (0x945ff0) of two doubles, result in st0')])
CAMPOS = [('gerenciador de transferências +0x8ac: as 28 regras de nação (regras +0xa nação, métodos +0x24 e +0x30); clube +0x53 nação; jogador +0x39 clube; lê (texto_do_fim_do_prazo, pode_encerrar_emprestimo)',
           'transfer manager +0x8ac: the 28 nation rules (rules +0xa nation, methods +0x24 and +0x30); club +0x53 nation; player +0x39 club; reads (texto_do_fim_do_prazo, pode_encerrar_emprestimo)')]
INTERESSA = ['SGE']
