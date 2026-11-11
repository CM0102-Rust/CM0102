*English version: [fechamento_inferidos.en.md](fechamento_inferidos.en.md).*

# Fechamento do mapa do mercado: as marcas [inferido] fora das rotinas dos outros leitores (11/11/2026)

Pedido: [BRIEF_FECHAMENTO.en.md](BRIEF_FECHAMENTO.en.md), passos 1 e 4 só (sem porte). Fora deste fechamento (outros
leitores): 0x4d1600, 0x85fc50, 0x864550, 0x4ce4e0, 0x4c7ac0, 0x862720, 0x844780. Uma entrada por afirmação de
`docs/MAPA_MERCADO.md` (linhas do `.en.md`): o texto velho, o novo e a prova (endereços de instrução), ou "segue
inferido: por quê". As desmontagens são de `jogo/cm0102.exe` por `engenharia-reversa/desmonta.py`; os doubles lidos do
exe. "Rust comparado" = rotina já em Rust com 0 divergências na comparação.

Quando a entrada cita os itens `mkb_2` (0x83cb40) e `mkb_3` (0x83f3a0): esses portes foram feitos da desmontagem inteira
(ver os `PRONTO.txt`), e eu conferi na desmontagem os pontos citados aqui; o resto do texto novo é o cabeçalho deles.

## Resumo do que mudou

- **O empréstimo preso existe, e o caminho é outro (sem férias):** um empréstimo aceito pelo dono (estado 8) que o
  jogador recusa porque escolheu outra proposta fica guardado; se essa outra transferência for retirada no estágio das
  confirmações (estado ≥ 0xb) em até 10 dias, o empréstimo volta ao estado 8 com a oferta de contrato marcada como "não
  conta para decidir" (contrato tipo 6). Ninguém mais o decide, `process_due_offer` não tem ramo para 8 e a varredura
  semanal pula 8: fica preso para sempre, e o jogador continua "sob oferta" (C5).
- **Bit 4 de `contract::TRANSFER_STATUS` = preço pedido automático** (o botão "Auto" da tela "Set Staff Transfer
  Status"); bit 8 = listado a pedido do jogador (A4, B3).
- **Estados do elenco (nibble alto de +0x4f):** 0 Undefined, 1 Key, 2 First Team, 3 Rotation, 4 Backup, 5 Prospect,
  6 Youngster, 7 Not Needed (A4).
- **O `edi` do filtro de jovens (0x8400ae) é id da pessoa + id do clube**, fixo por par: o mesmo jovem é sempre
  conhecido ou sempre ignorado pelo mesmo clube (D1).
- **O campo +0x10 das transferências futuras = 1 quando a pessoa sai do clube, 0 quando chega** (A5).
- Correções de 0x83cb40/0x83f3a0 (B4, B5): lesão 150/30/400/300 dias; demissão de comissão só quando a lista volta de
  ACTIVE 2; o clube B (não o "sem parceiro") sai logo; o "quem o clube conhece" refeito.
- O presidente entra em 0x83cb40/0x83f3a0, mas só pelas chamadas (B8).
- 0x4dd0a1 não é do `save_to_disk`: é o diálogo "Transfer Request" (B2).

---

## Seção A

### A1 (linha 47) Os lances da IA todo dia da janela
- **Velho:** "The computer's bids run every day of the window for every computer club that buys **[inferred]** from
  the callers."
- **Novo:** O orçamento é calculado a cada lance montado (`TRANSFER_OFFER::build_bid` 0x8e5d60, que chama 0x598800 em
  0x8e6330) e na conferência das restrições de cada oferta (0x8cfe2d). Os lances novos da IA saem só de
  `make_club_bids` 0x84a020 (chamada única em 0x5cdd6d, no laço do dia), no ritmo da seção B §2 (todo dia ou só
  quando (random(2) + id) % 7 = 0), **sem olhar a janela**: o motivo 9 (fora da janela) de `can_club_bid_for_player`
  é aceito (as ofertas fora da janela ficam guardadas por 0x8c4e10). **[provado]**
- **Prova:** refs de 0x598800 (37 sítios) e de 0x8be300 `add_transfer_offer` (0x84a728 em 0x84a020; os outros são
  revisão 0x8c0810/0x8c0f40, empréstimo 0x8bed60, 0x8bf060 e telas); 0x84a020 em Rust comparado (cqe_3), o cabeçalho
  diz "the club can reach him (`can_club_bid_for_player`, or reason 9)"; o ritmo em 0x5cdc6e..0x5cdd6d.

### A2 (linhas 108-111) Saldo negativo, não-jogador
- **Velho:** "For a non-player, V × 0.65 (what looks like the intent) **[inferred]**."
- **Novo:** Sem ficha de jogador, V × 0.65 **[provado]**: 0x59970b (+0x61 nulo) → 0x599759 `fild V` → 0x59975d
  `fmul [0x96a5d0]` (= 0.65, lido do exe) → `_ftol` 0x599763. Com ficha, a razão de reputação (0.15 [0x968f20] a 0.85
  [0x969520]) entra no lugar de V (0x599712..0x599757) e é multiplicada pelo mesmo 0.65. Segue inferido só que a
  intenção fosse V × razão × 0.65 (intenção não se prova pelo código).

### A3 (linha 116) Clube sem liga
- **Velho:** "**[inferred]** Step 4 returns the budget with no reserve for a club without a league (background clubs
  with no competition)."
- **Novo:** O passo 4 está provado: `club::DIVISION` (+0x57) nulo (0x598af3..0x598afc) ou sem objeto em
  `G_COMP_OBJECT_TABLE` (0x598b02..0x598b0d) salta para o retorno 0x5997ec com o valor do passo 1. O mesmo teste é o
  que a lista de compras usa para "liga não carregada" (0x83cc68..0x83cc86). **Segue inferido:** quais clubes são esses
  numa carreira dada (o número depende das ligas carregadas; não contei).

### A4 (linha 143) Bit 4 de TRANSFER_STATUS e os estados 0x10/0x50
- **Velho:** "or without bit 4 of `contract::TRANSFER_STATUS` (+0x4e) **[inferred: the manager set the price]**: the
  floor. By squad status (… & 0xf0): 0x10 and 0x50 the price; 0x20 and 0x30 the mean …" (e, no item mka_1, "0x10 and
  0x50 [inference: key players]").
- **Novo:** **Bit 4 = preço pedido automático** **[provado]**. Sem ele o técnico fixou o preço à mão, e `staff::VALUE`
  (+0x52) é esse preço. Os estados: **0x10 Key e 0x50 Prospect** dão o preço calculado; **0x20 First Team e 0x30
  Rotation**, a média; 0 Undefined, 0x40 Backup, 0x60 Youngster e 0x70+ Not Needed, o piso. **[provado]**
- **Prova do bit 4:**
  - A tela "Set Staff Transfer Status" (0x8f6770, transfer_screens.cpp): o controle 0xa nasce do bit 4
    (0x8f6923..0x8f6936: `and edx, 4; shr edx, 2`); o botão "Auto<…automatic asking price>" (texto em 0x8f6fe9) é o
    evento 5 do tratador 0x8f7640, que inverte o controle 0xa e, ao ligar, põe no controle 1 (o preço) o
    `calc_player_value` 0x4d0c60 (0x8f772b..0x8f77bb); mexer no preço (evento 0xb) desliga o controle 0xa
    (0x8f780d..0x8f781e); na confirmação o controle 0xa vira o bit 4 (0x8f80b0..0x8f80da).
  - O contrato novo nasce com +0x4e = 4 (0x866f14, em 0x866ee0, chamado por 0x4cfb13/0x4cfccb/0x4cfef4).
  - Com o bit 4, o dia (0x4c7f9e..0x4c7fbe) e 0x84ae17..0x84ae30 recalculam `staff::VALUE` por 0x4d0c60; sem ele,
    não.
  - `set_player_squad_status` 0x8ce0e0 (só telas: 0x8efc88, 0x8f08b5, 0x8f0902, 0x8f0af6, 0x8f7fbc) grava o preço em
    +0x52 (0x8ce1e9) e apaga o bit 4 com o 8º argumento ≠ 0 e o clube fora do estado de crise (0x8ce8af..0x8ce8bf).
- **Prova dos estados:** 0x867f6d..0x867ffa (staff_contracts.cpp) lê `+0x4f >> 4` e salta pela tabela 0x868000
  (`0x867f81 Undefined, 0x867f90 Key, 0x867f97 First Team, 0x867fa6 Rotation, 0x867fb5 Backup, 0x867fbc Prospect,
  0x867fcb Youngster`; acima de 6, "Not Needed" em 0x867fda). Consequências no mapa: o "status 6 ou 7" de 0x84af00 é
  Youngster/Not Needed; 0x83cb40 põe Youngster (6) abaixo de 20 anos e Not Needed (7) no resto.

### A5 O campo +0x10 das transferências futuras (pedido do coordenador)
- **Velho:** §1 passo 3, "27-byte rows: person +8, sale +0x10, fee +0x14, stage +0x19" (no mka_1: "nonzero when the
  club sells the person [inference]").
- **Novo [provado]:** +0x10 = **1 quando a pessoa sai do clube consultado, 0 quando chega**. Em
  `get_club_future_transfer_list` 0x8ccb80, para cada oferta aberta do clube (0x8ccbf3..0x8ccc1b, as que ele fez; mais
  as recebidas por 0x8c6100 em 0x8ccc4d; puladas as de empréstimo, `offer::OFFER_TYPE` +0x1f = 1, em 0x8ccc78), a
  linha do alvo (0x8ccdb7..0x8cce4d): +8 = o alvo
  (`offer+0xc`); se o clube é o comprador (`[clube] = offer+8`, 0x8ccdca..0x8ccdcf), +0xc = o clube atual do alvo (−1
  sem) e **+0x10 = 0** (0x8ccdda/0x8cce02); senão +0xc = o comprador e **+0x10 = 1** (0x8cce0b..0x8cce0e); +0x14 = a
  taxa (`offer+0x1b`); +0x18 = 4 pré-contrato sem taxa (completamento 3/4), 1 com troca, 0 senão; +0x19 = o estágio
  pela tabela 0x8cd134/0x8cd114 (1 a 6, 0 fora); +0x1a = `offer+0x2d`; +0 a data. As linhas dos jogadores de troca
  (0x8cce89..0x8ccf33) invertem: com o clube comprador, +0x10 = 1 (ele cede o jogador de troca), senão 0; taxa 0, +0x18
  = 1. Em 0x598800 (mka_1), +0x10 = 0 soma a taxa às compras; ≠ 0 entra no ramo da venda nos estágios 3 a 6.

## Seção B

### B1 (linha 296) O % 7
- **Velho:** "[proved; effect: inferred, most likely meant "once a week"]".
- **Novo:** O teste está provado (0x5cdc6e..0x5cdc7b: random(2) uma vez por dia, antes do laço; 0x5cdd32..0x5cdd42 e
  0x5cdd58..: (r + id) % 7). O efeito está provado para a lista de compras: os lances novos e os pedidos de
  empréstimo da lista só saem de 0x84a020 (0x842740 tem um chamador só, 0x84ab5a, dentro de 0x84a020; os lances novos
  da IA, 0x84a728). Esses clubes ainda podem receber/responder ofertas e rever as suas. **Segue inferido:** a
  intenção ("uma vez por semana").

### B2 (linha 318) 0x4dd0a1 no `save_to_disk`
- **Velho:** "One in `CONTRACT_MANAGER::save_to_disk` 0x4dc3a0 (0x4dd0a1) is odd: maybe the map's boundary
  [inferred]."
- **Novo:** É a fronteira do mapa da comunidade **[provado]**: o intervalo de 0x4dc3a0 (3408 bytes) cobre várias rotinas;
  0x4dd0a1 está no tratador 0x4dcd70 do diálogo **"Transfer Request"** (construtor 0x4dcb30; os dois registrados em
  0x4dcab5/0x4dcaba; textos "Put <…> on the transfer list at his own request ?", "Yes", "No"). No "Yes": tira a
  lista de empréstimo (0x4dd060..0x4dd069), **liga o bit 8** de +0x4e (0x4dd06c..0x4dd075, logo bit 8 = listado a
  pedido), evento 0xc (0x544710), vitrine forçada (0x842c00 com 1, em 0x4dd0a1) e 0x4d71b0.

### B3 (linha 323) Outras formas de escrita do bit 2
- **Velho:** "writes through other instruction forms were not scanned: inferred there are none".
- **Novo:** Varredura de todo o `.text` por escritas de **byte** em `[reg + 0x4e]` (mov/or/and/add, imediato ou
  registrador) **[provado]**: o bit 2 só é ligado em 0x83dd42/0x83dd98, 0x83e49c, 0x83e651 (0x83cb40) e 0x8442c4
  (0x8442b0, telas). As outras escritas por registrador são cópias de contrato (0x4cc6dc, 0x4cc903, 0x4ce39f, 0x4cf9d8,
  0x4cfc10, 0x4cfdb9, 0x4d000f, 0x4d0546, 0x4d06a5, 0x5fbb51), limpezas (0x83d287 `& 0xfc`, 0x8442ef `& 0xfd`,
  0x4d8747/0x4dd069/0x784848/0x8f80a1 `& 0xfd`, `& 0xf7`, `& 0xfe`), zeros de construtores (0x85e458, 0x85e4f4,
  0x85e6a4, 0x85e9db, 0x866eeb) ou mexem nos bits 1, 4 e 8. **Segue inferido:** as escritas mais largas que cobrem +0x4e
  (265 `mov dword/word` em +0x4b..+0x4e em todas as estruturas) não foram olhadas uma a uma.

### B4 `assess_club_squad` 0x83cb40 (§3.1, passos marcados [inferred])
Prova geral: desmontagem 0x83cb40..0x83f3a0 (3168 linhas), lida inteira para o item `mkb_2`; conferi os pontos
abaixo.

- **Passo 1 (velho: "with a linked B team … without the partner, leaves [inferred; the call at 0x83cc04 proved]").**
  Novo **[provado]**: DAYS = 0 (0x83cbc4); o reset é forçado quando o +0x23 da vaga 0 é 0xff (0x83cbb4..0x83cbbb); sai
  com ACTIVE 0 (0x83cc16) para seleção (0x53b900), sem nação, humano com `daily` (0x600a50 com 0, 1), `PROFESSIONAL_STATUS`
  3, ou quando 0x540a50 acha o par e diz que este clube **é o B** (0x83cc04..0x83cc14: retorno ≠ 0 e o indicador de
  "é o A" 0): quem cuida da lista é o A.
- **Passo 3 (velho: "Without reset: the slots are only sorted (qsort 0x945333) [inferred]").** Novo **[provado]**:
  `qsort(vagas, 40, 0x25, 0x83b6c0)` em 0x83cfe5..0x83cff1 (ordem pelo índice da vaga).
- **Passo 4 (velho: vagas sem alvo … [inferred]).** Novo (cabeçalho do mkb_2, 0x83cff9..0x83d1c7): a vaga sem alvo, ou
  com alvo sem oferta (`OFFER_REF` −1) e com a data passada ou aposentado (cargo 0x10), perde alvo, substituto e teto
  (o alvo, se está nos pares da comissão, recebe o evento 3 em 0x843e10); a posição vai para a lista do seu estado
  (2 e 3, 4, 5).
- **Passo 5, o fim (velho: "Injured … for more than 30/300/400 days (by the rating 0x6e6340 against the target rating
  × 0.75) goes to the reserves; the others go to one of two lists [inferred]").** Novo **[provado]**: o limite é
  **150 dias** (0x83d32d); **30** quando random(nível de recaída 0x62ece0 + 1) ≠ 0 (0x83d33e..0x83d34b); senão, lesionado
  com nota (0x6e6340) ≥ alvo: **400** (0x83d37f..0x83d387); nota ≥ ¾ do alvo (0x969040 = 0.75): **300**
  (0x83d391..0x83d3a8). Dias restantes ≥ limite, ou anúncio de aposentadoria (0x7a7ed0), vai para os reservas
  (0x83d3f4; com time B só acima de 300 dias, 0x83d3d8..0x83d3ea) e, no clube humano, perde o estado (0x83d40e). Os
  outros vão para o resto ou para a lista dos emprestados a outro clube (0x4cf6d0 em 0x83d41b), e com `reset` a
  felicidade vai a 0.
- **Passo 6 (velho: "those older than goalkeeper/10 + 22 go to the end [inferred]").** Novo (mkb_2, 0x83d474..0x83d8ac;
  as três chamadas de 0x6e2000 em 0x83d512, 0x83d636, 0x83d7f9): o resto vai para as posições de estado 2 e 3, depois
  4; os emprestados a outro clube entram no resto; os mais velhos que `player_sheet::GOALKEEPER` / 10 + 22 (24 para um
  goleiro nota 20, 22 para os outros) saem dele (o trocado é olhado de novo); o resto vai às posições de estado 5; os
  velhos voltam.
- **Passo 7, "the rest" (velho: "[inferred as a whole; the marked points proved]").** Novo (mkb_2, 0x83d8bf..0x83de6b):
  o limite é ftol((idade × 0.13 − 1.98) × base × 0.02), o corte base/68 − 35 e o nível ftol(reputação × 0.25 + PA ×
  12.5 + CA × 12.5 + reputação do clube × 0.25); "sobra" = julgado abaixo do limite (0x6e86d0 modo 1, 0x83da02) e
  contrato tipo 2 com menos de profissionalismo + pressão + 450 dias no clube ou fim a 450 dias ou mais, ou
  `staff::VALUE` ≥ reputação × 50, ou o valor ≥ os salários que faltam (dias × salário / 30). Os pontos já provados no
  mapa ficam (0x83dba7..0x83dc42, 0x83db6b, 0x83dc47..0x83dc66, 0x83dc88, 0x83dce6..0x83ddb2). Acrescenta: com time B
  ou no clube humano, abaixo de 27 (goleiro 20) ou 23, o estado 5 quando ¾ do nível passa o limite + 20, senão 6.
- **Passo 8 (velho: "the contract's status from the slot's (except 1 and 2) [inferred]").** Novo (mkb_2,
  0x83e185..0x83e66f): o substituto é o escolhido; sem técnico de férias o contrato recebe o estado da vaga, menos na
  lista 2/3 sobre os estados 1 e 2; a data da vaga sai da chegada dele (+120 dias, com `reset` e técnico). Estado 4:
  na vaga 24, com time B ou no humano e fora dos grandes, o escolhido (não o técnico) vai para os reservas. Os
  empréstimos das listas 4 e 5 já estavam provados.
- **Passo 9 (velho: "[inferred]").** Novo (mkb_2, 0x83e66f..0x83ebfb): p = min(alvo, 36000) (× 0.75 numa vaga de
  goleiro sem substituto). Sem substituto: valor p e preço p × 2.5 + reputação, 2p + rep., p × 1.8 + rep., p × 0.8 ou
  p × 0.5 pelos estados 1, 2, 3, 4/5, outro (× 3 numa vaga de goleiro). Com substituto: vai ao time (2, 3, 4; com
  `reset` felicidade + 50, 10 jogos de fora) ou aos reservas (5) e recebe nota para a posição (nos primeiros 200 dias ×
  (1.25 − dias × 0.00125)), entre −10000 e 60000; nota ≥ p, sem queixas 0x100 ou 1 e mais de 364 dias de contrato: ele
  fica como alvo. Senão a felicidade muda ftol((1 − nota/p) × −100) (com `reset`; estados 2 e 3 ao menos 25 e a vaga o
  perde; os outros ao menos 0; estado 5 recebe o estado 6), e o preço (rep. × f + p × k) − nota × k.
- **Passo 10 (velho: "below it and with no offer (0x4cc9e0), sacked … [inferred]").** Novo **[provado]**: a demissão
  só acontece quando o indicador local [esp+0x60] está ligado (zerado em 0x83cb6c), e ele só liga em 0x83cc44..0x83cc64:
  **a lista estava em ACTIVE 2, sem lances, e o clube não é humano** (0x600a50 com 1, 1). Aí, nota de
  `score_staff_for_job` 0x69d1b0 abaixo de min(alvo/5, 5000) e sem oferta de contrato (0x4cc9e0): `remove_staff_from_club`
  0x690890 (0x83ee28..0x83ee72; igual para as outras 4 funções em 0x83ef46, 0x83f056, 0x83f15f, 0x83f275). Sem isso,
  preço (limite − nota) × 2.5.

### B5 `fill_shortlist_slot` 0x83f3a0 (§3.2, "[inferred except where marked]")
Prova geral: desmontagem 0x83f3a0..0x840d80 (2048 linhas), lida inteira para o item `mkb_3`; texto novo = cabeçalho
do mkb_3; conferi os pontos abaixo.

- **Passo 1 (saídas).** Confirmado pelo mkb_3: sem nação; humano (0x600a50 com 0, 1); humano (1, 1) de férias com a
  regra 4 = 1; vaga com oferta; seleção.
- **Passo 3 (fator de reputação).** Correção: reputação × 0.0008; na ITA, ENG, GER, FRA e ESP com finanças, pela faixa
  de riqueza (0x5986b0): faixa 1, × 1/1175 (× 1/1125 acima de 6250); faixa 2, × 1/1125 (× 1/1050 acima de 4750).
- **Passo 4 (filtros).** Correções **[provado]**: estado 5 (`(+0x24 & 0x1e) = 0xa`) e idade > 23 sai (0x83fb79..0x83fb8d);
  compra: teto (+7) menor que preço × 1.3 (0x968de8) sai (0x83fb97..0x83fbad); empréstimo: passa com clube e preço ≤ 0,
  senão precisa estar listado para empréstimo (0x83fbb8..0x83fbd7). Mais (mkb_3): id ≥ número de pessoas − 16;
  `job_for_club`/`job_for_nation` 5; um humano de férias com a regra 4 = 2 só considera a lista de observação dele;
  anúncio de aposentadoria (0x83fb53..0x83fb5d); saudita (0x83fb63..0x83fb73). Não-jogador: o resto de um contrato
  tipo 2 (dias × salário / 7) ≥ teto ou > (reputação + 500) × 10 sai. Empréstimo de humano: só listado (bit 2,
  0x83feb8).
- **Passo 5 ("whom the club knows").** Reescrito **[provado: 0x83fed8..0x8400d4]**: conhecido quando (0x83fed8) a lista
  do olheiro humano o tem, ou (0x83feea..0x83fefd) (id da pessoa + id do clube) % k = 0. Senão, de **outra nação**
  (0x83ff03) sem jogos pela seleção (+0x22, 0x83ff11) e cujo clube não é da nação do clube (0x83ff1c..0x83ff26): um
  jogador de menos de 23 anos estrangeiro para o clube (0x53d980 com 1) e reputação mundial (+0xd) < 3250 sai
  (0x83ff2c..0x83ff4f); de outra região (+0x75) precisa reputação do clube ≥ 2750 e (clube dele sem nação ou da mesma
  região, ou reputação do clube ≥ 4250, ou `league_standard` +0x85 da nação ≥ 12) (0x83ff55..0x83ffa3); e então a
  reputação vista pelo técnico (0x5407e0) ≥ 2750 + random(2000) nas vagas de estado 5/6 (com reputação mundial ≥ 1750)
  ou ≥ 3250 + random(2000) nas outras (mundial ≥ 2750) (0x83ffa9..0x840058). Da **mesma nação** (ou com jogos pela
  seleção, ou num clube da nação; 0x840060): não-jogador, vaga de empréstimo, sem ficha, reputação em casa (+9) ≥ 3250,
  clube dele ≥ 3250 ou 20 anos ou mais: conhecido; senão o filtro de jovens (D1).
- **Passo 6 (nota e preço).** Correção (mkb_3): abaixo da avaliação ou da nota a bater − 10000 sai; compra num vendedor
  em crise: no máximo o valor do vendedor; troca por compensação (0x84b6f0): a taxa dela; senão o preço pedido
  (0x4d42d0 humano em 0x8402e6, 0x4d1600 IA em 0x8402fe); estado 4 abaixo de 28 ou estado 5: o excesso sobre o valor
  soma min(pedido − valor, (rep./13000 + 0.5) × valor, 2.500.000) à oferta; a previsão 0x84af00 = 2 sai; preço acima do
  teto sai; nota × y, y = 1 − (pedido − valor)/(preço máx. × fator) (acima de 1 → y/3 + 0.67; 0.01..3); lesão > 15
  dias: − (dias − 15)²; empréstimo: previsão do dono 2 sai, lesão > 10 dias − (dias − 10)² × 200; sem
  `can_club_bid_for_player` sai; por fim nota × (92 + random(17))/100.
- **Passo 7 (vontade e salário).** Confirmado com acréscimos (mkb_3): vontade 0x846920 (não-jogador: 0x844780 em
  0x840682); notícia de interesse com random(79 − rep./150) = 0 e (rep. > 5000 + random(2000) ou clube humano);
  vontade ≥ 3.15: salário desejado (0x864550 em 0x840893, como f32) × max(3.5/vontade, 0.9) acima de 1.2 × teto
  (0x5968d0 em 0x84091d) sai; senão é o melhor até ali. Notícia do olheiro (0x587220) por sorteio.
- **Passo 8 (vaga de empréstimo).** Confirmado com o detalhe (mkb_3): só com o indicador da temporada (nação-chave:
  entre o início − 28 e o fim − 28 da temporada; outras nações sempre; vaga de não-jogador nunca), e a liga da nação
  carregada **ou** random(reputação) > random(20000).

### B6 (linha 375) WORK_RATE na lista 4
- **Velho:** "[code proved; intent inferred]".
- **Segue inferido (intenção).** O código está provado: `movsx edx, byte [eax+0x44]; cmp edx, 0xcb2` em
  0x83e482..0x83e486 (sempre menor). Que o pretendido fosse `WORLD_REPUTATION` (+0xd, palavra) não se prova pelo
  código.

### B7 (linhas 392-394) `chance_to_know` com 6 e 5
- **Segue inferido (intenção).** O código está provado (mkb_3, 0x83f4d8..0x83f5b8).

### B8 (linha 502) Presidente e diretores em 0x83cb40 e 0x83f3a0
- **Velho:** "not read … [inferred for 0x83cb40 and 0x83f3a0 from the pseudo-C]".
- **Novo:** **Não são lidos diretamente** **[provado]**: nenhum acesso a `[clube + 0xbf]` ou `[clube + 0xc3]` em
  0x83cb40..0x83f3a0 nem em 0x83f3a0..0x840d80. **Mas entram pelas chamadas**: 0x83cb40 chama
  `update_club_target_reputation` 0x83bf40 (0x83cda7), que lê a Paciência (+0x1d) e os Recursos (+0x20) do presidente
  (seção D §3); 0x83f3a0 chama `get_max_wage` 0x5968d0 (0x84091d: Negócios +0xf e Recursos do presidente) e o preço
  pedido 0x4d1600 (0x8402fe, 0x840bfb).

### B9 (§6, itens 2, 3, 4 e 5) As intenções
- **Segue inferido (intenção)** em todos: o código de cada um está provado; se era o pretendido não se prova pela
  desmontagem.

## Seção C

### C1 (linha 691) O +2 das datas da janela
- **Velho:** "+2 the weekday it moves to (0xff none) [inferred]".
- **Novo [provado]:** `transfer_window_get_date` 0x40a7d0 passa o byte +2 com sinal (0x40a7d1 `movsx ax, byte [ecx+2]`)
  como 4º argumento de `Date::Date` 0x549f70; lá, valor ≠ 0xffff (−1) chama `Date::get_nearest_weekday` 0x54a2d0
  (0x54a0ea..0x54a0fc). Logo +2 = o dia da semana para o qual a data é levada (a mais próxima), 0xff = nenhum.

### C2 (linha 733) O período de zero meses na tela do humano; e a dúvida do 1972 (§12)
- **Segue inferido:** só tela (não muda a carreira automática). O construtor 0x8f9690 põe o período nos controles 3-6 e
  0xc/0xd (0x8f980f..0x8f98ac); o tratador é 0x8fac30 (registrado em 0x8f9747) e a confirmação passa por 0x8fb5e0; não
  segui até o fim onde o fim 1972 ou o período zero viram (ou não) recusa.

### C3 (linha 739) O "58.2" do fórum
- **Segue inferido:** só se prova reproduzindo no jogo.

### C4 (§8) Ofertas paradas nas férias (estados 9, 0xb–0xd)
- **Velho:** "The defect [inferred, not reproduced] …".
- **Novo:** o mecanismo está provado; segue inferido só o efeito no jogo (não reproduzido). `process_due_offer` não
  compara 8, 9, 0xb, 0xc, 0xd (as comparações de estado são as de 0x8c6708..0x8c6972 e não há tabela de saltos); a
  varredura semanal pula 0x13, 7, 8 e 0xa (0x8c5964..0x8c5973). Escritores (varredura de `mov byte [reg+0x2c], imm`):
  9 só em 0x8c6a8c (`process_due_offer`, comprador humano); 0xb em 0x8c31e7 (0x8c2f50); 0xc/0xd em 0x8c3310
  (0x8c336e/0x8c335b). Escritas por registrador não varridas.

### C5 (§8, "And loans?") O empréstimo preso
- **Velho:** "The only one that does not lapse is 8 … a loan whose player decision never comes stays stuck. That this
  is xeno's item 88 is **inferred**; the exact path that loses the decision was not found."
- **Novo [provado, caminho de código; não reproduzido no jogo]:** um empréstimo pode ficar preso para sempre no estado
  8, sem férias:
  1. O dono (IA) aceita: `contract_manager_schedule_decision_for_transfer` 0x4cc930, estado 8 e **nenhum
     reagendamento** (0x8c6577..0x8c6592 em `process_due_offer`; 0x8c0680/0x8c068b em `accept_loan_offer`). 0x4cc930
     marca a oferta de contrato (pessoa, clube) que tem +0x62 = 0 e agenda a decisão (0x4cc975 é a única escrita de 1
     em +0x62 fora de `add_contract_offer`).
  2. A oferta de contrato do empréstimo é tipo 6 (`init_from_current_contract` 0x85e590: `& 0xc6 | 6`), e
     `add_contract_offer` 0x4cc490 grava +0x62 = (tipo ≠ 6): 0 para o empréstimo (0x4cc597..0x4cc5b7 na oferta nova;
     0x4cc741..0x4cc761 na atualizada).
  3. A decisão do jogador (0x4ce4e0) só olha ofertas com +0x62 ≠ 0 (0x4ce63c..0x4ce641). Se ele escolhe **outra**
     proposta ligada a uma transferência, a do empréstimo vira estado 1 e `handle_player_rejected_terms` 0x8c28c0
     (0x4ced1b..0x4ced32: a transferência vai a 0xf, rodada 3, volta em 10 dias); e a oferta de contrato do empréstimo
     é **guardada** na lista +0x38 (0x4cf0b5..0x4cf13d, clube diferente do escolhido e escolhido com vaga ≠ −1) e sai
     da lista viva (0x4cf35d..0x4cf383).
  4. Se a transferência escolhida é **retirada** com estado ≥ 0xb (`withdraw_transfer_offer` 0x8bfcb0, 0x8bfd3a; único
     chamador de 0x4cf4a0, em 0x8bfd4a) enquanto o empréstimo ainda existe (os 10 dias do passo 3),
     `resubmit_held_offers` 0x4cf4a0 põe a transferência de 0xf em **8** (0x4cf4fb..0x4cf501) e reenvia a oferta com
     `add_contract_offer(…, decide 1, combinado 0)` (0x4cf505..0x4cf52a): tipo 6, então +0x62 = 0.
  5. A decisão agendada pula essa oferta (passo 3); 0x4cc930 não volta a ser chamado (só nas aceitações do estado 0);
     `process_due_offer` não tem ramo para 8; a varredura pula 8. **O empréstimo fica em 8 para sempre** e o jogador
     segue "sob oferta" (o bit 0 do registro de 8 bytes só é limpo com as ofertas resolvidas, e 8 não conta).
  Os outros estados de empréstimo caducam (0 com dono humano: estado 4, 14 dias; 1 com comprador humano: estado 4, 10
  dias; 0xf: removido na volta com rodada 3). A decisão também é adiada de 2 em 2 dias enquanto o jogador tiver outra
  oferta em 0, 1, 5 ou 6 (`player_has_pending_offer` 0x8cd150 em 0x4ce5ce), mas isso termina quando a outra caduca.
  **As férias não entram** nesse caminho (nem 0x4ce4e0 nem 0x4cf4a0 olham o humano). Segue inferido que seja o item 88
  do xeno. **Correção pequena:** em `resubmit_held_offers`, para uma transferência de empréstimo (`offer_type` 1),
  chamar 0x4cc930 depois do reenvio (ou gravar +0x62 = 1), ou no `process_due_offer` (Rust) tratar 8 vencido de
  empréstimo como recusa.

### C6 (linha 791) Fim natural do empréstimo
- Fora deste fechamento: a chamada está em 0x4c7ac0 (outro leitor).

### C7 (linha 830) O laço de `ENGLAND_RULES::can_loan_player`
- **Velho:** "tests the id of the target `player`, not of the squad player … [inferred]".
- **Novo [provado]:** em 0x579ea8..0x579ec2 o teste lê `[esp+0x84]` (o 2º argumento, o jogador-alvo, também lido em
  0x579d65) e não `esi` (o jogador da vaga); com um alvo de id válido o teste sempre passa, sem efeito.

### C8 (§12) As dúvidas
- 1972 na tela: ver C2 (segue inferido).
- Qual estado prende o empréstimo: respondido em C5.
- Tipos 3 e 4 nos dados: **segue inferido**; precisa varrer os contratos de um save ou do banco (fora da desmontagem).

## Seção D

### D1 (linha 1019) O `edi` do filtro de jovens em 0x8400ae
- **Velho:** "goes on only if edi mod (25 − youngsters) = 0 … [the test proved; what edi is, inferred: a counter or
  id]".
- **Novo [provado]:** `edi = id da pessoa + id do clube` (0x83fee7..0x83feee: `ebx` = clube de [ebp+0xc], `lea edi,
  [eax+ecx]` com `[esi]` e `[ebx]`), a mesma soma do teste % k (0x83fef1..0x83fefd); nenhuma escrita em `edi` entre
  0x83feee e 0x8400b9 em nenhum dos três caminhos que chegam a 0x840060 (0x83ff0b, 0x83ff16, 0x83ff26). Logo é **fixo
  por par (pessoa, clube)**: um mesmo jovem é sempre aceito ou sempre ignorado pelo mesmo clube enquanto o técnico for o
  mesmo; Youngsters 20 aceita os pares com soma múltipla de 5. Condições exatas (0x840060..0x8400d4): vaga de jogador
  não de empréstimo, com ficha, reputação em casa (+9) < 3250, clube dele nulo ou < 3250, idade < 20; técnico com ficha
  de não-jogador: % (25 − `YOUNGSTERS` +0x22) (0x8400ae..0x8400c0), senão % 15 (0x8400c8..0x8400d4). E só chega aqui
  quem já falhou no % k.

### D2 (linha 1020) Youngsters em 0x4d3ccb
- Fora deste fechamento (0x4d1600, outro leitor).

### D3 (linha 1049) `calc_contract_value`, caso 0
- **Segue inferido (intenção).** O código está provado (mkd_1).

### D4 (linha 1053) `get_max_wage` "parece invertido"
- **Velho:** "looks inverted [inferred]".
- **Novo:** a regra está provada; o "invertido" **não se sustenta pelo código**: nas duas rotinas do dinheiro um
  presidente de Negócios alto gasta menos: no orçamento guarda mais semanas de salário de reserva (Negócios + 6,
  0x598b24..0x598b2a) e no teto salarial paga × 0.95 (0x597a0d). É um presidente prudente, coerente nas duas. Se era o
  pretendido segue sem prova.

### D5 §2.2, §2.3, §2.4
- Fora deste fechamento (0x4ce4e0, 0x85fc50, 0x864550, 0x862720, 0x4c7ac0).
