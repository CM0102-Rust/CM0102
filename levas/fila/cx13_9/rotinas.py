# Os recordes de pessoal e o texto da noticia do destino da First Division escocesa e a ordem da tabela da Premier
# depois do split (sco_first.cpp, sco_prm.cpp; leitor cx13, rodada 5, 24/11/2026). Simulacao com a Escocia (as ligas e
# a data dos itens mwb): o fim da temporada em maio de 2002; o split da Premier em abril.
# - 0x7ef5d0 ScoFirst::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18: o mesmo
#   corpo do PolSecond::update_staff_records_for_status (cx13_6, conferido instrucao a instrucao) com as chaves
#   escocesas (0x9cf5e0 a propria, 0x9cf5dc a Premier, cuja tabela propria guarda o clube da repescagem), o vencedor da
#   repescagem com status 0 e o recorde de vaga da temporada com 0xa0, 0, 0x28 (o polones: 0, 0xa0, 0x28). Prologo mov
#   edx, [0x9cf5dc] (6 bytes).
# - 0x7ef7c0 ScoFirst::build_fate_news_text thiscall(10 dwords) ret 0x28: fase diferente de 0, Comp::build_fate_news_text
#   0x48c6d0 com todos os argumentos como vieram (a7 inclusive); fase 0 (o grupo da repescagem, so em 1999): destino 0
#   (nos dois primeiros) com noticia 0xa7aea4 (com a competicao: nome +4, genero +0x37, e os dois vinculos 0x7d5 e
#   0x7d0), sem noticia 0x987594 (clube da Premier, a7) ou 0x989c18; outro destino com noticia 0xa7ae20, sem 0x989d78
#   (Premier) ou 0x989d40 "miss out on promotion". Prologo mov eax, [esp+0xc]; push ebx (5 bytes).
# - 0x7f3220 sco_prm_compare_split_half cdecl(a, b) -> i32 (a ordem da tabela posta em +0xa3 pelo
#   ScoPrm::create_split_stages): a linha cujo historico de posicoes (+9) tem o byte 0x20 >= 6 (com sinal) vem depois.
#   Prologo mov eax, [esp+4]; push ebx (5 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os recordes so a original contra o plano
# (compare_play_off_staff_records); o texto pelo compare_text_writer na fase 0 (so a original pelo texto comum); a ordem
# pelo compare_order!.
# Depende do cx13_6 (PlayOffDivision, play_off_update_staff_records, play_off_club_text).
SAVE = False
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
ROTINAS = [('sco_first_update_staff_records_for_status', 0x7ef5d0, [0x8b, 0x15, 0xdc, 0xf5, 0x9c, 0x00]),
           ('sco_first_build_fate_news_text', 0x7ef7c0, [0x8b, 0x44, 0x24, 0x0c, 0x53]),
           ('sco_prm_compare_split_half', 0x7f3220, [0x8b, 0x44, 0x24, 0x04, 0x53])]
DOCS = ('os recordes de pessoal e o texto do destino da First Division escocesa e a ordem da Premier depois do split',
        'the Scottish First Division\'s staff records and fate news text and the Premier\'s order after the split',
        [('sco_first_update_staff_records_for_status', 'ScoFirst::update_staff_records_for_status', '0x7ef5d0', 'sco_first.cpp', 'thiscall, ret 0x18',
          'na temporada: título, acesso, vaga na repescagem e rebaixamento; no grupo da repescagem de 1999, o clube da First que fica entre os dois primeiros sobe e o da Premier que fica em último cai',
          'in the season: title, promotion, play-off place and relegation; in the 1999 play-off group, the First Division club in the top two goes up and the Premier club that finishes bottom goes down'),
         ('sco_first_build_fate_news_text', 'ScoFirst::build_fate_news_text', '0x7ef7c0', 'sco_first.cpp', 'thiscall, ret 0x28',
          'fora do grupo da repescagem o texto comum; no grupo (1999): "terminou entre os dois primeiros" ou "terminou em último" com notícia, e sem notícia "escapa do rebaixamento", "conquista o acesso", "rebaixado" ou "perde o acesso"',
          'outside the play-off group the common text; in the group (1999): "finished in the top two" or "finished bottom" with a news item, and without one "avoid relegation", "win promotion", "relegated" or "miss out on promotion"'),
         ('sco_prm_compare_split_half', 'sco_prm_compare_split_half', '0x7f3220', 'sco_prm.cpp', 'cdecl',
          'depois do split, cada metade da tabela da Premier fica separada: quem estava do 7º para baixo no split nunca passa à frente de quem estava entre os seis primeiros',
          'after the split, each half of the Premier table stays apart: a club 7th or lower at the split never passes one of the top six')])
CAMPOS = []
INTERESSA = []
