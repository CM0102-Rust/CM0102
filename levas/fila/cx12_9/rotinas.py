# Os registros pela situacao, o texto da noticia do destino e o progresso dos clubes da Primeira Divisao norte-irlandesa
# e da repescagem com a Premier (nir_first.cpp; leitor cx12, rodada 5, 24/11/2026). Simulacao com a Irlanda do Norte
# (LIGAS): a temporada vai de agosto de 2001 a abril de 2002 e a repescagem (fase 0) e jogada a partir de 5 de maio de
# 2002, antes de 2002-06-01. O texto so com o modo CM_TEXTO_NOTICIAS e um tecnico humano como destinatario.
# - 0x78c300 NirFirst::update_staff_records_for_status thiscall(clube, situacao, fase, a4, a5, a6) ret 0x18: na liga
#   (fase 0xff) 5 titulo (0x868c50), 0 acesso (0x869480 com 0x64), 1 vaga na repescagem (0x86bdd0 com 0, 0xa0, 0x28), 3
#   rebaixamento (0x86a1c0); 0. Na repescagem: um clube da Primeira (a divisao +0x57 com o id de [0x9cf8b4]) na tabela
#   desta liga sobe com 1 (acesso 0x32, situacao da linha 0) ou fica (0xfe), devolvendo o id da Primeira; um clube da
#   Premier (objeto de [0x9cf8b0] pela tabela 0xadadfc) fica com 1 (0xfe) ou cai (rebaixamento pelo registro de
#   competicao de [0x9cf80c], situacao 3), devolvendo o id da Premier; sem linha, 0. DEFEITO: [0x9cf80c] e a chave da
#   Premier da Republica da Irlanda, nao a da Irlanda do Norte (o NorFirst 0x791080, o mesmo corpo, usa a propria
#   Premier). Prologo mov eax, [0xadadfc] (5 bytes).
# - 0x78c4f0 NirFirst::build_fate_news_text thiscall(clube, destino, fase, a4, jogo, a6, divisao, a8, noticia, texto)
#   ret 0x28 (a "divisao" e o retorno do update_staff_records_for_status): com a noticia, o texto base 0x48c6d0 (a7 = 0)
#   e, sem voltar: divisao = Premier, destino 1 na fase 0 "avoid relegation" (0x987594), senao "relegated" (0x989d78);
#   outra divisao, destino 1 na fase 0 "win promotion" (0x989c18), senao o texto base de novo. DEFEITO: com a noticia,
#   o texto base roda duas vezes nos destinos sem texto proprio, e e escrito e depois trocado nos que tem. O texto
#   proprio e o group_qualify_text do cx04_5. Prologo push ebx; mov ebx, [esp+0x28] (5 bytes).
# - 0x78c670 NirFirst::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: na liga, melhor 2 -> 1 e
#   pior 2 -> 3; na fase 0, um clube da Premier grava na entrada das estatisticas da Premier (1 -> 8, 2 -> 10 nos tres
#   codigos) e um da Primeira dobra os tres codigos; nas outras fases como vem. Prologo mov ecx, [ecx+8]; push ebx;
#   push esi (5 bytes).
# Os tres corpos sao os do NorFirst (0x791080, 0x791270, 0x7913f0; conferidos instrucao a instrucao, so as chaves e os
# codigos mudam): os auxiliares first_division_* servem ao cx12_11. O PolSecond (cx13_6) tem o mesmo molde.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os registros so a original contra o plano (retorno e situacao da
# linha); o texto lado a lado (compare_text_writer) sem a noticia; o progresso lado a lado nos 3 bytes da entrada.
# Depende do cx04_5 (group_qualify_text): integrar depois dele.
SAVE = False
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
ROTINAS = [('nir_first_update_staff_records_for_status', 0x78c300, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('nir_first_build_fate_news_text', 0x78c4f0, [0x53, 0x8b, 0x5c, 0x24, 0x28]),
           ('nir_first_set_club_progress', 0x78c670, [0x8b, 0x49, 0x08, 0x53, 0x56])]
DOCS = ('os registros pela situação, o texto da notícia do destino e o progresso dos clubes da Primeira Divisão norte-irlandesa',
        'the Northern Irish First Division staff records by outcome, fate news text and club progress',
        [('nir_first_update_staff_records_for_status', 'NirFirst::update_staff_records_for_status', '0x78c300', 'nir_first.cpp', 'thiscall, ret 0x18',
          'regra: na liga, o campeão ganha o registro de título, o promovido o de acesso, o da repescagem o de vaga e o rebaixado o de queda; na repescagem contra a Premier, o clube da Primeira que vence sobe (registro de acesso) e o da Premier que perde cai (registro de queda, pela Premier da República da Irlanda: defeito do original)',
          'rule: in the league, the champion gets the title record, the promoted club the promotion one, the play-off club the place one and the relegated club the relegation one; in the play-off against the Premier, the First Division club that wins goes up (promotion record) and the Premier club that loses goes down (relegation record, naming the Republic of Ireland Premier Division: a defect of the original)'),
         ('nir_first_build_fate_news_text', 'NirFirst::build_fate_news_text', '0x78c4f0', 'nir_first.cpp', 'thiscall, ret 0x28',
          'o texto da notícia da repescagem: o clube da Premier que vence "escapa do rebaixamento", o que perde é "rebaixado"; o clube da Primeira que vence "consegue o acesso"; os outros destinos ficam com o texto base',
          'the play-off news text: the Premier club that wins "avoids relegation", the one that loses is "relegated"; the First Division club that wins "wins promotion"; the other fates get the base text'),
         ('nir_first_set_club_progress', 'NirFirst::set_club_progress', '0x78c670', 'nir_first.cpp', 'thiscall, ret 0x14',
          'a previsão de colocação na Primeira norte-irlandesa: na liga, a melhor 2 vira 1 e a pior 2 vira 3; na repescagem, o clube da Premier grava nas estatísticas da Premier (1 vira 8 e 2 vira 10) e o da Primeira dobra os códigos',
          'the place forecast in the Northern Irish First Division: in the league, best 2 becomes 1 and worst 2 becomes 3; in the play-off, the Premier club writes into the Premier statistics (1 becomes 8 and 2 becomes 10) and the First Division club doubles its codes')])
CAMPOS = []
INTERESSA = []
