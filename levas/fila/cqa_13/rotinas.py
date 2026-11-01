# As reacoes dos elencos a transferencia (media.cpp):
# - 0x758200 (Media::process_transfer_squad_reactions): no fim de cada transferencia (TRANSFER_MANAGER::move_player_to_club
#   0x8c9720, chamada incondicional em 0x8ca879; "cpx_3 noticia da transferencia concluida" 64 na base), para jogadores
#   de 19 anos ou mais com reputacao 4250: as marcas e queixas dos contratos dos dois elencos (que levam a pedidos de
#   transferencia), a moral, e a torcida do novo clube (+8 da linha em [[0xb67a44]], no banco).
# - 0x75b6e0 (Media::queue_signing_celebration_story) e 0x75cfc0 (Media::queue_sale_unrest_story): as noticias, so por
#   ela (passageiras: o efeito so chega a noticia).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x758200 pelo chamador: thiscall (mov ecx, [0xb6b7c8] em 0x8ca870), 4 args (push flag, push velho, push novo, push
#   jogador), ret 0x10. 0x5481a0 mascara_da_posicao_natural: cdecl (add esp, 8 depois de duas em 0x7582a5), 1 arg.
#   0x4cf5f0 contrato_com_clube: thiscall (mov ecx, [0xae1050] em 0x7582d9/0x758451/0x758912), 2 args (push clube, push
#   pessoa). 0x867700 STAFF_CONTRACT::apply_event_flags: thiscall (mov ecx, esi = contrato em 0x758301/0x75832d), 2 args
#   (push pessoa ou 0, push marcas), ret 8 (a porta do pos-jogo so repassa: chamada pelo endereco). 0x8676f0
#   bandeiras_do_contrato: thiscall (mov ecx, esi), 0 args, eax (test ah). 0x867560 marcas_do_contrato: thiscall (mov
#   ecx, esi), 1 arg, ret 4 (no ensaio, a copia dela em ensaio, CPW7_ENSAIO). 0x8676d0 clear_flags: thiscall, 1 arg,
#   ret 4. 0x8674e0 clear_grievances: thiscall (mov ecx, esi em 0x75863c), 1 arg, ret 4. 0x90cfc0 sorteio: cdecl.
#   0x53c590 contar_elenco_do_clube: cdecl (add esp, 0xc em 0x7583b7), 3 args (0, 0, clube), al. 0x548150
#   contar_elenco_com_mascara: cdecl (add esp, 0xc cobre a mascara e ela), 2 args (push mascara, push clube), al.
#   0x54cdb0 dias_ate: thiscall (mov ecx, 0xae2c90 em 0x758434), 2 args (a data +0x3e por valor). 0x4ca470
#   categoria_do_contrato: o chamador poe ecx = [0xae1050] mas a rotina nao o le (stdcall no motor.rs), 2 args (push
#   contrato, push pessoa), al; chamada duas vezes quando a primeira nao da 1, como na original. 0x548810
#   melhor_candidato_a_favorito: cdecl (add esp, 0xc em 0x758a78), 3 args (push jogador, push novo, push membro), al.
#   0x75b6e0: thiscall (mov ecx, [esp+0x28] = this em 0x7586dc; [esp+0x1c] em 0x7587f0), 5 args (push porta-voz ou 0,
#   push 1, push 1 / 2, push novo, push jogador), ret 0x14. 0x75cfc0: thiscall (ecx = this em 0x758dca), 4 args (push
#   companheiro, push jogador, push novo, push velho), ret 0x10.
#   Nas noticias: 0x77bbe0 construir_noticia thiscall (lea ecx, local), 2 args; 0x785790 gravar_dado_da_noticia thiscall
#   (lea ecx, local), 2 args (push valor, push indice; o indice num dword de bytes de cima sem valor, a rotina le o
#   char); 0x757fa0 / 0x757f30 veiculo: cdecl (add esp, 4), 1 arg; 0x7dab90 sortear_comentarista: thiscall (mov ecx,
#   [0xb74430]), 3 args (push nacao ou -1, push veiculo, push clube); 0x5b7d20 mascara_de_posicoes: cdecl (add esp, 4),
#   o ax com sinal; 0x600a50 clube_humano: cdecl (add esp, 0x14), 5 args (clube, 0, 1, 0, 0); 0x735870
#   resposta_do_tecnico_da_maquina: thiscall (mov ecx, ebx = this da 0x75b6e0), 3 args (push 0x82, push jogador, push
#   tecnico); 0x53b900 e_selecao: cdecl, 1 arg; 0x7861e0 noticia_ao_tecnico_do_clube: thiscall (mov ecx, [0xb7431c]),
#   2 args (push clube, push &noticia).
# Comparadores: a 0x758200 em ensaio (a copia para antes de noticia, de pedido de transferencia pela 0x867560 ou pela
# 0x867700 e da retirada do pedido pela 0x8674e0; sem parar, lado a lado no sorteio, em [0xae1080], na data estatica
# 0xae2470, na linha da torcida e, dos dois elencos, nos contratos e nas linhas de opiniao em [0xae2d0c]; parando, so a
# original); as duas noticias so a original (alocam). Sem SEH com destrutor (o quadro das noticias so cobre a noticia
# local). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('reacoes_dos_elencos_a_transferencia', 0x758200, [0x83, 0xec, 0x18, 0x53, 0x55]),
           ('noticia_da_comemoracao_da_contratacao', 0x75b6e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_do_companheiro_desassossegado', 0x75cfc0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as reações dos elencos à transferência: os contratos dos dois elencos, a comemoração da contratação e o companheiro desassossegado pela venda',
        "the squads' reactions to a transfer: both squads' contracts, the signing celebration and the team mate unsettled by the sale",
        [('reacoes_dos_elencos_a_transferencia', 'Media::process_transfer_squad_reactions', '0x758200', 'media.cpp', 'thiscall',
          'depois da transferência de um jogador de 19 anos ou mais com reputação 4250: no elenco novo, as marcas e queixas dos contratos pela posição e pela força do elenco, a torcida (+500) e a comemoração da contratação de um craque; no elenco antigo, as queixas pela saída do companheiro, a moral −25, as marcas tiradas e a notícia do companheiro desassossegado',
          "after the transfer of a player aged 19 or more with reputation 4250: in the new squad, the contract flags and grievances by position and squad strength, the supporters (+500) and the celebration of a star signing; in the old squad, the grievances at the team mate leaving, morale −25, cleared flags and the unsettled team mate news"),
         ('noticia_da_comemoracao_da_contratacao', 'Media::queue_signing_celebration_story', '0x75b6e0', 'media.cpp', 'thiscall',
          'a notícia 0xbe5 da torcida ou do porta-voz do elenco comemorando a contratação, ao técnico do clube',
          'news 0xbe5 of the fans or a squad spokesman celebrating the signing, to the club manager'),
         ('noticia_do_companheiro_desassossegado', 'Media::queue_sale_unrest_story', '0x75cfc0', 'media.cpp', 'thiscall',
          'a notícia 0xbe4 do companheiro insatisfeito com a venda, ao técnico do clube antigo',
          "news 0xbe4 of the team mate unhappy at the sale, to the old club's manager")])
CAMPOS = [('contrato (0x50 bytes) +0x3d, +0x40 marcas de evento, +0x44 moral, +0x45 queixas; valores; lê e grava (reacoes_dos_elencos_a_transferencia)',
           'contract (0x50 bytes) +0x3d, +0x40 event flags, +0x44 morale, +0x45 grievances; values; reads and writes (reacoes_dos_elencos_a_transferencia)'),
          ('[[0xb67a44]] + 0x49 × id do clube, +8: a torcida (1..10000); valor; grava +500 (reacoes_dos_elencos_a_transferencia)',
           '[[0xb67a44]] + 0x49 × club id, +8: the supporters (1..10000); value; writes +500 (reacoes_dos_elencos_a_transferencia)'),
          ('pessoa +0x18 idade, +0x3e data de chegada, +0x57/+0x58/+0x59/+0x5a/+0x5b atributos, +0x61 ficha (+5, +0xb reputação, +0xd potencial); valores; lê (reacoes_dos_elencos_a_transferencia)',
           'person +0x18 age, +0x3e joining date, +0x57/+0x58/+0x59/+0x5a/+0x5b attributes, +0x61 player data (+5, +0xb reputation, +0xd potential); values; reads (reacoes_dos_elencos_a_transferencia)'),
          ('clube +0xd7 elenco (50), +0x80 reputação, +0x53 nação, +0x57 liga; ponteiros/valores; lê (reacoes_dos_elencos_a_transferencia)',
           'club +0xd7 squad (50), +0x80 reputation, +0x53 nation, +0x57 league; pointers/values; reads (reacoes_dos_elencos_a_transferencia)'),
          ('Media +0 fila das histórias (+8 pessoa, +0x10 tipo, +0x11, +0x13 notícia), +8 conta; valores; lê (reacoes_dos_elencos_a_transferencia)',
           'Media +0 story queue (+8 person, +0x10 kind, +0x11, +0x13 news), +8 count; values; reads (reacoes_dos_elencos_a_transferencia)')]
INTERESSA = ['CMRTE']
