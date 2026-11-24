# Os textos das historias do jogador com o tecnico: a gratidao pela fase, a euforia com a virada e a chance no time
# principal (european_cup.cpp pelo mapa; leitor sbD, rodada 5, 24/11/2026; as sobras do cx04). Chamados so pelo despacho
# das noticias (NEWS_WRITERS Plain: (4, 0xbde), (4, 0xbe1), (4, 0xbe2)) quando o texto e escrito: na carreira automatica
# pelo modo CM_TEXTO_NOTICIAS (CM_TEXTOS=1 na cadeia). Os postadores: 0x589300, 0x5896d0 e 0x589bc0 (cx04_9).
# Convencoes conferidas como no sbD_1, mais o sprintf da CRT 0x944c9f (cdecl, "%d" 0x98806c). cdecl(noticia, corpo, Text).
# Quadros grandes (chkstk); prologos mov eax, N (5 bytes, antes do call __chkstk), como o 0x58a900 do cx04_12. Rotinas
# inteiras iguais no cm0102-1080.exe.
# - 0x589480 news_thanks_manager_text: vagas 0..3 o jogador, 5 a fama (1 jornal local), 6..9 o tecnico.
# - 0x589880 news_ecstatic_at_turnaround_text: mais 4 o clube, 10 o adversario e 11 a sequencia.
# - 0x589d60 news_pleased_with_chance_text: mais 4 o clube e 10 a redacao sorteada (0 nos clubes menores).
# Auxiliar comum (pub(crate)): player_headline_text (a manchete de um jogador so, num buffer dado), que serve ao sbD_4.
# Usa o NewsPerson e o story_newspaper do sbD_1 (integrar depois dele).
SAVE = False
ROTINAS = [('news_thanks_manager_text', 0x589480, [0xb8, 0xf0, 0x16, 0x00, 0x00]),
           ('news_ecstatic_at_turnaround_text', 0x589880, [0xb8, 0x60, 0x17, 0x00, 0x00]),
           ('news_pleased_with_chance_text', 0x589d60, [0xb8, 0xfc, 0x16, 0x00, 0x00])]
DOCS = ('os textos das histórias do jogador com o técnico: gratidão, virada e chance no time',
        'the texts of the player-and-manager stories: thanks, turnaround and first-team chance',
        [('news_thanks_manager_text', 'news_thanks_manager_text', '0x589480', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("<jogador> agradece ao chefe") ou o corpo da história do jogador que credita ao técnico a boa fase, com o jogador, o jornal (local para os de fama 1, senão nacional) e o técnico; vínculo do jogador',
          'writes the headline ("<player> thanks boss") or the body of the story of the player crediting his manager for his form, with the player, the newspaper (local for fame 1, otherwise national) and the manager; link to the player'),
         ('news_ecstatic_at_turnaround_text', 'news_ecstatic_at_turnaround_text', '0x589880', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("<jogador> eufórico com a virada") ou o corpo da história da vitória depois de uma sequência sem vencer, com o jornal, o clube, o jogador, o tamanho da sequência e o adversário; vínculos do jogador, do clube e do adversário',
          'writes the headline ("<player> ecstatic at turn of fortune") or the body of the story of the win after a winless run, with the newspaper, the club, the player, the length of the run and the opponent; links for the player, the club and the opponent'),
         ('news_pleased_with_chance_text', 'news_pleased_with_chance_text', '0x589d60', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("<jogador> contente com a chance") ou o corpo da história do jogador contente com a chance no time principal: com a redação sorteada (clubes grandes) a frase cita o técnico, senão o nome curto do clube; com o jornal; vínculos do jogador e do clube',
          "writes the headline (\"<player> pleased to have his chance\") or the body of the first-team chance story: with a drawn wording (big clubs) the line names the manager, otherwise the club's short name; with the newspaper; links for the player and the club")])
CAMPOS = []
INTERESSA = []
