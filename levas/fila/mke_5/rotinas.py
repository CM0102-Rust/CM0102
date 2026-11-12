# Os textos das notícias de contrato (leitor mke do resto do mercado, 12/11/2026; comparador lado a lado do texto,
# compare_text_writer do commit 759c98f9d):
# - 0x8d7fb0 TRANSFER_MANAGER::write_contract_talks_news_text (864 bytes; 55 chamadas até 2002-06-01): a conversa de
#   contrato e o estado dela; pode gravar o estado no item guardado da caixa do humano (News::set_item_slot 0x786d30,
#   push 0; push estado; push 9; push [notícia+0xd6]; push [0xb74c26 + 0x300 × [0xb77c7a]]; mov ecx, [0xb7431c] ->
#   ret 0x14), pelo estado da oferta (0x8e7f10 TRANSFER_OFFER::get_player_talks_status, push pessoa; mov ecx, oferta ->
#   ret 4); no comparador a original grava antes e a cópia lê o mesmo estado;
# - 0x4d6a00 CONTRACT_MANAGER::write_contract_expiry_news_text (880 bytes; 26 chamadas): o contrato vence em 12, 6, 3,
#   2 ou 1 mês ou na semana (tabela de salto 0x4d6d4c, 7 entradas conferidas);
# - 0x4daa00 CONTRACT_MANAGER::write_contract_end_news_text (992 bytes; 16 chamadas): a pessoa sai no fim do contrato,
#   do teste ou do empréstimo.
# Convenções conferidas: as três thiscall(this, notícia, corpo, Text), ret 0xc, sem quadro SEH, sem retorno lido;
# 0x785830 push k; mov ecx, notícia -> ret 4; 0x53e140 (nome1, nome2, nome3, buf, 300, modo, id) cdecl; 0x66f4e0
# sprintf cdecl em 0xde1f64; 0x4ae660 / 0x4ae770 push texto; mov ecx, Text -> ret 4; 0x4ae8a0 push alvo; push tipo;
# push texto; mov ecx, Text -> ret 0xc; 0x53e990 (pessoa, nome1, nome2, nome3, 0) cdecl.
# Fidelidade: a 0x4d6a00 com um período fora de 1..7 não escreve o buffer e passa o que estava lá (a cópia também).
SAVE = False
ROTINAS = [('write_contract_talks_news_text', 0x8d7fb0, [0x81, 0xec, 0x44, 0x01, 0x00, 0x00, 0x53, 0x55, 0x56]),
           ('write_contract_expiry_news_text', 0x4d6a00, [0x81, 0xec, 0x40, 0x01, 0x00, 0x00, 0x53, 0x55, 0x56]),
           ('write_contract_end_news_text', 0x4daa00, [0x81, 0xec, 0x40, 0x01, 0x00, 0x00, 0x53, 0x55, 0x56])]
DOCS = ('os textos das notícias de conversa, vencimento e fim de contrato',
        'the news texts of contract talks, contract expiry and contract end',
        [('write_contract_talks_news_text', 'TRANSFER_MANAGER::write_contract_talks_news_text', '0x8d7fb0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "Offer contract to <nome>" ou o corpo com os dois clubes e o estado da conversa (1, ou 4 pelo campo 10); o estado vem do campo 9 ou, zerado, da oferta do campo 7 com o número de série do campo 8, e então é gravado no item guardado da caixa do humano; os vínculos aos clubes e à pessoa',
          'the headline "Offer contract to <name>" or the body with the two clubs and the talks state (1, or 4 by field 10); the state comes from field 9 or, when 0, from the offer of field 7 with the serial of field 8, and is then written into the stored item of the human\'s inbox; the links to the clubs and the person'),
         ('write_contract_expiry_news_text', 'CONTRACT_MANAGER::write_contract_expiry_news_text', '0x4d6a00', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'o título "<nome> contract" ou o corpo pelo período (12, 6, 3, 2 ou 1 mês, semana; três modelos por período pelos campos 7 e 8); os vínculos ao clube e à pessoa',
          'the headline "<name> contract" or the body by the period (12, 6, 3, 2 or 1 month, a week; three templates per period by fields 7 and 8); the links to the club and the person'),
         ('write_contract_end_news_text', 'CONTRACT_MANAGER::write_contract_end_news_text', '0x4daa00', 'contract_manager.cpp', 'thiscall, ret 0xc',
          'o título e o corpo de quem sai no fim do contrato, do teste (5) ou do empréstimo (6), pelo clube ser o do humano e pelo campo 8; os vínculos à pessoa, ao clube do empréstimo e ao clube',
          'the headline and body of someone leaving at the end of the contract, the trial (5) or the loan (6), by the club being the human\'s and by field 8; the links to the person, the loan club and the club')])
