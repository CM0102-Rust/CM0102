# Os vencedores do premio do ano e a noticia deles (award/year_award.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa
# award_base, year_award_obj, NewsPost, NewsSink, news_post e rehearse_news (sx05b_5): integrar depois dele.
# Rodam nos premios do fim da temporada das nacoes (NationAwards 0x768700..0x7698d0 chamam record_winners com a
# noticia): LIGAS = 'ENG' e ATE = '2002-08-01', como o sx04_2 e o mwg_5.
# Convencoes conferidas na desmontagem:
# - 0x9302f0 thiscall(registro, com noticia) ret 8: AWARD_MANAGER::add_year_award_record 0x4142c0 (registro, &+0x23,
#   &+0x27) ret 0xc; 0x9304f0 (registro) ret 4; os tres lugares (id em +0xe, +0x26, +0x3e; clube em +0x16, +0x2e,
#   +0x46; -1 nenhum); award_raise_winner_reputation 0x412f00 cdecl com as reputacoes (words) no 1o, x 0.75 (f64
#   0x969040) no 2o e x 0.5 (f64 0x968968) no 3o pelo _ftol 0x945640 (a mundial calculada primeiro); Staff_Records::
#   add_year_award_record 0x86c390 thiscall([0xdd7f00], pessoa, clube, [+5], ano (word), tipo (byte +0x1d), lugar) ret
#   0x18;
# - 0x9304f0 thiscall(registro) ret 4: NewsItem 0x77bbe0 (0x1b, 0); as vagas 0..0x16 (0xd a constante 0x420e9; 0x13
#   utils_random(3); 0x14 o dword +0x69 do vencedor; 0x15 formation_get_position_mask 0x5b7d20 (movsx) e 0x16
#   staff_get_player_strength_flags 0x546ab0 cdecl(vencedor, clube do registro) so com ficha de jogador);
#   transfer_manager_collect_player_news_recipients 0x8d1da0 thiscall([0xde1f54], vencedor, 0xda, buf, 0x1e) ret 0x10,
#   devolve al (com sinal); News::send_to_division_managers 0x7862d0, send_to_nation_managers 0x7863f0,
#   send_to_continent_managers 0x786640 e send_to_club_manager 0x7861e0 thiscall([0xb7431c], item, x) ret 8,
#   send_to_all_managers 0x786780 ret 4.
# Comparadores: record_winners so a original (historico, reputacoes e os registros sorteiam) e o historico conferido;
# post_winner_news a original posta e a copia ensaia (rehearse_news: o sorteio do modelo).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('year_award_record_winners', 0x9302f0, [0x53, 0x55, 0x56, 0x8b, 0xf1]),
           ('year_award_post_winner_news', 0x9304f0, [0x6a, 0xff, 0x68, 0xdb, 0x60, 0x96, 0x00])]
DOCS = ('os vencedores do prêmio do ano e a notícia deles',
        'the year award winners and their news',
        [('year_award_record_winners', 'YEAR_AWARD::record_winners', '0x9302f0', 'award/year_award.cpp', 'thiscall, ret 8',
          'registra o prêmio do ano: o registro vai para o histórico do gerente de prêmios; com o pedido, a notícia; depois cada um dos três lugares com pessoa: a reputação sobe pelas do prêmio (inteiras no 1º, três quartos no 2º, metade no 3º, truncadas) e o histórico dele recebe a entrada do prêmio (clube, competição, ano, tipo do registro e o lugar)',
          "records the year award: the record goes to the award manager's history; when asked, the news; then each of the three places with a person: his reputations rise by the award's (whole for the 1st, three quarters for the 2nd, half for the 3rd, truncated) and his history gets the award entry (club, competition, year, record kind and the place)"),
         ('year_award_post_winner_news', 'YEAR_AWARD::post_winner_news', '0x9304f0', 'award/year_award.cpp', 'thiscall, ret 4',
          'a notícia dos vencedores (tipo 0x1b): a nação e o índice do prêmio, o ano, os nomes, ids e clubes dos três, um sorteio de 3 (o modelo), e do vencedor com ficha de jogador a máscara de posições e as marcas de força; vai para a divisão (prêmio de competição de clubes) e os clubes interessados fora dela, ou para a nação ou o continente da competição de pessoal e os interessados de fora, ou para todos os técnicos',
          "the winners' news (type 0x1b): the award's nation and index, the year, the three's names, ids and clubs, a draw of 3 (the template), and for a winner with a player sheet his position mask and strength flags; it goes to the division (a club competition award) and the interested clubs outside it, or to the staff competition's nation or continent and the interested clubs outside, or to every manager")])
CAMPOS = []
INTERESSA = []
