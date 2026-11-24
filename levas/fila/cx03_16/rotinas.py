# Os codigos de progresso das duas copas de clubes da Asia e a noticia do destino na Recopa da Asia (asia_club_champ.cpp,
# asia_cup_winner.cpp; leitor cx03, rodada 5, 24/11/2026). Carreira de 8 ligas (o Japao jogavel).
# - 0x40ca20 AsiaClubChamp::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: Comp_Stats::
#   find_club_entry (push clube; mov ecx, [this+8] -> thiscall, ret 4); fase < 1: melhor 1, 9, 17; pior 7, 9, 17; atual
#   1, 9, 17 (codigos 1..3; os outros ficam); fases 1 e 2 (tabelas de salto 0x40cb64/74/84): melhor 1, 1, 5, 7; pior 4,
#   9, 17 e, com 4, o melhor vira 7 (o pior fica 4); atual 1, 9, 17 e, com 4, o melhor vira 7 (o atual fica 4); da fase 3
#   em diante como vem. +0x73 atual, +0x74 melhor, +0x75 pior.
# - 0x40e6b0 AsiaCupWinner::set_club_progress: fase < 1 (tabelas 0x40e778/88/98): melhor 1, 9, 17, 17; pior 7, 9, 17, 17;
#   atual 1, 9, 17, 17; senao como vem.
# - 0x40d460 AsiaCupWinner::build_fate_news_text thiscall(clube, destino, fase, a4..a8, noticia, texto) ret 0x28: fase < 1:
#   0x4b0b80(clube, a5, a8, destino, a6, texto) (6 pushes, cdecl); senao Comp::build_fate_news_text 0x48c6d0 (10 pushes,
#   o a7 trocado por 0; mov ecx, this -> thiscall, ret 0x28). Prologo mov eax, [esp+0xc]; cmp al, 1.
# Usa compare_club_progress do cx02_4, store_club_progress do cx03_9 e remap_group_code do cx03_15 (integrar depois deles).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: o progresso lado a lado nos tres bytes da entrada; a noticia nas rodadas de zona lado a lado
# (compare_text_writer, modo CM_TEXTO_NOTICIAS), a generica so a original.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('asia_club_champ_set_club_progress', 0x40ca20, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('asia_cup_winner_set_club_progress', 0x40e6b0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('asia_cup_winner_build_fate_news_text', 0x40d460, [0x8b, 0x44, 0x24, 0x0c, 0x3c, 0x01])]
DOCS = ('os códigos de progresso das duas copas de clubes da Ásia e a notícia do destino na Recopa da Ásia',
        "the progress codes of the two Asian club cups and the Asian Cup Winners' Cup fate news",
        [('asia_club_champ_set_club_progress', 'AsiaClubChamp::set_club_progress', '0x40ca20', 'asia_club_champ.cpp', 'thiscall, ret 0x14',
          'regra: na Liga dos Campeões da Ásia, nas rodadas de zona (fase abaixo de 1) os lugares 1, 2 e 3 viram 1, 9 e 17 no atual e no melhor e 7, 9 e 17 no pior; nos dois grupos (fases 1 e 2) o melhor dá 1 para o 1º e o 2º, 5 para o 3º e 7 para o 4º, o pior 4, 9 e 17, o atual 1, 9 e 17; um 4º lugar no pior ou no atual fica e faz o melhor virar 7; das semifinais em diante os códigos ficam',
          'rule: in the Asian Club Championship zone rounds (stages below 1) places 1, 2 and 3 become 1, 9 and 17 for the current and best codes and 7, 9 and 17 for the worst; in the two groups (stages 1 and 2) the best gives 1 for places 1 and 2, 5 for 3 and 7 for 4, the worst 4, 9 and 17, the current 1, 9 and 17; a 4th place in the worst or the current code stays and makes the best 7; from the semi-finals on the codes stay'),
         ('asia_cup_winner_set_club_progress', 'AsiaCupWinner::set_club_progress', '0x40e6b0', 'asia_cup_winner.cpp', 'thiscall, ret 0x14',
          'regra: nas rodadas de zona da Recopa da Ásia (fase abaixo de 1) os lugares 1 a 4 viram 1, 9, 17 e 17 no atual e no melhor e 7, 9, 17 e 17 no pior; depois os códigos ficam como vêm',
          "rule: in the Asian Cup Winners' Cup zone rounds (stages below 1) places 1 to 4 become 1, 9, 17 and 17 for the current and best codes and 7, 9, 17 and 17 for the worst; after that the codes stay as they come"),
         ('asia_cup_winner_build_fate_news_text', 'AsiaCupWinner::build_fate_news_text', '0x40d460', 'asia_cup_winner.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Recopa da Ásia: nas rodadas de zona o resultado do confronto; depois a notícia genérica da competição',
          "rule: the Asian Cup Winners' Cup news of a club's fate: in the zone rounds the tie result; after that the generic competition news")])
CAMPOS = []
INTERESSA = []
