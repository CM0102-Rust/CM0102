# Os textos das notícias do empréstimo cancelado, da transferência à espera da permissão de trabalho e do jogador que
# recusa a mudança (transfer_manager.cpp; leitor rxs1, rodada 6, 25/11/2026). As três inteiras (0x8e2570..0x8e29ff,
# 0x8e2cd0..0x8e312f, 0x8d84e0..0x8d890f), iguais no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até
# 2002-08-01 (os textos só são escritos quando alguém abre a notícia).
# Convenções conferidas na desmontagem: as três thiscall(this, notícia, corpo, Text) ret 0xc, sem quadro SEH; os
# nomes em 300 bytes pelo 0x53e140; 0x8e4ae0 transfer_manager_build_refusal_reasons_text stdcall(clube, comprador,
# vaga 8, vaga 9, motivos, 0.0, buffer) ret 0x20 (refusal_reasons_text), com o buffer de 400 bytes zerado antes; o
# técnico humano atual (scout_manager_current_human_manager) comparado com o técnico do clube (+0xcf).
# Depende do rxs1_1 (news_text_name_n, news_text_link_name_n, news_text_club) e do rx07_9 (news_text_link_club_field).
# Defeitos do original (a cópia mantém): 0x8e2570, os nomes longos dos clubes com o gênero dos nomes curtos, e motivo
# sem modelo põe no texto o que o buffer compartilhado tinha; 0x8e2cd0, o tipo 1 com a vaga 6 = 0 lê o clube vendedor
# mesmo quando a vaga 5 é −1.
# Comparadores: lado a lado do texto e dos vínculos (compare_text_writer).
SAVE = False
ROTINAS = [('write_loan_cancelled_news_text', 0x8e2570, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x40, 0x01, 0x00, 0x00]),
           ('write_work_permit_delay_news_text', 0x8e2cd0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x80, 0x02, 0x00, 0x00]),
           ('write_player_rejects_move_news_text', 0x8d84e0, [0x81, 0xec, 0xd8, 0x02, 0x00, 0x00])]
DOCS = ('os textos das notícias do empréstimo cancelado, da transferência à espera da permissão de trabalho e do jogador que recusa a mudança',
        'the news texts of the cancelled loan, of the transfer awaiting a work permit and of the player who rejects a move',
        [('write_loan_cancelled_news_text', 'TRANSFER_MANAGER::write_loan_cancelled_news_text', '0x8e2570', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "empréstimo de <nome> cancelado" ou o corpo "o empréstimo de (<clube>) <nome> para <clube> foi cancelado", com o motivo: sem espaço no elenco, prazo encerrado, mais de dois emprestados ao mesmo tempo, cinco empréstimos na temporada, quatro jogadores já emprestados, empréstimo proibido entre clubes da mesma competição ou cinco jogos pelo clube; motivo sem modelo deixa o texto que estava no buffer; os vínculos aos clubes e à pessoa',
          "the headline \"<name> loan cancelled\" or the body \"(<club>'s) <name> proposed loan move to <club> has been cancelled\", with the reason: no squad space, deadline passed, more than two on loan at once, five loans this season, four players already on loan, loans between clubs of the competition not allowed or five games for the club; a reason without a template keeps the text the buffer held; the links to the clubs and the person"),
         ('write_work_permit_delay_news_text', 'TRANSFER_MANAGER::write_work_permit_delay_news_text', '0x8e2cd0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "transferência de <nome> adiada pela permissão de trabalho" ou o corpo "a transferência de <nome> (de <vendedor>) para <comprador> foi adiada à espera da permissão de trabalho", dele, de uma segunda pessoa ou dos jogadores envolvidos; os vínculos aos clubes e às pessoas',
          "the headline \"<name> transfer delayed due to work permit\" or the body \"the proposed transfer of <name> (from <seller>) to <buyer> has been delayed awaiting a work permit decision\", his, a second person's or the players involved; the links to the clubs and the people"),
         ('write_player_rejects_move_news_text', 'TRANSFER_MANAGER::write_player_rejects_move_news_text', '0x8d84e0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "<nome> recusa a mudança para <clube>" ou o corpo "(o jogador do <clube>) <nome> recusou a mudança para <clube> (porque <motivos>)" ou "recusou a oferta de empréstimo de <clube>"; os motivos pela transfer_manager_build_refusal_reasons_text; os vínculos aos clubes e à pessoa',
          "the headline \"<name> rejects move to <club>\" or the body \"(<club>'s) <name> has rejected the move to <club> (because he <reasons>)\" or \"has rejected the loan offer from <club>\"; the reasons by transfer_manager_build_refusal_reasons_text; the links to the clubs and the person")])
