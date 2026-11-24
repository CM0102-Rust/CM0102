# Os recordes de pessoal, o texto da noticia do destino e o progresso dos clubes da Premier escocesa (sco_prm.cpp;
# leitor cx13, rodada 5, 24/11/2026). Simulacao com a Escocia (as ligas e a data dos itens mwb): o split em abril de
# 2002, o fim da temporada em maio.
# - 0x7f3010 ScoPrm::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18: so na fase
#   0xff, 5 titulo (0x868c50), 1 classificado (0x86bdd0 com 0x44b, 0, 0x28), 3 rebaixamento (0x86a1c0); sempre 0.
#   Prologo cmp byte [esp+0xc], 0xff (5 bytes).
# - 0x7f3080 ScoPrm::build_fate_news_text thiscall(10 dwords) ret 0x28: fase 0xff e destino 1, o texto do grupo do
#   titulo so no buffer 0xde1f64 (o mesmo do PolFirst: championship_group_text do cx13_4); senao, nas fases -1, 0 e 1,
#   Comp::build_fate_news_text 0x48c6d0 com a fase e a7 = 0. Prologo mov al, [esp+0xc]; cmp al, 0xff (6 bytes).
# - 0x7f31c0 ScoPrm::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: em 1999 melhor 10 -> 9 (o
#   teste do pior contra 10 escreve o mesmo nos dois ramos); senao como vieram. Prologo mov eax, [esp+4]; push esi (5).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os recordes so a original (retorno 0); o texto lado a lado no
# buffer; o progresso lado a lado nos 3 bytes da entrada.
SAVE = False
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
ROTINAS = [('sco_prm_update_staff_records_for_status', 0x7f3010, [0x80, 0x7c, 0x24, 0x0c, 0xff]),
           ('sco_prm_build_fate_news_text', 0x7f3080, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0xff]),
           ('sco_prm_set_club_progress', 0x7f31c0, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('os recordes de pessoal, o texto do destino e o progresso dos clubes da Premier escocesa',
        'the Scottish Premier Division\'s staff records, fate news text and club progress',
        [('sco_prm_update_staff_records_for_status', 'ScoPrm::update_staff_records_for_status', '0x7f3010', 'sco_prm.cpp', 'thiscall, ret 0x18',
          'na temporada: o campeão ganha o recorde de título, quem vai ao grupo do título no split o de classificado e o rebaixado o de rebaixamento',
          'in the season: the champion gets the title record, a club going into the championship group at the split the qualified record and a relegated club the relegation record'),
         ('sco_prm_build_fate_news_text', 'ScoPrm::build_fate_news_text', '0x7f3080', 'sco_prm.cpp', 'thiscall, ret 0x28',
          'na temporada, quem vai ao grupo do título no split recebe o texto "classificado para o grupo do título" (só no buffer); os outros destinos o texto comum',
          'in the season, a club going into the championship group at the split gets the "qualify for championship group" text (only in the buffer); the other fates the common text'),
         ('sco_prm_set_club_progress', 'ScoPrm::set_club_progress', '0x7f31c0', 'sco_prm.cpp', 'thiscall, ret 0x14',
          'os códigos de progresso como vieram; só em 1999 o melhor 10 vira 9',
          'the progress codes as they came; only in 1999 a best of 10 becomes 9')])
CAMPOS = []
INTERESSA = []
