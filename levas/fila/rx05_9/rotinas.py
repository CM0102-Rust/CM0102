# Os textos das notícias do empréstimo aceito, da negociação do empréstimo e da parte do clube anterior na
# transferência (transfer_manager.cpp; leitor rx05, rodada 6, 25/11/2026). As três inteiras (0x8e20a0..0x8e23cd,
# 0x8e47a0..0x8e4ad1, 0x8e51d0..0x8e54c7), iguais no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até
# 2002-08-01.
# Convenções conferidas na desmontagem: as três thiscall(this, notícia, corpo, Text) ret 0xc (0x8e20a0 e 0x8e51d0 com
# quadro SEH só para a data / a quantia; 0x8e47a0 sem); 0x549ef0 Date::Date(dia, ano) e 0x54ba20 (0xc8, 0xff); o
# técnico humano atual é [0xb74c26 + 0x300 × [0xb77c7a]] comparado com o técnico do clube (+0xcf); 0x786d30
# News::set_item_slot thiscall([0xb7431c], técnico, [notícia+0xd6], vaga, valor, 0) ret 0x14; os nomes das pessoas em
# 200 bytes (0x8e47a0) e em 400 bytes (0x8e51d0) pelo 0x53e140.
# Defeito do original (0x8e47a0, a cópia mantém): o estado achado numa oferta de outro status vai à vaga 11, não à
# vaga 10 do estado; a próxima escrita procura a oferta de novo.
# Comparadores: lado a lado do texto e dos vínculos (compare_text_writer). A 0x8e47a0 pode gravar uma vaga da notícia
# (News::set_item_slot) antes do texto, e roda duas vezes na comparação: a segunda acha o mesmo estado (o mesmo
# acontece com o write_contract_talks_news_text do mke_5, já publicado).
SAVE = False
ROTINAS = [('write_loan_accepted_news_text', 0x8e20a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('write_loan_negotiation_news_text', 0x8e47a0, [0x81, 0xec, 0xdc, 0x00, 0x00, 0x00]),
           ('write_former_club_fee_cut_news_text', 0x8e51d0, [0x6a, 0xff, 0x68, 0xcb, 0x48, 0x96, 0x00])]
DOCS = ('os textos das notícias do empréstimo aceito, da negociação do empréstimo e da parte do clube anterior na transferência',
        "the news texts of the accepted loan, of the loan negotiation and of the former club's cut of the transfer fee",
        [('write_loan_accepted_news_text', 'TRANSFER_MANAGER::write_loan_accepted_news_text', '0x8e20a0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "aceita o contrato de empréstimo" ou o corpo "(o jogador do clube) decidiu ir ao <clube> emprestado até <data>", sem o clube de origem quando é o do técnico humano; os vínculos aos clubes e à pessoa',
          'the headline "accepts loan contract" or the body "(the club\'s) player has decided to join <club> on loan until <date>", without the origin club when it is the human manager\'s; the links to the clubs and the person'),
         ('write_loan_negotiation_news_text', 'TRANSFER_MANAGER::write_loan_negotiation_news_text', '0x8e47a0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "negociação da oferta de empréstimo" ou o corpo "<clube> quer negociar a oferta de empréstimo de <clube> por <nome>", mais "cancelou o negócio" (estado 3) ou "fez nova oferta" (estado 0); o estado desconhecido é procurado na oferta e guardado na notícia',
          'the headline "loan offer negotiation" or the body "<club> want to negotiate over <club>\'s loan offer for <name>", plus "have cancelled the deal" (state 3) or "have made a new offer" (state 0); an unknown state is looked up in the offer and stored in the news item'),
         ('write_former_club_fee_cut_news_text', 'TRANSFER_MANAGER::write_former_club_fee_cut_news_text', '0x8e51d0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "<clube> fica com parte da transferência de <nome>" ou o corpo "<clube> recebeu <valor> pela transferência de <nome> de <vendedor> para <comprador>"; os vínculos aos clubes e à pessoa',
          'the headline "<club> get cut of <name>\'s transfer fee" or the body "<club> have been given <amount> as part of <name>\'s transfer from <seller> to <buyer>"; the links to the clubs and the person')])
