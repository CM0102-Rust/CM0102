# As estatísticas das competições (Comp_Stats) no fim da temporada: os recordes de sempre, o arquivo da temporada e as
# notícias de recorde (comp_screens.cpp; leitor tel41, rodada 2 das telas, 22/11/2026). Item de simulação: o fim de
# temporada de cada competição (update_season_end_records e send_record_news chamadas pela 0x51f92a/0x51f93d, copas, e
# 0x684a99/0x684ac8, ligas; archive_season_and_reset por 170 chamadores, no fechamento e no recomeço de cada
# competição). Na carreira ENG até 2002-06-01: o fim das copas (finais em maio) e da Premier (maio de 2002).
# Convenções conferidas na desmontagem: update_season_end_records thiscall ret 0 (devolve 1); archive_season_and_reset
# thiscall(stats, archive) ret 4 (devolve 1); send_record_news thiscall ret 0 (0 sem bandeira, senão 1). Chamadas:
# Friendly::is_friendly_competition 0x5c6710 thiscall(friendly, comp record) ret 4; comp_util_is_league_division
# 0x4b0700 cdecl(comp id); record_holder_assign 0x448490 thiscall ret 4; _ftol 0x945640; FILE_LLIST::add_node 0x58e340
# thiscall(list, &key1, &key2, row) ret 0xc; comp_season_records_construct 0x49f160 thiscall ret 0; NewsItem::NewsItem
# 0x77bbe0 thiscall(item, 0x3e9, 0) ret 8; News::send_to_comp_region 0x786860 thiscall(news, item, comp record) ret 8;
# já em Rust: update_most_titles_record, update_season_rating_record, comp_season_records_clear, save_team_runs,
# comp_season_records_assign, comp_stats_write_record_to_news.
# A linha arquivada (0x26b bytes) é montada na pilha com a cópia em linha dos recordes; os mesmos bytes que o
# operator= 0x4ab5f0 (as mesmas sub-cópias, conferido campo a campo; tel31_1 conferiu o mesmo na 0x4ab0c0).
# Comparadores: update_season_end_records lado a lado (o objeto inteiro, as duas tabelas, o histórico e o contexto);
# archive_season_and_reset e send_record_news só a original, com o plano conferido depois.
# Dúvida: o byte +0x42 da competição (CLUB_SIDES nos campos, 1 = clubes) comparado com 4 corta o recorde de pontos;
# não se sabe qual competição tem 4 ali. Defeito do original: o recorde de média de nota de sempre pede mais de 3 jogos
# só para a notícia, não para o recorde.
ROTINAS = [('comp_stats_update_season_end_records', 0x4a19f0, [0x55, 0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x02, 0x05, 0x00, 0x00]),
           ('comp_stats_archive_season_and_reset', 0x4a1c50, [0x6a, 0xff, 0x68, 0x0b, 0x67, 0x95, 0x00]),
           ('comp_stats_send_record_news', 0x4a2450, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as estatísticas das competições no fim da temporada: os recordes de sempre, o arquivo da temporada e as notícias de recorde',
        "the competition statistics at the season's end: the all-time records, the season archive and the record news",
        [('comp_stats_update_season_end_records', 'Comp_Stats::update_season_end_records', '0x4a19f0', 'comp_screens.cpp', 'thiscall',
          'no fim da temporada, fora dos amistosos: recontar os títulos (notícia com 5 ou mais), o recorde de pontos numa temporada e o de média de nota passam aos de sempre quando os batem (notícia a partir do terceiro fim de temporada, numa divisão de liga; a de nota com mais de 3 jogos); a contagem de fins de temporada sobe',
          "at a season's end, outside friendlies: the titles recounted (news from 5), the season's most points and best average rating become the all-time records when they beat them (news from the third season end, in a league division; the rating's with more than 3 games); the season end count goes up"),
         ('comp_stats_archive_season_and_reset', 'Comp_Stats::archive_season_and_reset', '0x4a1c50', 'comp_screens.cpp', 'thiscall',
          'fechando a temporada: com o pedido, o ano e os recordes da temporada vão para o arquivo compartilhado (comp records history.tmp); depois os recordes da temporada são zerados, as sequências guardadas e as duas tabelas liberadas',
          "closing the season: when asked, the year and the season's records go to the shared file (comp records history.tmp); then the season's records are cleared, the runs kept and both tables freed"),
         ('comp_stats_send_record_news', 'Comp_Stats::send_record_news', '0x4a2450', 'comp_screens.cpp', 'thiscall',
          'uma notícia (tipo 0x3e9) para a região da competição por recorde batido, dos tipos 1 a 22 em ordem; as bandeiras são zeradas',
          "one news item (type 0x3e9) to the competition's region per broken record, kinds 1 to 22 in order; the flags are cleared")])
