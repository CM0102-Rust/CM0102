# Os textos das historias do jogador infeliz com o treino, da briga resolvida e do relatorio de treino (european_cup.cpp
# pelo mapa; leitor sbD, rodada 5, 24/11/2026; as sobras do cx04). Chamados so pelo despacho das noticias
# (NEWS_WRITERS Plain: (4, 0xbcd), (4, 0xbca), (1, 0x138d)) quando o texto e escrito: na carreira automatica pelo modo
# CM_TEXTO_NOTICIAS (CM_TEXTOS=1 na cadeia). Os postadores: 0x5885c0, 0x5888a0 e 0x588cd0 (cx04_8).
# Convencoes conferidas como no sbD_1; cdecl(noticia, corpo, Text). Rotinas inteiras iguais no cm0102-1080.exe.
# - 0x5886d0 news_unhappy_with_training_text: vaga 5 = a redacao sorteada (0 ou nao). Prologo mov eax, [esp+8]; sub esp,
#   0x194 (10 bytes).
# - 0x5889e0 news_feud_settled_text: vagas 0..3 o jogador, 4..7 o outro. Prologo mov eax, [esp+8]; sub esp, 0x334 (10).
# - 0x588f00 news_training_report_text: vagas 0..3 o jogador, 4..7 o auxiliar, 8 a avaliacao (tabela de saltos 0x5892ec,
#   5 casos), 9 = 999 elogio. Prologo sub esp, 0x338 (6 bytes).
# Usa o NewsPerson e o player_story_text do sbD_1 (integrar depois dele).
SAVE = False
ROTINAS = [('news_unhappy_with_training_text', 0x5886d0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('news_feud_settled_text', 0x5889e0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x34, 0x03, 0x00, 0x00]),
           ('news_training_report_text', 0x588f00, [0x81, 0xec, 0x38, 0x03, 0x00, 0x00])]
DOCS = ('os textos das histórias do jogador infeliz com o treino, da briga resolvida e do relatório de treino',
        'the texts of the unhappy-with-training, feud settled and training report stories',
        [('news_unhappy_with_training_text', 'news_unhappy_with_training_text', '0x5886d0', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("<jogador> infeliz com o treino") ou o corpo da história do jogador infeliz com o treino, com uma de duas redações pelo sorteio; vínculo do jogador',
          'writes the headline ("<player> unhappy with training") or the body of the unhappy-with-training story, with one of two wordings by the draw; link to the player'),
         ('news_feud_settled_text', 'news_feud_settled_text', '0x5889e0', 'european_cup.cpp', 'cdecl',
          'escreve a manchete ("<jogador> e <outro> fazem as pazes") ou o corpo da história da briga resolvida, com os dois nomes (completos no corpo, curtos na manchete); vínculos de cada nome',
          'writes the headline ("<player> and <other> settle differences") or the body of the feud settled story, with both names (full in the body, short in the headline); links for each name'),
         ('news_training_report_text', 'news_training_report_text', '0x588f00', 'european_cup.cpp', 'cdecl',
          'escreve o corpo do relatório de treino (elogio com 3 redações pela avaliação; crítica com 6 pela avaliação 0 a 4 ou outra), com os nomes do auxiliar e do jogador, ou a manchete: "Relatório de treino de <auxiliar>" quando o id do jogador mais a avaliação dá par, senão "Relatório de treino de <jogador>"; vínculos dos nomes',
          "writes the body of the training report (praise with 3 wordings by the assessment; criticism with 6 by the assessment 0 to 4 or other), with the coach's and the player's names, or the headline: \"Training report from <coach>\" when the player's id plus the assessment is even, otherwise \"<player> training report\"; links for the names")])
CAMPOS = []
INTERESSA = []
