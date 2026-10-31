# O fisico depois da partida (Injury::apply_match_effects 0x62c080, pela fechar_partida_do_dia 0x6b5f00 e pelas
# estatisticas simuladas 0x84be20, em toda partida: o ritmo de recuperacao de cada jogador das duas escalacoes e as
# batidas da partida que viram lesao), a batida do jogador na partida (Injury::find_match_injury 0x62bf60, pelas telas
# do pessoal) e o jogador de ferias (Injury::is_player_on_holiday 0x631550, pela tela do clube). Chamadas originais pelo
# endereco: 0x456740 (data), 0x62d250 (a lesao nova), 0x62e700 (o fim da lesao, cpv_10), 0x62ed70 (a lesao atual) e
# _ftol (0x945640); o resto ja em Rust. Iguais no cm0102-1080.exe. Trocas: das candidatas da familia, a
# injury_get_type_name_gender 0x6306e0 comeca com chamada relativa (sem prologo para o desvio); a daily_update
# 0x62b7b0, a add_injury 0x62d250 e a update_player_fitness 0x62c680 (1,4 a 2,6 KB) ficam para outra leva. Simulacao:
# as mutacoes pela carreira ate 2002-06-01, todas no ritmo de recuperacao (0x62c080). Comparadores: o fisico lado a
# lado (estado do sorteio e o +0xa do registro de cada pessoa), com a partida com batida que chega a lesao so na
# original (a lesao nova aloca e manda a noticia; a gravidade da primeira batida pode ser sobra de pilha, a copia usa
# 0); a batida lado a lado no cursor da lista e na devolucao; as ferias pela devolucao.
SAVE = False
ROTINAS = [('efeitos_fisicos_da_partida', 0x62c080, [0x83, 0xec, 0x20, 0x53, 0x55]),
           ('batida_do_jogador', 0x62bf60, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x28]),
           ('jogador_de_ferias', 0x631550, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x28, 0x3d, 0xb6, 0x00])]
DOCS = ('o físico depois da partida: o ritmo de recuperação de cada jogador e as batidas que viram lesão, a batida do jogador na partida e o jogador de férias',
        "the fitness after the match: each player's recovery rate and the knocks that become injuries, the player's knock in the match and the player on holiday",
        [('efeitos_fisicos_da_partida', 'Injury::apply_match_effects', '0x62c080', 'injury.cpp', 'thiscall, ret 8',
          'para as 20 vagas de cada lado da partida: o ritmo (nível da vaga mais (10000 − condição) / 100; metade no jogo sem a marca +0x40; × 1,25 acima de 33 anos, o dobro abaixo de 23, com sorteio) vai para a recuperação (0x62bd10); as batidas da pessoa na lista da partida viram lesão (0x62d140, 0x62e700, 0x62d250)',
          "for the 20 slots of each side of the match: the rate (slot level plus (10000 − condition) / 100; half in a match without the +0x40 flag; × 1.25 above 33 years, double below 23, with a random draw) goes to the recovery (0x62bd10); the person's knocks in the match list become injuries (0x62d140, 0x62e700, 0x62d250)"),
         ('batida_do_jogador', 'Injury::find_match_injury', '0x62bf60', 'injury.cpp', 'thiscall, ret 4',
          'o item da lista das batidas da partida (+0x28) com o id da pessoa, ou 0',
          "the match knock list (+0x28) item with the person's id, or 0"),
         ('jogador_de_ferias', 'Injury::is_player_on_holiday', '0x631550', 'injury.cpp', 'thiscall, ret 4 (this não é lido)',
          '1 se a lesão atual da pessoa (0x62ed70 no [0xb63d28]) é de férias (tipo 0x51)',
          "1 if the person's current injury (0x62ed70 on [0xb63d28]) is a holiday (type 0x51)")])
CAMPOS = [('partida +0x1c0 + 0x625 × lado + 0x49 × vaga: −0x30 id da pessoa; −0x11 marca da batida; +0, +2, +4 níveis (0xff = nenhum); +6 condição (16 bits); id e valor; lê (efeitos_fisicos_da_partida)',
           'match +0x1c0 + 0x625 × side + 0x49 × slot: −0x30 person id; −0x11 knock flag; +0, +2, +4 levels (0xff = none); +6 condition (16 bits); id and value; reads (efeitos_fisicos_da_partida)'),
          ('partida +0x4c: marca (nível 90 nas onze primeiras); +0x4d: nível padrão; valor; lê (efeitos_fisicos_da_partida)',
           'match +0x4c: flag (level 90 for the first eleven); +0x4d: default level; value; reads (efeitos_fisicos_da_partida)'),
          ('jogo +0x40: marca do lado da casa (sem ela, o ritmo pela metade); +0: id (−2 sem jogo para a lesão); valor; lê (efeitos_fisicos_da_partida)',
           'fixture +0x40: home side flag (without it, half the rate); +0: id (−2 means no fixture for the injury); value; reads (efeitos_fisicos_da_partida)'),
          ('pessoa +0x18: idade; +0x61: ficha (+0xf nota com sinal, 15 ou mais para a batida de tipo 7); valor; lê (efeitos_fisicos_da_partida)',
           'person +0x18: age; +0x61: player data (+0xf signed rating, 15 or more for a type 7 knock); value; reads (efeitos_fisicos_da_partida)'),
          ('Injury +0x28: lista das batidas da partida (item: +0 id, +4 tipo, +5 gravidade; cursor +0xa recomeça de +2); ponteiro, id e valor; lê e grava o cursor (efeitos_fisicos_da_partida, batida_do_jogador)',
           'Injury +0x28: match knock list (item: +0 id, +4 type, +5 severity; cursor +0xa restarts from +2); pointer, id and value; reads and writes the cursor (efeitos_fisicos_da_partida, batida_do_jogador)'),
          ('[0xb63d28] Injury; tabela de lesões de 0x30 bytes (+0 tipo, 0x51 férias); ponteiro e valor; lê (jogador_de_ferias)',
           '[0xb63d28] Injury; 0x30-byte injury table (+0 type, 0x51 holiday); pointer and value; reads (jogador_de_ferias)')]
INTERESSA = ['CMRTE']
