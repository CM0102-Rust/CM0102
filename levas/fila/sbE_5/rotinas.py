# Os grupos da repescagem da Eerste Divisie (Primeira Divisao holandesa), os recordes de pessoal e o texto da noticia do
# destino (hol_first.cpp; leitor sbE, rodada 5, 24/11/2026). Simulacao com a Holanda (as ligas e a data dos itens mwc): os
# grupos nascem quando a Eerste Divisie chega a fase 4 (HolFirst::advance_stage 0x5f6650, mwc_25), em maio de 2002, e
# os recordes e os textos saem nos periodos e nos grupos.
# - 0x5f6720 HolFirst::create_play_off_groups thiscall() ret 0: completa ate 6 os status 1 (os vencedores dos periodos)
#   com os primeiros 0xff (marca 1, virtual 0x40 (clube, 1, +0x44, 0, 0), virtual 0x68 (clube, +0x44, 2, 2, 7)); malloc
#   0x20 (caixa 0x2a9) com os 6 de status 1 e os de status 2 da Eredivisie (0x6826d0 sem limite) ate 8; troca a vaga 0
#   com a 7; para as fases 4 e 5: virtual 0x3c (fase, &rodadas, &rotulo, 0), +0xc1 = 0 antes da primeira, League_Stage
#   0x68b100 (4 clubes, 2 turnos, desempates 4,1,2,0, lugares de +0xbe, importancia 0x14, bits 0x22, -1, modo 2); free da
#   primeira tabela, da lista e da segunda; com as duas fases +0x30 = 5, senao a caixa 0x2e6. Prologo push -1; push
#   imm32 (7 bytes).
# - 0x5f6ee0 HolFirst::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18: temporada
#   (0xff) 5 titulo, 0 acesso 0x64, 1 classificado (0xa0, 0, 0x28), 3 rebaixamento, retorno 0; periodos (fase < 4) o
#   classificado com status 1, retorno -1; grupos (4 e 5): clube da Eerste Divisie (+0x57 = 0x9cf56c) achado na tabela
#   com status 0 acesso 0x32, linha 0, retorno o id; com 0xfe a linha 0xfe e -1; senao a tabela da Eredivisie: 0 a
#   linha 0xfe, 0xfe rebaixamento (com a competicao da Eerste Divisie: defeito [inferencia]) e linha 3, retorno o id da
#   Eredivisie; nao achado 0. Prologo push ecx; mov eax, [0xadadfc] (6 bytes).
# - 0x5f7110 HolFirst::build_fate_news_text thiscall(10 dwords) ret 0x28: fora das fases 4 e 5, Comp::build_fate_news_text
#   0x48c6d0 (a7 = 0); nelas, destino 0 da Eerste Divisie (a7 = 0x9cf56c) 0x9cef4c com noticia ou 0x9cef28 sem, da
#   Eredivisie 0x9ceedc ou 0x9ceeb8; destino 0xfe 0x9cee64 ou 0x9cee44; outro destino nada. langlib_sprintf 0x66f4e0 no
#   buffer 0xde1f64, Text::set_string e os dois vinculos (clube 0x7d5, competicao 0x7d0). Prologo mov eax, [esp+0xc];
#   push ebx (5 bytes).
# Depende do cx08_16 (belgian_progress_virtual) e do cx13_6 (PlayOffRecord, table_row_of_club): integrar depois deles.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os grupos so a original (as duas fases de 4 e a fase atual 5); os
# recordes so a original contra o plano (o retorno e o status da linha); o texto pelo compare_text_writer nos grupos.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('hol_first_create_play_off_groups', 0x5f6720, [0x6a, 0xff, 0x68, 0x7c, 0xad, 0x95, 0x00]),
           ('hol_first_update_staff_records_for_status', 0x5f6ee0, [0x51, 0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('hol_first_build_fate_news_text', 0x5f7110, [0x8b, 0x44, 0x24, 0x0c, 0x53])]
DOCS = ('os grupos da repescagem da Eerste Divisie, os recordes de pessoal e o texto do destino',
        'the Eerste Divisie play-off groups, staff records and fate news text',
        [('hol_first_create_play_off_groups', 'HolFirst::create_play_off_groups', '0x5f6720', 'hol_first.cpp', 'thiscall, ret 0',
          'a repescagem de acesso: os vencedores dos períodos completados até seis com os mais bem colocados ainda indefinidos, mais os dois da repescagem da Eredivisie; os oito em dois grupos de quatro, ida e volta (fases 4 e 5), o primeiro e o último trocando de grupo',
          'the promotion play-off: the period winners completed to six with the best-placed clubs still undecided, plus the two Eredivisie play-off clubs; the eight in two home-and-away groups of four (stages 4 and 5), the first and the last swapping groups'),
         ('hol_first_update_staff_records_for_status', 'HolFirst::update_staff_records_for_status', '0x5f6ee0', 'hol_first.cpp', 'thiscall, ret 0x18',
          'na temporada: título, acesso, vaga na repescagem e rebaixamento; o vencedor de período ganha a vaga; nos grupos, o clube da Eerste Divisie que vence o grupo sobe e o da Eredivisie que não vence cai, e o status das tabelas diz quem fica onde',
          'in the season: title, promotion, play-off place and relegation; a period winner gets the place; in the groups, the Eerste Divisie club that wins its group goes up and the Eredivisie club that does not goes down, the table statuses telling who stays where'),
         ('hol_first_build_fate_news_text', 'HolFirst::build_fate_news_text', '0x5f7110', 'hol_first.cpp', 'thiscall, ret 0x28',
          'nos grupos da repescagem: quem vence "sobe" (Eerste Divisie) ou "escapa do rebaixamento" (Eredivisie), quem não vence "rebaixado"; com notícia, os textos citam a competição; nas outras fases o texto comum',
          'in the play-off groups: the winner "wins promotion" (Eerste Divisie) or "avoids relegation" (Eredivisie), the others "relegated"; with a news item, the texts name the competition; in the other stages the common text')])
CAMPOS = []
INTERESSA = []
