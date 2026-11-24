# Os registros dos tecnicos, o texto da noticia do destino e os codigos de progresso da Superettan (swe_first.cpp;
# leitor cx14, rodada 5, 24/11/2026). Simulacao com a Suecia (as ligas e a data dos itens mwe): a temporada sueca
# termina em novembro de 2001, com o mata-mata de acesso (fase 0) e a virada.
# - 0x88f600 SweFirst::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18: o corpo do
#   play_off_record_plan do cx13_6 (PlayOffDivision) com a Superettan (0x9cf5f0), a Allsvenskan (0x9cf5ec) na propria
#   tabela, vencedor 1 e o registro de classificado (0, 0xa0, 0x28), conferido instrucao a instrucao (registro de
#   campeao 0x868c50, acesso 0x869480 com 0x64 / 0x32, classificado 0x86bdd0, rebaixamento 0x86a1c0 com o registro da
#   Allsvenskan [0xae23d0] + id x 0x6b; status 0, 0xfe e 3; o retorno do mata-mata = o valor da chave do clube). Fases
#   que nao sao 0xff nem 0: 0 sem nada (o cx13_6 trata qualquer fase como o mata-mata).
# - 0x88f7f0 SweFirst::build_fate_news_text thiscall(10) ret 0x28: os pushes das chamadas ao texto comum conferidos um a
#   um (a7 sempre 0; sem noticia a noticia vai 0 e a fase vai como a constante 0 ou 1 nesses ramos). Os textos 0x987594,
#   0x989c18 e 0x987564 pelo play_off_club_text do cx13_6.
# - 0x88fa10 SweFirst::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: na fase 0 o clube da
#   Allsvenskan recebe os codigos 10/13 na entrada da Allsvenskan e logo por cima os codigos crus (defeito do original:
#   o mapeamento se perde); a divisao do clube lida sem conferir nulo.
# Prologos: mov edx, [0x9cf5ec] (6 bytes); mov eax, [esp+0x24]; push esi (5); mov ecx, [ecx+8]; push ebx; push esi (5).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os registros so a original contra o plano (o resultado e o
# status da linha); o texto lado a lado (compare_text_writer; o texto comum so a original); o progresso lado a lado.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('swe_first_update_staff_records_for_status', 0x88f600, [0x8b, 0x15, 0xec, 0xf5, 0x9c, 0x00]),
           ('swe_first_build_fate_news_text', 0x88f7f0, [0x8b, 0x44, 0x24, 0x24, 0x56]),
           ('swe_first_set_club_progress', 0x88fa10, [0x8b, 0x49, 0x08, 0x53, 0x56])]
DOCS = ('os registros dos técnicos, a notícia do destino e o progresso da Superettan sueca',
        'the staff records, the fate news text and the progress of the Swedish Superettan',
        [('swe_first_update_staff_records_for_status', 'SweFirst::update_staff_records_for_status', '0x88f600', 'swe_first.cpp', 'thiscall, ret 0x18',
          'regra: na temporada da Superettan, o campeão (status 5), quem sobe (0), quem vai ao mata-mata (1) e quem cai (3) ganham o registro; no mata-mata contra a Allsvenskan o clube da Superettan que vence sobe (registro de acesso, status 0) e o que perde fica (status 0xfe); o da Allsvenskan que vence fica (0xfe) e o que perde cai (registro de rebaixamento, status 3)',
          'rule: in the Superettan season, the champion (status 5), those going up (0), those going to the play-off (1) and those going down (3) get their record; in the play-off against the Allsvenskan the Superettan club that wins goes up (promotion record, status 0) and the loser stays (status 0xfe); the Allsvenskan club that wins stays (0xfe) and the loser goes down (relegation record, status 3)'),
         ('swe_first_build_fate_news_text', 'SweFirst::build_fate_news_text', '0x88f7f0', 'swe_first.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do mata-mata da Superettan: sem notícia, quem vence "escapa do rebaixamento" (clube da Allsvenskan) ou "conquista o acesso"; o clube da Allsvenskan que perde, "rebaixado"; o resto, o texto comum',
          'rule: the Superettan play-off news: without a news item, the winner "avoid relegation" (an Allsvenskan club) or "win promotion"; the Allsvenskan club that loses, "relegated"; the rest, the common text'),
         ('swe_first_set_club_progress', 'SweFirst::set_club_progress', '0x88fa10', 'swe_first.cpp', 'thiscall, ret 0x14',
          'regra: os códigos de progresso da Superettan: na liga o melhor código 3 vira 1; no mata-mata o clube da Superettan fica com 2 × código − 1 e o da Allsvenskan com os códigos como vieram na entrada da Allsvenskan (o original mapeia 1 → 10 e 2 → 13 e escreve por cima); nas outras fases, como vieram',
          'rule: the Superettan progress codes: in the league a best code 3 becomes 1; in the play-off a Superettan club gets 2 × code − 1 and an Allsvenskan club the codes as they came in its Allsvenskan entry (the original maps 1 → 10 and 2 → 13 and writes over them); in the other stages, as they came')])
CAMPOS = []
INTERESSA = []
