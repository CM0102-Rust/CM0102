# O texto da notícia de suspensão (Discipline::write_ban_news_text 0x5576e0, discipline.cpp; leitor gra, rodada 4,
# 21/11/2026; 5.854 instruções, 16.848 bytes, inteira num item). Chamada pelas notícias (0x77bc80, tipo 0x7d2, e pela
# 0x55b8b0 no tipo 0x7d3). Convenções conferidas:
# - a rotina: thiscall(Discipline, notícia, corpo, Text), ret 0xc (o ecx é o objeto da disciplina, passado ao 0x55ce40);
# - slots 0..14 lidos em ordem (1 e 13 descartados; 15 e 16 pulados), a pessoa pelo 0x7e52c0 a partir do slot 17
#   (avança 6), o slot 23 (tem jogo) e o registro do jogo pelo 0x7e4fa0 a partir do 24; as duas datas iniciadas como no
#   Date::Date (0x54a110) antes da leitura;
# - 0x7e6f20 thiscall([0xb74434], competição), 0x4b0700 cdecl(índice), 0x53e990 cdecl(pessoa, 3 ids, &data),
#   0x729500 thiscall([0xb6b7c0], índice +0x1b, clube +0x11, clube +0x15, dword +0, dword +2) ret 0x14 (só com jogo);
# - 0x55ce40 thiscall(this, &slots 6..9, buf 0x64, 0x64, slot 12 inteiro, forma) ret 0x14, devolve o gênero em al:
#   chamada sempre com a forma 4 (gênero descartado) e, fora da agressão ao árbitro, de novo com 5 (um jogo) ou 6;
# - 0x549ef0 Date(slot 4, slot 5) e 0x54c3b0 (data, buf, 0x64, 0xff) quando o ano (16 bits) não é 0;
# - o placar "%d-%d" (maior primeiro) pelo langlib (0x66f4e0); o fecho pelo sprintf do C (0x944c9f): "%s" ou
#   "%s\n\n%s" com a nota da apelação (0x9b8f68);
# - os argumentos dos 126 modelos (0x66f4e0) extraídos por execução simbólica de todos os caminhos da desmontagem
#   (1.628 caminhos, 1.472 viáveis) e conferidos contra a regra da cópia (0 diferenças); os da agressão, das frases do
#   resultado, da manchete e do fecho conferidos push a push à mão;
# - DEFEITO do original: sem jogo (suspensões por pontos ou cartões) os vínculos dos dois clubes e do placar vêm de
#   pilha não iniciada (B+0x20..0x3f e B+0xd8): a cópia não os põe e o comparador os tira da lista do original nesse
#   caso; tipo acima de 4: o original imprime o buffer não iniciado, a cópia um texto vazio.
# Comparador: compare_text_writer (sem jogo, a variante que ignora os vínculos 0x7d5/0x7d3).
SAVE = False
ROTINAS = [('write_ban_news_text', 0x5576e0, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o texto da notícia de suspensão',
        'the news text of a ban',
        [('write_ban_news_text', 'Discipline::write_ban_news_text', '0x5576e0', 'discipline.cpp', 'thiscall, ret 0xc',
          'a manchete ("<nome> suspenso por um jogo", "por <n> jogos" ou, em dias, "por <duração>") ou o corpo da notícia de suspensão: a agressão ao árbitro (conjunto de regras 9 = 0 com apelação pendente: os dias, a competição pelo nome curto quando nomeada, e a frase de como o jogo terminou com o vínculo do placar); senão o modelo do tipo (pontos, cartões amarelos, cartão amarelo num jogo, expulsão, conduta imprópria, amarelos em jogos seguidos), escolhido pela contagem (N jogos, um jogo, dias), por ser "mais" uma suspensão, pela data de início ou simples, e, nos tipos de jogo, pela competição nomeada (com a frase de como terminou) ou pelo resultado do clube (vitória, empate, derrota com o placar); com apelação feita, a nota de que o jogador apelou; os vínculos da pessoa, dos dois clubes e do placar. Defeito do original: sem jogo, os vínculos dos clubes e do placar saem de pilha não iniciada (a cópia não os põe)',
          'the headline ("<name> banned for one match", "for <n> matches" or, in days, "for <length>") or the body of the ban news: the assault on a referee (rule set 9 = 0 with an appeal pending: the days, the competition by its short name when named, and the sentence on how the match ended with the score link); otherwise the template of the kind (points, yellow cards, a yellow card in a match, a sending-off, misconduct, yellows in successive matches), picked by the count (N matches, one match, days), by "a further" ban, by the start date or plain, and, for the match kinds, by the named competition (with the sentence on how it ended) or by the club\'s result (win, draw, defeat with the score); with the appeal made, the note that the player appealed; the links of the person, the two clubs and the score. Defect of the original: without a match, the clubs\' and the score\'s links come from uninitialised stack (the copy leaves them out)')])
