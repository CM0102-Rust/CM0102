# O sorteio das finalistas do futebol olimpico (olympics.cpp; leitor cx07, rodada 5, 24/11/2026). Simulacao com as 8
# ligas e ATE 2004-11-01: o sorteio e em 20 de marco de 2004 (Olympics::update_for_date, ja em Rust pelo OLYMPICS_DAY);
# 0 chamadas na cobertura do mundo ate 2002-08-01.
# - 0x7a09e0 Olympics::draw_finalists thiscall() ret 0 -> 1/0 (2016 bytes, sozinha no item): +0xf2 = 16; em 2000 a lista
#   fixa das 16 de Sydney (database_get_national_u21_team 0x53b4f0, cdecl, push nacao; add esp no fim); nos outros anos
#   malloc 0x944e46 (cdecl) do reservatorio, HostCountry::find_hosts 0x5fa730 (push 1; push &segundo; push &primeiro;
#   push ano; push competicao [0x9cf7a0]; mov ecx, [0xb5e134] -> thiscall, ret 0x14; devolve quantas, byte com sinal),
#   as nacoes na ordem da tabela sem as de [0x9cf45c], [0x9cf2e4], [0x9cf510], [0x9cf2dc], [0x9cf3e4], [0x9cf408],
#   [0x9cf4a8] nem as de regiao (+0x75) 6 nem as das sedes; qsort 0x945333 (cdecl, 4 pushes) a partir da segunda com
#   0x4b07d0; a lista de 16 pelas cotas continentais ([0x9cfa10] 5, [0x9cfa08] 8, [0x9cfa14] 10, [0x9cfa1c] 12,
#   [0x9cfa0c] 15, [0x9cfa18] 16, ate cada marca); free 0x9452ca (cdecl); a lista reordenada nos grupos (0, 5, 9, 13 |
#   1, 6, 10, 14 | 2, 7, 11, 15 | 3, 8, 12, 4). Caixas nas linhas 0x337, 0x356, 0x38f e 0x3dd. Prologo sub esp, 0x214
#   (6 bytes).
# Defeitos do original: (1) os dois indices de sede sao locais nao iniciados (o find_hosts grava os que devolve; a copia
# inicia com -1, sem efeito); (2) com menos de 16 nacoes que caibam nas cotas, as vagas que sobram ficam com o lixo do
# malloc e viram times do torneio; (3) sem memoria para a lista (linha 0x38f) o reservatorio nao e liberado; (4) com o
# reservatorio vazio o qsort recebe a contagem -1.
# Iguais no cm0102-1080.exe (inteira).
# Depende do cx07_6 (OLYMPICS_SOURCE) e do cx07_7 (OLYMPICS_DRAWN_CLUBS, OLYMPICS_DRAWN_COUNT): integrar depois deles.
# Comparador: lado a lado (o sorteio nao usa numeros aleatorios): a original sorteia, a copia sorteia de novo sobre a
# mesma competicao, as duas listas de 16 e os retornos iguais; a lista da copia e liberada e a da original volta.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('olympics_draw_finalists', 0x7a09e0, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('o sorteio das finalistas do futebol olímpico',
        'the draw of the Olympic football finalists',
        [('olympics_draw_finalists', 'Olympics::draw_finalists', '0x7a09e0', 'olympics.cpp', 'thiscall, ret 0',
          'escolhe as 16 seleções sub-21 do futebol olímpico: em 2000 a lista fixa de Sydney; nos outros anos as sedes primeiro e depois todas as nações (menos as britânicas, a Noruega, a Suécia e as da região 6) por reputação, com 5 vagas da Europa, 3 da África, 2 da América do Norte, 2 da América do Sul, 3 da Ásia e 1 da Oceania; a lista é cortada nos quatro grupos (vagas 0, 5, 9, 13 no A; 1, 6, 10, 14 no B; 2, 7, 11, 15 no C; 3, 8, 12, 4 no D)',
          'picks the 16 under-21 nations of the Olympic football: in 2000 the fixed Sydney list; in other years the hosts first and then every nation (except the British associations, Norway, Sweden and those of region 6) by reputation, with 5 places for Europe, 3 for Africa, 2 for North America, 2 for South America, 3 for Asia and 1 for Oceania; the list is cut into the four groups (slots 0, 5, 9, 13 in A; 1, 6, 10, 14 in B; 2, 7, 11, 15 in C; 3, 8, 12, 4 in D)')])
CAMPOS = [('nação +0x75 região: 6 fica fora do futebol olímpico [inferência]', 'nation +0x75 region: 6 is left out of the Olympic football [inference]')]
INTERESSA = []
