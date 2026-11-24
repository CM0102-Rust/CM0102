# sbG_5: os três amistosos que ficaram de fora (leitor sbG, rodada 5, 24/11/2026)

Das quatro rotinas do `friendly.cpp` deixadas pelo sbF (ver `levas/fila/sbF_12/FORA.md`), o `process_tournament_replies`
(0x5c5820) entrou no sbG_4. As outras três não foram escritas nesta rodada: o tempo foi para os itens sbG_1 a sbG_4
(as sobras do cx14 e do cx15 e o 0x5c5820). O que se apurou, para a próxima rodada:

| Endereço | Rotina | Tamanho | Convenção | Tipo |
|---|---|---|---|---|
| 0x5c75a0 | `Friendly::get_club_date_status` | 3,2 KB (até o `ret 0x24` em 0x5c81c8) | thiscall, ret 0x24, devolve char | simulação (e telas) |
| 0x5c8a00 | `Friendly::load_from_temp_file` | 3,1 KB | thiscall, ret 4 | SAVE (CARGA) |
| 0x5c9650 | `Friendly::save_to_temp_file` | 2,5 KB (até o `ret 4` em 0x5c9f7f) | thiscall, ret 4 (o Ghidra a mostra como fastcall: o argumento não é lido) | SAVE |

## get_club_date_status (0x5c75a0)

Argumentos: (clube, &data, &torneio_saída, &amistoso_saída, &pedido_saída, datas escolhidas, n (palavra), tipo (byte),
recursão). Zera as três saídas; devolve:
- 4 quando a data está entre as `n` datas escolhidas (8 bytes cada, comparadas por dia e ano);
- na primeira passada pelos 127 torneios (0x69 bytes de +0xb1), com o clube entre os quatro convidados (+0xc6): o tipo
  (+0xbd) dá os dias (1 e 10 → 2, 2 → 3, 0/3/4/0xb → 1, outro → a caixa da linha 0xf18 e 1); a janela é [início − 1,
  início + dias); dentro dela, `*torneio = vaga` e 2 + (+0x116 == 0);
- na segunda passada, a mesma janela alargada pelos dias do tipo pedido (outro switch da linha 0xf18 sobre o argumento
  `tipo`): 2;
- a lista de amistosos (+0x34c8, `LLIST::get_next`): um jogo do clube (+0x10 ou +0x14) com a data na janela de um dia;
  no dia exato, sem o adversário livre (`friendly_is_club_free_on_date` 0x5c6b70(&data, outro, 0)), o jogo sai da
  lista (`LLIST::remove` 0x68d1f0) e a rotina chama a si mesma com o último argumento 1; senão 2 nas pontas e
  `*amistoso = nó`, 5;
- a lista de pedidos (+0x34cc, registros de 0x107 bytes): um pedido do clube com uma das datas (+9 + 0x19·k, +0x15 com
  adversário) igual à data: 2 se não for o dia pedido, senão `*pedido = nó` e 6;
- tipo 0xb (o pedido de datas): sem datas escolhidas, livre no dia e dentro de uma semana a partir de hoje (0xae2c90)
  → 1, senão 2; com datas, as janelas de 1 e 3 dias em volta delas, 4 na data exata, e o fim da janela de 7 dias contra
  hoje;
- os outros tipos: não livre → 2; com o último argumento e tipo 0, 1 ou 2, o estádio ocupado (0x5ae870 do estádio
  +0x69) → 2; senão 1 até uma semana a partir de hoje, 2 depois.

O pseudocódigo do Ghidra (`referencias/ferramentas/saida/0x5c6990_..._0x5c9650.c`, linhas 182 a 673) confunde as
cópias de datas (os `Date::operator=` 0x417c10 e os `Date::sub_days`/`add_days` 0x54c8e0/0x54c5b0 com o ponteiro do
resultado): escrever pela desmontagem (`desmonta.py jogo/cm0102.exe 0x5c75a0 0x5c81d0`). Chamadores: as telas
(0x45fcfd, 0x470870, 0x483238, 0x4834ca, 0x4837af), o dia dos amistosos (0x5c2bd0, 0x5c2c10) e ela mesma (0x5c7bd7).
Comparador sugerido: lado a lado no valor e nas três saídas, a não ser no ramo que remove o amistoso da lista (só a
original).

## save_to_temp_file (0x5c9650) e load_from_temp_file (0x5c8a00)

O `Friendly.tmp` em CM3_TEMP (0x90bde0 com "CM3_TEMP", "Friendly.tmp" de 0x9cd0f4 e o modo 0x9a7f28; fopen 0x945b00;
sem arquivo, a caixa da linha 0x135b). A gravação, em ordem (fwrite 0x945ce6):
- +0xa7 (8), +0xaf (2), +0x34dc, +0x34e0 e +0x34e4 (4 cada) e, com contagem e vetor, os amistosos planejados
  (+0x34d8, 0x14 bytes cada);
- os 127 torneios (0x69 bytes cada) com a sede e os quatro convidados trocados pelo id (−1 sem), gravados de uma vez e
  desfeitos depois (id → `G_CLUB_TABLE` + 0x245·id); cada competição criada (+0x65) grava a si mesma pela virtual +0x1c
  (`SAVE_TO_TEMP_FILE`, com 1);
- quatro listas, cada uma com a contagem reservada (ftell 0x946aae, um dword 0), os registros e a contagem
  regravada no lugar (fseek 0x946a22) e o fim (fseek 0, 2): os amistosos (+0x34c8; 0x19 bytes, três clubes em +0xc,
  +0x10 e +0x14 trocados por id; caixa 0x13ee), os torneios pendentes (+0x34d0; o byte +0x64 da vaga; caixa 0x141c), os
  pedidos com resposta (+0x34d4; 8 bytes do nó e o clube em +0xc por id; caixa 0x1457) e os pedidos de datas (+0x34cc;
  0x107 bytes, o clube em +0, a vaga +4 por índice da tabela 0xae23a8 de 0x122 bytes, e o adversário de cada data
  (+0x15, +0x19, +0x1d de cada 0x19) por id);
- o resto (as listas por nação +0x34f6, as datas internacionais +0x34ec e os pares proibidos +0x3502) fica além do
  trecho lido aqui: conferir na desmontagem (`desmonta.py jogo/cm0102.exe 0x5c9650 0x5ca000`).

A carga (0x5c8a00) lê na mesma ordem e é chamada pela vtable (+0x18) e pelo construtor 0x5c1f80 (0x5c2217, que espera o
CD). Os dois entram num item SAVE com CARGA, provados pelo `prova_save.py` (o save de férias tem torneios e pedidos?
conferir antes). Atenção: a gravação troca ponteiros por ids no próprio objeto e depois desfaz; um nó com clube nulo
volta nulo (−1 → 0).
