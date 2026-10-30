# As listas de posição da lista de observação (shortlist_manager.cpp): montar as 16 × 2 listas de candidatos por vaga
# (Shortlist_Manager::build_position_lists 0x839e60, a cada 30 dias pelo laço do dia 0x5cda46 e na carga), refazer as
# chaves e ordená-las a cada dia (Shortlist_Manager::sort_position_lists 0x83a780, 0x5cda56) e os clubes interessados
# num jogador (Shortlist_Manager::collect_interested_clubs 0x842180, 12 chamadores: contratos, transferências, lista de
# transferências). Sem SEH. A 0x839e60 sorteia a nação de quem não tem (0x90cfc0) e para o jogo sem memória (caixa
# "Error" e saída, erro_em); empurra a vaga num dword com 3 bytes de lixo de pilha acima (a 0x844300 só lê o byte).
# Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo real comparando.
# Comparadores: montar só a original (solta e aloca os vetores) conferindo o resultado; ordenar lado a lado pelos 32
# vetores e [0xae1080]; os clubes interessados pela lista: a cópia numa lista nova (0x68d020) e os pares comparados.
SAVE = False
ROTINAS = [('montar_listas_de_posicao', 0x839e60, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('ordenar_listas_de_posicao', 0x83a780, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('clubes_interessados_no_jogador', 0x842180, [0x83, 0xec, 0x14, 0x53, 0x55])]
DOCS = ('as listas de posição da lista de observação: montar as listas, ordená-las a cada dia e os clubes interessados no jogador',
        'the shortlist position lists: building the lists, sorting them every day and the clubs interested in the player',
        [('montar_listas_de_posicao', 'Shortlist_Manager::build_position_lists', '0x839e60', 'shortlist_manager.cpp', 'thiscall, ret 4',
          'refaz as 32 listas (16 vagas × 2) com 12 bytes por entrada: o tamanho é o maior número de pessoas que servem para uma vaga (0x844300); cada pessoa fora dos humanos entra em cada vaga que serve com o id, o id da nação (sorteada se faltar) e o valor ou o +0x3d do contrato',
          'rebuilds the 32 lists (16 slots × 2) at 12 bytes per entry: the size is the largest number of people fitting a slot (0x844300); each non-human person enters every slot he fits with his id, the nation id (drawn when missing) and the value or the contract +0x3d'),
         ('ordenar_listas_de_posicao', 'Shortlist_Manager::sort_position_lists', '0x83a780', 'shortlist_manager.cpp', 'thiscall, ret 0',
          'refaz a chave das entradas (o valor com contrato de faixa 0, menos 1.500.000 no jogador listado de clube humano; na segunda lista −1000, 2000 ou o +0x3d do contrato) e ordena os dois vetores de cada vaga (0x945333 com 0x83b670)',
          'recomputes the entry keys (the value when the contract band is 0, minus 1,500,000 for a listed player of a human club; in the second list −1000, 2000 or the contract +0x3d) and sorts both arrays of each slot (0x945333 with 0x83b670)'),
         ('clubes_interessados_no_jogador', 'Shortlist_Manager::collect_interested_clubs', '0x842180', 'shortlist_manager.cpp', 'thiscall, ret 0x14',
          'esvazia a lista de saída e põe os pares (jogador, clube): os humanos que têm o jogador na lista do olheiro e os clubes do vetor pela pessoa não suspensos, que não são o dele e põem nele mais que o valor',
          'empties the output list and adds the (player, club) pairs: the humans who have the player on the scout shortlist and the clubs in the by-person array that are not suspended, are not his and value him above his value')])
CAMPOS = [('Shortlist_Manager +0x18/+0x58: os 16 + 16 vetores das listas de posição (12 bytes: id da pessoa, id da nação, chave); ponteiro e valor; grava (montar_listas_de_posicao, ordenar_listas_de_posicao)',
           'Shortlist_Manager +0x18/+0x58: the 16 + 16 position list arrays (12 bytes: person id, nation id, key); pointer and value; writes (montar_listas_de_posicao, ordenar_listas_de_posicao)'),
          ('Shortlist_Manager +0x98: as 16 contas; +0xd8: o tamanho dos vetores; valor; grava (montar_listas_de_posicao), lê (ordenar_listas_de_posicao)',
           'Shortlist_Manager +0x98: the 16 counts; +0xd8: the array size; value; writes (montar_listas_de_posicao), reads (ordenar_listas_de_posicao)'),
          ('Shortlist_Manager +0/+0x10: o vetor de pares (pessoa, clube) ordenado pela pessoa e a conta; +0xdc: as entradas de 9 bytes dos clubes (+0 as 40 vagas, +8 o estado: 100 suspenso); ponteiro e valor; lê (clubes_interessados_no_jogador)',
           'Shortlist_Manager +0/+0x10: the (person, club) pair array sorted by person and its count; +0xdc: the 9-byte club entries (+0 the 40 slots, +8 the state: 100 suspended); pointer and value; reads (clubes_interessados_no_jogador)'),
          ('vaga da lista do clube (0x25 bytes) +7: quanto o clube põe no jogador; +0xf: o jogador; valor e id; lê (clubes_interessados_no_jogador)',
           'club shortlist slot (0x25 bytes) +7: what the club values the player at; +0xf: the player; value and id; reads (clubes_interessados_no_jogador)'),
          ('staff +0x1a: a nação; ponteiro; lê e grava a sorteada se nula (montar_listas_de_posicao)',
           'staff +0x1a: the nation; pointer; reads and writes a drawn one when null (montar_listas_de_posicao)'),
          ('staff +0x39, +0x52, +0x5f, +0x61: o clube, o valor, o humano ativo, a ficha de jogador; ponteiro e valor; lê',
           'staff +0x39, +0x52, +0x5f, +0x61: the club, the value, the active human, the player data; pointer and value; reads'),
          ('contrato +4: o clube; +0x3d: o estatuto; +0x4e: bits 0..3; valor; lê (montar_listas_de_posicao, ordenar_listas_de_posicao)',
           'contract +4: the club; +0x3d: the status; +0x4e: bits 0..3; value; reads (montar_listas_de_posicao, ordenar_listas_de_posicao)'),
          ('clube +0x57: a divisão (marca em [0xadadfc]); +0x80: a reputação; ponteiro e valor; lê (ordenar_listas_de_posicao)',
           'club +0x57: the division (flag in [0xadadfc]); +0x80: the reputation; pointer and value; reads (ordenar_listas_de_posicao)'),
          ('[0xae1080] o último contrato buscado; ponteiro; grava', '[0xae1080] the last contract looked up; pointer; writes')]
INTERESSA = ['CMRTE', 'CMScout']
