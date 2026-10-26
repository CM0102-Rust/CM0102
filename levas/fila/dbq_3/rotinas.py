# A máscara da posição natural (0x5481a0: 37 chamadas, entre elas a escolha do companheiro favorito, o motor de
# partidas 0x6b7f20, a mídia 0x758010/0x758200/0x72c1f0, os prêmios 0x768cf0 e as telas de pessoas), quantos do elenco a
# têm (0x548150: mídia 0x758010/0x758200) e o companheiro favorito escolhido (0x548da0: diário das pessoas
# game_daily_staff_update 0x5ce2d0 e tela da pessoa 0x87dd80; 856 bytes). Simulação: as mutações pela carreira até
# 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem sorteio direto; sem variável local lida sem valor (a
# 0x5481a0 grava três bytes de uma local e um no próprio argumento antes de lê-los; a 0x548da0 grava as quatro locais
# antes de ler e usa o próprio argumento como cursor; a cópia não escreve na pilha da original). A 0x548150 e a 0x548da0
# chamam a 0x5481a0 pelo endereço do jogo; a 0x548da0 chama também 0x54cdb0, 0x4cf5f0, 0x8cca10 e 0x7c1e30 pelo
# endereço (o comparador dela volta o estado do sorteio por garantia). Em laço quente: a 0x548da0 (dentro do diário de
# cada pessoa, laço de 50) e, por ela e pelo motor de partidas, a 0x5481a0; correm o risco da 0x547770 (233ª), de o
# desvio mudar o resíduo de pilha que uma original lê depois (a 0x5481a0 original deixa na pilha um byte da posição no
# lugar do argumento; a cópia não).
SAVE = False
ROTINAS = [('mascara_da_posicao_natural', 0x5481a0, [0x51, 0x8b, 0x4c, 0x24, 0x08]),
           ('contar_elenco_com_mascara', 0x548150, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('escolher_companheiro_favorito', 0x548da0, [0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x18])]
DOCS = ('a máscara da posição natural, quantos do elenco a têm e o companheiro favorito escolhido',
        'the natural position mask, how many of the squad have it and the picked favoured team mate',
        [('mascara_da_posicao_natural', 'staff_get_natural_position_mask', '0x5481a0', 'Database.cpp', 'cdecl',
          'a máscara das posições em que o jogador é natural (20): goleiro 1; laterais, zagueiros, meias, atacantes (bits 2..0xc00); sem nenhuma, a primeira posição com 15 ou mais; senão 0x10',
          'the mask of the positions where the player is natural (20): goalkeeper 1; full backs, centre backs, midfielders, forwards (bits 2..0xc00); with none, the first position at 15 or more; otherwise 0x10'),
         ('contar_elenco_com_mascara', 'club_count_squad_with_position_mask', '0x548150', 'Database.cpp', 'cdecl',
          'quantos dos 50 do elenco (+0xd7) com jogador têm a máscara da posição natural igual à dada',
          'how many of the 50 in the squad (+0xd7) with a player record have the given natural position mask'),
         ('escolher_companheiro_favorito', 'staff_pick_favoured_team_mate', '0x548da0', 'Database.cpp', 'cdecl',
          'o colega do elenco que a pessoa escolhe para favorito: filtros pela pessoa (idade, +0x57, +0x59) e pelo colega (rejeitado, posição, contrato, reputação, 0x8cca10), nota pela reputação, média 0x7c1e30, já favorito, ids, nível do contrato e nacionalidade; o de nota mais alta',
          'the squad mate the person picks as favoured: filters by the person (age, +0x57, +0x59) and by the mate (disliked, position, contract, reputation, 0x8cca10), score by reputation, the 0x7c1e30 average, already favoured, ids, contract level and nationality; the highest score')])
CAMPOS = [('jogador (TStaff +0x61) +0xf..+0x19 posições, +5, +0xb reputação, +0x21; lê (mascara_da_posicao_natural, escolher_companheiro_favorito)',
           'player (TStaff +0x61) +0xf..+0x19 positions, +5, +0xb reputation, +0x21; reads (mascara_da_posicao_natural, escolher_companheiro_favorito)'),
          ('TClub +0xd7 elenco (50), +0x53 nação, +0x80 reputação; TStaff +0x18, +0x1a nação, +0x3e/+0x42 nascimento, +0x57, +0x59, +0x5a, +0x5b; preferências +0x1c..+0x30; contrato +0x45, +0x4f; [0xae2364], [0xb74350] +0x10/+0x18; lê (contar_elenco_com_mascara, escolher_companheiro_favorito)',
           'TClub +0xd7 squad (50), +0x53 nation, +0x80 reputation; TStaff +0x18, +0x1a nation, +0x3e/+0x42 birth date, +0x57, +0x59, +0x5a, +0x5b; preferences +0x1c..+0x30; contract +0x45, +0x4f; [0xae2364], [0xb74350] +0x10/+0x18; reads (contar_elenco_com_mascara, escolher_companheiro_favorito)')]
INTERESSA = ['SGE', 'CMRTE']
