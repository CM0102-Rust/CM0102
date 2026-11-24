# A historia do seguro pago ao clube pela aposentadoria precoce de um jogador (european_cup.cpp pelo mapa; leitor cx04,
# rodada 5, 24/11/2026). Simulacao, carreira ENG: chamada pela aposentadoria por lesao (0x7a9629); rara antes de
# 2002-06-01 (o comparador so a ve quando acontece).
# - 0x58a980 news_post_insurance_payout(clube, jogador, valor) cdecl (add esp, 0xc): noticia 0xfc2; o jogador (0..3), o
#   valor (4) e o clube (5); sem testes; News::send_to_club_manager ao clube. Quadro SEH so para a noticia local.
#   Prologo push -1; push 0x958cfb (7 bytes). Igual no cm0102-1080.exe.
# Comparador: compare_story_post do cx04_7 (integrar depois dele).
SAVE = False
ROTINAS = [('news_post_insurance_payout', 0x58a980, [0x6a, 0xff, 0x68, 0xfb, 0x8c, 0x95, 0x00])]
DOCS = ('a história do seguro pela aposentadoria precoce', 'the insurance payout story',
        [('news_post_insurance_payout', 'news_post_insurance_payout', '0x58a980', 'european_cup.cpp', 'cdecl',
          'manda ao técnico do clube a história de que o clube recebe o seguro pela aposentadoria precoce de um jogador por lesão, com o jogador e o valor',
          'sends the club\'s manager the story that the club receives an insurance payout for a player\'s early retirement through injury, with the player and the amount')])
CAMPOS = []
INTERESSA = []
