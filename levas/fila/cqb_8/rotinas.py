# A reação do elenco à história de um jogador, o aviso aos rivais de posição e o rótulo das respostas à mídia
# (media.cpp).
# - 0x755340 (Media::apply_squad_reaction_to_player_story, 9 chamadas ate 2002-06-01 na cobertura de 17/10, pelo boato
#   de transferência 0x754a30, 321 chamadas): a chance de desafeição pelos clubes interessados, a rejeição do jogador
#   pelo clube (0x547940) e, por 57 pessoas do vetor do clube, a desafeição (0x547af0), a marca 1 do contrato
#   (0x867560) ou as queixas 0x800 (0x867700). Muitos sorteios condicionais: as mutações estão nela. O vetor tem 58
#   dwords: os 50 do elenco (0x541f90), o +0xbf, o técnico, o +0xd3 e os 5 de +0x19f; a 51ª ([esp+0xe4]) é o +0xbf do
#   clube, gravada com 8 dwords empurrados (o "lixo de pilha" que a cpx deixou de fora não existe).
# - 0x758010 (Media::warn_squad_of_position_rival; fora do mapa de funções, sem cobertura medida; pelo boato 0x754a30 e
#   pela resposta à história 0x741020): passageira.
# - 0x735160 (media_format_response_option_label, só pela tela das histórias 0x742b3b): passageira; a tabela de saltos
#   (22 casos) virou CQB8_RESPOSTAS, o índice continua lido da tabela de bytes do exe (0x7356f4).
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   0x755340: thiscall (mov ecx antes nos três chamadores 0x72d1c0, 0x733f51, 0x755193; ecx não é lido), 5 na pilha (ret
#   0x14); o tópico vai como registrador inteiro (só a palavra conta) e o n como -1 ou byte (movsx). Dentro: 0x547790,
#   0x5477d0, 0x547810, 0x547840 cdecl (add esp, 8), 2 args (jogador, clube/pessoa); 0x4b02e0 cdecl, 2 (clube, outro),
#   lê al; 0x90cfc0 cdecl, 1; 0x547940 cdecl (add esp, 0xc), 3 (clube, jogador, 1); 0x544520 e 0x544600 cdecl, 2 (pessoa,
#   0); 0x4cf5f0 thiscall (mov ecx, [0xae1050]), 2 (pessoa, clube); 0x867d30 thiscall (mov ecx, esi = contrato), 0, lê
#   al; 0x541f90 cdecl, 3 (clube, vetor, 0) e 0x600a50 cdecl, 5 (clube, 0, 1, 0, 0) (um add esp, 0x20 para os dois);
#   0x53b620 cdecl (add esp, 4), 1 (o byte +0x3d em ecx; só o byte conta); 0x547af0 cdecl (add esp, 0xc), 3 (pessoa,
#   jogador, 1); 0x867560 thiscall (mov ecx, esi), 1 (1); 0x867700 thiscall (mov ecx, esi), 2 (0x800, 0).
#   0x758010: thiscall (mov ecx, [0xb6b7c8] em 0x74313a; mov ecx, ebp em 0x7551d1), 4 na pilha (ret 0x10). Dentro:
#   0x5481a0 cdecl, 1; 0x548150 cdecl (add esp, 0xc junto), 2 (clube, máscara), lê al; 0x4cf5f0 thiscall, 2; 0x62c400
#   thiscall (mov ecx, [0xb63d28]), 1 (o jogador, não a pessoa do elenco), lê al; 0x544520 cdecl, 2 (pessoa, clube);
#   0x90cfc0 cdecl, 1 (add esp, 8 para os dois); 0x72c1f0 thiscall (mov ecx, [0xb6b7c8]), 5 (pessoa, clube, 0x25, 0,
#   jogador; ret 0x14 conferido); 0x8676f0 thiscall (mov ecx, ebp), 0, lê a palavra (test ah, 8); 0x867560 thiscall, 1.
#   0x735160: stdcall (o chamador 0x742b3b põe ecx = [0xb6b7c8], que ela não lê), 4 (tipo palavra, opção byte, saída,
#   tamanho palavra), ret 0x10; 0x66f4e0 cdecl (add esp, 8), 2.
# Comparadores: 0x755340 e 0x758010 pela fita das portas do pós-jogo (comparar_pos: 0x547af0, 0x867560, 0x867700 e
# 0x72c1f0 já são portas; rodam só na original e a cópia confere a chamada e o estado antes dela), com o sorteador,
# [0xae1080], as seis pessoas do clube, as preferências, os atributos +0x58..+0x5d e os contratos inteiros das pessoas
# do vetor; 0x735160 lado a lado na saída. Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('reacao_do_elenco_a_historia', 0x755340, [0x81, 0xec, 0xf4, 0x00, 0x00, 0x00]),
           ('aviso_ao_rival_de_posicao', 0x758010, [0x83, 0xec, 0x10, 0x53, 0x57]),
           ('rotulo_da_resposta_da_midia', 0x735160, [0x8b, 0x44, 0x24, 0x0c, 0x85, 0xc0])]
DOCS = ('a reação do elenco à história de um jogador, o aviso aos rivais de posição e o rótulo das respostas à mídia',
        'the squad reaction to a player story, the warning to position rivals and the media response labels',
        [('reacao_do_elenco_a_historia', 'Media::apply_squad_reaction_to_player_story', '0x755340', 'media.cpp', 'thiscall',
          'depois de uma história sobre o jogador (o boato de transferência): a chance de desafeição pelos clubes interessados (clássico de Glasgow, confronto na liga, rival, mesma divisão), a rejeição pelo clube e, para cada pessoa do elenco, da comissão e da diretoria, a nota pela posição no elenco, ambição, lealdade e preferências; sorteada, a pessoa passa a não gostar do jogador e o contrato dela ganha a marca 1 ou as queixas 0x800',
          'after a story about the player (the transfer rumour): the dislike chance from the interested clubs (Old Firm, league fixture, rival, same division), the club rejecting him and, for each squad, staff and board member, a score from squad status, ambition, loyalty and preferences; when drawn, the person comes to dislike the player and his contract gets flag 1 or the 0x800 complaints'),
         ('aviso_ao_rival_de_posicao', 'Media::warn_squad_of_position_rival', '0x758010', 'media.cpp', 'thiscall',
          'com o jogador de reputação alta ligado ao clube e mais de dois do elenco na posição dele: os rivais de posição insatisfeitos (pela ambição e pela lealdade, por sorteio) ganham a marca 0x800 no contrato, e o primeiro dá a história 0x25',
          'with a high-reputation player linked to the club and more than two squad players in his position: the unsettled position rivals (by ambition and loyalty, by a draw) get contract flag 0x800, and the first one gives story 0x25'),
         ('rotulo_da_resposta_da_midia', 'media_format_response_option_label', '0x735160', 'media.cpp', 'stdcall',
          'o rótulo da opção de resposta a uma história da mídia ("Deny rumour", "Defend Player", "No Comment"...), pela tabela de 22 casos do tipo da história',
          'the label of a media story response option ("Deny rumour", "Defend Player", "No Comment"...), from the 22-case table by story kind')])
CAMPOS = [('clube +0x9b..+0xb3 favoritos e rejeitados; ponteiro; lê e grava (reacao_do_elenco_a_historia, pela 0x547940)',
           'club +0x9b..+0xb3 favoured and disliked staff; pointer; reads and writes (reacao_do_elenco_a_historia, via 0x547940)'),
          ('clube +0xb3/+0xb7/+0xbb rivais, +0x57 divisão, +0xbf, +0xcf técnico, +0xd3, +0xd7 elenco, +0x19f; ponteiro; lê',
           'club +0xb3/+0xb7/+0xbb rivals, +0x57 division, +0xbf, +0xcf manager, +0xd3, +0xd7 squad, +0x19f; pointer; reads'),
          ('pessoa +0x18 idade, +0x59/+0x5a/+0x5b/+0x5d atributos, +0x61 ficha (+0xb reputação), +0x65 preferências (+0x1c..+0x33), +0x69 ficha de técnico (+0x1d); valor/ponteiro; lê (e grava as preferências pela 0x547af0)',
           'person +0x18 age, +0x59/+0x5a/+0x5b/+0x5d attributes, +0x61 player record (+0xb reputation), +0x65 preferences (+0x1c..+0x33), +0x69 non-player record (+0x1d); value/pointer; reads (and writes the preferences via 0x547af0)'),
          ('contrato +0x3d felicidade, +0x40 marcas, +0x45 queixas, +0x4f tipo; valor; lê e grava (pelas 0x867560/0x867700)',
           'contract +0x3d happiness, +0x40 flags, +0x45 complaints, +0x4f type; value; reads and writes (via 0x867560/0x867700)'),
          ('[0x9d0118]/[0x9d011c] Celtic e Rangers, [0x9cf764] divisão inferior, [0xadadfc] competições; id; lê', '[0x9d0118]/[0x9d011c] Celtic and Rangers, [0x9cf764] lower division, [0xadadfc] competitions; id; reads'),
          ('[0x7356f4] tabela de bytes do tipo da história; valor; lê (rotulo_da_resposta_da_midia)', '[0x7356f4] story kind byte table; value; reads (rotulo_da_resposta_da_midia)')]
INTERESSA = ['CMRTE', 'GPF2']
