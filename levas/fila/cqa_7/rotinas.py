# O valor do jogador para o clube (shortlist_manager.cpp, scout_manager.cpp, transfer_manager.cpp):
# Shortlist_Manager::value_player_for_club (0x840d80: 4,6 KB de x87, o mapa dá 1332 bytes; das listas de observação e
# das ofertas do computador 0x83f3a0, 0x842e10, 0x843800, 0x8c0810, 0x8c8090, 0x8c8860 e 0x8e5d60, a cada jogador
# avaliado: a nota pela vaga e o preço que o clube pagaria), SCOUT_MANAGER::find_club_search_entry (0x7f8610) e
# TRANSFER_MANAGER::find_international_appearance_money (0x8d0f00), estas duas só pelas telas: passageiras. Iguais no
# cm0102-1080.exe; sem SEH. É o que resta de shortlist_manager.cpp fora do construtor, do destrutor, da carga e da
# gravação. As constantes double são lidas do próprio exe (cqa7_k); pow (0x945ff0) e _ftol (0x945640) pelos ajudantes
# do motor.rs (pow_do_jogo, ftol). A ordem das contas x87 está no codigo.rs.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x840d80: 0x8400fb, 0x843324 e 0x843498 seis push (flag, mínimo, &saída, vaga, pessoa, clube) e mov
#   ecx, gerência -> thiscall(this, clube, pessoa, vaga, saída, mínimo, teto), ret 0x18.
# - dentro dela: 0x90cfc0 push n; add esp, 4 -> cdecl(n); 0x600a50 push 0; push 0; push 1; push 1; push clube; add
#   esp, 0x14 -> cdecl(clube, 1, 1, 0, 0); 0x5408c0 push observador; push pessoa; add esp, 8 -> cdecl(pessoa,
#   observador) (movsx al); 0x544590 e 0x544520 push 0; push pessoa; add esp, 8 -> cdecl(pessoa, 0); 0x53ca90 e
#   0x600e90 push clube -> cdecl(clube) (o argumento sai no add esp da 0x6e6340); 0x7f7490 mov ecx, [0xb74448]; push
#   técnico; push pessoa -> thiscall(olheiros, pessoa, técnico), ret 8; 0x6e6340 push observador; push corte; push
#   clube; push marcas; push ecx (a palavra da posição, alta com lixo); push pessoa; add esp, 0x18 -> cdecl(pessoa,
#   posição, marcas, clube, corte, observador); 0x544410 push nível; push ficha; (add esp junto com a 0x5408c0) ->
#   cdecl(ficha, nível) (al); 0x83bd00 push ecx (palavra da posição); mov ecx, [esp+0x2c] (this) -> thiscall(this,
#   posição), ret 4 (a cópia Rust é stdcall e não usa o this: mesma pilha); 0x6e86d0 push nota/4; push 1 ou 2; push
#   clube; push pessoa; push técnico; add esp, 0x14 -> cdecl(técnico, pessoa, clube, tipo, nota/4) (ax); 0x53d980 push
#   1; push nação; push pessoa; add esp, 0xc -> cdecl(pessoa, nação, 1); 0x945ff0 base em st1, expoente em st0;
#   0x945640 st0 -> eax (tira st0).
# - quem chama a 0x7f8610: 0x80d12b e 0x818ba3 mov ecx, [0xb74448]; push pessoa -> thiscall(olheiros, pessoa), ret 4.
# - dentro dela: 0x7f6cd0 push técnico; push dword com o índice no byte baixo; mov ecx, ebp -> thiscall(this, índice,
#   técnico), ret 8; 0x7bd700 push 0; push pessoa; mov ecx, registro -> thiscall(registro, pessoa, 0), ret 8; a caixa
#   com 0xa7b094 (scout_manager.cpp).
# - quem chama a 0x8d0f00: 0x884ea2 (tela) mov ecx, [0xde1f54]; push pessoa -> thiscall(gerência, pessoa), ret 4.
# - dentro dela: 0x946464 push 0x8b4e00; push 0xd; push conta; push vetor; push chave; add esp, 0x14 -> cdecl(chave,
#   vetor, conta, 0xd, comparação).
# Comparadores: a 0x840d80 lado a lado na nota de saída e na devolução (técnico sem a ficha +0x69, que a original
# sorteia: só a original); a 0x7f8610 pela devolução; a 0x8d0f00 lado a lado na marca da pessoa.
SAVE = False
ROTINAS = [('valor_do_jogador_para_o_clube', 0x840d80, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('entrada_do_jogador_na_busca_do_clube', 0x7f8610, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('premio_de_jogos_na_selecao_achado', 0x8d0f00, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('o valor do jogador para o clube (a nota pela vaga da lista de observação e o preço que o clube pagaria), a entrada do jogador na busca do clube observado e o prêmio por jogos na seleção achado',
        "the player's value for a club (the rating for the shortlist slot and the fee the club would pay), the player's entry in the scouted club search and the international appearance fee clause found",
        [('valor_do_jogador_para_o_clube', 'Shortlist_Manager::value_player_for_club', '0x840d80', 'shortlist_manager.cpp', 'thiscall, ret 0x18',
          'a nota do jogador para a vaga (pelo melhor observador do clube, pela lista do olheiro no clube humano, pela névoa) e o preço que o clube pagaria: o valor base, a idade, a habilidade vista dos jovens, a consistência, os tetos pela posição, pela reputação, pela ficha, pela idade, pela divisão e pela nação do clube',
          "the player's rating for the slot (through the club's best observer, the scout list at the human club, the fog of war) and the fee the club would pay: the base value, age, the young players' seen ability, consistency, the caps by position, reputation, record, age, division and the club's nation"),
         ('entrada_do_jogador_na_busca_do_clube', 'SCOUT_MANAGER::find_club_search_entry', '0x7f8610', 'scout_manager.cpp', 'thiscall, ret 4',
          'a vaga do jogador no registro da busca do humano que observa o clube dele, ou 0',
          "the player's slot in the record of the human's search that scouts his club, or 0"),
         ('premio_de_jogos_na_selecao_achado', 'TRANSFER_MANAGER::find_international_appearance_money', '0x8d0f00', 'transfer_manager.cpp', 'thiscall, ret 4',
          'com a marca 4 da pessoa, procura pela busca binária a cláusula de prêmio por jogos na seleção; sem achar, tira a marca',
          "with the person's flag 4, binary-searches the international appearance fee clause; when not found, clears the flag")])
CAMPOS = [('vaga da lista +1 / +7 / +0xb / +0xf / +0x24: a posição, o teto do preço, o valor de referência, a pessoa e o tipo (bit 0 empréstimo, bits 1..4 o tipo); valor e id; lê (valor_do_jogador_para_o_clube)',
           'shortlist slot +1 / +7 / +0xb / +0xf / +0x24: the position, the fee cap, the reference value, the person and the type (bit 0 loan, bits 1..4 the type); value and id; reads (valor_do_jogador_para_o_clube)'),
          ('clube +0xcf / +0x1b3 / +0x80 / +0x53 / +0x57: o técnico, os 7 olheiros, a reputação, a nação e a divisão (+0x69); ponteiro e valor; lê (valor_do_jogador_para_o_clube)',
           'club +0xcf / +0x1b3 / +0x80 / +0x53 / +0x57: the manager, the 7 scouts, the reputation, the nation and the division (+0x69); pointer and value; reads (valor_do_jogador_para_o_clube)'),
          ('pessoa +0x69: a ficha de conhecimento do observador; ponteiro; lê e grava uma sorteada quando falta (valor_do_jogador_para_o_clube)',
           "person +0x69: the observer's knowledge record; pointer; reads, and writes a random one when missing (valor_do_jogador_para_o_clube)"),
          ('pessoa +0x18 / +0x39 / +0x61: a idade, o clube e a ficha de jogador (+0xb, +0xd, +0x2d); ponteiro e valor; lê (valor_do_jogador_para_o_clube)',
           'person +0x18 / +0x39 / +0x61: the age, the club and the player record (+0xb, +0xd, +0x2d); pointer and value; reads (valor_do_jogador_para_o_clube)'),
          ('[0xae2d0c] + id × 0x4f + 0xb: os 4 bits da névoa da pessoa; valor; lê (valor_do_jogador_para_o_clube)',
           "[0xae2d0c] + id × 0x4f + 0xb: the person's 4 fog-of-war bits; value; reads (valor_do_jogador_para_o_clube)"),
          ('[this+0xdc] + id do clube × 9 + 5: a reputação do clube na tabela das listas; valor; lê (valor_do_jogador_para_o_clube)',
           "[this+0xdc] + club id × 9 + 5: the club's reputation in the shortlist table; value; reads (valor_do_jogador_para_o_clube)"),
          ('[0x9cf434] / [0x9cf300]: as duas nações com regra própria de preço; id; lê (valor_do_jogador_para_o_clube)',
           '[0x9cf434] / [0x9cf300]: the two nations with their own fee rule; id; reads (valor_do_jogador_para_o_clube)'),
          ('busca +0x170 / +0x190: o registro e o clube observados; ponteiro e id; lê (entrada_do_jogador_na_busca_do_clube)',
           'search +0x170 / +0x190: the scouted record and club; pointer and id; reads (entrada_do_jogador_na_busca_do_clube)'),
          ('[this+0x86e] + id × 8 bit 2 e [this+0x898] / [this+0x8a0]: a marca e as cláusulas de prêmio por jogos na seleção (0xd bytes, +0 a pessoa); valor e ponteiro; lê e grava a marca (premio_de_jogos_na_selecao_achado)',
           '[this+0x86e] + id × 8 bit 2 and [this+0x898] / [this+0x8a0]: the flag and the international appearance fee clauses (0xd bytes, +0 the person); value and pointer; reads and writes the flag (premio_de_jogos_na_selecao_achado)')]
INTERESSA = ['CMRTE', 'CMScout']
