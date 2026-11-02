# O contrato assinado e posto no vetor dos contratos (contract_manager.cpp):
# - 0x4cf720 (CONTRACT_MANAGER::sign_contract, 2647 bytes, sem SEH): pela conclusão da transferência (0x8c9720), pela
#   proposta de contrato aceita (0x8c38b0) e pela tela do pessoal (0x870a93). A pessoa entra no clube (0x53bd10) ou a
#   proposta cai sem vaga (0x4d64e0); no empréstimo, o elenco cheio derruba o teste (0x4dade0), senão o cargo 11, a
#   duração a partir de hoje e o nibble 8; o fim no mínimo 4 semanas depois do início e em +0x46 da pessoa; o lugar no
#   vetor dos contratos (o mesmo índice com os termos antigos no mesmo clube, a segunda nacionalidade pela nação de
#   [0x9cf434], a primeira vaga livre ou o fim do vetor, que cresce 100); o valor em +0x4e da pessoa, a moral no tipo 2,
#   a folha (0x598370) e a troca de clube (0x53c190, 0x53bd10). 1970 chamadas na carreira até 2002-06-01 (cobertura
#   17/10). Uma rotina grande, sozinha no item.
# Igual no cm0102-1080.exe (inteira).
# Determinismo: nada lido sem valor (os temporários de data são gravados pela 0x417c10 e pelas rotinas de data antes
# da leitura); o cargo vai às 0x53bd10/0x53b9a0 como chegou (o dword do chamador).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x4cf720: 0x870a93, 0x8c39a9, 0x8ca724 (push cargo (byte em registrador); push contrato; mov ecx,
#   [0xae1050]) -> thiscall(this, contrato, cargo), ret 8.
# - dentro: 0x53bd10 6 push; add esp, 0x18 -> cdecl(clube, pessoa, cargo, data, 1, 0); 0x53b9a0 push pessoa; push
#   cargo; push clube; add esp, 0xc -> cdecl; 0x4d64e0 e 0x4dade0 push clube; push pessoa; mov ecx, ebp -> thiscall(this,
#   pessoa, clube), ret 8; 0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl; 0x7f7270 push
#   comissão; mov ecx, [0xb74448] -> thiscall, ret 4; 0x8b2880 push 0; push clube; mov ecx, [0xde1f44] -> thiscall, ret 8;
#   0x53c590 push 0; push 0; push clube; add esp, 0xc -> cdecl (movsx al); 0x90d860 push 0; push clube; mov ecx,
#   [0xde2f70] -> thiscall, ret 8; 0x54cdb0 push data (2 dwords); lea ecx, +0x2d -> thiscall, ret 8; 0x54c5b0 push n;
#   push &saída; mov ecx, 0xae2c90 -> thiscall, ret 8; 0x417c10 push origem; push &temp; lea ecx, destino -> thiscall,
#   ret 8; 0x540980 push 1; push pessoa; add esp, 8 -> cdecl; 0x54c770 push 4; push &saída; ecx = +0x25 -> thiscall, ret
#   8; 0x53d740 push 0; push pessoa; add esp, 8 -> cdecl; 0x54cab0 push 0x64; push &saída; mov ecx, 0xae2c90 ->
#   thiscall, ret 8; 0x90cfc0 push 3; add esp, 4 -> cdecl; 0x866ee0 push clube; push pessoa; mov ecx, contrato ->
#   thiscall, ret 8; 0x945667 push tamanho; push vetor; add esp, 8 -> cdecl; 0x944e46 push tamanho; add esp, 4 -> cdecl;
#   0x5a0590 push clube; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x598370 mov ecx, finanças -> thiscall, ret 0; 0x544710
#   push 0; push 0x14; push pessoa; add esp, 0xc -> cdecl; 0x53c190 push 0; push 1; push 0; push pessoa; push clube; add
#   esp, 0x14 -> cdecl; a caixa "Error" (aviso_em) com o fonte 0x9a8190 (contract_manager.cpp).
# Comparador: só a original (inclui e tira a pessoa dos clubes, realoca o vetor, entrega notícias, sorteia, recalcula a
# folha).
SAVE = False
ROTINAS = [('assinar_contrato', 0x4cf720, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00])]
DOCS = ('o contrato assinado e posto no vetor dos contratos (a entrada no clube, o empréstimo com a duração a partir de hoje, o fim mínimo, o lugar no vetor com os termos antigos no mesmo clube, a vaga livre ou o fim do vetor, o valor na pessoa e a troca de clube)',
        'the contract signed and placed in the contract array (joining the club, loans with the length from today, the minimum end, the array slot keeping the old terms at the same club, a free slot or the end of the array, the value on the person and the club change)',
        [('assinar_contrato', 'CONTRACT_MANAGER::sign_contract', '0x4cf720', 'contract_manager.cpp', 'thiscall, ret 8',
          'põe a pessoa no clube do contrato (ou cancela sem vaga), trata o empréstimo (elenco cheio, cargo 11, duração a partir de hoje), garante 4 semanas de contrato, grava o contrato no vetor (o índice antigo, a vaga livre ou o fim, que cresce 100), o valor em +0x4e da pessoa, a folha e a troca de clube',
          'puts the person in the contract club (or cancels without space), handles loans (full squad, job 11, length from today), ensures 4 weeks of contract, stores the contract in the array (the old index, a free slot or the end, growing by 100), the value at person +0x4e, the payroll and the club change')])
CAMPOS = [('gerência de contratos +0 vetor dos contratos de 0x50 bytes, +4 capacidade, +8 conta; ponteiro e valor; lê e grava (assinar_contrato)',
           'contract manager +0 array of 0x50-byte contracts, +4 capacity, +8 count; pointer and value; reads and writes (assinar_contrato)'),
          ('contrato +0 pessoa (−1 vaga livre), +4 clube, +0xc valor [inferência], +0x25 início, +0x2d fim, +0x35 tipo (6 bits; 5 empréstimo, 2) e bits, +0x3a, +0x3b, +0x3d tipo pelo cargo, +0x3e, +0x3f, +0x44 felicidade no contrato, +0x45, +0x49, +0x4a, +0x4e, +0x4f nibble alto do cargo; id e valor; lê e grava (assinar_contrato)',
           'contract +0 person (−1 free slot), +4 club, +0xc value [inference], +0x25 start, +0x2d end, +0x35 type (6 bits; 5 loan, 2) and bits, +0x3a, +0x3b, +0x3d type by job, +0x3e, +0x3f, +0x44 contract happiness, +0x45, +0x49, +0x4a, +0x4e, +0x4f high nibble of the job; id and value; reads and writes (assinar_contrato)'),
          ('pessoa +0x1a nacionalidade, +0x1e segunda nacionalidade, +0x39 clube, +0x3d cargo, +0x3e data (+0x40 ano), +0x46 fim do contrato, +0x4e valor do contrato [inferência], +0x61 ficha; ponteiro e valor; lê, grava +0x1e, +0x46 e +0x4e (assinar_contrato)',
           'person +0x1a nationality, +0x1e second nationality, +0x39 club, +0x3d job, +0x3e date (+0x40 year), +0x46 contract end, +0x4e contract value [inference], +0x61 player record; pointer and value; reads, writes +0x1e, +0x46 and +0x4e (assinar_contrato)'),
          ('[0xae2d0c] registro de 79 bytes (+0 índice do contrato); [0x9cf434] e [0x9cf260] nações fixas; [0xae23a8] nações (0x122 bytes); índice; lê e grava (assinar_contrato)',
           '[0xae2d0c] 79-byte record (+0 contract index); [0x9cf434] and [0x9cf260] fixed nations; [0xae23a8] nations (0x122 bytes); index; reads and writes (assinar_contrato)')]
INTERESSA = ['CMRTE', 'SGE']
