# Contratos (contract_manager.cpp), 2º item: o lugar do jogador no elenco (CONTRACT_MANAGER::get_squad_placement
# 0x4cbb60, 428.820 chamadas na carreira até 2002-06-01; 0x53b620, 0x542740, 0x540a50 já em Rust) e as duas trocas que
# dependem dele: subir para o principal (move_to_first_team 0x4cc0b0, 88.084) e descer para os reservas
# (move_to_reserves 0x4cc0f0, 25.333), pela troca 0x4cbd20 (original, 363 instruções). Simulação: as mutações pela
# carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem sorteio. Local: a 0x4cbb60
# usa a vaga do 1º argumento (a pessoa) como saída da 0x540a50, que nem sempre grava; a cópia começa a vaga com o
# ponteiro da pessoa, como a original. A 0x4cbb60 grava a ficha [0xae1080] (o contrato achado por dentro) e devolve só
# o al (o resto do eax é lixo das chamadas; o jogo lê o al). Comparadores: o lugar lado a lado pela ficha (valor u8);
# as trocas decidem pelo lugar antes (0x4cbb60 pelo desvio, a ficha restaurada) e, fora dos lugares da troca, lado a
# lado (as duas devolvem 0); nos lugares da troca, só a original (a 0x4cbd20 mexe no elenco).
SAVE = False
ROTINAS = [('lugar_no_elenco', 0x4cbb60, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14]),
           ('subir_para_o_principal', 0x4cc0b0, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('descer_para_os_reservas', 0x4cc0f0, [0x53, 0x8b, 0x5c, 0x24, 0x08])]
DOCS = ('o lugar do jogador no elenco e as subidas e descidas entre o principal e os reservas',
        'the player squad placement and the moves between the first team and the reserves',
        [('lugar_no_elenco', 'CONTRACT_MANAGER::get_squad_placement', '0x4cbb60', 'contract_manager.cpp', 'thiscall, ret 8',
          '0 nenhum, 1 reservas, 2 principal (bit 0 de +0x5e no próprio clube com reservas), 3 no time B ligado, 4 no clube principal com time B (pelo clube parceiro 0x540a50 e pelo bit 6 de +0x35 do contrato)',
          '0 none, 1 reserves, 2 first team (bit 0 of +0x5e at his own club with reserves), 3 at the linked B side, 4 at the main club with a B side (through the partner club 0x540a50 and bit 6 of the contract +0x35)'),
         ('subir_para_o_principal', 'CONTRACT_MANAGER::move_to_first_team', '0x4cc0b0', 'contract_manager.cpp', 'thiscall, ret 8',
          'nos reservas ou no time B (lugar 1 ou 3), troca pela 0x4cbd20; senão 0',
          'in the reserves or at the B side (placement 1 or 3), swaps through 0x4cbd20; else 0'),
         ('descer_para_os_reservas', 'CONTRACT_MANAGER::move_to_reserves', '0x4cc0f0', 'contract_manager.cpp', 'thiscall, ret 8',
          'no principal ou no clube com time B (lugar 2 ou 4), troca pela 0x4cbd20; senão 0',
          'in the first team or at the club with a B side (placement 2 or 4), swaps through 0x4cbd20; else 0')])
CAMPOS = [('pessoa (0x6e bytes): +0x39 clube, +0x3d cargo, +0x5e bit 0 principal; contrato +0x35 bit 6; ficha [0xae1080]; lê (lugar_no_elenco, subir_para_o_principal, descer_para_os_reservas); grava [0xae1080] (lugar_no_elenco)',
           'person (0x6e bytes): +0x39 club, +0x3d job, +0x5e bit 0 first team; contract +0x35 bit 6; scratch [0xae1080]; reads (lugar_no_elenco, subir_para_o_principal, descer_para_os_reservas); writes [0xae1080] (lugar_no_elenco)')]
INTERESSA = ['SGE', 'CMRTE']
