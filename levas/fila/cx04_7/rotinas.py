# As historias do olheiro, da inquietacao no vestiario e do jogador infeliz com o papel (european_cup.cpp pelo mapa; sao
# historias da midia; leitor cx04, rodada 5, 24/11/2026). Simulacao, carreira ENG: chamadas pelo dia do jogo (olheiro
# 0x840a79, vestiario 0x5d70cb e outros, papel 0x8ce6c0).
# Convencoes conferidas: as tres cdecl (add esp, 8 / 0xc depois da chamada); NewsItem::NewsItem 0x77bbe0 (push 0; push
# tipo; lea ecx -> thiscall, ret 8), NewsItem::set_slot 0x785790 (push v; push k; lea ecx -> thiscall, ret 8),
# utils_random 0x90cfc0 (cdecl), News::club_has_human_manager 0x7875e0 (mov ecx, [0xb7431c]; push clube -> thiscall,
# ret 4), News::send_to_club_manager 0x7861e0 (push clube; push noticia; mov ecx, [0xb7431c] -> thiscall, ret 8),
# club_is_in_season 0x547770 (cdecl(clube, 0xae2c90)), Injury::get_player_injury 0x62ed70 (thiscall [0xb63d28]),
# Tactics::find_selection_slot 0x8972c0 (thiscall [0xde1d08], (clube, jogador, 0), al com sinal), staff_favours_club
# 0x547d60 (cdecl(jogador, clube)), staff_is_transfer_listed 0x5444a0 (cdecl(jogador, 0)). Quadro SEH so para a noticia
# local (0xe0 bytes), sem destrutor. Prologo mov eax, fs:[0] (6 bytes). Iguais no cm0102-1080.exe.
# - 0x587220 news_create_scout_watching_story(jogador, clube): noticia 0xfbd; o sorteio de 3 antes de montar.
# - 0x587c60 news_post_dressing_room_unrest(jogador, outro, tipo): noticia 0xbc7; a redacao (vaga 9) pelo tipo e pelos
#   atributos (profissionalismo +0x5b; do outro, profissionalismo, ritmo de trabalho +0x44 e espirito de equipe +0x3f da
#   ficha) com sorteios de 4 e de 3.
# - 0x5882d0 news_post_player_unhappy_with_role(jogador, clube): noticia 0xbc8; vaga 5 = esta na lista de
#   transferencias.
# Defeito do original: a 0x587c60 e a 0x5882d0, com o jogador sem clube (+0x39 = 0), pulam o teste do tecnico humano e
# enviam ao clube nulo.
# Comparador: compare_story_post (a copia ensaia sem enviar; os sorteios e a historia entregue aos humanos); os
# auxiliares (StoryTo, post_story, compare_story_post, new_local_news) servem aos itens cx04_8 a cx04_11.
SAVE = False
ROTINAS = [('news_create_scout_watching_story', 0x587220, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('news_post_dressing_room_unrest', 0x587c60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('news_post_player_unhappy_with_role', 0x5882d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as histórias do olheiro, da inquietação no vestiário e do jogador infeliz com o papel',
        'the scout watching, dressing room unrest and unhappy-with-role stories',
        [('news_create_scout_watching_story', 'news_create_scout_watching_story', '0x587220', 'european_cup.cpp', 'cdecl',
          'cria a história de que o olheiro de um clube observou um jogador e a manda ao técnico do clube do jogador, só se o clube que observa está em temporada, o jogador não está machucado e tem lugar na escalação; a redação é sorteada entre 3',
          'creates the story that a club\'s scout watched a player and sends it to the manager of the player\'s club, only when the watching club is in season, the player is not injured and has a place in the selection; the wording is drawn among 3'),
         ('news_post_dressing_room_unrest', 'news_post_dressing_room_unrest', '0x587c60', 'european_cup.cpp', 'cdecl',
          'manda ao técnico humano do clube a história da inquietação no vestiário entre um jogador e um colega ou o técnico: numa briga, a redação vai do jogador pouco profissional (às vezes ligado ao clube) ao muito profissional; com o técnico, pelo profissionalismo do jogador e pelo do outro, o ritmo de trabalho e o espírito de equipe; senão sorteada',
          'sends the human manager of the club the story of dressing room unrest between a player and a team-mate or the manager: in a feud, the wording goes from the unprofessional player (sometimes attached to the club) to the very professional one; with the manager, by the player\'s professionalism and the other\'s, his work rate and teamwork; otherwise drawn'),
         ('news_post_player_unhappy_with_role', 'news_post_player_unhappy_with_role', '0x5882d0', 'european_cup.cpp', 'cdecl',
          'manda ao técnico humano do clube a história de que um jogador está infeliz com o papel, dizendo se ele está na lista de transferências',
          'sends the human manager of the club the story that a player is unhappy with his role, saying whether he is transfer listed')])
CAMPOS = []
INTERESSA = []
