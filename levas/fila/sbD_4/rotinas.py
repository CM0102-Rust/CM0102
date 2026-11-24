# Os textos das historias do credito ao tecnico pelo premio e da multa disciplinar da FA (european_cup.cpp pelo mapa;
# leitor sbD, rodada 5, 24/11/2026; as sobras do cx04). Chamados so pelo despacho das noticias (NEWS_WRITERS Plain:
# (4, 0xbe3), (6, 0x178c)) quando o texto e escrito: na carreira automatica pelo modo CM_TEXTO_NOTICIAS (CM_TEXTOS=1 na
# cadeia). Os postadores: 0x58a070 e 0x58a4d0 (cx04_10).
# Convencoes conferidas como no sbD_1, mais nation_get_nationality_adjective 0x6181d0 (cdecl(nacao, buf, 0x78, 9)),
# Cash::get_display_amount 0x43f460 (thiscall(&valor, 6) -> edx:eax) e cash_format_money 0x441c10 (cdecl(buf, 0x1f4,
# i64, 6)). cdecl(noticia, corpo, Text). Rotinas inteiras iguais no cm0102-1080.exe.
# - 0x58a210 news_credits_manager_for_award_text: vagas 0..3 o jogador, 4 o clube, 5 a fama, 6..9 o tecnico, 10 o premio
#   (registro de 0x65 bytes de G_STAFF_COMP_TABLE: +4 o nome, +0x37 o genero). Prologo mov eax, 0x16f8 (5 bytes).
# - 0x58a660 news_fa_disciplinary_fine_text: vagas 0 o clube, 1..4 o tecnico, 5 a nacao, 6 o tipo, 7 o valor; quadro SEH
#   (nivel 0 com destrutor 0x958cbb, so no desenrolar). Prologo push -1; mov eax, fs:[0] (8 bytes).
# Usa o NewsPerson e o story_newspaper do sbD_1 e o player_headline_text do sbD_3 (integrar depois dos dois).
SAVE = False
ROTINAS = [('news_credits_manager_for_award_text', 0x58a210, [0xb8, 0xf8, 0x16, 0x00, 0x00]),
           ('news_fa_disciplinary_fine_text', 0x58a660, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os textos das histórias do crédito pelo prêmio e da multa da FA',
        'the texts of the award credit and FA fine stories',
        [('news_credits_manager_for_award_text', 'news_credits_manager_for_award_text', '0x58a210', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("<jogador> reconhece a contribuição do técnico") ou o corpo da história do jogador que credita ao técnico o prêmio, com o clube, o jogador, o jornal, o técnico e o nome do prêmio; vínculos do jogador e do clube',
          "writes the headline (\"<player> acknowledges his manager's contribution\") or the body of the story of the player crediting his manager for the award, with the club, the player, the newspaper, the manager and the award's name; links for the player and the club"),
         ('news_fa_disciplinary_fine_text', 'news_fa_disciplinary_fine_text', '0x58a660', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("Federação <nacionalidade> multa <clube>") ou o corpo da história da multa disciplinar, com o adjetivo da nação, o nome do clube e o valor formatado, em uma de duas redações pelo tipo; vínculos do clube',
          "writes the headline (\"<nationality> FA impose fine on <club>\") or the body of the disciplinary fine story, with the nation's adjective, the club's name and the formatted amount, in one of two wordings by the kind; links for the club")])
CAMPOS = []
INTERESSA = []
