# A vaga no elenco para a oferta e a recusa do empréstimo (transfer_manager.cpp): TRANSFER_MANAGER::check_squad_limits
# (0x8cd450: das restrições da oferta 0x8cfc90, que as cópias Rust da avaliação dos lances, da conclusão e da
# reconfirmação chamam a cada oferta; o limite de 50 no elenco do clube e do parceiro, a liberação do mais barato do
# comprador do computador sem troca e as vagas da comissão), TRANSFER_MANAGER::reject_loan_offer (0x8bfa30: o vendedor
# do computador recusa todo lance de empréstimo, 0x8c8090 com +0x1f = 1; a gêmea da recusa 0x8bf680 da cpx_1, com os
# mesmos ajudantes cpx1_* e o mesmo comparador) e TRANSFER_MANAGER::find_next_sale_percentage (0x8d0dd0, só das telas
# da pessoa: passageira). Iguais no cm0102-1080.exe; sem SEH. A collect_offer_news_recipients (0x8d1130, que o
# candidatos.py dava com 596 bytes) tem 2,9 KB com SEH e x87: fica para outra leva.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x8cd450: 0x8cfc90 push ebx; push eax; push esi; push edi; mov ecx, ebp; call -> thiscall(this,
#   clube, oferta, contar_fila, motivo), ret 0x10 (o mapa diz stdcall; o ecx é usado dentro, [esp+0x24]).
# - quem chama a 0x8bfa30: 0x8c8090 push 0; push 0; push esi; mov ecx, ebx; call e 0x8c63a0 push 0; push 0; push ebp;
#   mov ecx, edi -> thiscall(this, vaga, marcar, motivo), ret 0xc.
# - dentro da 0x8cd450: 0x540a50 push 1; push 0; push edi; add esp, 0xc -> cdecl(clube, 0, 1); 0x4cf5a0 e 0x4cf6d0 mov
#   ecx, [0xae1050]; push pessoa -> thiscall(gerência, pessoa), ret 4; 0x53b620 e 0x53b610 push byte; add esp, 4 ->
#   cdecl(tipo); 0x68d1b0 mov ecx, lista -> thiscall(lista); 0x8c1780 push clube; mov ecx, [esp+0x28] (this) ->
#   thiscall(this, clube), ret 4; 0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl(clube, 1, 1,
#   0, 0); 0x8c5cf0 mov ecx, [0xde1f54]; push 0; push 0; push pessoa -> thiscall(g, pessoa, 0, 0), ret 0xc; 0x8c5e40 e
#   0x8cca10 mov ecx, [0xde1f54]; push 0; push pessoa -> thiscall(g, pessoa, 0), ret 8; 0x86d800 mov ecx, [0xdd7f00];
#   push pessoa -> thiscall(registros, pessoa), ret 4; 0x8c1af0 push 0; push -1; push 0; push clube; push pessoa; mov
#   ecx, [esp+0x34] (this) -> thiscall(this, pessoa, clube, 0, -1, 0), ret 0x14; a caixa (aviso_em) com o fonte
#   0xa8d514 (transfer_manager.cpp).
# - dentro da 0x8bfa30 (como na 0x8bf680): 0x84aeb0 mov ecx, [this+0x856]; push pessoa; push clube -> thiscall(listas,
#   clube, pessoa), ret 8; 0x8d5170 push motivo; push oferta; mov ecx, ebp -> thiscall(this, oferta, motivo), ret 8;
#   0x8c6220 push 0; push 0xa; push oferta; mov ecx, ebp -> thiscall(this, oferta, 10, 0), ret 0xc; 0x4cca70 mov ecx,
#   [0xae1050]; push clube; push pessoa -> thiscall(gerência, pessoa, clube), ret 8; 0x4ccb30 push [oferta+0x20]; push
#   pessoa; mov ecx, [0xae1050] -> thiscall(gerência, pessoa, vaga), ret 8; 0x849ce0 push 0; push flag; push pessoa;
#   push clube; mov ecx, [this+0x856] -> thiscall(listas, clube, pessoa, flag, 0), ret 0x10; 0x843e10 push 0; push 1;
#   push clube; push pessoa; mov ecx, [this+0x856] -> thiscall(listas, pessoa, clube, 1, 0), ret 0x10.
# - dentro da 0x8d0dd0: 0x4cf5a0 como acima; 0x946464 push 0x8e54d0; push 0xd; push conta; push vetor; push chave;
#   add esp, 0x14 -> cdecl(chave, vetor, conta, 0xd, comparação).
# - quem chama a 0x8d0dd0: 0x884ea2 (tela) mov ecx, gerência; push pessoa -> thiscall(this, pessoa), ret 4.
# Comparadores: a 0x8cd450 com a cópia em ensaio (sem liberar) e depois a original do mesmo estado; iguais a devolução,
# [0xae1080], o motivo e os cursores, ou, havendo liberação, o escolhido ter saído do clube; a 0x8bfa30 pelo
# cpx1_comparar (cópia em ensaio sem as chamadas que mexem no resto do jogo); a 0x8d0dd0 lado a lado na marca e em
# [0xae1080].
SAVE = False
ROTINAS = [('recusar_oferta_de_emprestimo', 0x8bfa30, [0x8b, 0x44, 0x24, 0x04, 0x55]),
           ('vaga_no_elenco_para_a_oferta', 0x8cd450, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('percentual_da_proxima_venda_achado', 0x8d0dd0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('a vaga no elenco para a oferta (o limite de 50 jogadores e das vagas da comissão, com a liberação do mais barato), a oferta de empréstimo recusada pelo clube e a cláusula de percentual da próxima venda achada',
        "the squad room for an offer (the 50-player limit and the staff slots, releasing the cheapest player), the loan offer rejected by the club and the next-sale percentage clause found",
        [('recusar_oferta_de_emprestimo', 'TRANSFER_MANAGER::reject_loan_offer', '0x8bfa30', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o clube recusa a oferta de empréstimo: estado 2, os lances desmarcados, a notícia, a oferta volta em 10 dias para sair, a marca de oferta da pessoa sai sem rival em aberto, as ofertas de contrato fecham e a situação na lista de observação do comprador',
          "the club rejects the loan offer: state 2, the bids unmarked, the news, the offer comes back in 10 days to be removed, the person's offer mark cleared with no open rival, the contract offers closed and the buyer's shortlist status"),
         ('vaga_no_elenco_para_a_oferta', 'TRANSFER_MANAGER::check_squad_limits', '0x8cd450', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'confere se a oferta deixa o clube (com o parceiro) dentro de 50 jogadores e das vagas de treinadores (5), fisioterapeutas (3), olheiros (7) e auxiliar; o comprador do computador sem troca libera de graça o jogador mais barato que pode sair; senão o motivo 4 ou 0x22 (mais 1 pelo vendedor) e 0',
          "checks that the offer keeps the club (with its partner) within 50 players and the coach (5), physio (3), scout (7) and assistant slots; a computer buyer without exchange releases for free the cheapest player who can leave; otherwise reason 4 or 0x22 (plus 1 for the seller) and 0"),
         ('percentual_da_proxima_venda_achado', 'TRANSFER_MANAGER::find_next_sale_percentage', '0x8d0dd0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'com a marca 8 da pessoa, procura pela busca binária a cláusula de percentual da próxima venda da pessoa com o clube do contrato; sem achar, tira a marca',
          "with the person's flag 8, binary-searches the next-sale percentage clause of the person with the contract club; when not found, clears the flag")])
CAMPOS = [('oferta +4 / +8 / +0xc / +0x11: a troca (LLIST), o comprador, a pessoa e o tipo da oferta; id e valor; lê (vaga_no_elenco_para_a_oferta, recusar_oferta_de_emprestimo)',
           'offer +4 / +8 / +0xc / +0x11: the exchange (LLIST), the buyer, the person and the offer type; id and value; reads (vaga_no_elenco_para_a_oferta, recusar_oferta_de_emprestimo)'),
          ('oferta +0x2c / +0x2d / +0x10: o estado (2), a marca e a etapa; valor; grava (recusar_oferta_de_emprestimo)',
           'offer +0x2c / +0x2d / +0x10: the state (2), the flag and the stage; value; writes (recusar_oferta_de_emprestimo)'),
          ('clube +0xd7: os 50 do elenco; ponteiro; lê (vaga_no_elenco_para_a_oferta)',
           'club +0xd7: the 50 squad slots; pointer; reads (vaga_no_elenco_para_a_oferta)'),
          ('clube +0x19f / +0x1b3 / +0x1cf / +0xd3: treinadores (5), olheiros (7), fisioterapeutas (3) e auxiliar; ponteiro; lê (vaga_no_elenco_para_a_oferta)',
           'club +0x19f / +0x1b3 / +0x1cf / +0xd3: coaches (5), scouts (7), physios (3) and assistant; pointer; reads (vaga_no_elenco_para_a_oferta)'),
          ('pessoa +0x3d / +0x52: o tipo e o valor; valor; lê (vaga_no_elenco_para_a_oferta)',
           'person +0x3d / +0x52: the type and the value; value; reads (vaga_no_elenco_para_a_oferta)'),
          ('contrato +0x3d / +0x35: o estatuto no elenco (com sinal) e o tipo (5 = empréstimo); valor; lê (vaga_no_elenco_para_a_oferta)',
           'contract +0x3d / +0x35: the squad status (signed) and the type (5 = loan); value; reads (vaga_no_elenco_para_a_oferta)'),
          ('[this+0x86e] + id × 8: as marcas da pessoa (bit 0 oferta em andamento, bit 3 percentual da próxima venda); valor; lê e grava (recusar_oferta_de_emprestimo, percentual_da_proxima_venda_achado)',
           "[this+0x86e] + id × 8: the person's flags (bit 0 offer in progress, bit 3 next-sale percentage); value; reads and writes (recusar_oferta_de_emprestimo, percentual_da_proxima_venda_achado)"),
          ('[this+0x880] / [this+0x888]: as cláusulas de percentual da próxima venda (0xd bytes: +1 clube, +9 pessoa) e a conta; ponteiro e valor; lê (percentual_da_proxima_venda_achado)',
           '[this+0x880] / [this+0x888]: the next-sale percentage clauses (0xd bytes: +1 club, +9 person) and their count; pointer and value; reads (percentual_da_proxima_venda_achado)'),
          ('[0xde1f54] + 0x92c: a fila da transferência livre (LLIST de ids); id; lê (vaga_no_elenco_para_a_oferta)',
           'the free transfer queue (LLIST of ids); id; reads (vaga_no_elenco_para_a_oferta)'),
          ('[0xae1080]: o último contrato consultado; ponteiro; grava (as três)',
           '[0xae1080]: the last contract looked up; pointer; writes (all three)')]
INTERESSA = ['CMRTE']
