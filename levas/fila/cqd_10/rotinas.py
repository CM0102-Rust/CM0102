# As aposentadorias do dia: as anunciadas e as que vencem, a noticia da aposentadoria e a do jovem promovido
# (player_regen.cpp).
# - 0x7a59e0 (PLAYER_REGEN::process_retirements, 3.056 bytes, quadro SEH sem destrutor no caminho normal; x87 nos
#   atributos do tecnico, transcrito em f64 na ordem da original): todo dia pelo laco do dia (0x5cda73; 322 chamadas ate
#   2002-06-01 na cobertura de 17/10). Desiste das aposentadorias do jovem com clube, de quem tem ficha de contrato ou
#   esta sob oferta e de quem mudou de clube ha pouco; aposenta (noticia, ofertas, contrato, selecao, listas, elenco,
#   faltas, disputa), faz o jogador virar tecnico ou guarda a ficha no banco dos regens, poe a vaga para o regen e
#   anuncia as futuras. As mutacoes estao nela.
# - 0x7a92e0 (player_regen_post_retirement_news): pela 0x7a59e0 em toda aposentadoria; a noticia so sai para clube de
#   tecnico humano (0x7875e0); com o motivo 8 (lesao) e jogador de peso, o seguro (0x58a980).
# - 0x7a8ba0 (player_regen_post_youth_promotion_news): pelo regen jovem (0x7abac0), so para clube de tecnico humano:
#   passageira.
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x7a59e0: thiscall (mov ecx, esi = [0xb7434c] em 0x5cda71), 0 (ret). Dentro: 0x54a110 thiscall (lea ecx, a data
#   local), 0; 0x945333 cdecl (add esp, 0x10), 4; 0x7a8690 thiscall (mov ecx, ebx = this), 1 (o item); 0x4cf6d0 thiscall
#   (mov ecx, [0xae1050]), 1; 0x8cde70 thiscall (mov ecx, [0xde1f54]), 2 (pessoa, 0); 0x54cdb0 thiscall (mov ecx,
#   0xae2c90), 2 (+0x3e, +0x42 da pessoa: a data por valor); 0x4cf5a0 thiscall (mov ecx, [0xae1050]), 1, sem guardar em
#   [0xae1080]; 0x544520 cdecl (add esp, 8), 2 (pessoa, 0); 0x7a92e0 stdcall (o ecx = this sem uso; push ecx/edx com o
#   motivo e a marca nos bytes de baixo: ela le os bytes), 3; 0x4ccbf0, 0x4d0b50 thiscall (mov ecx, [0xae1050]), 1;
#   0x85f800 thiscall (mov ecx, o contrato), 0; 0x775ba0 thiscall (mov ecx, [0xb74318]), 1; 0x843e10 thiscall (mov ecx,
#   ebp = [[0xde1f54]+0x856]), 4 (pessoa, clube, 4, 2); 0x842b00 thiscall (o mesmo), 2 (pessoa, 0); 0x7aace0 stdcall (o
#   ecx = this sem uso), 1; 0x4196a0, 0x4197c0 thiscall (mov ecx, [0xad9c6c]), 1; 0x56a090 thiscall (mov ecx,
#   [0xae2a30]), 1; 0x90cfc0 cdecl (add esp, 4; o sorteio de [0xae2350] empurra -1 antes do argumento), 1; 0x945ff0
#   _CIpow (st1 base, st0 expoente) e 0x945640 _ftol; 0x7a82b0 thiscall (mov ecx, ebx), 1; 0x539490 cdecl (add esp, 4),
#   1; 0x7c6650 thiscall (mov ecx, [0xb74350]), 1; 0x55e1d0 thiscall (mov ecx, [0xae28f0]), 1; 0x631f90 thiscall (mov
#   ecx, [0xb63d28]), 1; 0x7a8350 thiscall (mov ecx, ebx), 3 (id, nacao, segunda); 0x85e960 thiscall (mov ecx, o
#   contrato), 2 (pessoa, 0); 0x86fc20 thiscall (mov ecx, [0xdd7f00]), 1 (o registro de nao jogador); 0x600a50 cdecl
#   (add esp, 0x14), 5 (clube, 1, 1, 0, 0); 0x7aaeb0 stdcall (o ecx = this sem uso), 1; 0x7a87d0 thiscall (mov ecx,
#   ebx), 1 (a vaga local); 0x7ab340 (mov ecx, ebx; ret 4), 1; 0x7aa340 stdcall (o ecx = this sem uso; o motivo no dl),
#   2.
#   0x7a92e0: stdcall (push ecx/edx/esi, ret 0xc; nao le o ecx). Dentro: 0x7875e0 stdcall (o ecx = [0xb7431c] sem
#   uso), 1; 0x77bbe0 thiscall (lea ecx, a noticia), 2 (0xbba, 0); 0x785790 thiscall (lea ecx, a noticia), 2 (indice,
#   valor); 0x8d1da0 thiscall (mov ecx, [0xde1f54]; ret 0x10, sem add esp), 4 (pessoa, 0xa, &clubes, 0x1e), le al;
#   0x786780 thiscall (mov ecx, [0xb7431c]), 1; 0x7863f0 thiscall (o mesmo), 2; 0x7862d0 thiscall (o mesmo), 2; 0x7861e0
#   thiscall (o mesmo), 2; 0x4cf5a0 thiscall (mov ecx, [0xae1050]), 1, guardado em [0xae1080]; 0x58a980 cdecl (add esp,
#   0xc), 3 (clube, pessoa, +0x52).
#   0x7a8ba0: stdcall (mov ecx, ebp sem uso; push edi = clube, push esi = jogador em 0x7aced2; ret 8). Dentro: 0x77bbe0 e
#   0x785790 como acima; 0x90cfc0 cdecl (o argumento fica na pilha ate o add esp, 0x2c), 1; 0x945640 _ftol; 0x6e86d0
#   cdecl (add esp, 0x2c com as duas chamadas e o sorteio), 5 (quem, jogador, clube, 2 ou 1, a reputacao vista: le a
#   palavra); 0x786090 thiscall (mov ecx, [0xb7431c]), 6 (noticia, clube, 0, 0, 0, 0).
# Comparadores: os tres so a original (a 0x7a59e0 chama noticias, contratos novos, o regen como nao jogador e tira itens
# das listas; as duas noticias alocam e entregam, a do jovem sorteia); as copias levam os contadores e se provam pela
# carreira e pelas mutacoes. Iguais no cm0102-1080.exe (conferido byte a byte).
SAVE = False
ROTINAS = [('aposentadorias_do_dia', 0x7a59e0, [0x6a, 0xff, 0x68, 0xa8, 0xf1, 0x95, 0x00]),
           ('noticia_da_aposentadoria', 0x7a92e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_do_jovem_promovido', 0x7a8ba0, [0x6a, 0xff, 0x68, 0x0b, 0xf2, 0x95, 0x00])]
DOCS = ('as aposentadorias do dia: as anunciadas e as que vencem, a notícia da aposentadoria e a do jovem promovido',
        'the daily retirements: the announced and the due ones, the retirement news and the youth promotion news',
        [('aposentadorias_do_dia', 'PLAYER_REGEN::process_retirements', '0x7a59e0', 'player_regen.cpp', 'thiscall',
          'as aposentadorias pendentes do dia: desiste das do jovem com clube, de quem tem contrato pendente, está sob oferta ou mudou de clube há pouco; aposenta (notícia, contrato, seleção, elenco), pode fazer o jogador virar técnico (os atributos pela reputação e pela ficha), guarda a ficha no banco dos regens ou põe a vaga para o regen, e anuncia as futuras',
          'the pending retirements of the day: drops those of young players with a club, of players with a pending contract, under offer or recently moved; retires (news, contract, national team, squad), may turn the player into a coach (attributes from reputation and record), stores the record in the regen bank or queues the regen slot, and announces the future ones'),
         ('noticia_da_aposentadoria', 'player_regen_post_retirement_news', '0x7a92e0', 'player_regen.cpp', 'stdcall',
          'a notícia 0xbba da aposentadoria para os técnicos (a todos, aos do país, da divisão, do clube ou aos clubes interessados, pela fama) e o seguro do clube quando a lesão aposenta um jogador de peso',
          'news item 0xbba about the retirement for the managers (all, the country, the division, the club or the interested clubs, by fame) and the club insurance payout when injury retires a valuable player'),
         ('noticia_do_jovem_promovido', 'player_regen_post_youth_promotion_news', '0x7a8ba0', 'player_regen.cpp', 'stdcall',
          'a notícia 0xbce do jovem que chega ao clube, com o veredito do auxiliar ou de um treinador sobre a habilidade dele',
          'news item 0xbce about the youth player joining the club, with the assistant manager or a coach verdict on his ability')])
CAMPOS = [('regen (PLAYER_REGEN [0xb7434c]) +4/+8 aposentadorias pendentes (26 bytes: +0 pessoa, +4 clube, +8/+0xa anúncio, +0x10/+0x12 data, +0x18 motivo, +0x19 anunciada); ponteiro/valor; lê e grava (aposentadorias_do_dia)',
           'regen (PLAYER_REGEN [0xb7434c]) +4/+8 pending retirements (26 bytes: +0 person, +4 club, +8/+0xa announcement, +0x10/+0x12 date, +0x18 reason, +0x19 announced); pointer/value; reads and writes (aposentadorias_do_dia)'),
          ('regen +0x10/+0x14 vagas para o regen (12 bytes: pessoa, dia, ano, bissexto 1); +0x1c/+0x20 reserva de fichas de não jogador; ponteiro/valor; lê e grava',
           'regen +0x10/+0x14 regen slots (12 bytes: person, day, year, leap flag 1); +0x1c/+0x20 spare non-player record pool; pointer/value; reads and writes'),
          ('[0xae2d0c] + 79 × id +0xb marcas da aposentadoria (0x10 anunciada, 0x20 vaga, 0xf); valor; lê e grava',
           '[0xae2d0c] + 79 × id +0xb retirement flags (0x10 announced, 0x20 slot, 0xf); value; reads and writes'),
          ('pessoa +0x18 idade, +0x1a/+0x1e nações, +0x22 jogos pela seleção (byte), +0x39 clube, +0x3d cargo, +0x3e/+0x42 data de chegada, +0x52, +0x57, +0x5b, +0x5f, +0x61 ficha (+9, +0xb, +0xd, +0xf), +0x69 registro de não jogador (+8, +0xa, +0xc, +0x10, +0x11); valor/ponteiro; lê e grava',
           'person +0x18 age, +0x1a/+0x1e nations, +0x22 international caps (byte), +0x39 club, +0x3d job, +0x3e/+0x42 joined date, +0x52, +0x57, +0x5b, +0x5f, +0x61 player record (+9, +0xb, +0xd, +0xf), +0x69 non-player record (+8, +0xa, +0xc, +0x10, +0x11); value/pointer; reads and writes'),
          ('clube +0x53 nação, +0x57 divisão, +0x7f força, +0x80 reputação, +0xd3 auxiliar, +0x19f treinadores; ponteiro/valor; lê (as notícias)',
           'club +0x53 nation, +0x57 division, +0x7f strength, +0x80 reputation, +0xd3 assistant, +0x19f coaches; pointer/value; reads (the news)'),
          ('[[0xde1f54] + 0x856] (+0xdc linhas de 9 bytes por clube, +8 = 100); ponteiro/valor; grava', '[[0xde1f54] + 0x856] (+0xdc 9-byte rows per club, +8 = 100); pointer/value; writes')]
INTERESSA = ['CMRTE', 'SGE', 'GPF2']
