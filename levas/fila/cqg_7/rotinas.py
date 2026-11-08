# O regen jovem (player_regen.cpp) e a saída da disputa (dispute.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x7abac0 (PLAYER_REGEN::regenerate_as_youth_player, 5376 bytes, 1655 instruções, x87 transcrito em f64 na ordem da
#   original): a pessoa aposentada da fila dos regens vira um jogador novo com a ficha da frente do banco de fichas:
#   nação, segunda nação, nomes, idiomas, idade, habilidade, reputações, personalidade e o clube. Centenas de chamadas
#   na carreira automática até 2002-06-01 (pela fila 0x7a65d0: "cpv_9 regen signed", "cpw_10 queued regen"). As
#   mutações estão nela. Item grande sozinho (brief cqe: rotina grande e passageiras).
# - 0x56a090 (DISPUTE::remove_dispute, 100 bytes, dispute.cpp): passageira, chamada pela 0x7abac0 em todo regen jovem e
#   pelo diário (0x5cf3b6...).
# Iguais no cm0102-1080.exe (as duas inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x7abac0: thiscall (mov ecx, edi = o PLAYER_REGEN; push ecx = a pessoa em 0x7a664c), 1 (ret 4). Dentro: 0x775ba0
#   thiscall (mov ecx, [0xb74318]), 1; 0x7a8470 thiscall (mov ecx, this), 1 (a ficha); 0x5afd00 thiscall (mov ecx,
#   [0xae2a60]), 1; 0x7c6650 (mov ecx, [0xb74350]), 0x86fb80 ([0xdd7f00]), 0x55e1d0 ([0xae28f0]), 0x631f90 ([0xb63d28]),
#   0x56a090 ([0xae2a30]), 0x4d0b50 e 0x4d0bd0 ([0xae1050]) thiscall, 1 (a pessoa); 0x556830 ([0xae28f0]) e 0x62bfa0
#   ([0xb63d28]) thiscall, 1 (o id); 0x90cfc0 cdecl, 1; 0x539c60, 0x53a1b0, 0x53a5b0 cdecl (add esp, 0xc no fim dos três),
#   1 (a nação; a 0x53a5b0 não lê o argumento); 0x5fff30 thiscall (mov ecx, [0xb63c98]), 1; 0x7c6bf0 thiscall (mov ecx,
#   [0xb74350]), 2 (pessoa e o dword do argumento com o byte de baixo trocado pelo idioma: a chamada lê o byte); 0x7c6930
#   (mov ecx, [0xb74350]; push idioma; push pessoa; ret 8, o ecx não é lido: stdcall), 2; 0x7c6a30 thiscall (mov ecx,
#   [0xb74350]), 4 (pessoa, idioma, a fluência num dword da pilha com só o byte de baixo gravado, 1); 0x945640 _ftol;
#   0x549ea0 cdecl (add esp, 4), 1; 0x53ab90 cdecl (add esp, 0xc junto do sorteio), 2 (pessoa, 0xae2c90); 0x7ad0e0 e
#   0x7ad590 thiscall (mov ecx, this), 1; 0x7a6f10 (mov ecx, this; push clube; push pessoa; ret 8, o ecx não é lido:
#   stdcall), 2; 0x7875e0 (mov ecx, [0xb7431c]; ret 4: stdcall), 1; 0x7a8ba0 (mov ecx, this; ret 8: stdcall), 2;
#   0x547a10 cdecl (add esp, 0xc), 3 (o auxiliar, a pessoa, 0); 0x538ec0 e 0x538bc0 cdecl (add esp, 8 no fim), 1.
#   0x56a090: thiscall (mov ecx, [0xae2a30]; push esi em 0x7abc22), 1 (ret 4). Dentro: 0x68d1b0 thiscall (mov ecx, a
#   lista), 0; 0x68d1f0 thiscall (mov ecx, a lista), 1 (o nó).
# Leitura de pilha: a 0x7abac0 passa à 0x7c6a30 a fluência num dword local com só o byte de baixo gravado (os de cima
#   são sobra de pilha); a 0x7c6a30 (já em Rust) lê só o byte, e a cópia passa o valor limpo. O argumento da pessoa tem
#   o byte de baixo trocado pelo contador dos idiomas e depois pela idade (o ponteiro não é mais lido).
# Ramos mortos copiados como estão: na nação pequena a idade é 17..27 e o ramo "acima de 26" fica depois do "acima de
#   20" (nunca roda); idem o "acima de 23" no outro lado.
# Comparadores: 0x7abac0 só a original (ponytail: tira a ficha do banco, limpa listas e históricos, aloca idiomas e o
#   contrato e manda notícia; não pode rodar duas vezes): a cópia é provada no modo Rust pelas mutações e pelas duas
#   carreiras Rust iguais. 0x56a090 lado a lado sem a marca; com a marca, só a original com o plano (a marca zerada e a
#   conta da lista uma a menos quando a pessoa está nela).
SAVE = False
ROTINAS = [('refazer_como_jovem', 0x7abac0, [0x83, 0xec, 0x08, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('tirar_da_disputa', 0x56a090, [0x53, 0x8b, 0x5c, 0x24, 0x08])]
DOCS = ('o regen jovem: a pessoa aposentada refeita como jogador novo do banco de fichas (nação, nomes, idiomas, idade, habilidade, reputações, personalidade) e posta num clube; a saída da disputa',
        'the youth regen: the retired person rebuilt as a new player from the record pool (nation, names, languages, age, ability, reputations, personality) and placed at a club; the dispute removal',
        [('refazer_como_jovem', 'PLAYER_REGEN::regenerate_as_youth_player', '0x7abac0', 'player_regen.cpp', 'thiscall',
          'refaz a pessoa aposentada da fila dos regens como jogador jovem: a ficha da frente do banco de fichas, a nação (ou outra pequena), a segunda nação, os nomes e os idiomas da nação, a idade e a habilidade (pela reputação da nação pequena ou pela posição e por sorteio), as reputações e seus tetos, o nascimento, os atributos de personalidade pela idade e, com chance, o melhor clube ou um clube da nação',
          'rebuilds the retired person of the regen queue as a young player: the front record of the pool, the nation (or another small one), the second nation, the nation names and languages, the age and ability (by the small nation reputation or by position and by draw), the reputations and their caps, the birth, the personality attributes by age and, by chance, the best club or a club of the nation'),
         ('tirar_da_disputa', 'DISPUTE::remove_dispute', '0x56a090', 'dispute.cpp', 'thiscall',
          'zera a marca de disputa da pessoa e tira a entrada dela da lista das disputas',
          'clears the person dispute flag and removes her entry from the dispute list')])
CAMPOS = [('PLAYER_REGEN +0x3c o banco de fichas (entradas de 16 bytes: +0 id da ficha, +4 nação, +8 segunda nação); valor; lê (refazer_como_jovem)',
           'PLAYER_REGEN +0x3c the record pool (16-byte entries: +0 record id, +4 nation, +8 second nation); value; reads (refazer_como_jovem)'),
          ('[0xae2d0c] + 0x4f × id: +8 = 0, +9 = 0x9c, +0xb bits 0x30 aposentado (zerados), 0x40 a marca do regen (docs/MARCA_DO_REGEN.md), +0xc bits 0/1, +0xd = 0; valor; lê e grava',
           '[0xae2d0c] + 0x4f × id: +8 = 0, +9 = 0x9c, +0xb bits 0x30 retired (cleared), 0x40 the regen mark (docs/MARCA_DO_REGEN.md), +0xc bits 0/1, +0xd = 0; value; reads and writes'),
          ('pessoa +4/+8/+0xc nomes, +0x10 dia e +0x12 ano do nascimento, +0x18 idade, +0x1a nação, +0x1e segunda nacionalidade, +0x22 jogos pela seleção (0), +0x23 (0), +0x24, +0x39 clube, +0x3e/+0x40 a data de entrada, +0x4e, +0x52, +0x56, +0x5f tipo (2 jogador), +0x61 ficha, +0x69 ficha de não jogador (0), +0x28/+0x3d = 11; valor/ponteiro; grava',
           'person +4/+8/+0xc names, +0x10 birth day and +0x12 birth year, +0x18 age, +0x1a nation, +0x1e second nationality, +0x22 international caps (0), +0x23 (0), +0x24, +0x39 club, +0x3e/+0x40 the joined date, +0x4e, +0x52, +0x56, +0x5f kind (2 player), +0x61 record, +0x69 non-player record (0), +0x28/+0x3d = 11; value/pointer; writes'),
          ('ficha do jogador (0x46 bytes em [0xae23c4]) +5 habilidade (CA), +7 PA, +9/+0xb/+0xd reputações [inferência: atual, local, mundial], +0xf/+0x10/+0x11/+0x12/+0x13/+0x14/+0x15/+0x19 posições, +0x1b, +0x1d, +0x1e, +0x21, +0x24, +0x2c, +0x2e, +0x2f, +0x34, +0x36, +0x3c, +0x3d, +0x44 atributos [inferência: os de personalidade e os mentais]; valor; grava',
           'player record (0x46 bytes at [0xae23c4]) +5 ability (CA), +7 PA, +9/+0xb/+0xd reputations [inference: current, home, world], +0xf/+0x10/+0x11/+0x12/+0x13/+0x14/+0x15/+0x19 positions, +0x1b, +0x1d, +0x1e, +0x21, +0x24, +0x2c, +0x2e, +0x2f, +0x34, +0x36, +0x3c, +0x3d, +0x44 attributes [inference: personality and mental ones]; value; writes'),
          ('nação +0x71 continente, +0x88 reputação, +0x11c a marca das nações com clubes de base [inferência]; valor; lê; [0x9cf488]/[0x9cf424] as chaves da Espanha e do País Basco; id; lê',
           'nation +0x71 continent, +0x88 reputation, +0x11c the flag of nations with youth clubs [inference]; value; reads; [0x9cf488]/[0x9cf424] the Spain and Basque Country keys; id; reads'),
          ('DISPUTE ([0xae2a30]) +0 a lista das disputas (LLIST, +0 conta, +2/+0xa cursor), +4 a tabela de marcas por id; valor; lê e grava (tirar_da_disputa)',
           'DISPUTE ([0xae2a30]) +0 the dispute list (LLIST, +0 count, +2/+0xa cursor), +4 the flag table per id; value; reads and writes (tirar_da_disputa)')]
INTERESSA = ['CMRTE', 'SGE', 'CMScout', 'GPF2']
