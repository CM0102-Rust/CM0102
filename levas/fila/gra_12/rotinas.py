# O texto da notícia da resposta do jogador a um contrato (TRANSFER_MANAGER::write_contract_response_news_text
# 0x8d9480, transfer_manager.cpp; leitor gra, rodada 4, 21/11/2026). Chamado pelas notícias (0x77bc80, tipo 0xfa4).
# Convenções conferidas:
# - a rotina: thiscall(TRANSFER_MANAGER, notícia, corpo, Text), ret 0xc (ponto de chamada 0x7822f4; o this só vai
#   para a 0x8e4ae0, que não o lê);
# - 0x8e4ae0 (transfer_manager_build_refusal_reasons_text): stdcall(vendedor, comprador, slot 18, slot 22, bandeiras,
#   interesse em double, buf), ret 0x20, devolve al;
# - o interesse: fild do slot 27 × 0.001f (0x9690c0) no x87, guardado em float (a cópia multiplica em double e
#   arredonda para float, como o x87 com 53 bits);
# - 0x43f460/0x441c10 com buffer de 0x64; 0x53e140 (nomes, 0x190 ou 0x12c); 0x53e990 só para a segunda pessoa (os
#   vínculos do jogador e da outra pessoa não conferem os nomes, como no original); 0x4ae8a0 da segunda pessoa ANTES
#   do 0x4ae660 (o texto), como no original;
# - o clube europeu: +0x53 (nação) → +0x71 (continente) → id = [KEY_CONTINENT_EUROPE];
# - os argumentos dos 30 formatos (0x66f4e0) conferidos push a push (as leituras [esp+k] depois de cada push).
# Comparador: compare_text_writer.
SAVE = False
ROTINAS = [('write_contract_response_news_text', 0x8d9480, [0x6a, 0xff, 0x68, 0x6b, 0x44, 0x96, 0x00])]
DOCS = ('o texto da notícia da resposta do jogador a um contrato',
        "the news text of a player's answer to a contract",
        [('write_contract_response_news_text', 'TRANSFER_MANAGER::write_contract_response_news_text', '0x8d9480', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia de que um jogador recusou um contrato ou um empréstimo, ou acertou com o clube comprador: recusa (sem interesse no empréstimo abaixo de 1,0, pelo salário com o valor e o nome de outra pessoa, ou com até duas razões quando o interesse passa de 1,25); acerto pelas bandeiras dos motivos (encantado, perto de casa, clube europeu, o agente, a taxa que ajuda o clube, alívio por sair...), ou com outro clube no lugar; depois os vínculos e as frases dos clubes recusados (um, dois, três ou mais, com o número); a manchete: recusa, esnoba ou rejeita (pelo resto da divisão por 3 do id), acertado para empréstimo, acertado ou aceita a proposta',
          "the headline or body of the news that a player rejected a contract or a loan, or agreed terms with the buying club: rejected (no interest in a loan below 1.0, over the wage with the amount and another person's name, or with up to two reasons when the interest is above 1.25); agreed by the reason flags (delighted, close to home, a European club, the agent, the fee that helps the club, relief at leaving...), or with another club instead; then the links and the sentences of the rejected clubs (one, two, three or more, with the number); the headline: rejects, snubs or turns down (by the id mod 3), set for a loan deal, set for or accepts the offer")])
