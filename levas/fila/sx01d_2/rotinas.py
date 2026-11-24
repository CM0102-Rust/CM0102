# As noticias da imprensa sobre o gesto obsceno de um jogador (Media::queue_obscene_gesture_story 0x7570b0) e sobre o
# jogador que quer jogar no time titular (Media::queue_wants_first_team_story 0x75a640), media.cpp; leitor sx01d,
# rodada 5, 23/11/2026. As duas rotinas inteiras (0x7570b0..0x75729f, ret 0xc; 0x75a640..0x75a93e, ret 8; o resto ate
# a proxima fronteira e enchimento).
# Chamadas: a 0x7570b0 so pela apply_obscene_gesture_ban (0x7daf50, Rust em printouts.rs, pelo endereco
# FN_MEDIA_OBSCENE_GESTURE_STORY); a 0x75a640 pela revisao do elenco (0x5d24a2) e pela apply_wants_first_team_response
# (0x75b440, a reacao do jogador). Nenhuma das duas aparece na cobertura ENG ate 2002-06-01 (docs/cobertura_simulacao.txt).
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - 0x7570b0 thiscall(midia, jogador, clube, partida), ret 0xc; o ecx nao e lido; 0x54a110 Date::Date thiscall(registro)
#   antes dos testes; 0x77bbe0 NewsItem::NewsItem thiscall(local, 0xbcb, 0); 0x785790 set_slot thiscall(noticia, k,
#   valor) ret 8; 0x7e4750 cdecl(partida, clube, registro); 0x7e4de0 cdecl(registro, noticia, &k) com k = 6;
#   0x8d1da0 thiscall([0xde1f54], jogador, 0xda, buf, 0x1e) -> al (com sinal), so com [0xde1f54]; 0x7861e0
#   thiscall([0xb7431c], noticia, clube) ret 8;
# - 0x75a640 thiscall(midia, jogador, reacao), ret 8; 0x4cf5f0 thiscall([0xae1050], jogador, clube); 0x5481a0 cdecl
#   (jogador); 0x757fa0 cdecl(jogador); 0x7dab90 thiscall([0xb74430], clube, veiculo, nacao ou -1); 0x546ab0 cdecl
#   (jogador, clube); 0x5b7d20 cdecl(ficha) -> ax (movsx); 0x600a50 cdecl(clube, 0, 1, 0, 0); 0x735870 thiscall(midia,
#   tecnico, jogador, 0x83) ret 0xc; 0x544520 cdecl(jogador, 0); 0x7863f0 thiscall([0xb7431c], noticia, nacao) ret 8.
SAVE = False
ROTINAS = [('queue_obscene_gesture_story', 0x7570b0, [0x6a, 0xff, 0x68, 0x06, 0xd9, 0x95, 0x00]),
           ('queue_wants_first_team_story', 0x75a640, [0x6a, 0xff, 0x68, 0xbb, 0xd9, 0x95, 0x00])]
DOCS = ('as noticias do gesto obsceno e do jogador que quer ser titular',
        'the news of the obscene gesture and of the player who wants first team football',
        [('queue_obscene_gesture_story', 'Media::queue_obscene_gesture_story', '0x7570b0', 'media.cpp', 'thiscall, ret 0xc',
          'monta a noticia 0xbcb do jogador filmado fazendo gestos obscenos para a torcida: nada sem o jogador, a ficha com reputacao atual de pelo menos 3500 ou o clube; vagas 0..3 o jogador e os nomes (o nome comum -1 quando nao ha), 4 o clube, 5 sorteio(2), depois o registro da partida (a partir da vaga 6) e, na vaga seguinte, a disputa de penaltis: 0 sem disputa, 1 se o clube venceu, 2 se perdeu, nenhuma vaga com os gols iguais; vai para cada clube humano que acompanha o jogador (ate 30, marcas 0xda)',
          'builds news item 0xbcb about the player filmed making obscene gestures to the fans: nothing without the player, the sheet with a current reputation of at least 3500 or the club; slots 0..3 the player and his names (the common name -1 when he has none), 4 the club, 5 draw(2), then the match record (from slot 6) and, in the next slot, the shoot-out: 0 without one, 1 when the club won it, 2 when it lost it, no slot with equal goals; it goes to every human club following the player (up to 30, marks 0xda)'),
         ('queue_wants_first_team_story', 'Media::queue_wants_first_team_story', '0x75a640', 'media.cpp', 'thiscall, ret 8',
          'monta a noticia 0xbe8 do jogador insatisfeito que quer jogar no time titular: nada sem o jogador, a nacao, o clube com tecnico e estadio, a ficha ou o contrato no clube; vagas 0..3 o jogador, 4..7 o tecnico, 8 o clube, 9 as posicoes naturais, 10 sorteio(2), 11 o veiculo, 12 o comentarista, 13 as marcas de forca, 14 a mascara de posicao da ficha, 15 a reacao (argumento), 16 a moral do contrato, 17 a idade, 18 a resposta do tecnico do computador (tipo 0x83; 0 num clube humano), 19 a situacao no elenco >> 4, 20 um quando agressividade + temperamento fica abaixo de 10 e a ambicao passa de 10, 21 se esta na lista de transferencias ou pediu; com a reacao vai para o tecnico do clube, sem ela para todos os tecnicos da nacao quando a reputacao mundial passa de 4750, senao para o tecnico do clube',
          "builds news item 0xbe8 about the unhappy player who wants first team football: nothing without the player, the nation, the club with a manager and a stadium, the sheet or the contract at the club; slots 0..3 the player, 4..7 the manager, 8 the club, 9 the natural positions, 10 draw(2), 11 the outlet, 12 the pundit, 13 the strength flags, 14 the sheet's position mask, 15 the reaction (argument), 16 the contract's morale, 17 the age, 18 the computer manager's response (kind 0x83; 0 for a human club), 19 the squad status >> 4, 20 one when aggression + temperament is below 10 and ambition above 10, 21 whether he is transfer listed or asked to be; with the reaction it goes to the club's manager, without it to every manager of the nation when the world reputation is above 4750, otherwise to the club's manager")])
