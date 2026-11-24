# Os recordes de pessoal, o texto da noticia do destino e o progresso dos clubes da Primeira Divisao irlandesa e da
# repescagem com a Premier (ire_first.cpp; leitor sbE, rodada 5, 24/11/2026). Simulacao com a Irlanda (as ligas do
# cx10_12): a temporada 2001/02 acaba na primavera de 2002 e a repescagem (fase 0) logo depois.
# - 0x636020 IreFirst::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18: o mesmo corpo
#   do PolSecond::update_staff_records_for_status (cx13_6, conferido instrucao a instrucao) com as chaves irlandesas
#   (0x9cf810 a propria, 0x9cf80c a Premier, cuja tabela propria guarda o clube da repescagem): na temporada 5 titulo,
#   0 acesso 0x64, 1 classificado (0, 0xa0, 0x28), 3 rebaixamento, retorno 0; na repescagem o vencedor (1) da Primeira
#   sobe (0x32, linha 0), o perdedor 0xfe; o da Premier que vence 0xfe, o que perde rebaixamento com o registro da
#   Premier e linha 3; o retorno e o id da chave do clube; nao achado 0. Prologo mov edx, [0x9cf80c] (6 bytes).
# - 0x636210 IreFirst::build_fate_news_text thiscall(10 dwords) ret 0x28: o corpo do PolSecond::build_fate_news_text
#   (cx13_6) com a Premier irlandesa: com noticia (a9) o texto comum (a7 = 0) e segue; destino 1 na repescagem (fase 0)
#   0x987594 "avoid relegation" (a7 = id da Premier) ou 0x989c18 "win promotion"; senao clube da Premier 0x989d78
#   "relegated"; senao de novo o texto comum. Prologo push ebx; mov ebx, [esp+0x28] (5 bytes).
# - 0x636390 IreFirst::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: fase 0xff, melhor 3 vira 2
#   e pior 3 vira 4; fase 0, clube da Premier na entrada das estatisticas da Premier com 1 -> 10 e 2 -> 12, clube da
#   Primeira dobrado; outra fase como vieram. Prologo mov ecx, [ecx+8]; push ebx; push esi (5 bytes).
# Depende do cx13_6 (PlayOffDivision, play_off_update_staff_records, compare_play_off_staff_records, play_off_club_text):
# integrar depois dele.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os recordes so a original contra o plano; o texto pelo
# compare_text_writer sem noticia nos textos da repescagem; o progresso lado a lado nos 3 bytes da entrada escrita.
SAVE = False
LIGAS = 'ENG IRL NIR'
ROTINAS = [('ire_first_update_staff_records_for_status', 0x636020, [0x8b, 0x15, 0x0c, 0xf8, 0x9c, 0x00]),
           ('ire_first_build_fate_news_text', 0x636210, [0x53, 0x8b, 0x5c, 0x24, 0x28]),
           ('ire_first_set_club_progress', 0x636390, [0x8b, 0x49, 0x08, 0x53, 0x56])]
DOCS = ('os recordes de pessoal, o texto do destino e o progresso dos clubes da Primeira Divisão irlandesa e da repescagem',
        'the Irish First Division\'s and play-off\'s staff records, fate news text and club progress',
        [('ire_first_update_staff_records_for_status', 'IreFirst::update_staff_records_for_status', '0x636020', 'ire_first.cpp', 'thiscall, ret 0x18',
          'na temporada: título, acesso, vaga na repescagem e rebaixamento; na repescagem contra a Premier, o clube da Primeira que vence sobe (recorde de acesso), o da Premier que perde cai (recorde de rebaixamento), e o status da linha marca quem fica onde',
          'in the season: title, promotion, play-off place and relegation; in the play-off against the Premier Division, the First Division club that wins goes up (promotion record), the Premier club that loses goes down (relegation record), and the row status tells who stays where'),
         ('ire_first_build_fate_news_text', 'IreFirst::build_fate_news_text', '0x636210', 'ire_first.cpp', 'thiscall, ret 0x28',
          'na repescagem, quem vence: "escapa do rebaixamento" (clube da Premier) ou "conquista o acesso" (clube da Primeira); clube da Premier que perde: "rebaixado"; os outros o texto comum; com notícia, o texto comum vem antes',
          'in the play-off, the winner: "avoid relegation" (a Premier club) or "win promotion" (a First Division club); a Premier club that loses: "relegated"; the others the common text; with a news item, the common text comes first'),
         ('ire_first_set_club_progress', 'IreFirst::set_club_progress', '0x636390', 'ire_first.cpp', 'thiscall, ret 0x14',
          'na temporada, o melhor 3 vira 2 e o pior 3 vira 4; na repescagem, o clube da Premier vai às estatísticas da Premier com 1 → 10 e 2 → 12, o da Primeira com os códigos dobrados',
          'in the season, a best of 3 becomes 2 and a worst of 3 becomes 4; in the play-off, a Premier club goes into the Premier\'s statistics with 1 → 10 and 2 → 12, a First Division club with its codes doubled')])
CAMPOS = []
INTERESSA = []
