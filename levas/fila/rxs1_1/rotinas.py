# Os textos das notícias da porcentagem da venda, do dinheiro por jogos e do pedido de adiar a transferência
# (transfer_manager.cpp; leitor rxs1, rodada 6, 25/11/2026). As três inteiras (0x8db580..0x8db89f,
# 0x8dba80..0x8dbeef, 0x8ddc70..0x8de14f), iguais no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até
# 2002-08-01 (os textos só são escritos quando alguém abre a notícia).
# Convenções conferidas na desmontagem: as três thiscall(this, notícia, corpo, Text) ret 0xc (0x8db580 e 0x8dba80 com
# quadro SEH só para a quantia); 0x785830 NewsItem::get_slot; 0x53e140 database_format_name_from_ids (400 bytes nas
# duas primeiras, 300 na 0x8ddc70); 0x43f460 + 0x441c10 a quantia (news_text_money); 0x66f4e0 langlib_sprintf no
# 0xde1f64; 0x4ae660 / 0x4ae770 / 0x4ae8a0 do Text; 0x53e990 a conferência dos nomes.
# Os três auxiliares news_text_name_n, news_text_link_name_n e news_text_club são usados pelos itens rxs1_2..rxs1_6.
# Defeitos do original (a cópia mantém): 0x8db580, os marcadores de gramática do corpo fora de ordem (o possessivo do
# nome leva o gênero do clube 2); 0x8ddc70, motivo sem modelo põe no texto o que o buffer compartilhado tinha.
# Comparadores: lado a lado do texto e dos vínculos (compare_text_writer).
SAVE = False
ROTINAS = [('write_sale_percentage_news_text', 0x8db580, [0x6a, 0xff, 0x68, 0xfb, 0x44, 0x96, 0x00]),
           ('write_appearance_money_news_text', 0x8dba80, [0x6a, 0xff, 0x68, 0x46, 0x45, 0x96, 0x00]),
           ('write_transfer_delay_request_news_text', 0x8ddc70, [0x81, 0xec, 0x3c, 0x01, 0x00, 0x00])]
DOCS = ('os textos das notícias da porcentagem da venda, do dinheiro por jogos e do pedido de adiar a transferência',
        'the news texts of the sell-on percentage, of the appearance money and of the transfer delay request',
        [('write_sale_percentage_news_text', 'TRANSFER_MANAGER::write_sale_percentage_news_text', '0x8db580', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "<clube> fica com <n>% da transferência de <nome>" ou o corpo "<clube> recebe <quantia> pela cláusula de <n>% da transferência de <nome> de <clube 2> para <clube 3>"; os vínculos aos clubes e à pessoa',
          "the headline \"<club> get <n>% of <name>'s transfer fee\" or the body \"<club> have been given <money> because of a <n>% sell-on clause for <name>'s transfer from <club 2> to <club 3>\"; the links to the clubs and the person"),
         ('write_appearance_money_news_text', 'TRANSFER_MANAGER::write_appearance_money_news_text', '0x8dba80', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "<clube> recebe <quantia> da transferência de <nome>" ou o corpo "<pagador> pagou <quantia> a <clube> pela transferência de <nome>", com a segunda frase pelo tipo (jogos da liga ou internacionais) e pela contagem (o primeiro jogo ou <n> jogos); os vínculos aos clubes e à pessoa',
          "the headline \"<club> receive <money> from <name>'s transfer deal\" or the body \"<payer> have paid <club> <money> as part of <name>'s transfer deal\", with the second sentence by the kind (league or international games) and the count (his first game or <n> games); the links to the clubs and the person"),
         ('write_transfer_delay_request_news_text', 'TRANSFER_MANAGER::write_transfer_delay_request_news_text', '0x8ddc70', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "transferência de <nome> adiada" ou o corpo "<clube> quer adiar por uma semana a transferência de <nome> para/de <outro>", com o motivo (sem espaço no elenco, sem verba, gente demais) e, se houve resposta, "<clube> aceitou o adiamento" ou "<clube> cancelou a transferência"; motivo sem modelo deixa o texto que estava no buffer; os vínculos à pessoa e aos clubes',
          "the headline \"<name> transfer delayed\" or the body \"<club> would like to delay <name>'s transfer to/from <other> by a week\", with the reason (no squad space, no funds, too many staff) and, when answered, \"<club> have accepted the delay\" or \"<club> have cancelled the transfer\"; a reason without a template keeps the text the buffer held; the links to the person and the clubs")])
