# Os textos das historias de que o clube nao vende o jogador, da inquietacao no vestiario e do jogador infeliz com o papel
# (european_cup.cpp pelo mapa; leitor sbD, rodada 5, 24/11/2026; as sobras do cx04). Chamados so pelo despacho das
# noticias (NewsItem::write_text 0x77bc80, NEWS_WRITERS Plain: (5, 0xfbe), (4, 0xbc7), (4, 0xbc8)) quando o texto e
# escrito: na carreira automatica pelo modo CM_TEXTO_NOTICIAS (CM_TEXTOS=1 na cadeia); sem ele, abrindo a noticia jogando
# comparando. Os postadores: 0x587600 (media.rs), 0x587c60 e 0x5882d0 (cx04_7).
# Convencoes conferidas: as tres cdecl(noticia, corpo, Text) (ret simples); NewsItem::get_slot 0x785830 (push k; mov
# ecx, noticia -> thiscall, ret 4), database_format_name_from_ids 0x53e140 (cdecl, 7), langlib_sprintf 0x66f4e0 (cdecl,
# variadico), Text::set_string 0x4ae660 (thiscall, ret 4), database_staff_matches_names 0x53e990 (cdecl, 5),
# Text::add_link 0x4ae8a0 (thiscall, ret 0xc). Rotinas inteiras iguais no cm0102-1080.exe.
# - 0x587800 news_not_for_sale_text: vagas 0..3 o jogador, 5 o clube, 6 a redacao, 7..10 o presidente (-1 sem ele);
#   nomes em 0x78 bytes; no buffer 0xde1f64. Prologo sub esp, 0x184 (6 bytes).
# - 0x587e90 news_dressing_room_unrest_text: vagas 0..3 o jogador, 4..7 o outro, 8 o tipo, 9 a redacao; vinculos 0x7da.
#   Prologo mov eax, [esp+8]; sub esp, 0x338 (10 bytes).
# - 0x5883f0 news_unhappy_with_role_text: vaga 5 (byte) = na lista de transferencias. Prologo mov eax, [esp+8]; sub esp,
#   0x198 (10 bytes).
# Auxiliares comuns (pub(crate)): NewsPerson (a pessoa de 4 vagas: ler, formatar, vincular), story_newspaper (o jornal
# local/nacional) e player_story_text (a historia de um jogador so); servem aos itens sbD_2 a sbD_4. O link_story_person
# do cx04_12 pode passar a usar o NewsPerson.
# Comparadores: compare_text_writer (o texto, os vinculos e o gerador).
SAVE = False
ROTINAS = [('news_not_for_sale_text', 0x587800, [0x81, 0xec, 0x84, 0x01, 0x00, 0x00]),
           ('news_dressing_room_unrest_text', 0x587e90, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x38, 0x03, 0x00, 0x00]),
           ('news_unhappy_with_role_text', 0x5883f0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x98, 0x01, 0x00, 0x00])]
DOCS = ('os textos das histórias do clube que não vende, da inquietação no vestiário e do jogador infeliz com o papel',
        'the texts of the not-for-sale, dressing room unrest and unhappy-with-role stories',
        [('news_not_for_sale_text', 'news_not_for_sale_text', '0x587800', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("<clube> descarta a venda de <jogador>") ou o corpo da história de que o clube não vende o jogador: a redação sorteada (0, 1 ou outra) e, quando o presidente é citado, a frase dele (na redação 0 a escolha pela paridade do id do presidente; nas outras redações o nome do estádio do clube, se houver); vínculos dos nomes do jogador, do presidente e do clube',
          "writes the headline (\"<club> dismiss <player> transfer talk\") or the body of the story that the club will not sell the player: the drawn wording (0, 1 or another) and, when the chairman is quoted, his line (in wording 0 the choice by the parity of his id; in the others the club's stadium name, if any); links for the player's, the chairman's and the club's names"),
         ('news_dressing_room_unrest_text', 'news_dressing_room_unrest_text', '0x587e90', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ou o corpo da história de inquietação no vestiário pelo tipo (0 briga, 1 com o técnico, outro) e pela redação sorteada: o corpo com os nomes completos do jogador e do outro, a manchete com o nome curto do jogador (ou "Inquietação no vestiário" sem nome); vínculos de cada nome',
          "writes the headline or the body of the dressing room unrest story by the kind (0 a feud, 1 with the manager, other) and the drawn wording: the body with the player's and the other's full names, the headline with the player's short name (or \"Dressing room unrest reported\" with no name); links for each name"),
         ('news_unhappy_with_role_text', 'news_unhappy_with_role_text', '0x5883f0', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("<jogador> infeliz com o papel") ou o corpo da história do jogador infeliz com o papel, com uma redação para quem está na lista de transferências e outra para quem não está; vínculo do jogador',
          'writes the headline ("<player> unhappy with role") or the body of the unhappy-with-role story, with one wording for a transfer-listed player and another for the rest; link to the player')])
CAMPOS = []
INTERESSA = []
