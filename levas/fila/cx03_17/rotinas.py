# A noticia do destino na Liga dos Campeoes da Asia e dois auxiliares da Recopa da Asia (asia_club_champ.cpp,
# asia_cup_winner.cpp; leitor cx03, rodada 5, 24/11/2026). Carreira de 8 ligas (o Japao jogavel).
# - 0x40b460 AsiaClubChamp::build_fate_news_text thiscall(clube, destino, fase, a4..a8, noticia, texto) ret 0x28: fase < 1:
#   0x4b0b80(clube, a5, a8, destino, a6, texto); fase < 3: destino 6, 0x4b4590(clube, a4, +4, 6, noticia, texto); 0xfe, o
#   texto 0x987ae0 (com noticia: o byte +0x37 e o nome +4 da competicao) ou 0x987a80 "knocked out of" (sem: +0x52 e +0x38),
#   0x66f4e0 (8 pushes, cdecl), 0x4ae660 e os dois 0x4ae8a0 (clube 0x7d5, competicao 0x7d0): european_cup_fate_text;
#   fase >= 3: Comp::build_fate_news_text 0x48c6d0 com o a7 = 0. Prologo mov eax, [esp+0xc]; push ebx (5 bytes).
# - 0x40cba0 asia_cup_winner_find_club_nation_slot cdecl(clube, lista, n) -> al: o id da nacao do clube ([[+0x53]]; sem
#   clube ou nacao -1) procurado na lista de n (byte com sinal) ids; 0xff se nao esta.
# - 0x40e7b0 asia_cup_winner_club_in_club_championship cdecl(clube) -> 0/1: a Liga dos Campeoes da Asia guardada em
#   0xad9c5c (achada uma vez: a entrada ativa de [0xb64738] da tabela de 0x48 bytes em 0xb63d60 sem nacao e com o
#   continente +4 = [0x9cfa0c], o objeto [[+0x10]]); o clube em algum dos +0x3e times da copa ou na mesma vaga da fase 0.
#   Prologo push ebx; push ebp; push esi; mov esi, [0xad9c5c] (9 bytes, endereco absoluto).
# Usa european_cup_fate_text (HEAD) e key_nation_entry (HEAD).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: a noticia nos caminhos so de texto lado a lado (compare_text_writer, modo CM_TEXTO_NOTICIAS), os outros so
# a original; os dois auxiliares (puros; o segundo so grava o cache, igual nos dois) rodam os dois e comparam o retorno.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('asia_club_champ_build_fate_news_text', 0x40b460, [0x8b, 0x44, 0x24, 0x0c, 0x53]),
           ('asia_cup_winner_find_club_nation_slot', 0x40cba0, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xc9, 0xff]),
           ('asia_cup_winner_club_in_club_championship', 0x40e7b0, [0x53, 0x55, 0x56, 0x8b, 0x35, 0x5c, 0x9c, 0xad, 0x00])]
DOCS = ('a notícia do destino na Liga dos Campeões da Ásia e dois auxiliares da Recopa da Ásia',
        "the Asian Club Championship fate news and two Asian Cup Winners' Cup helpers",
        [('asia_club_champ_build_fate_news_text', 'AsiaClubChamp::build_fate_news_text', '0x40b460', 'asia_club_champ.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Liga dos Campeões da Ásia: nas rodadas de zona o resultado do confronto; nos dois grupos a classificação pela frase genérica dos grupos e a eliminação com "{clube} have failed to qualify from Phase 2 of the {competição}." (o nome inteiro) com objeto de notícia, ou "{clube} knocked out of {competição}" (o nome curto) sem; das semifinais em diante a notícia genérica',
          "rule: the Asian Club Championship news of a club's fate: in the zone rounds the tie result; in the two groups qualification through the generic group sentence and elimination with \"{club} have failed to qualify from Phase 2 of the {competition}.\" (the full name) with a news object, or \"{club} knocked out of {competition}\" (the short name) without; from the semi-finals on the generic news"),
         ('asia_cup_winner_find_club_nation_slot', 'asia_cup_winner_find_club_nation_slot', '0x40cba0', 'asia_cup_winner.cpp', 'cdecl',
          'a posição da nação do clube numa lista de ids de nação, ou 0xff se não está',
          "the slot of the club's nation in a list of nation ids, or 0xff when it is not there"),
         ('asia_cup_winner_club_in_club_championship', 'asia_cup_winner_club_in_club_championship', '0x40e7b0', 'asia_cup_winner.cpp', 'cdecl',
          'regra: se o clube joga a Liga dos Campeões da Ásia na temporada: está entre os times da copa ou da primeira fase dela',
          'rule: whether the club plays the Asian Club Championship this season: it is among the teams of the cup or of its first stage')])
CAMPOS = []
INTERESSA = []
