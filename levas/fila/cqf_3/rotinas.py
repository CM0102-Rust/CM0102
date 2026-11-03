# O contrato que o clube oferece a uma pessoa (contract_manager.cpp):
# - 0x4cda70 (contract_manager_offer_contract, 2517 bytes, quadro SEH do STAFF_CONTRACT local sem destrutor; x87 com o
#   pow da CRT 0x945ff0): pelo dia dos contratos (0x4c7ac0), pelos lances da IA (0x84a020), pelos registros do pessoal
#   e pelas telas. O contrato desejado (0x85fc50) ajustado pelo fator, a multa mínima pelas luvas e pelo valor da oferta,
#   o teto (o salário máximo do clube no estatuto 11, senão o do pessoal), o fim pela idade e pela habilidade (no ano que
#   vem, em 2 anos ou pela idade, com o valor e os bônus reduzidos), o total pela idade (pow) que reduz o valor ou o
#   teto, os cortes (contrato atual que acaba depois, lesão longa, 5 anos com [0xae233c]) e o valor acima do teto
#   (recusa acima de teto × [0x968dc0], senão o valor no teto e o salário e os bônus na razão). Devolve os termos em
#   `saida` ou faz a proposta (0x4cc490). 6093 chamadas na carreira até 2002-06-01 (cobertura 17/10). Uma rotina grande,
#   sozinha no item.
# Igual no cm0102-1080.exe (inteira).
# Determinismo: o STAFF_CONTRACT local é zerado e montado pela 0x85e410 e pela 0x85fc50 (na original, sobra de pilha
# antes da 0x85e410): os bytes +0x38..+0x39, +0x44 e o nibble baixo de +0x4f nenhuma das duas grava, e a original os
# copia com a sobra para a saída e para a proposta (achado no jogo comparando: 20 divergências só nesses bits); a cópia
# grava 0, e o comparador os ignora; o estatuto e o cargo vão à 0x85fc50 e à 0x597db0 em dwords com os bytes altos sem valor na
# original (os chamados leem o byte; a cópia passa o byte estendido com zero); o nibble do cargo à 0x5968d0 e o +0x3d à
# 0x4cc490 idem (leem o byte). O pow pela própria 0x945ff0 (pow_do_jogo), o x87 em precisão dupla.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x4cda70: 0x4c847a, 0x4c8588, 0x4c8d64 (push 2; push 0x3ff00000; push 0; push −1; push 0; push 0;
#   push pessoa; mov ecx, edi) -> thiscall(this, pessoa, clube, saida, vaga, fator (double), modo), ret 0x1c.
# - dentro: 0x85e410 lea ecx, local -> thiscall, ret 0; 0x53b900 push clube; add esp, 4 -> cdecl; 0x53b620 push byte;
#   add esp, 4 -> cdecl; 0x544820 push clube; add esp, 4 -> cdecl (al); 0x600a50 push 0; push 0; push 1; push 0; push
#   clube; add esp, 0x14 -> cdecl; 0x8cca10 push 0; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret 8; 0x8c61e0 push
#   vaga; mov ecx, [0xde1f54] -> thiscall, ret 4; 0x849f00 push &cargo; push clube; push pessoa; mov ecx,
#   [[0xde1f54]+0x856] -> thiscall, ret 0xc (al); 0x85fc50 push 0; push 0; push cargo; push estatuto; push vaga; push
#   clube; push pessoa; lea ecx, local -> thiscall, ret 0x1c; 0x945640 _ftol; 0x8e7b50 push 1; push 0 ×4; mov ecx, oferta
#   -> thiscall, ret 0x14; 0x5a0590 push clube; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x5968d0 push nibble; push
#   pessoa; mov ecx, finanças -> thiscall, ret 8; 0x597db0 push cargo; mov ecx, finanças -> thiscall, ret 4; 0x4cf5f0
#   push clube; push pessoa; mov ecx, this -> thiscall, ret 8; 0x54cdb0 push data (2 dwords); mov ecx, 0xae2c90 / lea
#   ecx, local+0x2d -> thiscall, ret 8; 0x945ff0 _CIpow (base em st1, expoente em st0); 0x62ed70 push pessoa; mov ecx,
#   [0xb63d28] -> thiscall, ret 4; 0x867520 push 0; lea ecx, local -> thiscall, ret 4; 0x417c10 push origem; push &temp;
#   lea ecx, destino -> thiscall, ret 8; 0x54c5b0 push 0x96; push &saída; mov ecx, 0xae1068 -> thiscall, ret 8; 0x4cc490
#   push 0; push 1; push 0; push cargo; push −1; push vaga; push &local; mov ecx, this -> thiscall, ret 0x1c.
# Comparador: com `saida`, lado a lado (a saída de 0x50 bytes, [0xae1080] e o estado do sorteador; a cópia sem contar
# os sorteios); sem `saida` (o caminho da proposta, que aloca), só a original.
# Provas (02/11, cópia fora do projeto com o motor.rs da principal = 268ª + cqc_6, mais cqf_1..3 e cqg_1): jogo comparando
# até 2001-09-01 em ENG (0 divergências, nenhuma linha DIVERGENCIA) e em ENG ESP ITA FRA (0 do total; uma linha
# DIVERGENCIA da aviso_ao_rival_de_posicao, a mesma com a DLL sem os itens: anterior a eles); carreira Rust até 2002-06-01
# duas vezes iguais e o detector de lixo igual (impressão 825918943f68aa97, 473125915 sorteios, a mesma da base sem os itens).
SAVE = False
ROTINAS = [('contrato_oferecido', 0x4cda70, [0x6a, 0xff, 0x68, 0x68, 0x6d, 0x95, 0x00])]
DOCS = ('o contrato que o clube oferece a uma pessoa (o desejado ajustado pelo fator, a multa mínima, o teto salarial, o fim pela idade e pela habilidade, o total pela idade, os cortes e o valor no teto) e a proposta',
        'the contract a club offers a person (the desired one scaled by the factor, the minimum release fee, the wage cap, the end by age and ability, the total by age, the cut-offs and the value capped) and the offer',
        [('contrato_oferecido', 'contract_manager_offer_contract', '0x4cda70', 'contract_manager.cpp', 'thiscall, ret 0x1c',
          'o contrato desejado (0x85fc50) × fator, a multa mínima, o teto (salário máximo do clube ou do pessoal), o fim no ano que vem, em 2 anos ou pela idade, o total pela idade que reduz o valor ou o teto, os cortes e o valor no teto; devolve os termos ou propõe (0x4cc490)',
          'the desired contract (0x85fc50) × factor, the minimum release fee, the cap (club or staff maximum wage), the end next year, in 2 years or by age, the total by age that reduces the value or the cap, the cut-offs and the value capped; returns the terms or makes the offer (0x4cc490)')])
CAMPOS = [('contrato (STAFF_CONTRACT, 0x50 bytes) +4 clube, +8 salário, +0xc valor [inferência], +0x10..+0x18 três bônus (−1 sem), +0x1c, +0x1d luvas, +0x1f, +0x21 multa, +0x25 data, +0x2d fim (+0x2f ano), +0x35 bits, +0x4f nibble alto do cargo; valor; grava (contrato_oferecido)',
           'contract (STAFF_CONTRACT, 0x50 bytes) +4 club, +8 wage, +0xc value [inference], +0x10..+0x18 three bonuses (−1 none), +0x1c, +0x1d signing fee, +0x1f, +0x21 release fee, +0x25 date, +0x2d end (+0x2f year), +0x35 bits, +0x4f high nibble of the job; value; writes (contrato_oferecido)'),
          ('pessoa +0 id, +0x10 nascimento, +0x39 clube, +0x3d tipo, +0x5b, +0x61 ficha de jogador; id e valor; lê (contrato_oferecido)',
           'person +0 id, +0x10 birth date, +0x39 club, +0x3d type, +0x5b, +0x61 player record; id and value; reads (contrato_oferecido)'),
          ('ficha do jogador +0xb habilidade atual, +0xd habilidade potencial, +0xf posição, +0x34, +0x3c; valor; lê (contrato_oferecido)',
           'player record +0xb current ability, +0xd potential ability, +0xf position, +0x34, +0x3c; value; reads (contrato_oferecido)'),
          ('clube +0 id, +0x80 reputação, +0xcf comissão; [0xae2d0c] registro de 79 bytes (+0 índice do contrato, +0xb bits 0x30); [0xae1080] o último contrato; [0xae233c] limite de 5 anos; lê, grava [0xae1080] (contrato_oferecido)',
           'club +0 id, +0x80 reputation, +0xcf board; [0xae2d0c] 79-byte record (+0 contract index, +0xb bits 0x30); [0xae1080] the last contract; [0xae233c] 5-year limit; reads, writes [0xae1080] (contrato_oferecido)')]
INTERESSA = ['CMRTE']
