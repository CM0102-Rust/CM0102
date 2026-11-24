# Os recordes de pessoal, o texto da noticia do destino e o progresso dos clubes da Segunda Divisao polonesa e da
# repescagem com a Primeira (pol_second.cpp; leitor cx13, rodada 5, 24/11/2026). Simulacao com a Polonia (as ligas e a
# data dos itens mwd): a temporada acaba em junho de 2002 e a repescagem (fase 0) e criada quando a Primeira acaba.
# - 0x7cc490 PolSecond::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18: na
#   temporada (fase 0xff) 5 titulo (0x868c50), 0 acesso (0x869480 com 0x64, ret 0xc), 1 vaga na repescagem (0x86bdd0
#   com 0, 0xa0, 0x28), 3 rebaixamento (0x86a1c0), sempre 0. Na repescagem: clube da Segunda achado na tabela, venceu
#   (1) acesso com 0x32 e status da linha 0, perdeu 0xfe; clube da Primeira achado na tabela da fase final (fases[1]),
#   venceu 0xfe, perdeu rebaixamento com o registro da Primeira (0xae23d0 + id * 0x6b) e status 3; o retorno e o valor
#   da chave do clube (0x9cf85c ou 0x9cf858); nao achado, 0. A fase final da Primeira e lida na entrada. Prologo mov
#   edx, [0x9cf858] (6 bytes).
# - 0x7cc680 PolSecond::build_fate_news_text thiscall(10 dwords) ret 0x28: com noticia (a9), Comp::build_fate_news_text
#   0x48c6d0 (a7 = 0) e segue sem voltar; destino 1 na repescagem (fase 0): 0x987594 "avoid relegation" para clube da
#   Primeira (a7 = id da Primeira), 0x989c18 "win promotion" para os outros; senao clube da Primeira 0x989d78
#   "relegated"; senao de novo o texto comum. O texto: langlib_sprintf 0x66f4e0 (cdecl, 5 pushes) no buffer 0xde1f64,
#   Text::set_string 0x4ae660 (ret 4) e Text::add_link 0x4ae8a0 (nome, 0x7d5, clube; ret 0xc). Prologo push ebx; mov
#   ebx, [esp+0x28] (5 bytes).
# - 0x7cc800 PolSecond::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: fase 0xff, melhor 3 ou 4
#   vira 2 e pior 3 vira 5 (o segundo teste le o melhor ja trocado: pior 4 fica 4, defeito do original [inferencia]);
#   fase 0, clube da Primeira na entrada das estatisticas da Primeira com 1 -> 12 e 2 -> 16, clube da Segunda dobrado.
#   Prologo mov ecx, [ecx+8]; push ebx; push esi (5 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os recordes so a original contra o plano (o retorno e o status da
# linha); o texto pelo compare_text_writer sem noticia nos textos da repescagem (so a original pelo texto comum); o
# progresso lado a lado nos 3 bytes da entrada escrita.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('pol_second_update_staff_records_for_status', 0x7cc490, [0x8b, 0x15, 0x58, 0xf8, 0x9c, 0x00]),
           ('pol_second_build_fate_news_text', 0x7cc680, [0x53, 0x8b, 0x5c, 0x24, 0x28]),
           ('pol_second_set_club_progress', 0x7cc800, [0x8b, 0x49, 0x08, 0x53, 0x56])]
DOCS = ('os recordes de pessoal, o texto do destino e o progresso dos clubes da Segunda Divisão polonesa e da repescagem',
        'the Polish Second Division\'s and play-off\'s staff records, fate news text and club progress',
        [('pol_second_update_staff_records_for_status', 'PolSecond::update_staff_records_for_status', '0x7cc490', 'pol_second.cpp', 'thiscall, ret 0x18',
          'na temporada: título, acesso, vaga na repescagem e rebaixamento; na repescagem contra a Primeira, o clube da Segunda que vence sobe (recorde de acesso), o da Primeira que perde cai (recorde de rebaixamento), e o status da linha marca quem sobe, fica ou cai',
          'in the season: title, promotion, play-off place and relegation; in the play-off against the First Division, the Second Division club that wins goes up (promotion record), the First Division club that loses goes down (relegation record), and the row status marks who goes up, stays or goes down'),
         ('pol_second_build_fate_news_text', 'PolSecond::build_fate_news_text', '0x7cc680', 'pol_second.cpp', 'thiscall, ret 0x28',
          'na repescagem, quem vence: "escapa do rebaixamento" (clube da Primeira) ou "conquista o acesso" (clube da Segunda); clube da Primeira que perde: "rebaixado"; os outros o texto comum; com notícia, o texto comum vem antes',
          'in the play-off, the winner: "avoid relegation" (a First Division club) or "win promotion" (a Second Division club); a First Division club that loses: "relegated"; the others the common text; with a news item, the common text comes first'),
         ('pol_second_set_club_progress', 'PolSecond::set_club_progress', '0x7cc800', 'pol_second.cpp', 'thiscall, ret 0x14',
          'na temporada, o melhor 3 ou 4 vira 2 (a faixa da repescagem) e o pior 3 vira 5; na repescagem, o clube da Primeira vai às estatísticas da Primeira com 1 → 12 e 2 → 16, o da Segunda com os códigos dobrados',
          'in the season, a best of 3 or 4 becomes 2 (the play-off band) and a worst of 3 becomes 5; in the play-off, a First Division club goes into the First Division\'s statistics with 1 → 12 and 2 → 16, a Second Division club with its codes doubled')])
CAMPOS = []
INTERESSA = []
