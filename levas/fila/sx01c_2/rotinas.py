# A noticia da imprensa sobre um jogador expulso (Media::queue_player_sent_off_story 0x759610, media.cpp; leitor sx01c,
# rodada 5, 23/11/2026). A rotina inteira (0x759610..0x759a0b, ret 0xc em 0x759a08; o mapa dava 1024 bytes ate a
# proxima fronteira, o resto e enchimento). Chamada so pelo pos-jogo (0x6b7555, os eventos da partida, tipo 0x82; em
# Rust match_pl.rs pelo despacho dispatch_post_match_call). Igual no cm0102-1080.exe.
# Convivencia com a porta de gravacao do pos-jogo (POS_PROLOGUES em match_day.rs, a porta da 0x759610): no modo
# comparar a porta e instalada antes (instalar(true)) e o desvio deste item e recusado (os bytes ja sao o jmp da
# porta): ORIGINAIS fica com o endereco e a porta chama a original pela sua trampolim, como na 0x5afe00
# (fog_after_match) e na 0x7daf50 (apply_obscene_gesture_ban). O porte so roda no modo Rust (carreiras e mutacoes); o
# comparador (so a original) nao chega a rodar no jogo real. Nada a mudar no motor.
# Convencoes conferidas na desmontagem:
# - a rotina: thiscall(midia, jogador, outro, partida), ret 0xc; o ecx (midia) vai para a 0x735870;
# - 0x77bbe0 NewsItem::NewsItem thiscall(local, 0xbe6, 0); 0x785790 set_slot thiscall(noticia, k, valor) ret 8 (k em
#   cl, o resto do ecx e lixo: so o byte e lido);
# - 0x7c1e30 thiscall([0xb74350], ficha, 0xb, 2, 6.5f) ret 0x10, st0 -> _ftol;
# - 0x600a50 cdecl(clube, 0, 1, 0, 0); 0x735870 thiscall(midia, tecnico, jogador, 0x82) ret 0xc;
# - 0x54a110 Date::Date thiscall(registro); 0x7e4750 cdecl(partida, clube, registro); 0x7e4de0 cdecl(registro,
#   noticia, &k);
# - 0x7863f0 thiscall([0xb7431c], noticia, nacao) ret 8 (reputacao mundial da ficha, palavra com sinal, acima de
#   0x128e); senao 0x7861e0 thiscall([0xb7431c], noticia, clube) ret 8.
SAVE = False
ROTINAS = [('queue_player_sent_off_story', 0x759610, [0x6a, 0xff, 0x68, 0x83, 0xd9, 0x95, 0x00])]
DOCS = ('a noticia da imprensa sobre um jogador expulso',
        'the press story about a player sent off',
        [('queue_player_sent_off_story', 'Media::queue_player_sent_off_story', '0x759610', 'media.cpp', 'thiscall, ret 0xc',
          'monta a noticia 0xbe6 do jogador expulso: nada sem a nacao, o clube com tecnico e a ficha do jogador, e o clube com tecnico e a ficha do outro jogador; vagas 0..3 o jogador e os nomes, 4..7 o outro, 8 sorteio(2), 9 e 10 os clubes, 11 a estatistica 0xb da ficha (os cartoes vermelhos, pelo mapa), 12..15 e 16..19 os tecnicos dos dois clubes, 20 a resposta do tecnico do computador (tipo 0x82) quando o clube do jogador nao e humano, senao 0, e o registro da partida; vai para todos os tecnicos da nacao quando a reputacao mundial passa de 4750, senao para o tecnico do clube',
          'builds news item 0xbe6 about the player sent off: nothing without the nation, the club with a manager and the player\'s sheet, and the other player\'s club with a manager and sheet; slots 0..3 the player and his names, 4..7 the other, 8 draw(2), 9 and 10 the clubs, 11 the sheet\'s stat 0xb (red cards, by the map), 12..15 and 16..19 the two clubs\' managers, 20 the computer manager\'s response (kind 0x82) when the player\'s club is not human, otherwise 0, and the match record; it goes to every manager of the nation when the world reputation is above 4750, otherwise to the club\'s manager')])
