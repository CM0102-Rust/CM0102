# Os clubes humanos que recebem uma noticia sobre um jogador (transfer_manager_collect_player_news_recipients
# 0x8d1da0, transfer_manager.cpp; leitor sx02, rodada 5, 23/11/2026). A rotina do transfer_manager.cpp que mais roda na
# carreira ENG (8.456 chamadas ate 2002-06-01): chamada pelas noticias de contrato (0x21a), lesao (0x5a), tecnico
# (0x52), regens (0xa), estatisticas e premios (0xda, 0x25a, 0x2da, 0x1a, 0x48, 0xd8).
# Convencoes conferidas:
# - a rotina: thiscall(TRANSFER_MANAGER, jogador, bandeiras, vetor de saida, maximo), ret 0x10, devolve al (a conta);
#   pontos de chamada 0x4d4f50, 0x62dcdb, 0x69fe9d... (push max; push vetor; push bandeiras; push jogador);
# - 0x842180 (Shortlist_Manager::collect_interested_clubs): thiscall(shortlist, jogador, lista, 1, 1, 0), ret 0x14;
# - 0x7c1e30 (PlayerStats::get_stat): thiscall(G_PLAYER_STATS, ficha, 2, 0x12, 6.5f), ret 0x10, st0;
# - a conta em bl (byte com sinal): comparacoes `cmp bl, [max]` com sinal; as reputacoes em words com sinal;
# - os 16 tecnicos humanos: G_STAFF_TABLE[G_STAFF_COUNT - k - 1], k = 0..15.
# Comparador: lado a lado com G_CONTRACT_LOOKUP_RESULT e o vetor de saida (max entradas).
SAVE = False
ROTINAS = [('collect_player_news_recipients', 0x8d1da0, [0x6a, 0xff, 0x68, 0xfb, 0x40, 0x96, 0x00])]
DOCS = ('os clubes humanos que recebem uma notícia sobre um jogador',
        'the human clubs that receive news about a player',
        [('collect_player_news_recipients', 'transfer_manager_collect_player_news_recipients', '0x8d1da0', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'regra: pelas bandeiras: 2, o clube do contrato do jogador quando tem técnico humano; 8, os clubes humanos que o observam (lista de observação); 0x10, cada um dos 16 técnicos humanos, pelo alcance da reputação do jogador: mundial acima de 9500 (com 0x200) ou 9200, todos; acima de 9000 ou 8300, os do mesmo continente (da nação do técnico ou da nação do clube dele); nacional ou atual acima de 8300 ou 7200, os da mesma nação (Inglaterra e Escócia se alcançam); senão, em forma (nota média acima de 9,25 − reputação × 0,00015 ou 0,00025) com reputação acima de 3000 e a bandeira 0x80, a nação do clube; senão, com 0x40, os clubes da mesma divisão do clube do jogador (reputação acima da da divisão × 750, no mínimo 5000, ou × 350 e acima de 2500, ou em forma contra 9,0). No fim as repetições saem (defeito: uma entrada movida não é conferida de novo)',
          "rule: by the flags: 2, the player's contract club when a human manages it; 8, the human clubs shortlisting him; 0x10, each of the 16 human managers, by the reach of the player's reputation: world above 9500 (with 0x200) or 9200, everyone; above 9000 or 8300, those of the same continent (the manager's nation or his club's nation); home or current above 8300 or 7200, those of the same nation (England and Scotland reach each other); otherwise, in form (average rating above 9.25 − reputation × 0.00015 or 0.00025) with reputation above 3000 and flag 0x80, the club's nation; otherwise, with 0x40, the clubs in the same division as the player's club (reputation above the division's × 750, at least 5000, or × 350 and above 2500, or in form against 9.0). At the end the duplicates go (defect: a moved entry is not checked again)")])
CAMPOS = []
INTERESSA = []
