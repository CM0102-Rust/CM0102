# O texto da notícia de como os jogadores de um clube foram pela seleção (NationalTeams::write_club_players_international_text
# 0x770d20, national_teams.cpp, 16.912 bytes; leitor sx15, rodada 5, 23/11/2026). Escritor do NewsItem::write_text
# (0x77bc80, ponto de chamada 0x77be87) para a notícia 0x24 que send_club_international_report_news (0x7706c0) monta.
# A rotina inteira desviada num item só, partida em pedaços com nome (manchete, textos do corpo, corpo sem e com o
# jogador em destaque, vínculos) e com as chamadas ao formatador numa tabela (árvores de modelos por resultado,
# código de atuação e sorteio). Convenções conferidas na desmontagem (cm0102.exe):
# - a rotina: thiscall(objeto de 0xb74318, notícia, corpo, Text), ret 0xc; o this não é lido; quadro SEH; fim real
#   0x774f30 (ret 0xc em 0x774f2d), sem tabela de salto;
# - 0x7e4fa0 cdecl(notícia, &k, &registro) e 0x785830 thiscall(notícia, k) com o contador de byte; o registro começa
#   com Date::Date (0x54a110);
# - 0x53b3d0 cdecl(nação) e 0x542100 cdecl(seleção, 0 ou o flag da seleção B, buf, 300, 1, 0 no corpo ou 1 na
#   manchete); 0x4bfa30 cdecl(competição, dword +0xc, dword +0xe, 1, buf, 200) e 0x4bff70 cdecl(..., 1): o gênero do
#   rótulo (é o par de inteiros que o Ghidra chama "cVar4" nos modelos, não a quantidade de jogadores);
# - 0x53e140 cdecl(primeiro, segundo, comum, buf, 0x78 ou 300, modo 1 ou 5, id); 0x53e990 cdecl(pessoa, ids, 0);
#   0x729500 thiscall([0xb6b7c0], partida, clube, adversário, dword +0, dword +2) (RecordMatch::link_score);
# - os 81 formatos (0x66f4e0, cdecl variádico): os argumentos de cada um tirados da desmontagem/pseudocódigo e
#   conferidos contra o próprio modelo (cada modelo descreve os seus: {} = um inteiro, <%s - Club Name>, <%s - Nation
#   (e.g.England)>...): a contagem de chaves e a ordem dos rótulos batem em todos.
# Defeito do original: com mais de 4 jogadores do clube os slots dos jogadores não são lidos, a marca 0x420e9 não é
# achada e, com jogador em destaque, o modelo sai de um sorteio nunca lido (dword da pilha sem início); a cópia usa 0
# e o comparador deixa esse caso só com a original.
# Comparador: compare_text_writer (texto, vínculos, gerador).
SAVE = False
ROTINAS = [('write_club_players_international_text', 0x770d20, [0x6a, 0xff, 0x68, 0xbb, 0xde, 0x95, 0x00])]
DOCS = ('o texto da notícia de como os jogadores de um clube foram pela seleção',
        "the news text on how a club's players did for their nation",
        [('write_club_players_international_text', 'NationalTeams::write_club_players_international_text', '0x770d20', 'national_teams.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia 0x24 ao técnico do clube depois de um jogo de seleção. Manchete: um jogador ("X convocado para o jogo da Inglaterra", o do destaque ou o único, com o vínculo quando os nomes ainda batem) ou o clube ("jogadores do Leeds no jogo da Inglaterra"), e o vínculo da seleção (0x7d6 nos jogos da seleção B, senão 0x7d5). Corpo: os nomes dos jogadores, das duas seleções, o rótulo do jogo ("friendly" em minúscula) e o placar com o maior primeiro; sem jogador em destaque, um jogador (ou dois com código) segue a árvore do jogador sozinho, três ou quatro (ou dois sem código) a frase da lista, mais de quatro a frase da contagem; com destaque, a árvore do destaque e depois "e X", "e a lista" ou "e N outros do clube". As árvores escolhem o modelo pelo resultado (vitória, empate, derrota), pela primeira máscara que o código do primeiro jogador tem (0x44, 0x14, 0x40, 0x24, 0x20, 8, 0x12, 0x10, 2, 4, 1: gols, saiu antes, melhor em campo, pênalti perdido) e pelo sorteio (0 ou 1). Depois os vínculos: jogadores, placar, clube e as duas seleções. Defeito: com mais de quatro jogadores do clube o destaque escolhe o modelo por um sorteio nunca lido (lixo da pilha)',
          "the headline or body of news 0x24 to the club's manager after an international. Headline: one player (\"X selected for England match\", the highlighted or the only one, with his link when his names still match) or the club (\"Leeds players selected in England match\"), and the national side's link (0x7d6 in the B internationals, else 0x7d5). Body: the players' names, both sides' names, the fixture label (\"friendly\" lowercased) and the score line, highest first; without a highlighted player, one player (or two with a code) follows the lone-player tree, three or four (or two without a code) the list sentence, more than four the count sentence; with one, the highlighted tree and then \"and X\", \"and the list\" or \"and N others from the club\". The trees pick the template by the result (won, drawn, lost), the first mask the first player's code holds (0x44, 0x14, 0x40, 0x24, 0x20, 8, 0x12, 0x10, 2, 4, 1: goals, left early, man of the match, missed penalty) and the draw (0 or 1). Then the links: players, score, club and both sides. Defect: with more than four of the club's players the highlighted text picks its template by a draw never read (stack garbage)")])
