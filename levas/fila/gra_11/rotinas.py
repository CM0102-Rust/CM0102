# O texto da notícia de transferência acertada (TRANSFER_MANAGER::write_transfer_finalised_news_text 0x8da8f0,
# transfer_manager.cpp; leitor gra, rodada 4, 21/11/2026). Chamado pelas notícias (0x77bc80, tipo 0xfa5, postado por
# post_transfer_finalised_news 0x8da5e0). Convenções conferidas:
# - a rotina: thiscall(TRANSFER_MANAGER, notícia, corpo, Text), ret 0xc (o ecx não é lido; ponto de chamada 0x78231a);
# - 0x785830 (NewsItem::get_slot): thiscall, ret 4; 0x786d30 (News::set_item_slot): thiscall(News, técnico, id da
#   notícia [+0xd6], 9, valor, a própria notícia), ret 0x14 (grava também na notícia: a cópia relê o valor novo, que já
#   é final ou dá o mesmo valor);
# - 0x90c520 (utils_number_to_words): cdecl(n, buf, 0x32, 1); 0x43f460 thiscall(&valor, 6) -> i64 e 0x441c10
#   cdecl(buf, 0x32, i64, 6); 0x549ef0 thiscall(data, dia, ano), ret 8; 0x54c3b0 thiscall(data, buf, 0xc8, 0xff),
#   ret 0xc; 0x53e140 cdecl (nomes, buf, 0x12c, modo, id); 0x53e990 cdecl(pessoa, n1, n2, n3, 0); 0x53b610 cdecl(char);
# - os argumentos dos 17 formatos (0x66f4e0, cdecl) conferidos push a push na desmontagem.
# Comparador: compare_text_writer.
SAVE = False
ROTINAS = [('write_transfer_finalised_news_text', 0x8da8f0, [0x6a, 0xff, 0x68, 0xc1, 0x44, 0x96, 0x00])]
DOCS = ('o texto da notícia de transferência acertada',
        'the news text of a transfer set to complete',
        [('write_transfer_finalised_news_text', 'TRANSFER_MANAGER::write_transfer_finalised_news_text', '0x8da8f0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia de que uma transferência, um contrato ou uma troca de jogadores vai ser concluída: antes, para o técnico humano, o estado da proposta é atualizado na notícia (o clube comprador guarda cancelada, adiada ou confirmada; o vendedor também); o corpo traz o valor (o total quando maior que a taxa e sem pré-contrato, com os jogadores da troca por extenso), a data da conclusão quando marcada, e a frase do estado (os dois clubes confirmaram, um confirmou, cancelou ou pediu uma semana de adiamento); os vínculos do comprador, do vendedor e da pessoa; a manchete: acertado para o cargo (cargo que não é de jogador, com pré-contrato), vai comprar (com valor) ou vai contratar',
          'the headline or body of the news that a transfer, a contract or a player exchange is set to complete: first, for the human manager, the offer status is brought up to date in the news (the buying club stores cancelled, delayed or confirmed; the seller too); the body has the value (the total when above the fee and with no pre-contract, with the exchange players in words), the completion date when set, and the status sentence (both clubs confirmed, one confirmed, cancelled or asked for a week\'s delay); the links of the buyer, the seller and the person; the headline: set for the role (a job that is not a player\'s, with a pre-contract), set to buy (with a value) or set to sign')])
