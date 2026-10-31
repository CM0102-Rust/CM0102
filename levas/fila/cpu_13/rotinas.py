# A nota-alvo da busca de jogadores (player_search.cpp, arquivo sem dono nesta rodada, pela falta de rotinas pequenas
# na família de competições): player_search_get_club_target_rating (0x7bca90, 5994 chamadas na cobertura de 17/10: a
# nota que o clube da IA procura nas buscas de transferência) e duas do histórico de jogos do jogador acompanhado,
# player_search_slot_get_match_entry (0x7bd830) e player_search_slot_sum_goals (0x7bd870), só pelas telas
# (passageiras). A 0x7bca90 só chama rotinas já portadas (0x53c590, 0x5a0590) e a 0x5986b0 (a faixa de riqueza, uma
# folha de finance.cpp, original); as contas de ponto flutuante em f64 (o x87 do jogo trabalha em 53 bits). Sem SEH.
# Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira (o comparador confere a nota e a base a cada chamada)
# e a confirmação no jogo real comparando. Comparadores: as três pela devolução (a nota com as bases em saídas próprias).
SAVE = False
ROTINAS = [('nota_alvo_do_clube', 0x7bca90, [0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10]),
           ('jogo_do_historico', 0x7bd830, [0x8a, 0x54, 0x24, 0x04, 0x8a, 0x81, 0xa1, 0x01, 0x00, 0x00]),
           ('gols_do_historico', 0x7bd870, [0x8a, 0x81, 0xa1, 0x01, 0x00, 0x00])]
DOCS = ('a nota-alvo da busca de jogadores do clube e o histórico de jogos do jogador acompanhado: o jogo da vez e a soma dos gols',
        "the club's player search target rating and the tracked player's match history: the entry and the goal total",
        [('nota_alvo_do_clube', 'player_search_get_club_target_rating', '0x7bca90', 'player_search.cpp', 'cdecl',
          'a nota que o clube procura: a base pela reputação, o dinheiro, a nação e a liga, por faixas, e os ajustes pelo elenco e pela riqueza; no mínimo 500',
          'the rating the club looks for: the base from reputation, money, nation and league, by bands, and the adjustments for squad size and wealth; at least 500'),
         ('jogo_do_historico', 'player_search_slot_get_match_entry', '0x7bd830', 'player_search.cpp', 'thiscall, ret 4',
          'o jogo i (0x29 bytes) do anel de 10 do jogador acompanhado, ou 0 além da conta',
          "match entry i (0x29 bytes) of the tracked player's ring of 10, or 0 past the count"),
         ('gols_do_historico', 'player_search_slot_sum_goals', '0x7bd870', 'player_search.cpp', 'thiscall, ret 0',
          'a soma dos gols dos jogos guardados no anel',
          'the sum of the goals of the matches stored in the ring')])
CAMPOS = [('clube +0x77: o dinheiro (dword); +0x80 a reputação (palavra); +0x57 a nação (+0x69 a reputação, palavra); +0x1db a liga em [0xae23d0] (0x6b bytes, +0x69 a reputação); +0x64 profissional (byte, 1); ponteiro e valor; lê (nota_alvo_do_clube)',
           'club +0x77: the money (dword); +0x80 the reputation (word); +0x57 the nation (+0x69 the reputation, word); +0x1db the league in [0xae23d0] (0x6b bytes, +0x69 the reputation); +0x64 professional (byte, 1); pointer and value; reads (nota_alvo_do_clube)'),
          ('[0xadadfc] as nações com liga (dword por id); [0xae2a38] as finanças; ponteiro; lê (nota_alvo_do_clube)',
           '[0xadadfc] the nations with a league (dword per id); [0xae2a38] the finances; pointer; reads (nota_alvo_do_clube)'),
          ('jogador acompanhado +6: o anel de 10 jogos (0x29 bytes, +0x16 os gols, byte com sinal); +0x1a0 o começo; +0x1a1 a conta; valor; lê (jogo_do_historico, gols_do_historico)',
           'tracked player +6: the ring of 10 matches (0x29 bytes, +0x16 the goals, signed byte); +0x1a0 the start; +0x1a1 the count; value; reads (jogo_do_historico, gols_do_historico)')]
INTERESSA = ['CMRTE', 'CMScout']
