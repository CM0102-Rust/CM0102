# Os amistosos dos clubes (friendly.cpp, fora da fila): Friendly::process_friendly_reply (0x5c5f40, 464 chamadas na
# carreira até 2002-06-01, pela atualização diária 0x5c2460 sobre a lista das respostas +0x34d4: a IA decide se aceita
# o amistoso de pré-temporada e o aceito vai ao calendário), Friendly::send_friendly_request (0x5c5570, 293, pela
# criação dos amistosos planejados 0x5c2ae0 e pela excursão 0x5c4040, as duas em Rust) e Friendly::build_nation_list
# (0x5ca3e0, 1, pela construção do Friendly 0x5c1f80 na criação do jogo: a lista das nações que os amistosos
# internacionais 0x5cbb00/0x5cbdf0 percorrem). Quadros SEH nas duas primeiras só para a notícia e o nó da lista no
# desenrolar de exceção (sem destrutor no caminho normal). Iguais no cm0102-1080.exe (corpo inteiro).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x600a50 (clube_humano): push 0; push 0; push 1; push 0; push clube; add esp, 0x14 -> cdecl de 5.
# - 0x53b900 (e_selecao): push clube; add esp, 4 -> cdecl de 1.
# - 0x77bbe0 (construir a notícia): push 0; push tipo; ecx = a local; sem add esp -> thiscall, ret 8.
# - 0x785790 (dado da notícia): push valor; push k; ecx = a local; sem add esp -> thiscall, ret 8.
# - 0x7861e0 (notícia ao técnico do clube): push clube; push &local; ecx = [0xb7431c]; sem add esp -> thiscall, ret 8.
# - 0x7875e0 (clube de técnico humano): push clube; ecx = [0xb7431c]; sem add esp -> ret 4; a cópia Rust é
#   stdcall(clube) (o this não é lido), chamada assim.
# - 0x54a3f0 (dia da semana): ecx = a data (o pedido, +0 dia e +2 ano); sem push -> thiscall, ret 0.
# - 0x5aa680 (FIXTURE_MANAGER::add_fixture): push 1; push &local; ecx = [0xae2a58]; sem add esp -> thiscall, ret 8.
# - 0x5c6740 (vaga do amistoso marcado): push +0x10; push +0x14; push ano; push dia; ecx = this; sem add esp ->
#   thiscall, ret 0x10 (dia e ano em registradores com lixo nos 16 bits altos; a Rust só lê as palavras).
# - 0x5c87b0 (tirar o amistoso): push 0; push pedido; ecx = this; sem add esp -> thiscall, ret 8.
# - 0x68d1b0 (próximo da lista): ecx = a lista -> thiscall, ret 0; 0x68d1f0 (tirar da lista): push item; ecx = a
#   lista; sem add esp -> thiscall, ret 4; 0x68d0d0 (guardar na lista): push 4; push &dado; ecx = a lista -> thiscall,
#   ret 8; 0x68d020 (construtor da lista): ecx = o bloco -> thiscall, ret 0, devolve o bloco.
# - 0x944cf1 (operator new): push 0xe; add esp, 4 -> cdecl; 0x944e46 (malloc): push n; add esp, 4 -> cdecl.
# - 0x945333 (qsort): push 0x5cc5f0; push 4; push n; push base; add esp, 0x10 -> cdecl de 4.
# Comparadores: as três só a original (alocam: a partida no calendário, a notícia, o nó da lista, a lista das nações;
# a 0x5c5f40 também libera o pedido pela 0x5c87b0). Os planos, feitos antes: a decisão e a vaga do amistoso marcado (ou
# a situação e a saída da lista das respostas); o nó a mais com o pedido e a situação 2 do humano; a lista das nações
# ordenada numa local pelo mesmo qsort.
SAVE = False
ROTINAS = [('amistoso_responder_pedido', 0x5c5f40, [0x6a, 0xff, 0x68, 0xbb, 0xa0, 0x95, 0x00]),
           ('amistoso_enviar_pedido', 0x5c5570, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('amistoso_lista_das_nacoes', 0x5ca3e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os amistosos dos clubes: a resposta ao pedido de amistoso, o pedido enviado e a lista das nações para os amistosos internacionais',
        'club friendlies: the reply to a friendly request, the sent request and the nation list for international friendlies',
        [('amistoso_responder_pedido', 'Friendly::process_friendly_reply', '0x5c5f40', 'friendly.cpp', 'thiscall, ret 8',
          'o convidado (sem técnico humano, ou com forcar) aceita ou recusa pela reputação, com o fator das nações, regiões, continentes e da distância das cidades dos estádios; avisa quem pede (notícia 7); o aceito vai ao calendário (0x5aa680) e marca a vaga do amistoso; com 3 ou 4 o pedido sai (0x5c87b0); noutra situação, só sai da lista das respostas',
          'the invited club (without a human manager, or when forced) accepts or refuses by reputation, with the nation, region, continent and stadium city distance factor; tells the requester (news 7); an accepted one goes to the fixture list (0x5aa680) and marks the friendly slot; with 3 or 4 the request is removed (0x5c87b0); otherwise it only leaves the reply list'),
         ('amistoso_enviar_pedido', 'Friendly::send_friendly_request', '0x5c5570', 'friendly.cpp', 'thiscall, ret 4',
          'guarda o pedido na lista das respostas (+0x34d4, criada se falta); convidado de técnico humano: situação 2 e a notícia 6',
          'stores the request on the reply list (+0x34d4, created when missing); an invited club with a human manager: status 2 and news 6'),
         ('amistoso_lista_das_nacoes', 'Friendly::build_nation_list', '0x5ca3e0', 'friendly.cpp', 'thiscall, ret 0',
          'aloca a lista (+0x34fa) das nações com continente, conta em +0x34fe e ordena pela reputação (0x5cc5f0)',
          'allocates the list (+0x34fa) of nations with a continent, counts it in +0x34fe and sorts it by reputation (0x5cc5f0)')])
CAMPOS = [('pedido de amistoso (0x19 bytes): +0/+2 dia e ano, +0xc quem pede, +0x10/+0x14 os clubes do jogo, +0x18 situação (1 pendente, 2 com o humano, 3 aceito, 4 recusado); lê, grava +0x18 (amistoso_responder_pedido, amistoso_enviar_pedido)',
           'friendly request (0x19 bytes): +0/+2 day and year, +0xc requester, +0x10/+0x14 the match clubs, +0x18 status (1 pending, 2 with the human, 3 accepted, 4 refused); reads, writes +0x18 (amistoso_responder_pedido, amistoso_enviar_pedido)'),
          ('clube +0x53 / +0x69 / +0x77 / +0x80: a nação, o estádio (+0x38 a cidade, +0x23/+0x2b as coordenadas), o inteiro +0x77 e a reputação; ponteiro e valor; lê (amistoso_responder_pedido)',
           'club +0x53 / +0x69 / +0x77 / +0x80: nation, stadium (+0x38 the city, +0x23/+0x2b the coordinates), the +0x77 integer and reputation; pointer and value; reads (amistoso_responder_pedido)'),
          ('nação +0x71 / +0x76: o continente e a região; ponteiro e valor; lê (amistoso_responder_pedido, amistoso_lista_das_nacoes)',
           'nation +0x71 / +0x76: continent and region; pointer and value; reads (amistoso_responder_pedido, amistoso_lista_das_nacoes)'),
          ('Friendly +0x34d4: a lista das respostas dos amistosos; ponteiro; lê e grava (amistoso_responder_pedido, amistoso_enviar_pedido)',
           'Friendly +0x34d4: the friendly reply list; pointer; reads and writes (amistoso_responder_pedido, amistoso_enviar_pedido)'),
          ('Friendly +0x34fa / +0x34fe: a lista das nações e a conta; ponteiro e valor; grava (amistoso_lista_das_nacoes)',
           'Friendly +0x34fa / +0x34fe: the nation list and its count; pointer and value; writes (amistoso_lista_das_nacoes)'),
          ('vaga do amistoso marcado (0x5c6740) +0x10 / +0x18: o adversário e a marca; ponteiro e valor; grava (amistoso_responder_pedido)',
           'scheduled friendly slot (0x5c6740) +0x10 / +0x18: opponent and mark; pointer and value; writes (amistoso_responder_pedido)'),
          ('[0xae2c92] o ano de hoje; [0xae2350] / [0xae23a8] a conta e a tabela das nações (0x122 bytes); valor e ponteiro; lê',
           '[0xae2c92] the current year; [0xae2350] / [0xae23a8] the nation count and table (0x122 bytes); value and pointer; reads')]
INTERESSA = ['CMRTE']
