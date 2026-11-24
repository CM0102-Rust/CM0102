# O texto da noticia do premio de equipe e as datas da semana (award/team_award.cpp; leitor sx05b, rodada 5,
# 23/11/2026). Usa award_base e team_award_obj (sx05b_5): integrar depois dele.
# O texto e pedido pelo AWARD_MANAGER::write_award_news (0x4143f0, ja Rust, virtual +0xc do premio) quando a noticia de
# tipo 0x1b do sx05b_1 e lida; set_dates pela equipe da semana (NationAwards::update_team_of_the_week 0x76a21b). A
# Inglaterra tem as duas: LIGAS = 'ENG' e ATE = '2002-08-01', como o sx05b_1.
# Convencoes conferidas na desmontagem:
# - 0x8b0370 thiscall(noticia, Text, corpo) ret 0xc: NewsItem::get_slot 0x785830 thiscall(k) ret 4 (2, 3, 4, 5 e, com
#   ate tres jogadores, 6.. quatro por jogador: id, nome, sobrenome, apelido); 0x53e140 cdecl(n1, n2, n3, buf, 0xc8, 1,
#   id); sprintf 0x944c9f "%d/%02d" (o fim modulo 100) ou "%d" no buffer local; 0x66f4e0 langlib_sprintf no
#   0xde1f64 (os generos movsx, como dword); 0x90c520 utils_number_to_words(conta, buf, 0x32, 1) e 0x90c2d0 (o dword do
#   buffer) para a maiuscula; Text::set_string 0x4ae660 e Text::add_link 0x4ae8a0 thiscall; 0x53e990 cdecl(pessoa, n1,
#   n2, n3, 0); os nomes sao montados antes do teste da manchete;
# - 0x8aff00 thiscall(inicio, fim, atualizacao) ret 0xc: tres Date::operator= 0x417c10 (destino em ecx, retorno local,
#   origem) ret 8.
# Comparadores: compare_text_writer; set_dates lado a lado nas tres datas.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('team_award_write_news', 0x8b0370, [0x81, 0xec, 0xf4, 0x02, 0x00, 0x00]),
           ('team_award_set_dates', 0x8aff00, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08])]
DOCS = ('o texto da notícia do prêmio de equipe e as datas da semana',
        'the team award news text and the week dates',
        [('team_award_write_news', 'TEAM_AWARD::write_news', '0x8b0370', 'award/team_award.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia do prêmio de equipe: a manchete com o nome do prêmio e o vínculo da competição; o corpo com o período ("%d" num ano, "%d/%02d" em dois), um, dois ou três jogadores pelo nome nos modelos deles, ou a conta por extenso (maiúscula na frente) com o clube; os vínculos dos jogadores cujos nomes ainda batem (até três), ou o do clube com mais de três, e o da competição',
          "the headline or body of the team award news: the headline with the award's name and the competition's link; the body with the period (\"%d\" in one year, \"%d/%02d\" in two), one, two or three players by name in their templates, or the count in words (capital first) with the club; the links of the players whose names still match (up to three), or the club's with more than three, and the competition's"),
         ('team_award_set_dates', 'TEAM_AWARD::set_dates', '0x8aff00', 'award/team_award.cpp', 'thiscall, ret 0xc',
          'copia as três datas do prêmio: o início e o fim do período e a data da atualização',
          "copies the award's three dates: the period start and end and the update date")])
CAMPOS = []
INTERESSA = []
