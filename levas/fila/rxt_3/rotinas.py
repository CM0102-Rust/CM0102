# O relatorio do olheiro sobre o proximo adversario, do player_search.cpp (leitor rxt, rodada 6, 25/11/2026): 0x7be670,
# 5,4 KB, inteira num desvio. Chamada so pelo texto da noticia da busca (0x7b81f0, rxt_1: o corpo da observacao de
# clube, 0x7b8e89), que so sai com o olheiro observando o proximo adversario por ordem do tecnico humano. A carreira
# automatica nao passa.
# TELAS: a prova e o Lucas jogando comparando: mandar um olheiro observar o proximo adversario e abrir a noticia do fim
# da observacao (o corpo).
# Fronteiras: 0x7be670..0x7bfbae (rets em 0x7be708, 0x7bee8f, 0x7bfb6c, 0x7bfb94 e 0x7bfbad), depois a tabela de saltos
# 0x7bfbb0 (7, a faixa de forca) ate 0x7bfbcc; 0x7bfbd0 ja e outra rotina (sx17_8). Prologo mov eax, 0x1b30 (5 bytes,
# sem relativas; o _chkstk 0x945610 vem depois); igual no cm0102-1080.exe.
# Convencao: cdecl(saida, clube do tecnico, clube observado, faixa (byte), &respostas (13 bytes), n (dword; o byte baixo
# no ferido), &jogadores (0x1a bytes cada), &ids, &quantos (byte)), sem retorno. Chamadas conferidas: 0x53dba0
# cdecl(tecnico, buf, 0x78, 1); 0x53e140 cdecl(n1, n2, n3, buf, 0xc8, estilo, id); 0x66f4e0 e 0x944c9f cdecl; as tres
# escolhas por posicao 0x7bfcd0, 0x7bfbd0, 0x7bfc50 cdecl(n (byte), jogadores, clube (so conferido contra nulo),
# saida) -> al (o sx17_8 as le com tres argumentos: ver o relatorio do rxt).
# Defeitos do original (DEFEITOS_DO_ORIGINAL): o tecnico sem ficha de nao jogador (+0x69) cita a formacao de um buffer
# nao iniciado; a frase do ponta le as marcas do segundo meia com um meia so; o paragrafo do ponto fraco sem o da forca
# (faixa abaixo de 0, inalcancavel com a faixa 0..6 do club_squad_strength_band) imprime o que o langlib deixou em edx;
# as frases dos jogadores-chave testam a posicao do primeiro dos melhores jogadores, nao a da escolha; as frases de
# "sem ponto fraco" escrevem por cima do paragrafo da forca. A copia nao inventa os valores nao iniciados: escreve
# vazio e marca OPPOSITION_REPORT_UNSET, e o comparador deixa o texto de lado nesses casos.
# Comparador: lado a lado do texto (0x1000 bytes da saida), dos ids, de quantos e do gerador.
TELAS = True
ROTINAS = [('write_opposition_report_text', 0x7be670, [0xb8, 0x30, 0x1b, 0x00, 0x00])]
DOCS = ('o relatorio do olheiro sobre o proximo adversario', "the scout's opposition report",
        [('write_opposition_report_text', 'player_search_write_opposition_report_text', '0x7be670', 'player_search.cpp', 'cdecl',
          'escreve o relatorio do olheiro sobre o clube observado em tres paragrafos. 1: a formacao e o estilo que o tecnico (ou o auxiliar) prefere, pela ficha dele (formacao 1..9, senao 4-4-2; ataque e objetividade: 15 ou mais, 5 ou menos), ou "espera-se um 4-4-2" sem tecnico. 2: a forca do elenco pela faixa 0..6 (pateticos .. de nivel mundial), com a organizacao, a posicao ruim na liga (abaixo de -3) e a reputacao dos dois clubes. 3: abaixo da faixa 2, o ponto fraco pelas respostas (cruzamentos, velocidade, criatividade); da faixa 2 em diante, os jogadores-chave, na ordem: o melhor jogador lesionado, a tabela entre o primeiro meia e o primeiro atacante, dois atacantes, dois meias, dois zagueiros, um atacante pelas marcas e pela habilidade atual (acima de 160, abaixo de 100, abaixo de 125, o resto), o ponta, o meia criativo ou combativo, um zagueiro, um meia; sem nenhum, "nao tem ponto fraco evidente" no lugar do paragrafo da forca. Os jogadores citados vao para os ids (para os vinculos). Sem saida, a caixa de erro; sem clubes, jogadores ou ids, "nao achei o centro de treinamento". Defeitos do original: a formacao de buffer nao iniciado sem ficha do tecnico; as marcas do segundo meia com um meia so; o paragrafo do ponto fraco sem o da forca; os testes de posicao pelo primeiro jogador; o paragrafo da forca sobrescrito',
          "writes the scout's report on the scouted club in three paragraphs. 1: the formation and style its manager (or assistant) prefers, by his staff sheet (formation 1..9, else 4-4-2; attacking and directness: 15 and up, 5 and below), or \"expected to play a 4-4-2\" without a manager. 2: the squad strength by the band 0..6 (pathetic .. world class), with the organisation, a poor league position (below -3) and the two clubs' reputations. 3: below band 2, the weak point by the answers (crosses, pace, creativity); from band 2, the key players, in this order: the injured best player, the link-up between the first midfielder and striker, two strikers, two midfielders, two defenders, one striker by his flags and current ability (over 160, below 100, below 125, the rest), the winger, the creative or combative midfielder, one defender, one midfielder; without any, \"no obvious weakness\" in place of the strength paragraph. The players named go to the ids (for the links). Without the output, the error box; without the clubs, players or ids, \"couldn't find their training ground\". Defects of the original: the formation from an uninitialised buffer without the manager's sheet; the second midfielder's flags with one midfielder; the weak-point paragraph without the strength one; the position checks by the first player; the strength paragraph overwritten")])
