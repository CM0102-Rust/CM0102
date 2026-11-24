# Os registros do tecnico pelo destino e o texto do destino na Segunda Divisao argentina (arg_second.cpp; leitor sbD,
# rodada 5, 24/11/2026; as sobras do cx08, ver levas/fila/cx08_10/FORA.md). Simulacao com a Argentina
# ('ENG KOR ARG ITA FRA ESP AUS', ate 2002-08-01, como os irmaos cx08): os virtuais 0x44 e o do texto do destino rodam
# com os resultados das zonas, da copa de acesso e das repescagens.
# - 0x408cd0 ArgSecond::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18 -> eax
#   (virtual 0x44): fase com sinal < 3: League::update_staff_records_for_status 0x686940 (clube, status, fase, a4, 0,
#   0), 0. Fase 3: status 1 a linha do clube (+0xb1, 0x3b bytes) +0x37 = 0 e add_promotion_record 0x869480 (0x32);
#   status 2 add_qualified_for_round_record 0x86bdd0 (0x45b, 0x82, 0x28); -1. Fases 4 e 5: as linhas do clube na
#   Primeira ([0x9cf654]) e nas fases 0 e 1 dela (League::find_club_row 0x68ab70, thiscall(clube, 0), ret 8); com as
#   tres: status 2 -> 3 nas tres e add_relegation_record 0x86a1c0 (clube, +0x57 do clube), senao 0xfe; devolve
#   [0x9cf654]; sem elas: a linha daqui, status 2 -> 0xfe, senao 0 e promocao (0x28); devolve [0x9cf658]. Acima: 0.
#   Prologo mov eax, [esp+0xc]; push ebx (5 bytes).
# - 0x408ec0 ArgSecond::build_fate_news_text thiscall(clube, destino, fase, a4, jogo, a6, a7, a8, corpo, Text) ret
#   0x28: fase 0xff o comum 0x48c6d0 (fase -1); fases < 3: destino 6 (0x987880 / 0x98783c), 3 (0x987420 com a7 = 1 ou
#   0x9877b8 / 0x987784), outro: o Text com o buffer 0xde1f64 como esta (defeito); sempre os vinculos do clube e da
#   competicao (nome curto, 0x7d0); fase 3: destino 1 (0x987704 / 0x9876cc, com o registro da Primeira [0xae23d0] +
#   0x6b x [0x9cf654]), 2 (0x98760c / 0x9875c8), outro o comum com fase 3; acima de 3: com corpo o comum, sem corpo pelo
#   a7 (= id da Primeira: 0x987594 / 0x987564; senao 0x9876cc / 0x987528), so o vinculo do clube.
#   Prologo mov eax, [esp+0xc]; push ebx (5 bytes).
# Auxiliares comuns (pub(crate)): set_club_and_competition_links e club_and_competition_line (o relegated_by_average_text
# do cx08_6 pode passar a usa-lo). Usa o club_fate_line do sbD_7 (integrar depois dele).
# Defeito do original: na 0x408ec0, fases < 3 com destino fora de 3 e 6, o Text recebe o buffer compartilhado sem
# escrever nada nele (o ultimo texto montado por qualquer rotina).
# Comparadores: os registros so a original, conferida contra o plano da copia (retorno e status das linhas); o texto
# lado a lado (compare_text_writer) nos textos proprios da liga, os comuns so a original.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_second_update_staff_records_for_status', 0x408cd0, [0x8b, 0x44, 0x24, 0x0c, 0x53]),
           ('arg_second_build_fate_news_text', 0x408ec0, [0x8b, 0x44, 0x24, 0x0c, 0x53])]
DOCS = ('os registros pelo destino e o texto do destino na Segunda Divisão argentina',
        'the Argentine Second Division staff records by outcome and fate news text',
        [('arg_second_update_staff_records_for_status', 'ArgSecond::update_staff_records_for_status', '0x408cd0', 'arg_second.cpp', 'thiscall, ret 0x18',
          'os registros do técnico pelo destino do clube na Segunda Divisão argentina: nas zonas o comum das ligas; na copa de acesso (fase 3) o acesso (status 1) ou a classificação (2); nas repescagens (fases 4 e 5) o clube da Primeira rebaixado ou mantido, o da Segunda promovido ou não',
          "the manager's records by the club's outcome in the Argentine Second Division: in the zones the leagues' common one; in the promotion cup (stage 3) promotion (status 1) or qualification (2); in the play-offs (stages 4 and 5) the Premier Division club relegated or kept, the Second Division one promoted or not"),
         ('arg_second_build_fate_news_text', 'ArgSecond::build_fate_news_text', '0x408ec0', 'arg_second.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino de um clube na Segunda Divisão argentina: nas zonas, a classificação para a Segunda Promoção ou o rebaixamento (pela média com a7 = 1); na copa de acesso, o acesso à Primeira ou a classificação para a Terceira Promoção; nas repescagens, a permanência ou o rebaixamento (clube da Primeira) e o acesso ou a eliminação (os outros)',
          "the text of the news of a club's fate in the Argentine Second Division: in the zones, qualifying for Second Promotion or relegation (by the average with a7 = 1); in the promotion cup, promotion to the Premier Division or qualifying for Third Promotion; in the play-offs, staying up or relegated (Premier Division club) and promoted or missing out (the others)")])
CAMPOS = []
INTERESSA = []
