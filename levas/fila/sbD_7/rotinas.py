# Os registros do tecnico pelo destino e o texto do destino na Segunda Divisao belga (bel_second.cpp; leitor sbD, rodada
# 5, 24/11/2026; as sobras do cx08, ver levas/fila/cx08_10/FORA.md). Simulacao com a Belgica ('ENG BEL HOL GER DEN', ate
# 2002-08-01, como os irmaos cx09_8 a cx09_13): os virtuais 0x44 e o do texto do destino rodam com os resultados da
# temporada e das repescagens (primavera de 2002).
# - 0x420790 BelSecond::update_staff_records_for_status thiscall(clube, status, fase, a4, jogo, a6) ret 0x18 -> eax
#   (virtual 0x44): fase 0xff: status 5 Staff_Records::add_league_champions_record 0x868c50, 0 add_promotion_record
#   0x869480 (0x64), 1 add_qualified_for_round_record 0x86bdd0 (0, 0xa0, 0x2d), 3 add_relegation_record 0x86a1c0; 0.
#   Fase com sinal < 3: -1. Fase 3: cada linha da tabela (+0xb1, 0x3b bytes) do clube: status 0 -> promocao (0x32) e
#   +0x37 = 0; outro status -> a primeira linha +0x37 = 0xff e -1. Fase 4: so clube da divisao (+0x57 = +4 da liga, senao
#   -1); status 1 com a palavra +0x34 do jogo = 2: a linha 0xfe; 2/0xff: a linha 3 e rebaixamento; 0. Acima: 0.
#   Prologo mov al, [esp+0xc]; push ebx (5 bytes).
# - 0x4209c0 BelSecond::build_fate_news_text thiscall(clube, destino, fase, a4, jogo, a6, a7, a8, corpo, Text) ret 0x28:
#   fase 0xff Comp::build_fate_news_text 0x48c6d0 (os mesmos argumentos, fase -1); fase 3: destino 0 (0x989c4c com corpo,
#   0x989c18 sem) e 0xfe (0x989b90 / 0x987528), no buffer 0xde1f64, vinculos do clube (0x7d5) e, com corpo, da
#   competicao (0x7d0); fase 4: com corpo comp_util_write_cup_tie_result_news 0x4b0b80 (cdecl, 6; destino 1 passado
#   como 1); sem corpo destino 1 0x987594, 2/0xff 0x987564, outro comp_util_write_cup_win_or_defeat_news 0x4b4420
#   (cdecl, 4). Prologo mov al, [esp+0xc]; push ebx (5 bytes).
# Auxiliares comuns (pub(crate)): club_fate_line (o mesmo corpo do group_qualify_text privado do cx04_5; unificar ao
# integrar) e club_competition_fate_line.
# Comparadores: os registros so a original, conferida contra o plano da copia feito antes (retorno e status das
# linhas); o texto lado a lado (compare_text_writer) nos textos proprios da liga, os comuns so a original.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_second_update_staff_records_for_status', 0x420790, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('bel_second_build_fate_news_text', 0x4209c0, [0x8a, 0x44, 0x24, 0x0c, 0x53])]
DOCS = ('os registros pelo destino e o texto do destino na Segunda Divisão belga',
        'the Belgian Second Division staff records by outcome and fate news text',
        [('bel_second_update_staff_records_for_status', 'BelSecond::update_staff_records_for_status', '0x420790', 'bel_second.cpp', 'thiscall, ret 0x18',
          'os registros do técnico pelo destino do clube na Segunda Divisão belga: na temporada o título, o acesso, a classificação e o rebaixamento; no grupo da repescagem de acesso (fase 3) o acesso de cada linha do clube ou, fora dele, a marca de eliminado; na repescagem de rebaixamento (fase 4) a permanência ou o rebaixamento dos clubes da própria divisão',
          "the manager's records by the club's outcome in the Belgian Second Division: in the season the title, promotion, qualification and relegation; in the promotion play-off group (stage 3) promotion for each of the club's rows or, otherwise, the eliminated mark; in the relegation play-off (stage 4) staying up or relegation for the division's own clubs"),
         ('bel_second_build_fate_news_text', 'BelSecond::build_fate_news_text', '0x4209c0', 'bel_second.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino de um clube na Segunda Divisão belga: no grupo da repescagem, "<clube> conquista o acesso" ou "<clube> perde o acesso" (com o nome da divisão no corpo); na repescagem de rebaixamento, "<clube> escapa do rebaixamento" ou "<clube> rebaixado", ou o texto do confronto de copa; na temporada o texto comum',
          'the text of the news of a club\'s fate in the Belgian Second Division: in the play-off group, "<club> win promotion" or "<club> miss out on promotion" (with the division\'s name in the body); in the relegation play-off, "<club> avoid relegation" or "<club> relegated", or the cup tie text; in the season the common text')])
CAMPOS = []
INTERESSA = []
