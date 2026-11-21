# O texto de uma notícia (NewsItem::write_text 0x77bc80, news.cpp, 28.400 bytes; leitor gra, rodada 4, 21/11/2026).
# A rotina inteira desviada num item só, partida em pedaços com nome (um por tipo escrito ali mesmo). Convenções
# conferidas na desmontagem (cada push e cada lea [esp+k] seguidos por um script até a 0x66f4e0/0x4ae8a0):
# - a rotina: thiscall(notícia, técnico, Text, corpo, flag), ret 0x10; sem técnico ou sem Text a caixa de news.cpp
#   (linhas 0x9f e 0xa7) e 0; o Text é limpo (Text::clear 0x4afa40) e a notícia vai pela categoria (0x785670) e pelo
#   tipo (dword +0); tipo desconhecido: a caixa da linha 0x1226 e 0;
# - 140 tipos vão para o escritor do assunto (tabela NEWS_WRITERS, gerada da desmontagem): thiscall(objeto do global,
#   notícia, corpo, Text) ret 0xc, ou cdecl(notícia, corpo, Text); com outros argumentos: 0x1b (0x4143f0: +5, +9,
#   notícia, Text, corpo), 0x1c (0x414420: notícia, Text, corpo), 0x1f/0x23/0x1a (o objeto da competição
#   G_COMP_OBJECT_TABLE[+9 ou +5]; 0x1a sem o objeto devolve 0 sem caixa);
# - os pedaços escritos ali: categoria 2 tipos 1, 3..13, 16..23 e 0x26..0x28 (jogo remarcado, televisionado, sedes,
#   amistosos e torneios amistosos, tabela da liga, vagas de play-off, ligas remanejadas, sorteios de copa e de
#   grupos, Intertoto, a folga da FA para o Mundial de Clubes); categoria 7 tipo 0x3e9 (recorde da competição);
#   categoria 3 tipo 0x7d0 (lesão e doença, 5 KB: manchete, doença, lesão no jogo com e sem o registro do jogo,
#   lesão no treino, sueco); categoria 1 tipo 0x1388 (mensagem de técnico, com a notícia citada escrita por esta
#   mesma rotina, flag 1);
# - tipos 12 e 16: o método virtual +0x48 do objeto da competição (10 argumentos, o callee limpa 0x28);
# - funções chamadas por endereço sem invólucro: 0x5c89a0 e 0x5ca090/0x5ca290 (amistosos, thiscall em [0xae2c74]),
#   0x54c3b0 (Date, 3 args), 0x4b0010 (cdecl), 0x62ef30/0x6306e0 (stdcall; o original põe G_INJURY no ecx),
#   0x62ed30 (thiscall), 0x54d170 e 0x7e5780 (cdecl), 0x786b70 (thiscall, ret 0xc), 0x785a20 (thiscall), 0x4ae590
#   (Text::Text); os bytes e palavras que o original empilha com lixo em cima vão estendidos com zero (os callees
#   conferidos leem só o byte ou a palavra).
# Defeitos do original: no tipo 9 sem torneio e resposta fora de 2..4, e no tipo 21 com menos de 2 clubes, o texto
# junta um buffer nunca escrito (lixo da pilha); a cópia usa texto vazio.
# Comparador: compare_text_writer (texto, vínculos, gerador) + o retorno. Os escritores dos outros tipos rodam nas duas
# pernas (os portados com o próprio comparador por dentro).
SAVE = False
ROTINAS = [('newsitem_write_text', 0x77bc80, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o texto de uma notícia',
        'the text of a news item',
        [('newsitem_write_text', 'newsitem_write_text', '0x77bc80', 'news.cpp', 'thiscall, ret 0x10',
          'escreve a manchete ou o corpo de uma notícia no Text: sem técnico ou sem Text, a caixa de erro e 0; senão limpa o Text e escolhe pela categoria e pelo tipo: 140 tipos vão para o escritor do assunto (mídia, relatório do treinador, transferências, competições, disciplina, recordes...), e a própria rotina escreve o jogo remarcado, o jogo escolhido para a televisão, as candidaturas e a escolha da sede, a proposta, a resposta e o cancelamento de amistosos e de torneios amistosos, o relatório da competição, a tabela da liga com os três primeiros jogos do clube, a vaga de play-off por terço ou quarto da temporada, as ligas remanejadas, os sorteios de copa (com o estádio e a data do jogo) e de grupos (com a lista dos adversários), o convite e a escolha da Intertoto, a folga da FA ao clube do Mundial de Clubes, o recorde de uma competição, a lesão ou doença de um jogador (por categoria: tratamento, doença, lesão; no jogo, com o placar e a fase, ou no treino; o texto sueco próprio) e a mensagem de um técnico (citando a notícia respondida); tipo desconhecido, a caixa de erro e 0',
          "writes the headline or body of a news item into the Text: with no manager or no Text, the error box and 0; otherwise clears the Text and picks by category and type: 140 types go to their subject's writer (media, coach report, transfers, competitions, discipline, records...), and the routine itself writes the rearranged match, the match chosen for television, the bids for and choice of a host, the friendly and friendly tournament offers, answers and cancellations, the competition's report, the league's fixture list with the club's first three games, the play-off place won over a third or a quarter of the season, the reshuffled leagues, the cup draws (with the stadium and the date) and group draws (with the list of opponents), the Intertoto invitation and choice, the FA's bye for the World Club Championship club, a competition record, a player's injury or illness (by category: treatment, illness, injury; in a match, with the score and the round, or in training; its own Swedish text) and a manager's message (quoting the news it answers); an unknown type gives the error box and 0")])
