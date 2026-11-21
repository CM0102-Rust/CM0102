# O texto da notícia de proposta feita por uma pessoa (TRANSFER_MANAGER::write_offer_made_news_text 0x8d3520,
# transfer_manager.cpp; leitor gra, rodada 4, 21/11/2026). Chamado pelas notícias (0x77bc80, tipo 0xfa0, ponto de
# chamada 0x7821ca). Convenções conferidas:
# - a rotina: thiscall(TRANSFER_MANAGER, notícia, corpo, Text), ret 0xc (o ecx não é lido);
# - o estado (slot 9) atualizado pela proposta (índice do slot 6, série do slot 7) e gravado por 0x786d30
#   (News::set_item_slot, thiscall, ret 0x14) para o técnico humano do slot atual, mesmo nulo (como no original);
# - os jogadores da troca a partir do slot 24 (contador em [esp+0x1c], 4 slots cada, até 3) e o slot do cargo lido no
#   contador sem avançar; 0x53ad00 (database_get_job_name) cdecl(cargo, 0, 0, &gênero) escreve o gênero no byte das
#   bandeiras (slot 16), que esse caminho não lê mais;
# - o clube que faz a proposta pelo nome longo (+4) e o gênero dele (+0x37), o vendedor e o terceiro clube (slot 17)
#   pelo curto/longo como no original; o vínculo do clube da proposta pelo nome longo;
# - 0x549ef0 Date::Date(dia do slot 12, ano do slot 13); 0x54ba20 thiscall(data, buf, 0xc8, 0xff); 0x43f460/0x441c10
#   (0x32 bytes); 0x90c520 cdecl(n, buf, 0x32, 1);
# - o caminho "bandeiras sem os bits 1 e 2 mas com o bit 2" (0x8d401c) é inalcançável no original (fica fora);
# - os argumentos dos ~55 formatos (0x66f4e0) conferidos push a push.
# Comparador: compare_text_writer.
SAVE = False
ROTINAS = [('write_offer_made_news_text', 0x8d3520, [0x6a, 0xff, 0x68, 0x12, 0x43, 0x96, 0x00])]
DOCS = ('o texto da notícia de proposta feita',
        'the news text of an offer made',
        [('write_offer_made_news_text', 'TRANSFER_MANAGER::write_offer_made_news_text', '0x8d3520', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia de que um clube ofereceu uma taxa, jogadores, um contrato ou um cargo por uma pessoa: antes, o estado da proposta é atualizado na notícia; a oferta de contrato (com o nome do cargo, ou pelo tipo de contrato e a data de início), a proposta com o total acima da taxa (os dois valores), ou a taxa (com os jogadores da troca, com um terceiro clube pelas bandeiras, com a taxa de pelo menos 500 mil), cada uma pelo tipo (1, 2 ou outro) e por o vendedor ser outro técnico ou não; depois a compensação devida ao vendedor (contrato com taxa) ou a frase do estado do vendedor; os vínculos do clube da proposta (nome longo), do vendedor, da pessoa e dos jogadores trocados; a manchete: proposta de transferência por',
          "the headline or body of the news that a club offered a fee, players, a contract or a job for a person: first, the offer status is brought up to date in the news; the contract offer (with the job name, or by the contract kind and the start date), the offer with the total above the fee (both amounts), or the fee (with the swapped players, with a third club by the flags, with a fee of at least 500 thousand), each by the kind (1, 2 or other) and by whether the seller is another manager; then the compensation due to the seller (a contract with a fee) or the seller's status sentence; the links of the bidding club (long name), the seller, the person and the swapped players; the headline: transfer bid for")])
