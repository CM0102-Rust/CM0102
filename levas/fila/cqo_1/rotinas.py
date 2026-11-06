# A escolha do estádio de um jogo (stadium.cpp): o estádio da partida (Stadium::set_fixture_venue, 0x85c260; ~19 mil
# chamadas na carreira de 8 ligas até 2002-06-01: todo jogo marcado, pela tabela da liga 0x6835c0, pelos jogos da copa
# 0x51cb90/0x51ebe0, pela TV 0x685640...) e a busca do estádio neutro (Stadium::find_available_venue, 0x85c800; ~470
# chamadas: finais, jogos de seleção, torneios com sede).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x85c260: os chamadores (ex. 0x683ee0/0x6835c0): push estádio; push &partida; push marcas; mov ecx, [0xdd7ef4]
#   -> thiscall(this, marcas, partida, estádio), ret 0xc.
# - 0x85c800: só pela 0x85c260 (0x85c76b e 0x85c7dc): 7 push (partida, fora, casa, &data, nação, continente, marcas);
#   mov ecx, ebp (this) -> thiscall, ret 0x1c.
# - dentro: 0x53b900 push clube; add esp, 4 -> cdecl (Rust); 0x5c6710 push comp; mov ecx, [0xae2c74] -> thiscall, ret 4
#   (Rust); 0x5fa730 push 1, &b, &a, ano (word +0x2c no eax, bytes altos de sobra: ela só lê o word), comp; mov ecx,
#   [0xb5e134] -> thiscall, ret 0x14 (Rust); 0x85de20 push &data; push estádio; mov ecx, ebp -> thiscall, ret 8
#   (Rust); 0x85ddb0 push &data; push estádio; mov ecx, edi -> thiscall, ret 8 (Rust); 0x85da00 5 push (partida,
#   fora, casa, marcas, estádio); mov ecx, edi -> thiscall, ret 0x14 (Rust); 0x945333 (qsort, 4 push, add esp, 0x10
#   -> cdecl, Rust); 0x946464 (bsearch, 5 push, add esp, 0x14 -> cdecl, original da CRT); os comparadores 0x85e2f0,
#   0x85e320, 0x85e380 (call ebx / call [esp+0x18]: 2 push, add esp, 8 -> cdecl); 0x549ef0/0x417c10/0x54a110 (a data
#   local: a cópia monta a Data direto, com crate::data::bissexto); a caixa (aviso_em, linha 0x1f0).
# Quadro SEH da 0x85c260: só as Datas locais (sem destrutor no caminho normal).
# Comparadores: lado a lado (a cópia sem contar os sorteios: a nota do estádio neutro 0x85da00 sorteia) pelo estado do
# sorteador, pela partida (0x44 bytes), pelo cabeçalho do this (0x1c), pelas reservas (com a vaga da nova) e pela
# lista dos estádios (a ordem muda pelo qsort); sem lista de reservas ou com ela cheia (a reserva aloca)
# só a original. A busca compara também a devolução.
# Determinismo: a chave da busca binária é montada inteira (os campos que os comparadores não leem ficam 0; na
# original ficam com sobra da pilha, que nenhum comparador lê); nada lê endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('estadio_da_partida', 0x85c260, [0x6a, 0xff, 0x68, 0xd1, 0x29, 0x96, 0x00]),
           ('estadio_neutro', 0x85c800, [0x83, 0xec, 0x18, 0x53, 0x55])]
DOCS = ('a escolha do estádio de um jogo: o estádio da partida e a busca do estádio neutro',
        'choosing a match venue: the fixture venue and the neutral venue search',
        [('estadio_da_partida', 'Stadium::set_fixture_venue', '0x85c260', 'stadium.cpp', 'thiscall, ret 0xc',
          'o estádio da partida (+0x18, id em +4, +0x3c neutro): o jogo comum é no estádio do mandante; a seleção da Inglaterra até 2002 e os clubes da Itália e da França em amistoso vão a campo neutro; o time finlandês na Liga dos Campeões com estádio de menos de 40 mil lugares joga no estádio nacional se livre; com a marca 0x8000 o estádio do Monaco; a competição inglesa (marca 4) com o estádio guardado vai ao estádio nacional do País de Gales [inferência: Cardiff, com Wembley em obras]; nas outras marcas o estádio nacional ou a busca do estádio neutro nas sedes da competição (0x5fa730), com rodízio entre as duas sedes de um torneio',
          'the fixture venue (+0x18, id in +4, +0x3c neutral): an ordinary match is at the home club ground; the England national team until 2002 and Italian and French clubs in friendlies go to a neutral ground; a Finnish side in the Champions League with a ground under 40,000 plays at the national stadium if free; flag 0x8000 the Monaco ground; the English competition (flag 4) with the stashed venue goes to the Welsh national stadium [inference: Cardiff, with Wembley being rebuilt]; other flags the national stadium or the neutral venue search in the competition hosts (0x5fa730), rotating between the two hosts of a tournament'),
         ('estadio_neutro', 'Stadium::find_available_venue', '0x85c800', 'stadium.cpp', 'thiscall, ret 0x1c',
          'a busca do estádio neutro: na lista dos estádios ordenada pela nação, pelo continente ou pelos lugares, até 30 candidatos da chave (fora o estádio guardado e, em jogo de clubes, os dos dois clubes); o neutro comum é o livre de maior nota (0x85da00); o rodízio das sedes de um torneio (marcas 0x40..0x4000) pega o candidato de posição módulo 9 da marca; sem achar, o estádio nacional (marcas 4/8/0x20) ou nenhum',
          'the neutral venue search: in the venue list sorted by nation, continent or capacity, up to 30 candidates of the key (skipping the stashed venue and, in club matches, both clubs\' grounds); the ordinary neutral venue is the free one with the best score (0x85da00); the host rotation of a tournament (flags 0x40..0x4000) takes the candidate whose position modulo 9 matches the flag; failing that, the national stadium (flags 4/8/0x20) or none')])
CAMPOS = [('partida +0 competição (id); +4 id do estádio (grava); +0x14 competição (ponteiro: +0x59 continente, +0x5d nação); +0x18 estádio (ponteiro; grava); +0x1c/+0x20 mandante e visitante; +0x28 ano, +0x2a dia; +0x2c ano da edição (word); +0x3c 1 em campo neutro (grava); +0x42 fase; lê e grava (estadio_da_partida)', 'fixture +0 competition (id); +4 stadium id (writes); +0x14 competition (pointer: +0x59 continent, +0x5d nation); +0x18 stadium (pointer; writes); +0x1c/+0x20 home and away; +0x28 year, +0x2a day; +0x2c edition year (word); +0x3c 1 on neutral ground (writes); +0x42 stage; reads and writes (estadio_da_partida)'),
          ('estádios [0xdd7ef4] +0 reservas (12 bytes: estádio, data), +4 conta, +8 capacidade; +0xc lista dos estádios (16 bytes: +0 nação, +4 continente, +8 de clube [inferência], +0xc estádio), +0x10 conta; +0x18 a ordem atual (1 nação, 2 continente, 3 lugares); lê e grava', 'stadiums [0xdd7ef4] +0 bookings (12 bytes: stadium, date), +4 count, +8 capacity; +0xc venue list (16 bytes: +0 nation, +4 continent, +8 club ground [inference], +0xc stadium), +0x10 count; +0x18 current order (1 nation, 2 continent, 3 capacity); reads and writes'),
          ('clube +0x53 nação (ponteiro); +0x69 estádio (ponteiro); estádio +0 id, +0x3c lugares; nação +0x80 estádio nacional; [0xa835a4] estádio guardado; lê', 'club +0x53 nation (pointer); +0x69 stadium (pointer); stadium +0 id, +0x3c capacity; nation +0x80 national stadium; [0xa835a4] stashed venue; reads')]
INTERESSA = ['CMRTE']
