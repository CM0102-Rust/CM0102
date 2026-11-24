# Os vencedores do premio de equipe, as noticias aos clubes humanos e as datas da temporada (award/team_award.cpp;
# leitor sx05b, rodada 5, 23/11/2026). Usa TeamAward e team_award_clear_selection (sx05_6) e award_base,
# team_award_obj, NewsPost, NewsSink, news_post (sx05b_5): integrar depois deles.
# Rodam com as equipes da semana e da temporada das nacoes (NationAwards 0x769a10/0x769c04: set_season_dates e
# record_winners; a Inglaterra tem as duas): LIGAS = 'ENG' e ATE = '2002-08-01', como o sx05_6 e o mwg_5.
# Convencoes conferidas na desmontagem (cada push e cada [esp+k] seguidos ate a chamada):
# - 0x8affb0 thiscall(this), sem argumentos: AWARD_MANAGER::add_team_award_record 0x414300 (registro +0x4e, &+0x254,
#   &+0x258) ret 0xc; o laco dos onze titulares (contador 0xb; os reservas nao); o clube conta so com id > 0 (jle);
#   award_raise_winner_reputation 0x412f00 cdecl (pessoa, clube, rep. nacional (word), rep. mundial (word), +0x19);
#   Staff_Records::add_team_of_year_record 0x86c730 thiscall([0xdd7f00], pessoa, clube, [+5], ano do fim (word +0x244),
#   anos da temporada (byte +0x25)) ret 0x14; 0x8b0140; o esvaziamento das 20 vagas igual ao clear_selection (sx05_6);
# - 0x8b0140 thiscall(this): NewsItem::NewsItem 0x77bbe0 (0x1b, 0); NewsItem::set_slot 0x785790 (k (byte), valor);
#   human_manager_is_human_club 0x600a50 cdecl(clube, 0, 1, 0, 0); News::send_to_club_manager 0x7861e0
#   thiscall([0xb7431c], item, clube) ret 8; o indice do laco e um byte com sinal;
# - 0x8aff50 thiscall(this, ano) ret 4: Date::Date 0x549ef0 (dia -1, ano) ret 8 e Date::operator= 0x417c10 (destino em
#   ecx, retorno, origem) ret 8; o ano menos o byte com sinal +0x25.
# Comparadores: record_winners so a original (historico, reputacoes, registros e noticias) e o plano conferido (vagas
# vazias, o historico andou); send_club_news a original posta e a copia ensaia, lado a lado nos ids de clube das vagas
# e no gerador; set_season_dates lado a lado nas datas.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('team_award_record_winners', 0x8affb0, [0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1]),
           ('team_award_send_club_news', 0x8b0140, [0x6a, 0xff, 0x68, 0x0b, 0x38, 0x96, 0x00]),
           ('team_award_set_season_dates', 0x8aff50, [0x83, 0xec, 0x10, 0x56, 0x8b, 0xf1])]
DOCS = ('os vencedores do prêmio de equipe, as notícias aos clubes e as datas da temporada',
        'the team award winners, the club news and the season dates',
        [('team_award_record_winners', 'TEAM_AWARD::record_winners', '0x8affb0', 'award/team_award.cpp', 'thiscall',
          'registra a equipe: o registro da seleção vai para o histórico de equipes do gerente de prêmios; cada um dos onze titulares (os reservas não) ganha o prêmio: a reputação sobe pelas reputações do prêmio e o histórico dele recebe a entrada de equipe do ano (com o clube, a competição, o ano do fim do período e os anos da temporada); depois as notícias aos clubes humanos e a seleção esvaziada. Defeito do original: o clube só conta com id acima de 0 (os jogadores do clube 0 ganham sem clube)',
          "records the team: the selection record goes to the award manager's team history; each of the eleven starters (not the substitutes) gets the award: his reputations rise by the award's and his history gets the team of the year entry (with the club, the competition, the period end year and the season span); then the human clubs' news and the selection cleared. Defect of the original: a club counts only with an id above 0 (club 0's players get it without a club)"),
         ('team_award_send_club_news', 'TEAM_AWARD::send_club_news', '0x8b0140', 'award/team_award.cpp', 'thiscall',
          'avisa cada clube humano de quais jogadores dele entraram: uma notícia de tipo 0x1b, montada uma vez e reaproveitada; para cada vaga com clube de técnico humano, as vagas dali em diante do mesmo clube são contadas (as três primeiras guardadas) e perdem o clube (o clube é avisado uma vez); a notícia leva a nação e o índice do prêmio, os anos do início e do fim do período, a conta, o clube e, de um a três jogadores, o id e os três ids de nome de cada um; vai para o clube. Com mais de três, as vagas da notícia guardam o que um clube anterior deixou (o texto conta em vez de nomear)',
          "tells each human club which of its players made the team: one type 0x1b news item, built once and reused; for each slot whose club a human runs, the slots from there on of the same club are counted (the first three kept) and lose their club (the club is told once); the item carries the award's nation and index, the period start and end years, the count, the club and, with one to three players, each one's id and three name ids; it goes to the club. With more than three, the item's player slots keep what an earlier club left (the text counts instead of naming)"),
         ('team_award_set_season_dates', 'TEAM_AWARD::set_season_dates', '0x8aff50', 'award/team_award.cpp', 'thiscall, ret 4',
          'o período do prêmio em anos inteiros: o início no dia −1 do ano menos os anos da temporada, o fim no dia −1 do ano dado',
          'the award period in whole years: the start on day −1 of the year minus the season span, the end on day −1 of the given year')])
CAMPOS = []
INTERESSA = []
