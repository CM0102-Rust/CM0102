# A noticia do desabafo de um jogador depois da partida (Media::create_player_outburst_story 0x733a60, media.cpp;
# leitor sx01d, rodada 5, 23/11/2026). A rotina inteira (0x733a60..0x733fb6, ret 8 em 0x733fb4; o resto e enchimento).
# Chamada pelos eventos do pos-jogo (0x6b7787; em Rust match_pl.rs pelo despacho dispatch_post_match_call, tipo T2) e
# pela create_player_media_story (0x72c4cd, 0x72cc59). Atras da porta de gravacao do pos-jogo (POS_PROLOGUES em
# match_day.rs): no modo comparar a porta e instalada antes e o desvio deste item e recusado (a porta chama a
# original, como na 0x759610 do sx01c_2); o porte so roda no modo Rust.
# Depende do sx01d_2 (news_shootout_result); a struct NewsSlots deste item e usada pelos seguintes (sx01d_6, sx01d_7).
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - a rotina: thiscall(midia, registro, partida), ret 8; o registro: +0 id do jogador, +4 indice do clube, +0xc byte,
#   +0xe marca do relatorio do auxiliar, +0xf id da outra pessoa (-1), +0x13 o tipo (palavra);
# - 0x54a110 Date::Date thiscall(registro da partida) antes dos testes; 0x757da0 thiscall(midia, jogador, clube, 0xbd9,
#   tipo (palavra com o resto do edx), 0) ret 0x14; 0x77bbe0 NewsItem::NewsItem(local, 0xbd9, 0); 0x785790 set_slot
#   (k corrente em [esp+0x13]); 0x600a50 cdecl(clube, 0, 1, 0, 0) (duas vezes); 0x735870 thiscall(midia, tecnico,
#   jogador, tipo (cx com o resto do ecx)) ret 0xc; 0x7e4750 cdecl(partida, clube, registro); 0x7e4de0 cdecl(registro,
#   noticia, &k); 0x757fa0 cdecl(jogador); 0x7dab90 thiscall([0xb74430], clube, veiculo, nacao ou -1); 0x7861e0
#   thiscall([0xb7431c], noticia, clube) ret 8; 0x8d1da0 thiscall([0xde1f54], jogador, 0x1a, buf, 0x1e) -> al;
#   0x755340 thiscall(midia, tipo (ax com o resto do eax), jogador, clube, 0, -1) ret 0x14; 0x757a50 thiscall(midia,
#   jogador, clube, 0xbd9, byte do tipo, 0) ret 0x14; 0x741020 thiscall(midia, jogador, clube, tipo (dx), 0) ret 0x10.
# A copia passa o tipo como palavra sem sinal estendida (o original deixa lixo na palavra alta do registro; as
# rotinas chamadas so leem a palavra ou o byte).
# Defeito do original: o indice do clube e aceito ate a contagem de clubes inclusive (jg), um alem da tabela.
SAVE = False
ROTINAS = [('create_player_outburst_story', 0x733a60, [0x6a, 0xff, 0x68, 0xe6, 0xd6, 0x95, 0x00])]
DOCS = ('a noticia do desabafo de um jogador',
        "the news of a player's outburst",
        [('create_player_outburst_story', 'Media::create_player_outburst_story', '0x733a60', 'media.cpp', 'thiscall, ret 8',
          'cria a noticia 0xbd9 do desabafo de um jogador contra o clube ou os colegas (ou, com a marca do relatorio, o relato do auxiliar sobre a atitude dele) depois de uma partida, a partir do registro do desabafo: nada sem o registro, a ficha com reputacao atual de pelo menos 3500, o indice do clube entre 0 e a contagem de clubes (inclusive) ou a nacao do clube; sem a marca, nada se a historia ja foi registrada; com ela, nada sem a outra pessoa. Vagas: o jogador e os nomes, o indice do clube, o byte +0xc, o tipo, sorteio(2), a marca, 30, a resposta do tecnico do computador (so sem a marca e em clube nao humano; senao 0), a outra pessoa e os nomes (-1 x 4 sem ela), depois com partida 1, o registro da partida e a disputa de penaltis, sem partida 0, o veiculo e o comentarista. Com a marca vai para o tecnico do clube; sem ela para cada clube humano que acompanha o jogador (marcas 0x1a, ate 30), o elenco reage e a historia e registrada; por fim, sem a marca e em clube nao humano, o tecnico do computador responde (0)',
          "creates news item 0xbd9 of a player's outburst at his club or team-mates (or, with the report flag, the assistant's report on his attitude) after a match, from the outburst record: nothing without the record, the sheet with a current reputation of at least 3500, a club index from 0 to the club count (inclusive) or the club's nation; without the flag, nothing when the story is already logged; with it, nothing without the other person. Slots: the player and his names, the club index, the byte +0xc, the kind, draw(2), the flag, 30, the computer manager's response (only without the flag and for a club that is not human; otherwise 0), the other person and his names (-1 x 4 without him), then with a match 1, the match record and the shoot-out, without one 0, the outlet and the pundit. With the flag it goes to the club's manager; without it to every human club following the player (marks 0x1a, up to 30), the squad reacts and the story is logged; last, without the flag and for a club that is not human, the computer manager answers (0)")])
