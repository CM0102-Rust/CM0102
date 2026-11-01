# O veto dos fisioterapeutas e da diretoria a transferencia, a noticia do ultimato da diretoria e o texto dela
# (manager_manager.cpp).
# - 0x696b10 (check_board_transfer_veto, 1098 chamadas ate 2002-06-01 na cobertura de 17/10, pela 0x8cdb80): comprando
#   jogador lesionado, os fisioterapeutas sorteiam (semeando o rand da CRT pelos ids e voltando ao relogio); clube
#   humano (o de ferias conta): o limite pela reputacao, a verba (comprando) e o pedido (vendendo) mexem nas confiancas
#   da diretoria (+6/+0x14/+0x16 do registro de 0x49 bytes) e vetam (0x200, 0x800, 0x2000).
# - 0x6ac700 (post_board_ultimatum_news, 4 chamadas; "cpq_1 ultimato dado" 2 na base): a noticia 0x178d, com sorteio.
# - 0x6ac880 (news_board_ultimatum_text, 11 chamadas): o texto dela (so pelas telas e caixas de entrada).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x62ed70 lesao: thiscall (mov ecx, [0xb63d28]), 1 arg. 0x9469fa srand: cdecl (add esp, 4), 1 arg. 0x7a7ed0
#   aposentadoria: thiscall (mov ecx, [0xb7434c]), 2 args. 0x544410 nota: cdecl (2 pushes, add esp 0xc/0x8/0x10
#   juntando o sorteio), 2 args. 0x90cfc0 sorteio: cdecl, 1 arg. 0x62ece0 recaida: thiscall (mov ecx, [0xb63d28]), 1
#   arg. 0x945ff0 _CIpow: x87 (base em st1, expoente em st0), pelo pow_do_jogo. 0x945640 _ftol: x87, pelo ftol.
#   0x946ebb time: cdecl (add esp, 8 junto com o srand), 1 arg. 0x4cf5a0 contrato_de: thiscall (mov ecx, [0xae1050]),
#   1 arg. 0x600a50 clube_humano: cdecl (add esp, 0x14), 5 args. 0x53c590 contar_elenco: cdecl (add esp, 0xc), 3 args.
#   0x4d1600 preco pedido: thiscall (mov ecx, [0xae1050] logo antes do call; a rotina guarda o ecx), 6 args.
#   0x4d0c60 valor do jogador: a original poe ecx = [0xae1050] e empurra 5 args (ret 0x14); declarada thiscall (mesma
#   pilha; a rotina nao depende do ecx). 0x5a0590 financas: thiscall (mov ecx, [0xae2a38]), 1 arg. 0x598800 verba:
#   thiscall (mov ecx, eax = financas), 5 args (os 5 pushes antes da 0x5a0590 ficam para ela), devolve __int64.
#   0x945e40 _allmul / 0x945e80 _alldiv: em linha (i64 × 12 / 10). 0x5986b0 faixa: thiscall (mov ecx, eax), 0 args.
#   0x77bbe0 construir / 0x785790 gravar: thiscall (lea ecx, local), 2 args. 0x7861e0 / 0x7862d0 / 0x7863f0 entregar:
#   thiscall (mov ecx, [0xb7431c]), 2 args. 0x785830 vaga_da_noticia: thiscall (mov ecx, esi = noticia), 1 arg.
#   0x53dba0 nome: cdecl (add esp, 0x10), 4 args. 0x66f4e0 sprintf: cdecl (add esp 0x30 / 0x14), 12 ou 5 args.
#   0x4ae660 por_texto: thiscall (mov ecx, edi = saida), 1 arg. 0x53e990 confere: cdecl (add esp, 0x14), 5 args.
#   0x4ae8a0 ligar: thiscall (mov ecx, edi = saida), 3 args.
# Comparadores: o veto lado a lado (registro da diretoria do clube, [0xae1080] e o sorteio; o rand da CRT so sem a
# semente do relogio); a noticia e o texto so a original (alocam). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('veto_dos_fisios_e_da_diretoria', 0x696b10, [0x83, 0xec, 0x14, 0x8a, 0x44, 0x24, 0x24]),
           ('noticia_do_ultimato', 0x6ac700, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('texto_do_ultimato', 0x6ac880, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o veto dos fisioterapeutas e da diretoria à transferência, a notícia do ultimato da diretoria e o texto dela',
        'the physios\' and board\'s transfer veto, the board ultimatum news and its text',
        [('veto_dos_fisios_e_da_diretoria', 'check_board_transfer_veto', '0x696b10', 'manager_manager.cpp', 'thiscall',
          'comprando um jogador lesionado, os fisioterapeutas sorteiam o veto; num clube humano, a diretoria veta pela verba, pelo pedido ou pelo valor e perde confiança',
          'when buying an injured player, the physios draw the veto; at a human club, the board vetoes on budget, asking price or fee and loses confidence'),
         ('noticia_do_ultimato', 'post_board_ultimatum_news', '0x6ac700', 'manager_manager.cpp', 'thiscall',
          'a notícia 0x178d do ultimato antes do jogo, ao técnico do clube ou, por sorteio, aos da divisão ou do país',
          'the 0x178d news of the ultimatum before the match, to the club manager or, by a draw, to those of the division or country'),
         ('texto_do_ultimato', 'news_board_ultimatum_text', '0x6ac880', 'manager_manager.cpp', 'stdcall',
          'o título ou o corpo da notícia do ultimato, com os nomes e as ligações do técnico e dos clubes',
          'the headline or body of the ultimatum news, with the names and links of the manager and the clubs')])
CAMPOS = [('diretoria (0x49 bytes em [gerência dos técnicos] + id × 0x49) +6, +0x14, +0x16 confianças; valor; lê e grava (veto_dos_fisios_e_da_diretoria)',
           'board (0x49 bytes at [manager manager] + id × 0x49) +6, +0x14, +0x16 confidences; value; reads and writes (veto_dos_fisios_e_da_diretoria)'),
          ('clube +0x1cf três fisioterapeutas, +0x53 nação (+0x11c), +0x57 divisão, +0x80 reputação; ponteiro/valor; lê', 'club +0x1cf three physios, +0x53 nation (+0x11c), +0x57 division, +0x80 reputation; pointer/value; reads'),
          ('lesão +0x28 gravidade; valor; lê', 'injury +0x28 severity; value; reads'),
          ('pessoa +0x39 clube atual, +0x61 ficha (+0x2d); ponteiro/valor; lê', 'person +0x39 current club, +0x61 player record (+0x2d); pointer/value; reads'),
          ('contrato +0x35 & 0x3f tipo, +0x3d, +4 clube; valor; lê', 'contract +0x35 & 0x3f type, +0x3d, +4 club; value; reads'),
          ('[0xad6c30] rand da CRT; valor; grava (srand pelos ids e pelo relógio)', '[0xad6c30] CRT rand state; value; writes (srand by ids and clock)'),
          ('[0x9cf764] chave da divisão inferior; id; lê', '[0x9cf764] lower division key; id; reads'),
          ('clube +0xcf técnico, +4 e +0x38 nomes, +0x37 e +0x52 gêneros; ponteiro/valor; lê (noticia_do_ultimato, texto_do_ultimato)',
           'club +0xcf manager, +4 and +0x38 names, +0x37 and +0x52 genders; pointer/value; reads (noticia_do_ultimato, texto_do_ultimato)'),
          ('competição (0x6b bytes em [0xae23d0]) +0x38 nome, +0x52 gênero; valor; lê (texto_do_ultimato)', 'competition (0x6b bytes at [0xae23d0]) +0x38 name, +0x52 gender; value; reads (texto_do_ultimato)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava', '[0xae1080] last contract looked up; pointer; writes')]
INTERESSA = ['CMRTE']
