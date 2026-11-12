# O texto da notícia de oferta aceita (TRANSFER_MANAGER::write_offer_accepted_news_text 0x8d7870, transfer_manager.cpp,
# 1.120 bytes; 165 chamadas até 2002-06-01; leitor mke do resto do mercado, 12/11/2026). Usa os ajudantes de texto do
# item mke_5 (NEWS_TEXT_BUFFER, news_text_person_name, news_text_link_club, news_text_link_person): integrar junto
# com o mke_5 ou depois dele.
# Convenções conferidas (0x8d7870..0x8d7cbf): thiscall(this, notícia, corpo, Text), ret 0xc, sem quadro SEH; as mesmas
# chamadas do mke_5 (0x785830, 0x53e140, 0x66f4e0, 0x4ae660, 0x4ae8a0, 0x53e990); o clube do humano atual é
# [[0xb74c26 + 0x300 × [0xb77c7a]] + 0x39].
# Comparador: lado a lado do texto e dos vínculos (compare_text_writer).
SAVE = False
ROTINAS = [('write_offer_accepted_news_text', 0x8d7870, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x44, 0x01, 0x00, 0x00])]
DOCS = ('o texto da notícia de oferta aceita',
        'the news text of an accepted offer',
        [('write_offer_accepted_news_text', 'TRANSFER_MANAGER::write_offer_accepted_news_text', '0x8d7870', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título ("aceita discutir contrato" sem clube vendedor, "oferta de empréstimo aceita", "oferta de transferência aceita") ou o corpo pelo acordo de conversa (campo 6), sem clube vendedor, empréstimo (campo 9) ou transferência, cada um em duas versões pelo clube comprador ser o do humano; os vínculos aos dois clubes e à pessoa',
          'the headline ("agrees to contract discussion" without a selling club, "loan offer accepted", "transfer offer accepted") or the body by the talks agreement (field 6), no selling club, a loan (field 9) or a transfer, each in two versions by the buying club being the human\'s; the links to the two clubs and the person')])
