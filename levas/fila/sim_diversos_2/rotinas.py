ROTINAS = [('ofertas_pelo_jogador', 0x8c5cf0, [0x51, 0x8b, 0x44, 0x24, 0x08]),
           ('reputacao_alvo_do_clube', 0x83bf40, [0x56, 0x8b, 0x91, 0xdc, 0x00, 0x00, 0x00]),
           ('gravar_jogo_na_noticia', 0x4afeb0, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14])]
DOCS = ('as ofertas pelo jogador, a reputação-alvo do clube e o jogo na notícia',
        'the offers for a player, the club target reputation and the fixture in the news item',
        [('ofertas_pelo_jogador', 'TRANSFER_MANAGER::get_offers_for_player', '0x8c5cf0', 'transfer_manager.cpp', 'thiscall',
          'conta as ofertas de transferência pelo jogador (sem as encerradas, salvo pedido) e pode listar os índices delas',
          'counts the transfer offers for a player (skipping closed ones unless asked) and may list their indices'),
         ('reputacao_alvo_do_clube', 'Shortlist_Manager::update_club_target_reputation', '0x83bf40', 'shortlist_manager.cpp', 'thiscall',
          'a reputação que o clube procura nas contratações, ajustada pelo técnico e pelo presidente, entre o piso do clube e 10000',
          'the reputation a club aims for in signings, adjusted by manager and chairman, between the club floor and 10000'),
         ('gravar_jogo_na_noticia', 'comp_util_write_fixture_to_news_slots', '0x4afeb0', 'comp_util.cpp', 'cdecl',
          'grava clubes, local, competição, placar e rodada do jogo em 18 dados seguidos da notícia e avança o cursor',
          'writes the fixture clubs, venue, competition, score and round into 18 consecutive news slots and advances the cursor')])
