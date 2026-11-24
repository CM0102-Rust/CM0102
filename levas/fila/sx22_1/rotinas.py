# weather.cpp (leitor sx22, rodada 5, 24/11/2026): as notícias de partida adiada e abandonada.
# - 0x923cc0 Weather::post_postponed_news: thiscall(Weather [0xde2f74], partida, motivo) ret 8; quadro SEH sem
#   destrutor (a notícia local não tem destrutor). Partida nula: nada. A data das notícias 0xdd6ec8 (ano palavra sem
#   sinal, depois o dia com sinal, setg) depois de hoje: nada. NewsItem::NewsItem 0x77bbe0 thiscall(local, 2, 0) ret 8;
#   set_slot 0x785790 thiscall(k, v) ret 8: 0 = id do mandante ([[+0x1c]]), 1 = id do visitante ([[+0x20]]), 2 = id da
#   competição ([[+0x14]]), 3 = o motivo (movsx do byte); News::send_to_club_manager 0x7861e0 thiscall([0xb7431c],
#   &notícia, clube) ret 8, mandante e depois visitante (o [0xb7431c] relido). Chamada pelo set_up_the_day 0x6b3c10
#   (0x6b3d4f; já em Rust, match_day, pelo exe) quando o check_postponement 0x923630 (Rust) adia a partida.
# - 0x923dd0 weather_write_postponed_news_text: thiscall(this, notícia, completa, Text) ret 0xc (o escritor da notícia
#   de tipo 2, tabela de news.rs). Manchete (0xad5220): get_slot 0x785830 0, 1, 2; os nomes curtos dos clubes (+0x38)
#   com o gênero (+0x52) e o nome da competição (registros de 107 bytes de [0xae23d0], +4) com o gênero (+0x37), na
#   ordem de push conferida (gêneros h, h, v, v, c, c; nomes h, v, c); langlib_sprintf 0x66f4e0 no 0xde1f64;
#   Text::set_string 0x4ae660; sem vínculo. Corpo: slots 0..3; motivo 4 (0xad5164), 8 (0xad50a0), 0x10 (0xad4fdc), nessa
#   ordem; argumentos: gêneros c, c, h, h, v, v (+0x37); nomes c, h, v (+4); depois Text::add_link 0x4ae8a0
#   thiscall(nome +4, 0x7d5, clube) ret 0xc do mandante e do visitante.
#   Defeito do original: sem nenhum dos três bits o corpo não formata nada e o texto é o que o 0xde1f64 guardava.
# - 0x924050 Weather::post_abandoned_news: o mesmo molde com o tipo 0x2a; chamada pelo close_day_match 0x6b5f00
#   (0x6b6074, já em Rust, pelo exe).
# Iguais no cm0102-1080.exe. Tipo: simulação (a carreira ENG passa pelo inverno: adiamentos por chuva e gelo nos
# estádios sem aquecimento; os abandonos são mais raros, ver os covered!).
# Comparadores: as duas postagens só a original (postam a notícia); o escritor por compare_text_writer.
SAVE = False
ROTINAS = [('post_postponed_news', 0x923cc0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('write_postponed_news_text', 0x923dd0, [0x8b, 0x44, 0x24, 0x08, 0x53]),
           ('post_abandoned_news', 0x924050, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o tempo: as notícias de partida adiada e abandonada',
        'the weather: the news of a postponed and an abandoned match',
        [('post_postponed_news', 'Weather::post_postponed_news', '0x923cc0', 'weather.cpp', 'thiscall, ret 8',
          'a partida de liga adiada vira notícia do tipo 2 (o mandante, o visitante, a competição e os bits do motivo) para os técnicos dos dois clubes, só a partir da data das notícias',
          'a postponed league match becomes a kind 2 news item (the home club, the away club, the competition and the reason bits) for the managers of both clubs, only from the news date on'),
         ('write_postponed_news_text', 'weather_write_postponed_news_text', '0x923dd0', 'weather.cpp', 'thiscall, ret 0xc',
          'manchete "<mandante> vs <visitante> <liga> match postponed"; corpo pelo motivo, nesta ordem: 4 mau tempo, 8 gramado encharcado, 0x10 falha dos refletores; vínculos dos dois clubes. Defeito do original: sem nenhum desses bits o texto é o que o buffer compartilhado guardava',
          'headline "<home> vs <away> <league> match postponed"; body by the reason, in this order: 4 bad weather, 8 water-logged pitch, 0x10 flood-light failure; links of both clubs. Defect of the original: with none of these bits the text is whatever the shared buffer last held'),
         ('post_abandoned_news', 'Weather::post_abandoned_news', '0x924050', 'weather.cpp', 'thiscall, ret 8',
          'a partida de liga abandonada vira notícia do tipo 0x2a (o mandante, o visitante, a competição e os bits do motivo) para os técnicos dos dois clubes, só a partir da data das notícias',
          'an abandoned league match becomes a kind 0x2a news item (the home club, the away club, the competition and the reason bits) for the managers of both clubs, only from the news date on')])
CAMPOS = []
INTERESSA = []
