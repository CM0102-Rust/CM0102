# A contraproposta com cláusula e os motivos da recusa (transfer_manager.cpp): a contraproposta com percentual da
# próxima venda ou prêmio por jogo (TRANSFER_MANAGER::build_clause_offer_option 0x8c8fd0, pela resposta do clube do
# computador ao lance 0x8c8090; quadro SEH só do `new` da oferta), se o motivo da recusa cabe na oferta
# (transfer_manager_is_rejection_reason_valid 0x8c5560, pela tela da oferta 0x8ef1e0) e o motivo pelo status no
# elenco (transfer_manager_get_squad_status_reason 0x8c54c0, pela tela 0x8e9550). Iguais no cm0102-1080.exe.
# Comparadores: só a original na contraproposta (aloca e solta a oferta; a cópia leva os ramos); lado a lado pelo
# contrato da vez [0xae1080] e pela devolução nas outras duas (só rodam na tela).
SAVE = False
ROTINAS = [('oferta_com_clausula', 0x8c8fd0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('motivo_da_recusa_vale', 0x8c5560, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('motivo_pelo_status_no_elenco', 0x8c54c0, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('a contraproposta com cláusula e os motivos da recusa do lance',
        'the clause counter offer and the bid rejection reasons',
        [('oferta_com_clausula', 'TRANSFER_MANAGER::build_clause_offer_option', '0x8c8fd0', 'transfer_manager.cpp', 'thiscall (this sem uso), ret 0x14',
          'com o clube do lance bem acima e um jogador de potencial até 30 anos: uma oferta nova (0x8e5680, 0x8e78f0) com 20% da próxima venda (apelo 80), o prêmio por jogo na seleção (70) ou no time (60), e o valor × 0,8; nenhuma, solta a oferta e 0',
          'with a much bigger bidding club and a high-potential player up to 30: a new offer (0x8e5680, 0x8e78f0) with a 20% sell-on (appeal 80), international (70) or first-team (60) appearance money, and the fee × 0.8; none, frees the offer and 0'),
         ('motivo_da_recusa_vale', 'transfer_manager_is_rejection_reason_valid', '0x8c5560', 'transfer_manager.cpp', 'stdcall, ret 8',
          '0 quando o motivo não cabe: promessa (0x2000) com 24 anos ou mais, divisão (0x1000) igual, rival (0x800) sem confronto nem parceria; senão 1',
          '0 when the reason does not fit: hot prospect (0x2000) aged 24 or more, same division (0x1000), rival (0x800) without a fixture or link; otherwise 1'),
         ('motivo_pelo_status_no_elenco', 'transfer_manager_get_squad_status_reason', '0x8c54c0', 'transfer_manager.cpp', 'stdcall, ret 4',
          'pelo status no elenco do contrato com o clube atual: 1 → 0x200, 2 e 3 → 0x400, 5 → 0x2000; senão 0',
          'by the squad status of the current club contract: 1 → 0x200, 2 and 3 → 0x400, 5 → 0x2000; otherwise 0')])
CAMPOS = [('oferta +0x1b: valor; +0x3a/+0x3e: prêmio por jogo na seleção e os jogos; +0x35/+0x39: prêmio por jogo no time e os jogos; +0x3f: percentual da próxima venda; valor; grava (oferta_com_clausula)',
           'offer +0x1b: fee; +0x3a/+0x3e: international appearance money and caps; +0x35/+0x39: first-team appearance money and games; +0x3f: sell-on percentage; value; writes (oferta_com_clausula)'),
          ('oferta +8 / +0xc: clube do lance e pessoa; id; lê (motivos)', 'offer +8 / +0xc: bidding club and person; id; reads (reasons)'),
          ('clube +0x80: reputação; clube +0x57: divisão; clube +0xb3/+0xb7/+0xbb: parceiros; valor e ponteiro; lê',
           'club +0x80: reputation; club +0x57: division; club +0xb3/+0xb7/+0xbb: linked clubs; value and pointer; reads'),
          ('ficha de jogador +5/+7: habilidade e potencial; +0xb: reputação; valor; lê (oferta_com_clausula)',
           'player record +5/+7: current and potential ability; +0xb: reputation; value; reads (oferta_com_clausula)'),
          ('staff +0x18: idade; +0x1a: nação (+0x8e reputação); +0x22: jogos pela seleção; lê', 'staff +0x18: age; +0x1a: nation (+0x8e reputation); +0x22: international caps; reads'),
          ('contrato +0x4f >> 4: status no elenco; valor; lê (motivo_pelo_status_no_elenco)', 'contract +0x4f >> 4: squad status; value; reads (motivo_pelo_status_no_elenco)'),
          ('[0xae1080] contrato da vez; ponteiro; grava', '[0xae1080] current contract; pointer; writes')]
INTERESSA = ['CMRTE']
