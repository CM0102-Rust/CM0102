# sbD: o que ficou para a próxima rodada

Leitor sbD, rodada 5, 24/11/2026. Estas três rotinas da lista (as sobras do cx08 e do cx09) não viraram item: o tempo da rodada acabou nos textos, nos calendários e nos promédios. Nenhuma está portada. As notas vêm da desmontagem e poupam a releitura.

## ArgSecond (arg_second.cpp), com a Argentina (`LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'`, `ATE = '2002-08-01'`)

- **0x408630 `ArgSecond::create_promotion_cup_stage`** e **0x408910 `ArgSecond::create_promotion_playoffs`**: montam as fases 3 e 4/5 (operator new 0xb2 e Cup_Stage 0x522e00, como o `bel_third_create_title_decider_stage` do cx09_10). O prólogo da 0x408630 é `mov eax, fs:[0]` (6 bytes). As fases que elas montam são lidas pelos registros e pelo texto do sbD_8.
- Os promédios da Segunda (sbD_9) já dizem quem é rebaixado; a copa de acesso usa os vencedores das zonas.

## FinFirst (fin_first.cpp), com a Finlândia (`LIGAS = 'ENG SWE FIN NOR POR'`, `ATE = '2002-08-01'`)

- **0x594080 `FinFirst::create_play_off_stages`**: o mapa diz 196 bytes, mas a rotina vai até 0x59460c (1,4 KB; quadro alinhado `and esp, -8` e SEH). O que ela faz:
  - junta até 50 clubes (do índice 0 até `[0xae235c] − 2 × [0xae2350]`) com nação (+0x53) e divisão (+0x57) numa das quatro chaves 0x9cf7fc, 0x9cf804, 0x9cf800 e 0x9cf808; com menos de 6, a caixa da linha 0x312;
  - `comp_util_sort_and_shuffle_top_clubs` 0x4afbd0 (lista, n, 6), cdecl;
  - preenche os quatro índices de +0xee: até 2 clubes do norte (a latitude, +0x23 em double da cidade (+0x38) do estádio (+0x69), maior que o double de 0x96a4b8), depois do sul até completar 2 (a contagem continua da primeira passada), depois quaisquer até 4; cada clube usado sai da lista (vaga = 0);
  - o par da fase 1: malloc(0x10) (caixa 0x33f) com a primeira linha da tabela daqui, o +0x3b da primeira linha da fase 0, o +0x3b da primeira linha daqui e a primeira linha da fase 0; virtual 0x3c (1, &rodadas, &extra, 0); Cup_Stage na vaga 1 ([+0xc] + 4) com (this, 4, par, rodadas, competição, tabela, ano, 1, 1, extra, 0x14, 0, 0, 0, 0) (caixa 0x352, e a tabela de rodadas não é liberada nesse caminho); free da tabela;
  - o par da fase 2, no mesmo bloco: [0] e [2] = o +0x19d da primeira linha daqui e da fase 0, [1] = o primeiro clube do sul que sobrou na lista, [3] = o primeiro do norte, e os vazios com os que sobraram; virtual 0x3c (2, ...) e Cup_Stage na vaga 2 (caixa 0x393);
  - free da tabela, `Cup::set_up_round` 0x51c800 (0) nas vagas 1 e 2, free do par e +0x30 = 2.
- Comparador sugerido: só a original (aloca e monta fases), conferindo depois os quatro índices de +0xee contra o plano da cópia e as vagas 1 e 2 com a fase atual 2, como os irmãos do cx09_20.
- O sbE_1 (0x595f00) lê os quatro índices de +0xee que esta rotina grava.
