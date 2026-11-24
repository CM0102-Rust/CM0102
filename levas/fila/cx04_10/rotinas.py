# As historias do credito ao tecnico pelo premio, da multa disciplinar da FA e da mudanca do sistema de transferencias
# (european_cup.cpp pelo mapa; leitor cx04, rodada 5, 24/11/2026). Simulacao, carreira ENG: os premios do mes
# (0x86c681, 0x86cba1), a disciplina dos clubes (0x4a080d, 0x4a0877) e o dia do jogo (0x5ccc34, 0x5cdbb6, 0x82e300).
# Convencoes conferidas: as tres cdecl (add esp, 0xc / 0xc / 4); as do exe como no cx04_7, mais
# News::send_to_nation_managers 0x7863f0 (push nacao; push noticia; mov ecx, [0xb7431c] -> thiscall, ret 8) e
# News::send_to_all_managers 0x786780 (push noticia -> thiscall, ret 4). Quadro SEH so para a noticia local. Prologos
# mov eax, fs:[0] (6 bytes) e, na 0x58a890, push -1; push 0x958cdb (7 bytes). Iguais no cm0102-1080.exe.
# - 0x58a070 news_post_player_credits_manager_for_award(jogador, clube, premio): noticia 0xbe3; o molde do cx04_9
#   (club_player_story) com o premio obrigatorio e o id dele na vaga 10.
# - 0x58a4d0 news_post_fa_disciplinary_fine(clube, tipo, valor): noticia 0x178c; o clube (0), o tecnico (1..4, ou -1
#   nas quatro, ramo morto: o tecnico ja foi exigido), a nacao (5), o tipo (6) e o valor (7); com tipo e reputacao do
#   clube acima de 4750 aos tecnicos da nacao, senao ao do clube.
# - 0x58a890 news_post_transfer_system_change(tipo): noticia 0xfc1; vaga 0 = tipo; a todos os tecnicos.
# Comparador: compare_story_post do cx04_7; usa tambem o cx04_9 (integrar depois dos dois).
SAVE = False
ROTINAS = [('news_post_player_credits_manager_for_award', 0x58a070, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('news_post_fa_disciplinary_fine', 0x58a4d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('news_post_transfer_system_change', 0x58a890, [0x6a, 0xff, 0x68, 0xdb, 0x8c, 0x95, 0x00])]
DOCS = ('as histórias do crédito pelo prêmio, da multa da FA e do sistema de transferências',
        'the award credit, FA fine and transfer system stories',
        [('news_post_player_credits_manager_for_award', 'news_post_player_credits_manager_for_award', '0x58a070', 'european_cup.cpp', 'cdecl',
          'manda ao técnico do clube a história de que um jogador credita ao técnico um prêmio que ganhou, com o prêmio',
          'sends the club\'s manager the story that a player credits the manager for an award he won, with the award'),
         ('news_post_fa_disciplinary_fine', 'news_post_fa_disciplinary_fine', '0x58a4d0', 'european_cup.cpp', 'cdecl',
          'publica a história da multa da federação a um clube pelo histórico disciplinar, com o técnico, a nação, o tipo e o valor: um clube de reputação acima de 4750, com tipo, sai para os técnicos da nação; senão só para o do clube',
          'posts the story of the FA fining a club for its disciplinary record, with the manager, the nation, the kind and the amount: a club of reputation above 4750, with a kind, goes to the nation\'s managers; otherwise only to the club\'s'),
         ('news_post_transfer_system_change', 'news_post_transfer_system_change', '0x58a890', 'european_cup.cpp', 'cdecl',
          'publica para todos os técnicos a história da mudança do sistema de transferências da FIFA (ou, antes dela, a corrida dos clubes para renovar contratos)',
          'posts to every manager the story of the FIFA transfer system change (or, before it, the clubs\' rush to renew contracts)')])
CAMPOS = []
INTERESSA = []
