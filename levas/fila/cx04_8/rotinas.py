# As historias do jogador infeliz com o treino, da briga resolvida e do relatorio de treino (european_cup.cpp pelo
# mapa; leitor cx04, rodada 5, 24/11/2026). Simulacao, carreira ENG: chamadas pelo treino (0x8b211d) e pelo dia do jogo
# (0x5d6e89, 0x5cf389, 0x5d4ac4).
# Convencoes conferidas: as tres cdecl (add esp, 4 / 8 / 0x10); as do exe como no cx04_7, mais
# CONTRACT_MANAGER::get_contract 0x4cf5a0 (mov ecx, [0xae1050]; push jogador -> thiscall, ret 4) e
# contract_manager_get_deserved_squad_status 0x4ca470 (mov ecx, [0xae1050]; push contrato; push jogador -> thiscall,
# ret 8; al com sinal). Quadro SEH so para a noticia local. Prologo mov eax, fs:[0] (6 bytes). Iguais no
# cm0102-1080.exe.
# - 0x5885c0 news_post_player_unhappy_with_training(jogador): noticia 0xbcd; vaga 5 = sorteio de 3.
# - 0x5888a0 news_post_feud_settled(jogador, outro): noticia 0xbca; o jogador e o outro.
# - 0x588cd0 news_post_training_report(jogador, relator, clube, elogio): noticia 0x138d; elogio: sorteio de 2 (mais de
#   24 anos) ou 3 e 0x3e7; critica: a avaliacao pelo contrato (status merecido, profissionalismo, ambicao, pressao abaixo
#   de 21 anos, as queixas 0x30321 do contrato +0x45) e 0.
# Defeitos do original: a 0x5888a0, com o jogador sem clube, envia ao clube nulo sem o teste do humano; a 0x588cd0, na
# critica de um jogador sem contrato, le as queixas do endereco 0x45 (queda); a copia faz o mesmo.
# Comparador: compare_story_post do cx04_7 (integrar depois dele).
SAVE = False
ROTINAS = [('news_post_player_unhappy_with_training', 0x5885c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('news_post_feud_settled', 0x5888a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('news_post_training_report', 0x588cd0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as histórias do jogador infeliz com o treino, da briga resolvida e do relatório de treino',
        'the unhappy-with-training, feud settled and training report stories',
        [('news_post_player_unhappy_with_training', 'news_post_player_unhappy_with_training', '0x5885c0', 'european_cup.cpp', 'cdecl',
          'manda ao técnico humano do clube a história de que um jogador está infeliz com o treino, com a redação sorteada entre 3',
          'sends the human manager of the club the story that a player is unhappy with training, with the wording drawn among 3'),
         ('news_post_feud_settled', 'news_post_feud_settled', '0x5888a0', 'european_cup.cpp', 'cdecl',
          'manda ao técnico humano do clube a história de que dois jogadores que brigavam fizeram as pazes',
          'sends the human manager of the club the story that two feuding players settled their differences'),
         ('news_post_training_report', 'news_post_training_report', '0x588cd0', 'european_cup.cpp', 'cdecl',
          'manda ao técnico do clube o relatório de treino de um auxiliar sobre um jogador: o elogio com a redação sorteada (entre 2 para os de mais de 24 anos, entre 3 para os outros) ou a crítica pela situação no contrato (o status que ele merece, o profissionalismo, a ambição, a pressão nos de menos de 21 anos e as queixas)',
          'sends the club\'s manager an assistant\'s training report on a player: the praise with a drawn wording (among 2 for those over 24, among 3 for the others) or the criticism by his contract situation (the status he deserves, professionalism, ambition, pressure for those under 21 and grievances)')])
CAMPOS = []
INTERESSA = []
