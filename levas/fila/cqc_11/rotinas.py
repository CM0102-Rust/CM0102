# Os amistosos entre selecoes e os convites (friendly.cpp):
# - 0x5cbdf0 (Friendly::arrange_international_friendlies): pelo dia dos amistosos internacionais 0x5cbb00 (Rust,
#   amistosos_internacionais_do_dia), as entradas de hoje + 12 semanas; 20 chamadas na carreira automatica ate
#   2002-06-01 (docs/cobertura_simulacao.txt). Os amistosos criados vao ao calendario (0x5c6f00/0x5c5f40): partidas.
# - 0x5ca090 (friendly_check_match_invitation): pela noticia do convite (0x77bc80/0x782b70, originais); 45 chamadas.
# - 0x5ca290 (friendly_check_tournament_invitation): o mesmo para torneio (passageira; nao roda ate 2002-06-01).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   As proprias: 0x5cbdf0 thiscall (mov ecx, [0xae2c74]... no chamador; mov esi, ecx; ret 8: datas, quantas); 0x5ca090
#   thiscall (mov ebp, ecx; usado como o Friendly; ret 0x20: 8 args, o 7o o texto e o 8o o tamanho em palavra);
#   0x5ca290 thiscall (mov ebx, ecx; ret 0x10: clube, torneio, texto, tamanho).
#   Na 0x5cbdf0: 0x945501 calloc cdecl (push 1; push [0xae2350]; add esp, 8), 0x9452ca free cdecl (add esp, 4),
#   0x90cfc0 sorteio cdecl (push n; add esp, 4 em 0x5cc0df/0x5cc19b/0x5cc20e), 0x53b3d0 selecao principal cdecl
#   (push; add esp, 4 em 0x5cc21f/0x5cc22d; o push anterior e argumento da 0x5c6f00), 0x5ae6a0 stdcall (3 pushes: a
#   data, a fatia, o grupo; sem add esp; o mov ecx, [0xae2a58] sobra; ret 0xc), 0x68d1b0 thiscall (mov ecx, lista),
#   0x68d0d0 thiscall (push 4; push item; mov ecx, lista; ret 8), 0x5cc3e0 thiscall (push b; push a; mov ecx, esi;
#   ret 8), 0x5c6f00 thiscall (4 pushes: em_casa = sorteio(2), a selecao de b, a data +3, a selecao de a; mov ecx,
#   esi; ret 0x10), 0x5c5f40 thiscall (push 1; push amistoso; mov ecx, esi; ret 8), 0x54c8e0/0x54c5b0 thiscall (mov
#   ecx, data; push n; push saida; ret 8), 0x417c10 thiscall (mov ecx, destino; push src; push saida; ret 8).
#   Na 0x5ca090: 0x68d020/0x68d040 thiscall (lea ecx, lista local), 0x5acbe0 thiscall (9 pushes; mov ecx,
#   [0xae2a58]; ret 0x24), 0x66f4e0 cdecl variadica (add esp, 8 / 0x14), 0x5c6b70 stdcall (3 pushes: data, clube, 0;
#   sem add esp; o mov ecx, ebp sobra; ret 0xc), 0x54c5b0 thiscall (mov ecx, 0xae2c90). Na 0x5ca290 as mesmas
#   0x54c5b0, 0x5c6b70 (o tipo e o byte +0xc empurrado no ecx inteiro: a 0x5c6b70 so le o byte) e 0x66f4e0.
#   As caixas pelo aviso_em (cdecl, ja no motor.rs) ou pela original.
# Comparadores: 0x5cbdf0 so a original (cria amistosos, acrescenta as listas, sorteia) com o plano em ensaio: a
# primeira parte antes dela, a segunda lado a lado pelo estado do sorteador (sem_contar), decidindo cada amistoso pelo
# que a original fez; confere o sorteador e os itens novos de cada lista de nacoes. 0x5ca090 e 0x5ca290 lado a lado
# pelo texto e a devolucao (com a caixa, so a original). SEH sem destrutor no caminho normal (as datas; a LLIST local
# da 0x5ca090 e construida e destruida pelas originais 0x68d020/0x68d040). Iguais no cm0102-1080.exe (as tres inteiras).
SAVE = False
ROTINAS = [('amistosos_entre_selecoes', 0x5cbdf0, [0x6a, 0xff, 0x68, 0xc6, 0xa3, 0x95, 0x00]),
           ('convite_de_amistoso_valido', 0x5ca090, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('convite_de_torneio_valido', 0x5ca290, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('os amistosos entre seleções nas datas internacionais e os convites de amistoso e de torneio',
        'the friendlies between national teams on international dates and the friendly and tournament invitations',
        [('amistosos_entre_selecoes', 'Friendly::arrange_international_friendlies', '0x5cbdf0', 'friendly.cpp', 'thiscall',
          'os amistosos das seleções na data internacional: as nações com jogo de 4 dias antes a 4 depois ficam usadas (e uma vai à lista da outra); depois cada nação livre da lista ordenada, na confederação da data, procura a primeira seguinte livre que serve (sorteio de 500, mais 1000 no mesmo continente, acima de 400, uma das duas com +0x88 ≥ 5, o par não proibido e sem jogo entre elas no ano) e o amistoso entra com o mando sorteado',
          'the national team friendlies on an international date: the nations with a match from 4 days before to 4 after become used (and each goes to the other one\'s list); then each free nation of the sorted list, in the date\'s confederation, looks for the first following free one that fits (a draw of 500, plus 1000 on the same continent, above 400, one of the two with +0x88 >= 5, the pair not banned and no match between them in the year) and the friendly is added with a drawn home side'),
         ('convite_de_amistoso_valido', 'friendly_check_match_invitation', '0x5ca090', 'friendly.cpp', 'thiscall',
          'o convite de amistoso ainda vale: sem data, o texto diz se o jogo foi marcado ou cancelado (os jogos do clube no dia); com data, antes de depois de amanhã é cancelado, o clube livre na data vale (1), senão o texto diz que o clube não pode',
          'the friendly invitation still holds: without a date, the text says whether the match was arranged or cancelled (the club\'s matches on the day); with a date, before the day after tomorrow it is cancelled, the club free on the date holds (1), else the text says the club is unavailable'),
         ('convite_de_torneio_valido', 'friendly_check_tournament_invitation', '0x5ca290', 'friendly.cpp', 'thiscall',
          'o convite para torneio ainda vale: sem torneio ou com a data antes de depois de amanhã, cancelado; o clube livre na data (pelo tipo do torneio) vale (1), senão o texto diz que o clube não pode',
          'the tournament invitation still holds: without a tournament or with its date before the day after tomorrow, cancelled; the club free on the date (by the tournament type) holds (1), else the text says the club is unavailable')])
CAMPOS = [('Friendly +0x34f6 listas de nações (uma por nação; LLIST de ids de 4 bytes); ponteiro; lê e grava (amistosos_entre_selecoes: pela id na primeira parte, pela posição na segunda)',
           'Friendly +0x34f6 nation lists (one per nation; LLIST of 4-byte ids); pointer; reads and writes (amistosos_entre_selecoes: by id in the first part, by position in the second)'),
          ('Friendly +0x34fa nações ordenadas (ponteiros), +0x34fe quantas; ponteiro/valor; lê (amistosos_entre_selecoes)',
           'Friendly +0x34fa sorted nations (pointers), +0x34fe count; pointer/value; reads (amistosos_entre_selecoes)'),
          ('registro de data internacional (20 bytes): +2 só a África (1), +3 a data, +0xc o continente (−1 qualquer); valores; lê (amistosos_entre_selecoes)',
           'international date record (20 bytes): +2 Africa only (1), +3 the date, +0xc the continent (−1 any); values; reads (amistosos_entre_selecoes)'),
          ('nação +0 id, +0x71 continente (+0 id), +0x88 valor (uma das duas ≥ 5); id/ponteiro/valor; lê (amistosos_entre_selecoes)',
           'nation +0 id, +0x71 continent (+0 id), +0x88 value (one of the two >= 5); id/pointer/value; reads (amistosos_entre_selecoes)'),
          ('partida +0x1c/+0x20 os clubes; clube +0x53 a nação, +0x52 byte do artigo, +0x38 o nome; ponteiro; lê (amistosos_entre_selecoes, convite_de_amistoso_valido, convite_de_torneio_valido)',
           'fixture +0x1c/+0x20 the clubs; club +0x53 the nation, +0x52 article byte, +0x38 the name; pointer; reads (amistosos_entre_selecoes, convite_de_amistoso_valido, convite_de_torneio_valido)'),
          ('torneio +4 a data, +0xc o tipo; valor; lê (convite_de_torneio_valido)', 'tournament +4 the date, +0xc the type; value; reads (convite_de_torneio_valido)'),
          ('[0x9cfa08] a chave da África; id; lê (amistosos_entre_selecoes)', '[0x9cfa08] the Africa key; id; reads (amistosos_entre_selecoes)')]
INTERESSA = ['GPF2']
