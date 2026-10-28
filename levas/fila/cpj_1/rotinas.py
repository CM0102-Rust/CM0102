# O recorde do jogador mais velho (oldest_player_record_construct 0x450cd0, club_records.cpp, irmão da 0x450c70 da
# 245ª) e as permissões de trabalho: achar a do jogador (TRANSFER_MANAGER::find_work_permit 0x8c7260, chamada pelas
# notas do motor 0x6e2f20/0x6e6340 e pelas telas; bsearch da CRT 0x946464 com a comparação 0x8e55c0) e se o jogador
# passa no limite de estrangeiros do comprador (passes_work_permit_check 0x84b180, shortlist_manager.cpp; 0x53d740 já
# em Rust, 0x8c7950 original). Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem caixa;
# quadro SEH só no construtor (não preciso). Comparadores: o construtor lado a lado; a busca pelo valor; o limite lado
# a lado pelo estado do sorteio (a 0x8c7950 tem um sorteio opcional, pedido como 0 aqui).
SAVE = False
ROTINAS = [('construir_recorde_do_mais_velho', 0x450cd0, [0x6a, 0xff, 0x68, 0x63, 0x57, 0x95, 0x00]),
           ('permissao_de_trabalho_de', 0x8c7260, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x10]),
           ('passa_na_permissao', 0x84b180, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('o recorde do jogador mais velho e as permissões de trabalho: achar a do jogador e se ele passa no limite de estrangeiros',
        'the oldest player record and the work permits: finding the player permit and whether he passes the foreign player limit',
        [('construir_recorde_do_mais_velho', 'oldest_player_record_construct', '0x450cd0', 'club_records.cpp', 'thiscall',
          'constrói o recorde do jogador mais velho: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110)',
          'constructs the oldest player record: the age (+0), the holder date (+0x18) and the match date (+0x20) (0x54a110)'),
         ('permissao_de_trabalho_de', 'TRANSFER_MANAGER::find_work_permit', '0x8c7260', 'transfer_manager.cpp', 'thiscall, ret 4',
          'com a marca 0x20 do id em [+0x86e], a busca binária pela permissão (registros de 0xe bytes em [+0x93a]); senão 0',
          'with the 0x20 flag of the id in [+0x86e], the binary search for the permit (0xe-byte entries in [+0x93a]); else 0'),
         ('passa_na_permissao', 'passes_work_permit_check', '0x84b180', 'shortlist_manager.cpp', 'stdcall, ret 0xc',
          'sem a regra 0xe na nação do comprador, mesma nação, 2ª nação ou comunitário: sim; sem seleção: não; senão a permissão (0x8c7950)',
          'without rule 0xe in the buyer nation, same nation, second nation or EU: yes; no national side: no; else the permit (0x8c7950)')])
CAMPOS = [('gerenciador de transferências +0x86e marcas por pessoa (8 bytes, 0x20 = permissão), +0x93a/+0x93e permissões (0xe bytes); pessoa +0x1a nação, +0x1e 2ª nação, +0x22 jogos pela seleção, +0x24 seleção; clube +0x53 nação; nação +0x76 regra de estrangeiros; recorde do mais velho (idade +0, recordista +0x18, partida +0x20); lê (permissao_de_trabalho_de, passa_na_permissao); escreve (construir_recorde_do_mais_velho)',
           'transfer manager +0x86e flags per person (8 bytes, 0x20 = permit), +0x93a/+0x93e permits (0xe bytes); person +0x1a nation, +0x1e second nation, +0x22 caps, +0x24 national side; club +0x53 nation; nation +0x76 foreign player rule; oldest player record (age +0, holder +0x18, match +0x20); reads (permissao_de_trabalho_de, passa_na_permissao); writes (construir_recorde_do_mais_velho)')]
INTERESSA = ['SGE', 'CMRTE']
