# Os textos das notícias do jogador da troca que recusa, do jogador dispensado de graça e da negociação da
# transferência (transfer_manager.cpp; leitor rxs1, rodada 6, 25/11/2026). As três inteiras (0x8d8af0..0x8d8e8f,
# 0x8dffe0..0x8e059f, 0x8d6e80..0x8d76cf), iguais no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até
# 2002-08-01 (os textos só são escritos quando alguém abre a notícia).
# Convenções conferidas na desmontagem: as três thiscall(this, notícia, corpo, Text) ret 0xc, com quadro SEH só para a
# quantia; 0x53dba0 database_format_staff_name cdecl(pessoa, buffer, 500, 1) no corpo da 0x8dffe0 (o título pelo
# 0x53e140 em 500 bytes); a quantia da 0x8d8af0 em 100 bytes, as outras em 0x32 (news_text_money); 0x786d30
# News::set_item_slot na vaga 11 da 0x8d6e80 (último argumento 0); a lista dos jogadores da troca da 0x8d6e80 (vaga 13
# a quantidade, da 14 em diante 4 vagas por jogador, 400 bytes começando pelo byte de 0xad9c64) é a do NewsPlayerList.
# Depende do rxs1_1 (news_text_name_n, news_text_link_name_n, news_text_club), do rxs1_2 (news_offer_at,
# news_store_item_slot, news_text_money_100) e do rx05_5 (NewsPlayerList).
# Comparadores: lado a lado do texto e dos vínculos (compare_text_writer). A 0x8d6e80 pode gravar a vaga 11 da notícia
# antes do texto e roda duas vezes na comparação: a segunda acha o estado já gravado.
SAVE = False
ROTINAS = [('write_swap_player_rejects_news_text', 0x8d8af0, [0x6a, 0xff, 0x68, 0x2b, 0x44, 0x96, 0x00]),
           ('write_player_released_news_text', 0x8dffe0, [0x6a, 0xff, 0x68, 0x7b, 0x46, 0x96, 0x00]),
           ('write_offer_negotiation_news_text', 0x8d6e80, [0x6a, 0xff, 0x68, 0x96, 0x43, 0x96, 0x00])]
DOCS = ('os textos das notícias do jogador da troca que recusa, do jogador dispensado de graça e da negociação da transferência',
        'the news texts of the exchange player who refuses, of the player released on a free transfer and of the transfer negotiation',
        [('write_swap_player_rejects_news_text', 'TRANSFER_MANAGER::write_swap_player_rejects_news_text', '0x8d8af0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "<nome> sai do negócio" ou o corpo "<jogador> do <clube> recusou ir ao <clube> na troca por <alvo>", com "o valor passa a <quantia>" quando o valor novo é maior que o antigo; os vínculos aos clubes e às duas pessoas',
          "the headline \"<name> removed from transfer deal\" or the body \"<club>'s <player> has rejected a possible move to <club> as part of a swap deal for <target>\", with \"the fee will be adjusted to <money>\" when the new fee is above the old one; the links to the clubs and both people"),
         ('write_player_released_news_text', 'TRANSFER_MANAGER::write_player_released_news_text', '0x8dffe0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "<nome> dispensado de graça" ou o corpo "<nome> foi dispensado de graça pelo <clube>", mais, pelo tipo, "a proposta do <clube> por ele foi cancelada" ou "ele saiu da troca do <clube> por <jogador> (e o valor subiu a <quantia>)"; os vínculos às pessoas e aos clubes',
          "the headline \"<name> given free transfer\" or the body \"<name> has been released on a free transfer by <club>\", plus, by the kind, \"<club>'s transfer bid for him has been cancelled\" or \"he has been removed from <club>'s part exchange bid for <player> (and the fee increased to <money>)\"; the links to the people and the clubs"),
         ('write_offer_negotiation_news_text', 'TRANSFER_MANAGER::write_offer_negotiation_news_text', '0x8d6e80', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "<clube> negocia <nome>" ou o corpo com o que o clube vendedor quer: um negócio de <quantia> (mais <jogadores>), nada, <quantia>, <jogadores> em troca ou <quantia> mais <jogadores>; mais "<comprador> cancelou o negócio" (estado 3) ou "<comprador> fez nova proposta" (estado 0); o estado desconhecido é procurado na oferta e guardado na notícia',
          "the headline \"<club> negotiation over <name>\" or the body with what the selling club wants: a deal worth <money> (plus <players>), nothing, <money>, <players> in exchange or <money> plus <players>; plus \"<bidder> have cancelled the deal\" (state 3) or \"<bidder> have made a new bid\" (state 0); an unknown state is looked up in the offer and stored in the news item")])
