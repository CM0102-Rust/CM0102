# O valor do jogador, a comparacao da moral de dois jogadores e as marcas de elenco da tela do clube
# (contract_manager.cpp).
# - 0x4d0c60 (CONTRACT_MANAGER::calc_player_value, 5.648.958 chamadas ate 2002-06-01 na cobertura de 17/10; 36
#   chamadoras: transferencias, listas de observacao, regens, financas da semana, diretoria): o valor pelo auge, notas
#   da ficha, habilidade vista, nacao do clube, dias de contrato e clausula. Sorteia so sem clube (sorteio(3)).
# - 0x4cc3a0 (contract_manager_write_morale_comparison_text) e 0x4cc130 (CONTRACT_MANAGER::get_squad_flags_for_club):
#   so pelas telas (passageiras).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x4cf5f0 contrato com o clube: thiscall com o ecx DE ENTRADA (a 0x4d0c60 nao o muda; os chamadores poem
#   [0xae1050]), 2 args: a copia e thiscall e repassa o this. 0x8d0780 compensacao: a original poe ecx = [0xde1f54] e
#   empurra 3 (ret 0xc; a copia Rust e stdcall): declarada thiscall com 3 na pilha. 0x5407e0 reputacao vista: cdecl (add
#   esp, 0xc), 3 args. 0x6e86d0 habilidade vista: cdecl (add esp, 0x14), 5 args (a reputacao empurrada inteira, so o
#   word conta). 0x945640 _ftol: x87, pelo ftol. 0x90cfc0 sorteio: cdecl (add esp, 4), 1 arg. 0x867b50 clausula: thiscall
#   (mov ecx, esi = contrato), 0 args. 0x8d0a70 lei Bosman: a original poe ecx = [0xde1f54] e empurra 1 (ret 4; a copia
#   Rust e stdcall): declarada thiscall com 1 na pilha. 0x66f4e0 formatar: cdecl (add esp, 0xc), 3 args. 0x540a50
#   parceiro: cdecl (add esp, 0xc), 3 args (o segundo nulo).
# Comparadores: 0x4d0c60 lado a lado sem alocar (estado do sorteador, [0xae1080], as estaticas das datas da
# compensacao, CPQ6_ESTATICAS) e a devolucao; 0x4cc3a0 pelo texto e pela diferenca; 0x4cc130 pelo valor (sem clube ou
# sem pessoa, so a original). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('valor_do_jogador', 0x4d0c60, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('diferenca_de_moral_em_texto', 0x4cc3a0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('marcas_de_elenco_para_a_tela', 0x4cc130, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o valor do jogador, a comparação da moral de dois jogadores e as marcas de elenco da tela do clube',
        'the player value, the morale comparison of two players and the squad flags of the club screen',
        [('valor_do_jogador', 'CONTRACT_MANAGER::calc_player_value', '0x4d0c60', 'contract_manager.cpp', 'thiscall',
          'o valor do jogador pelo auge e pela idade, pelas notas da ficha, pela habilidade vista pelo técnico do clube, pela nação do clube, pelos dias de contrato e pela cláusula de rescisão',
          'the player value by peak age and age, record ratings, ability as seen by the club manager, club nation, contract days left and release clause'),
         ('diferenca_de_moral_em_texto', 'contract_manager_write_morale_comparison_text', '0x4cc3a0', 'contract_manager.cpp', 'stdcall',
          'a diferença de moral de dois jogadores em palavra ("Same", "Marginal", "Better", "Superior"), pela tela de comparação',
          'the morale difference of two players as a word ("Same", "Marginal", "Better", "Superior"), for the comparison screen'),
         ('marcas_de_elenco_para_a_tela', 'CONTRACT_MANAGER::get_squad_flags_for_club', '0x4cc130', 'contract_manager.cpp', 'thiscall',
          'as marcas de elenco que a tela do clube mostra, pelo bit de reserva do contrato quando o jogador está emprestado a outro clube',
          'the squad flags the club screen shows, by the contract reserve bit when the player is on loan elsewhere')])
CAMPOS = [('ficha +0xf, +0x12..+0x15 posições; +0x27, +0x28, +0x2d, +0x2f, +0x36, +0x43 notas; +9 reputação; +0xb atual; +0xd potencial; valor; lê (valor_do_jogador)',
           'player record +0xf, +0x12..+0x15 positions; +0x27, +0x28, +0x2d, +0x2f, +0x36, +0x43 ratings; +9 reputation; +0xb current; +0xd potential; value; reads (valor_do_jogador)'),
          ('pessoa +0x18 idade, +0x22 e +0x1a (+0x8e) experiência, +0x61 ficha, +0x5e marcas de elenco; valor/ponteiro; lê', 'person +0x18 age, +0x22 and +0x1a (+0x8e) experience, +0x61 player record, +0x5e squad flags; value/pointer; reads'),
          ('clube +0xcf técnico, +0x53 nação (+0x7e, +0x85), +0x57 divisão (+0x69), +0x64, +0x80 reputação; ponteiro/valor; lê', 'club +0xcf manager, +0x53 nation (+0x7e, +0x85), +0x57 division (+0x69), +0x64, +0x80 reputation; pointer/value; reads'),
          ('contrato +0x2d/+0x2f fim, +0x35 tipo e bit 0x40, +0x3d felicidade, +0x21 cláusula, +4 clube; valor; lê', 'contract +0x2d/+0x2f end, +0x35 type and 0x40 bit, +0x3d happiness, +0x21 clause, +4 club; value; reads'),
          ('[0xae1080] contrato consultado; ponteiro; grava', '[0xae1080] contract looked up; pointer; writes'),
          ('[0x9a7f30] tabela de 20 bytes pela idade; valor; lê', '[0x9a7f30] 20-byte table by age; value; reads'),
          ('ficha +0x45 moral; valor; lê (diferenca_de_moral_em_texto)', 'player record +0x45 morale; value; reads (diferenca_de_moral_em_texto)')]
INTERESSA = ['CMRTE', 'CMScout']
