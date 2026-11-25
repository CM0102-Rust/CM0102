# Os textos da volta do empréstimo, da saída pela Lei Bosman e do pedido de transferência retirado
# (contract_manager.cpp; leitor rx07, rodada 6, 25/11/2026). Simulação, carreira ENG (sem LIGAS), no molde dos
# escritores de texto do contract_manager (write_contract_expiry_news_text). Contagem 0 na carreira automática e no
# mundo: os textos só são montados quando a caixa de um técnico humano mostra a notícia (a tabela dos escritores do
# news.rs, Global(0xae1050, ...)); a prova é a comparação numa carreira do Lucas. Iguais no cm0102-1080.exe.
# Convenções conferidas na desmontagem (as três thiscall(notícia, completo, texto) ret 0xc): get_slot 0x785830
# thiscall(k); database_format_name_from_ids 0x53e140 cdecl(n1, n2, n3, buf, 0x12c, modo, pessoa); langlib_sprintf
# 0x66f4e0 em 0xde1f64; Text::set_string 0x4ae660; Text::add_link 0x4ae8a0 thiscall(texto, link, tipo, objeto) ret
# 0xc; database_staff_matches_names 0x53e990 cdecl(pessoa, n1, n2, n3, 0).
# - 0x4db2e0 write_loan_recall_news_text (notícia 0xfb8): o nome no modo 1; o corpo lê o gênero do nome (+0x37) e o
#   nome curto (+0x38) dos clubes; a manchete não lê o slot 4. Prólogo mov eax, [esp+8]; sub esp, 0x138 (10).
# - 0x4d6ee0 write_bosman_departure_news_text (notícia 0xbbe): o corpo no modo 1, a manchete no modo 5. Prólogo sub
#   esp, 0x138 (6).
# - 0x4d95e0 write_transfer_request_withdrawn_news_text (notícia 0xbc0): o corpo no modo 1 pelo byte baixo do motivo,
#   a manchete no modo 5. Prólogo sub esp, 0x138 (6).
# Comparadores: lado a lado (compare_text_writer).
SAVE = False
ROTINAS = [('write_loan_recall_news_text', 0x4db2e0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x38, 0x01, 0x00, 0x00]),
           ('write_bosman_departure_news_text', 0x4d6ee0, [0x81, 0xec, 0x38, 0x01, 0x00, 0x00]),
           ('write_transfer_request_withdrawn_news_text', 0x4d95e0, [0x81, 0xec, 0x38, 0x01, 0x00, 0x00])]
_F = 'contract_manager.cpp'
DOCS = ('os textos da volta do empréstimo, da saída pela Lei Bosman e do pedido de transferência retirado',
        'the texts of the loan recall, the Bosman departure and the withdrawn transfer request',
        [('write_loan_recall_news_text', 'CONTRACT_MANAGER::write_loan_recall_news_text', '0x4db2e0', _F, 'thiscall, ret 0xc',
          'o texto da notícia 0xfb8: o corpo diz que o clube do contrato chamou o jogador de volta ou que o clube encerrou o empréstimo, com os dois clubes ligados; a manchete "<nome> chamado de volta do empréstimo" ou "<nome> empréstimo encerrado"; e o vínculo da pessoa quando os nomes ainda batem',
          'the text of news item 0xfb8: the body says the contract club recalled the player or the club ended the loan, with both clubs linked; the headline "<name> recalled from loan" or "<name> loan terminated"; and the person\'s link when the names still match'),
         ('write_bosman_departure_news_text', 'CONTRACT_MANAGER::write_bosman_departure_news_text', '0x4d6ee0', _F, 'thiscall, ret 0xc',
          'o texto da notícia 0xbbe: o corpo diz que o jogador deixa o clube no fim do contrato pela Lei Bosman, para o clube novo quando há um; a manchete "<nome> vai para <clube>" ou "<nome> vai deixar o clube"; os clubes e a pessoa ligados',
          'the text of news item 0xbbe: the body says the player leaves the club at the end of his contract under the Bosman ruling, for the new club when there is one; the headline "<name> to join <club>" or "<name> to leave club"; the clubs and the person linked'),
         ('write_transfer_request_withdrawn_news_text', 'CONTRACT_MANAGER::write_transfer_request_withdrawn_news_text', '0x4d95e0', _F, 'thiscall, ret 0xc',
          'o texto da notícia 0xbc0: o corpo pelo motivo (o bit 1, os bits 0x14 com o jogador deixado de fora, ou o texto comum) com o clube ligado; a manchete "<nome> feliz em ficar"; e o vínculo da pessoa',
          'the text of news item 0xbc0: the body by the reason (bit 1, bits 0x14 with the player left out, or the common text) with the club linked; the headline "<name> happy to stay"; and the person\'s link')])
CAMPOS = []
INTERESSA = []
