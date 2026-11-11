*English version: [secao_b.en.md](secao_b.en.md)*

# Mercado, seção B: como a IA compra e vende (11/11/2026, leitor B)

Parte do mapa do mercado pedido pelo Rust Tweaks (release 1.5; [BRIEF.en.md](BRIEF.en.md)). Endereços em VA do
`cm0102.exe` 3.9.68. **[provado]** = lido na desmontagem (`desmonta.py`) ou no código Rust já comparado com a original;
**[inferido]** = lido só no pseudocódigo do Ghidra (`pacotes/0x83cb40`, `pacotes/0x83f3a0`) ou concluído sem prova
direta. Os nomes de campo são os de `rust/src/motor/fields.rs` e `records.rs`.

## 1. As rotinas

| Endereço | Nome (mapa da comunidade) | Em Rust | Item |
|---|---|---|---|
| 0x84a020 | `Shortlist_Manager::make_club_bids` (a IA percorre as 40 vagas, faz lances e pede empréstimos) | sim | cqe_3 (leva 268) |
| 0x83f3a0 | `Shortlist_Manager::fill_shortlist_slot` (escolhe o alvo de uma vaga) | não | não portada (§8) |
| 0x8c8090 | `TRANSFER_MANAGER::ai_respond_to_bid` (resposta do vendedor da IA a um lance) | sim | cqe_1 |
| 0x84af00 | `shortlist_manager_predict_selling_club_response` (aceita, negocia, recusa) | sim | cpo_9 |
| 0x8c0810 | `TRANSFER_MANAGER::review_ai_club_bid` (o comprador revê o lance depois da contraproposta) | não | **mkb_1** (compila; prova pendente) |
| 0x842c00 | `Shortlist_Manager::add_player_to_club_shortlists` (a vitrine) | sim | cpp_6 |
| 0x83cb40 | `Shortlist_Manager::assess_club_squad` (avalia o elenco; põe nas listas de transferência e de empréstimo) | não | não portada (§8) |

Vizinhas que entram nas regras abaixo: 0x83a730 `update_club_shortlist` (Rust), 0x842320 `transfer_list_player`
(Rust), 0x8442b0 `add_to_loan_list` (Rust, só telas), 0x842740 `try_loan_offer_for_player` (Rust), 0x846920
`staff_interest_in_joining_club` (Rust), 0x840d80 `value_player_for_club` (Rust), 0x83b6e0 `allocate_transfer_budget`
(Rust), 0x8c63a0 `process_due_offer` (Rust), 0x842e10 `try_add_player_to_club_shortlist` (original), 0x83a1b0
`create_initial_shortlists` e 0x84abe0 `rebuild_club_shortlist` (originais).

## 2. Quem chama e quando

- **O ritmo da lista do clube** (laço do dia `game_main_day_loop` 0x5cccd0, 0x5cdc97..0x5cdd7d) [provado]: para cada
  clube com nação, na ordem de `[0xae2d1c]`, o contador `club_shortlist::DAYS` sobe 1 por dia (até 200). A lista é
  refeita (`update_club_shortlist` 0x83a730 → `assess_club_squad` com daily 1, reset 0 → `allocate_transfer_budget`)
  quando o clube não tem lance em vaga (`count_slots_with_bids(clube, −1)` = 0) e `club_shortlist::ACTIVE` = 2, ou
  quando DAYS > 30 + sorteio(70) **e** sorteio(5) = 0. Na criação da carreira, `create_initial_shortlists` 0x83a1b0
  (chamada por `game_run_to_start_date` 0x5cc850) chama a `assess_club_squad` com daily 0, reset 1; a
  `rebuild_club_shortlist` 0x84abe0 (técnico novo, `reset_board_for_new_manager` 0x694c80, e o menu 0x7615d0) também.
- **Os lances** (`make_club_bids` 0x84a020, chamada em 0x5cdd6d) [provado]: com a liga da nação carregada
  (`nation::LEAGUE_SELECTED` +0x11c ≠ 0), todo dia, salvo clube sem liga ou na liga de `[KEY_CLUB_COMP_A_LOWER_DIVISION]`
  (0x9cf764); fora disso, todo dia se a reputação passar de 6250 (0x186a). Nos outros casos só quando
  (sorteio(2) + id do clube) % 7 = 0, com o sorteio(2) feito **uma vez por dia** antes do laço (0x5cdc76). **Defeito:**
  a conta não usa a data; só os clubes de id ≡ 0 ou 6 (mod 7) passam, metade dos dias cada; os outros 5/7 desses
  clubes **nunca** fazem lance pela lista [provado; efeito: inferido, provavelmente a intenção era "uma vez por semana"].
- **A resposta a um lance da IA** (`process_due_offer` 0x8c63a0) [provado]: estado `offer::STATUS` 0 com vendedor da
  IA: empréstimo (`offer::OFFER_TYPE` = 1) → 0x84af00(vendedor, jogador, 0, comprador, −1, 1): ≠ 0 recusa o empréstimo
  (0x8bfa30), 0 segue para o aceite (0x4cc930, estado 8, e a vontade do jogador 0x8d8310); transferência → 0x8c8090.
  Estado 1 (`offer_status::ON_HOLD`, a contraproposta) com comprador da IA: empréstimo → 0x8c0f40; transferência →
  **0x8c0810** com keep_existing 0. Estados 0x11/0x12: 0x84af00(vendedor, jogador, valor ponderado, comprador, −1, 0):
  0 aceita (0x8c0080), 1 ou 2 recusa (0x8bf680) — aqui "negociar" também vira recusa.
- **A previsão 0x84af00** tem 7 chamadas [provado, refs.py]: 0x784b4d e 0x784e0b (`NewsItem::auto_respond` 0x784100,
  a resposta automática das férias), 0x84049c e 0x8405a2 (dentro da 0x83f3a0, fragmento 0x840084), 0x8435c3
  (`try_add_player_to_club_shortlist` 0x842e10, a vitrine), 0x8c6560 e 0x8c6a1b (`process_due_offer`). O pedido citou 5;
  faltavam 0x784b4d e 0x8c6a1b.
- **0x8c0810** [provado]: 0x784bb1 (férias), 0x8c2e9f (`handle_player_rejected_terms` 0x8c28c0), 0x8c685c
  (`process_due_offer`), 0x8f2cea (diálogo de tirar jogador de troca 0x8f2870). 13 chamadas até 2002-06-01 na cobertura
  de 17/10.
- **0x8c8090** [provado]: 0x784b85 (férias), 0x8c0527 (`accept_transfer_bid`), 0x8c2e3d (`handle_player_rejected_terms`),
  0x8c42fa (`remove_exchange_player`), 0x8c66fe (`process_due_offer`). A recusa de empréstimo no topo dela só é
  alcançável por um chamador que mande um empréstimo, e o `process_due_offer` manda empréstimo para a 0x84af00: a
  correção do pedido (07/11) está **certa** [provado].
- **A vitrine 0x842c00** [provado, refs.py]: 16 chamadas, entre elas `transfer_list_player` 0x842320 (com news),
  `CONTRACT_MANAGER::daily_update` 0x4c7ac0 (3), `handle_transfer_request` 0x4d8610, `game_daily_staff_update` 0x5ce2d0
  (2), `Media::post_player_praise_story` 0x744240, `award_raise_winner_reputation` 0x412f00, `NewsItem::auto_respond`,
  `move_player_to_club` 0x8c9720, `set_player_squad_status` 0x8ce0e0 e as telas de pessoa (2). Uma em
  `CONTRACT_MANAGER::save_to_disk` 0x4dc3a0 (0x4dd0a1) é estranha: pode ser a fronteira do mapa [inferido].
- **A lista de transferência pela IA**: `transfer_list_player` 0x842320 tem 3 chamadas: 0x83dc42 (dentro da
  `assess_club_squad`, a única da IA) e as telas 0x8f8030/0x8f8607 [provado]. **A lista de empréstimos**: os únicos
  `or byte [r+0x4e], 2` do `.text` são 0x83e49c e 0x83e651 (`assess_club_squad`) e 0x8442c4 (`add_to_loan_list`, que
  só as telas chamam: 0x8f806c, 0x8f8538); mais dois por registrador na `assess_club_squad` (0x83dd42, 0x83dd98)
  [provado pela varredura; escritas por outras formas de instrução não foram varridas: inferido que não há].

## 3. As regras

### 3.1 A avaliação do elenco (0x83cb40, original)

`assess_club_squad(this, entrada, clube, daily, reset)`, thiscall, ret 0x10. Ordem [provado onde marcado; o resto
pseudocódigo, inferido]:

1. DAYS (+8 da entrada) = 0. Sai (e ACTIVE = 0) se o clube é seleção, sem nação, humano com daily, ou
   `club::PROFESSIONAL_STATUS` = 3; com time B ligado (`club_get_linked_b_team` 0x540a50) sem o par, sai [inferido;
   a chamada em 0x83cc04 provada].
2. Sem lance em vaga: se ACTIVE era 2, vira 1 e reset = 1. Se a liga da nação não está carregada ou a liga do clube não
   está ativa em `[0xadadfc]`, e a reputação < 4000: se sorteio(reputação + 1000) < sorteio(5000), ACTIVE = 0 e sai
   (clubes pequenos de ligas não carregadas desligam a lista) [provado: 0x83cc6b..0x83ccab].
3. Com reset: as 40 vagas refeitas (`shortlist_set_up_formation_slots` 0x83c090; o estatuto de cada vaga pelo índice
   e pela reputação-alvo `local_840` de `player_search_get_club_target_rating` 0x7bca90: até 6000 ou 8500 e com/sem
   competição continental). Sem reset: só ordena as vagas (qsort 0x945333) [inferido].
4. As vagas sem alvo, ou com alvo sem oferta e data vencida (ou alvo com cargo 0x10): alvo e substituto = −1, preço =
   0, e as vagas de jogador vão para as listas de posições a preencher por estatuto (2/3, 4, 5) [inferido].
5. **O elenco** (`database_collect_club_squad` 0x541f90, 50): para cada jogador com ficha, cargo ≠ 0x10, fora da lista
   do clube, com contrato no clube, sem transferência futura (0x8cca10) e sem `contract::LEAVING_ON_BOSMAN`: num clube da
   IA sem técnico de férias, **apaga as marcas de transferência e de empréstimo** (`contract::TRANSFER_STATUS` & 0xfc),
   põe o estatuto (`contract::SQUAD_STATUS` nibble alto) 6 abaixo de 20 anos ou 7 (clube da IA) e manda ao time
   principal (`move_to_first_team` 0x4cc0b0) [provado: 0x83d281..0x83d2d7]. Lesionado (`Injury::get_player_injury`
   0x62ed70) com dias acima de 30/300/400 (pela nota 0x6e6340 contra a reputação-alvo × 0,75) vai às reservas
   (`move_to_reserves` 0x4cc0f0); os outros vão a uma de duas listas (com ou sem contrato de empréstimo em outro
   clube) [inferido].
6. Escalação por estatuto (`match_man_assign_players_to_positions` 0x6e2000) para cada lista de vagas; quem foi
   escolhido sai da lista dos sobrantes; os de idade > goleiro/10 + 22 vão para o fim [inferido].
7. **Quem sobra** (o fim do elenco), para cada jogador do clube, sem contrato de empréstimo e sem fila de transferência
   livre (0x8c1870) [inferido no conjunto; os pontos marcados provados]:
   - jogador abaixo da nota do técnico (`match_man_judge_player_ability` 0x6e86d0 < limiar) e (contrato tipo 2 com
     pouco tempo, ou reputação × 50 ≤ `staff::VALUE`, ou o salário restante/30 ≤ valor) e o elenco (`database_count_club_squad`)
     > reputação/500 + 20: com reset, felicidade −100 (0x9c); clube da IA com nota 2 < reputação/68 − 35 e (idade >
     goleiro/5 + 21 ou `player_sheet::CURRENT_REPUTATION` > reputação do clube / 2): **lista de transferência**
     (`transfer_list_player` 0x842320, com news só sem reset) [provado: 0x83dba7..0x83dc42];
   - senão, da IA, sem reset, sem ofertas nem troca, e elenco > reputação/500 + 25: **fila de transferência livre**
     (`queue_free_transfer` 0x8c16b0) [provado: 0x83db6b];
   - nos dois casos às reservas (salvo time B com reputação mundial ≥ 5000) [provado: 0x83dc47..0x83dc66];
   - jogador **acima** do limiar, num clube da IA: com reset, felicidade −50 (0xce) [provado: 0x83dc88]; às reservas;
     e, com felicidade negativa (`contract::HAPPINESS` < 0) e cargo de jogador, **lista de empréstimos** (bit
     `transfer_status::LOAN_LISTED`) para goleiro de nota 20 com idade ≥ 20 e felicidade ≤ idade × 50/3 − 433, ou
     outro com idade ≥ 18 e felicidade ≤ idade × 50/3 − 400; abaixo de 24 anos também estatuto 6 [provado:
     0x83dce6..0x83ddb2]. Ex.: aos 20 anos o goleiro precisa de −100 e o de linha de −34; aos 24, −33 e +0 (qualquer
     felicidade negativa); aos 30, qualquer negativa. **O jovem só entra muito infeliz.**
8. As vagas recebem os escolhidos: alvo do substituto e, sem técnico de férias, estatuto do contrato pelo da vaga
   (salvo 1 e 2) [inferido]. Nas listas 4 e 5 (reservas, sobra):
   - lista 4: **empréstimo** se a fadiga (`Injury::get_fatigue` 0x62c3d0) < −1000, `CURRENT_REPUTATION` < 4500,
     `player_sheet::WORK_RATE` (+0x44) < 3250 e felicidade < −50 [provado: 0x83e46a..0x83e49c]. **Defeito:** +0x44 é um
     byte com sinal comparado com 3250 (0xcb2): sempre verdadeiro; provavelmente era `WORLD_REPUTATION` (+0xd, palavra)
     [provado o código; a intenção é inferida];
   - lista 5: **empréstimo** se a fadiga < −500 − sorteio(300) e felicidade < 0 [provado: 0x83e613..0x83e651].
9. Preço das vagas de jogador (`shortlist_slot::VALUATION`, `shortlist_slot::PRICE`) pela reputação-alvo (até 36000)
   e pelo estatuto; com a nota do alvo abaixo, ou queixas 0x100/1, ou contrato a menos de 365 dias, a vaga é refeita
   com felicidade ajustada [inferido].
10. As vagas de comissão (cargos 6 a 10) com a pessoa do cargo (auxiliar +0xd3, `club::COACHES`, `club::PHYSIOS`,
    `club::SCOUTS`) avaliada por `score_staff_for_job` 0x69d1b0 contra reputação-alvo/5 (até 5000); abaixo e sem
    proposta (0x4cc9e0), **demitida** (`MANAGER_MANAGER::remove_staff_from_club` 0x690890) [inferido].

### 3.2 A escolha do alvo (0x83f3a0, original)

`fill_shortlist_slot(this, vaga, clube, excluído)`, thiscall, ret 0xc, recursiva [inferido salvo onde marcado]:

1. Sai: sem nação; clube humano; técnico de férias com regra 4 = 1; vaga com oferta; clube seleção.
2. **O divisor de conhecimento k** [provado: 0x83f4d8..0x83f5b8]: 100; reputação < 3500 → (6025 − reputação) × 4;
   Bélgica, Holanda, Suécia, Dinamarca, Noruega → 50. Cada uma das 7 vagas de olheiro (`club::SCOUTS`): vazia +5 (+10
   com a liga ativa); com olheiro, + 30 − 2 × `chance_to_know`(olheiro, 6) − `chance_to_know`(olheiro, 5). k = k/10,
   mínimo 2. **Defeito provável:** a 0x544410 recebe as constantes 6 e 5, não atributos do olheiro; o resultado depende
   só da habilidade atual dele (+4 da ficha de não jogador): `JUDGEMENT`/`JUDGING_POTENTIAL` não entram [provado o
   código; a intenção é inferida].
3. Fator de reputação (×0,0008, ou ×1/1175, 1/1125, 1/1050 pela faixa de riqueza 0x5986b0 em ITA, ENG, GER, FRA,
   ESP), teto da vaga (`max_fee_for_squad_status` 0x84b230).
4. Percorre as listas de posição (`POSITION_LISTS_BY_PERSON`) da posição da vaga. O candidato passa por: não excluído,
   cargo ≠ 5, sem cargo 5 na seleção, sem vaga de técnico (0x69fc60); jogador com ficha, sem aposentadoria anunciada,
   não saudita; idade < 24 nas vagas de estatuto 5; **preço × 1,3 ≤ teto da vaga** (`shortlist_slot::MAX_PRICE`) ou
   listado para empréstimo; não do próprio clube; em menos de 10 listas; chegou há mais de 200 dias (ou estatuto 7,
   queixas, listado); fora de outra lista do clube; permissão de trabalho (0x84b180, 0x8c7560); no empréstimo, pode
   emprestar (0x8d0040) e humano só se listado.
5. **Quem o clube conhece**: (id da pessoa + id do clube) % k = 0; ou estrangeiro de outra nação com idade > 22, ou
   reputação mundial > 3249, ou não estrangeiro (0x53d980), e mesma região (+0x75 da nação) ou clube de reputação >
   2749 com…, e então a reputação vista (0x5407e0) ≥ 2750 + sorteio(2000) (vagas 5/6) ou ≥ 3250 + sorteio(2000); jovem
   (< 20) de reputação doméstica < 3250 em clube < 3250: (id + clube) % 15, ou % (25 − `non_player::YOUNGSTERS` do
   técnico) = 0. Os olheiros do humano (0x7f7490) contam para o clube de férias.
6. A nota (0x840d80, o valor do jogador para o clube) ≥ o valor da vaga e ≥ valor − 10000; o preço pedido (0x4d1600,
   0x4d42d0 humano; troca com compensação 0x84b6f0); **a previsão 0x84af00** com o lance previsto: 2 descarta. Lesão:
   (dias − 15)² ou (dias − 10)² × 200 a menos.
7. O melhor por nota com sorteio; **vontade** (0x846920) ≥ 3,15 (0x968e58) e salário: (3,5/vontade, mínimo 0,9) × o
   salário desejado (0x864550) ≤ 1,2 × o teto salarial (0x5968d0). Notícia de interesse (0x754740) com sorteios.
8. O escolhido vira o alvo (`shortlist_slot::TARGET_STAFF_ID`) e entra na lista (`add_shortlist_entry` 0x8423f0).
   Sem ninguém, vaga de estatuto 2/3 vira **vaga de empréstimo** (bit 0) e recomeça se (reputação² × 4e-9 + 0,6) ×
   valor < preço, ou teto < 50000, ou Itália com sorteio e reputação 3000..6000; nação não carregada com
   sorteio ≤ sorteio(20000). Senão o alvo fica −1.

### 3.3 Os lances e os pedidos (0x84a020, Rust)

O cabeçalho em `rust/src/motor/shortlist_manager.rs` (linhas 38-97) está completo [provado: Rust comparado, cqe_3].
Resumo: clube da IA (não seleção, não humano, nem humano de férias com regra 4 = 1), lista ativa, DAYS < 100, com
finanças; a conta = vagas com lance + elenco; vaga de jogador pulada com conta ≥ 50. **Compra**: valor da vaga
(`shortlist_slot::PRICE`, ×2 com a marca 0x20, mínimo 20000), ×3/×2 na liga ativa por reputação > 4750 e conta < 25/35;
substituto incapaz, lesionado (dias² × reputação/500), moral, em oferta; salário desejado ≤ 1,3 × teto; **sorteio(valor) >
sorteio(2 × reputação + 1000)** faz o lance. **Empréstimo**: não França amadora; pode emprestar; vendedor humano só com
o jogador listado; **sorteio(PRICE) > sorteio(reputação/10)**, ou no segundo par > sorteio(2 × reputação): o pedido
(0x842740, que exige vontade ≥ 3,5).

### 3.4 A previsão do vendedor (0x84af00, Rust)

`stdcall(vendedor, jogador, oferta i64, comprador, pedido i64, empréstimo)`, ret 0x20; 0 aceita, 1 negocia, 2 recusa.
Ordem [provado: Rust comparado, cpo_9, e cabeçalho]:

1. Sem vendedor, ou o contrato com ele (`contract_manager_get_contract_for_club`) com `contract::LEAVING_ON_BOSMAN`: **0**
   (também para empréstimo).
2. O técnico do vendedor é o jogador: 2.
3. Sem contrato, ou `contract::CONTRACT_TYPE` & 0x3f ≠ 2: empréstimo **2**, compra 0.
4. Empréstimo: vendedor humano: 0. Listado para empréstimo (`staff_is_loan_listed` 0x544590, valor 2 de +0x4e) e
   reputação do comprador × 1,25 > `CURRENT_REPUTATION`: 0. Estatuto 6 ou 7 (+0x4f) ou **goleiro de nota 20**
   (`player_sheet::GOALKEEPER` = 20), com reputação do comprador > 0,75 × `CURRENT_REPUTATION`, **`contract::HAPPINESS`
   (+0x3d) < −50** e `club::CONTINENTAL_COMP_ID` do vendedor ≠ `[KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE]`: 0.
   Senão 2.
5. Compra: o preço (humano: 0,8 × preço pedido do humano; sem cláusula e de férias com regra 0: 1 recusa, 2 o valor;
   IA: o pedido se ≥ 0, o valor sem ficha, ou `contract_manager_calc_asking_price`); oferta ≥ preço: 0; humano, oferta >
   preço − 1.500.000 e listado: 1; preço × 0,7142857 > oferta: 2; senão 1.

**A leitura do pedido (MERCADO.md §2, EMPRESTIMOS.md) confere**, com três correções: (a) a moral do item 2 é a
**felicidade** `contract::HAPPINESS` (+0x3d), não a moral (+0x44); (b) "ou goleiro" é o goleiro de **nota 20** na
posição; (c) faltaram as saídas de cima: contrato do vendedor de tipo ≠ 2 (ou sem contrato) **recusa** o empréstimo,
e Bosman marcado ou sem vendedor **aceita**. "O salário e o período não entram" confere: a 0x84af00 não lê nenhum dos
dois [provado].

### 3.5 O comprador revendo o lance (0x8c0810, original; item mkb_1)

Regra completa no cabeçalho de `levas/fila/mkb_1/codigo.rs` [provado: desmontagem 0x8c0810..0x8c0f38]. Resumo:
com keep_existing 0 revê a **contraproposta** (a negociação `offer::NEGOTIATION_INDEX`; sem ela, estado 0x10 e
reagenda); acha a vaga do comprador (0x849ce0; sem vaga, retira); valor = 0x840d80; preço = valor ponderado (0x8e7b50
com 0,1,1,1,1) + preço pedido de cada jogador de troca que caiba (os outros saem); **preço > valor**: retira, marca a vaga
(0x843e10 evento 1) e o jogador pode ficar **inquieto** (`apply_event_flags` 0x800000) se está no clube há mais de
(lealdade + 42) × 5 + profissionalismo dias, e o comprador tem reputação 1250 acima do vendedor (ou listado a pedido,
marca 8, queixas 0x14), e o lance passa de 17,5 milhões, de 1,5 × o nível (valor para o vendedor ×8 até 500 mil, (v −
500 mil) × 2 + 2 milhões até 2,5 milhões, (v − 2,5 milhões) × 1,25 + 6 milhões; vezes 0,9..2,5 pelo humor) ou, acima de
250 mil e do nível, com sorteio(temperamento + 1) = 0. **Dentro do valor**: completa a taxa com os jogadores de troca
tirados (até o valor) e, se a verba (`Finance::get_transfer_budget` 0x598800) cobre o total (0x8e7a70), **refaz o lance**
(`add_transfer_offer` 0x8be300).

### 3.6 A resposta do vendedor (0x8c8090, Rust)

Cabeçalho completo em `rust/src/motor/transfer_manager.rs` (linhas 520-565) [provado: Rust comparado]. Resumo:
empréstimo recusado sem olhar nada (código praticamente morto, ver §2); "tem de aceitar"
(`transfer_manager_selling_club_must_accept`) aceita; humano de férias com regra 0 recusa; piso = preço pedido
(0x4d1600) ou a melhor outra oferta; opções em dinheiro, troca e cláusula; a primeira com valor × 0,85 abaixo do pedido
aceita; a melhor × 0,75 abaixo do pedido vira **contraproposta** (salvo tipos 1 e 2); senão recusa (motivo 10 se o
total < 0,55 do valor com 2 milhões de diferença, ou < 0,1 com 100 mil).

### 3.7 A vitrine (0x842c00, Rust)

[provado: Rust comparado, cpp_6] Só jogador com ficha e `CURRENT_REPUTATION` ≥ 5500 (0x157c), salvo force; sem
aposentadoria anunciada; cargo de jogador; não técnico de seleção; não saudita; sem transferência futura; quem chegou há
menos de 200 dias só com estatuto 7 ou queixas. Percorre os clubes por reputação (`[G_CLUB_REPUTATION_ORDER]`) e oferece
(0x842e10, que usa a 0x84af00) até o jogador estar em 5 listas (a conta é o nibble baixo de +0xb do registro de 79
bytes), pulando seleções, o clube do contrato e o próprio. **Por isso o jogador abaixo de 5500 de reputação nunca é
oferecido**, e os clubes de reputação alta ficam com ele primeiro.

## 4. Campos lidos e gravados

| Registro | Campo (deslocamento) | Rotinas | L/G |
|---|---|---|---|
| contrato | `TRANSFER_STATUS` +0x4e (valor 1 listado para transferência, valor 2 listado para empréstimo) | 0x83cb40 (apaga e põe), 0x84af00, 0x84a020 | L/G |
| contrato | `SQUAD_STATUS` +0x4f (nibble alto 1..7) | 0x83cb40, 0x84af00, 0x84a020 | L/G |
| contrato | `HAPPINESS` +0x3d, `UNHAPPY_FLAG` +0x3e, `MATCHES_LEFT_OUT` +0x3f | 0x83cb40 (com reset grava −100, −50, ±), 0x84af00 (lê) | L/G |
| contrato | `CONTRACT_TYPE` +0x35, `LEAVING_ON_BOSMAN` +0x49, `GRIEVANCES` +0x45, `WAGE` +0xc, `CONTRACT_EXPIRES` +0x2d | 0x83cb40, 0x84af00, 0x8c0810, 0x83f3a0 | L |
| contrato | marca de evento 0x800000 (`apply_event_flags`) | 0x8c0810 | G |
| pessoa | `AGE` +0x18, `JOB_FOR_CLUB` +0x3d, `CLUB_JOB` +0x39, `DATE_JOINED_CLUB` +0x3e, `VALUE` +0x52, `AMBITION` +0x57, `LOYALTY` +0x59, `PRESSURE` +0x5a, `PROFESSIONALISM` +0x5b, `TEMPERAMENT` +0x5d, `PLAYER` +0x61, `NON_PLAYER` +0x69, `NATION` +0x1a | todas | L |
| ficha | `CURRENT_REPUTATION` +0xb, `WORLD_REPUTATION` +0xd, `GOALKEEPER` +0xf, `HOME_REPUTATION` +9, `WORK_RATE` +0x44 (defeito), `PLAYER_MORALE` +0x45 | 0x83cb40, 0x84af00, 0x84a020, 0x83f3a0, 0x842c00 | L |
| clube | `REPUTATION` +0x80, `NATION` +0x53, `DIVISION` +0x57, `PROFESSIONAL_STATUS` +0x64, `MANAGER` +0xcf, `ASSISTANT_MANAGER` +0xd3, `SQUAD` +0xd7, `COACHES` +0x19f, `SCOUTS` +0x1b3, `PHYSIOS` +0x1cf, `CONTINENTAL_COMP_ID` +0x1db | todas | L |
| comissão | `non_player` +4 (habilidade do olheiro), `YOUNGSTERS` +0x22 (técnico) | 0x83f3a0 | L |
| lista do clube | `club_shortlist::ACTIVE` +4, `DAYS` +8; vaga `shortlist_slot` (+0 cargo, +1 posição, +3, +7, +0xb, +0xf, +0x13, +0x17, +0x1b, +0x23, +0x24) | 0x83cb40, 0x83f3a0, 0x84a020, 0x8c0810 | L/G |
| registro de 79 bytes | `STATUS_FLAGS` +0xb (nibble baixo = em quantas listas; 0x30 aposentadoria) | 0x83f3a0, 0x842c00, 0x84a020 | L |
| oferta | `NEGOTIATION_INDEX` +0x28, `STATUS` +0x2c, `FEE` +0x1b, `EXCHANGE_PLAYERS` +4, `OFFER_KIND` +0x31, `JOB_TYPE` +0x11 | 0x8c0810, 0x8c8090 | L/G |

O presidente e os diretores (`club::CHAIRMAN` +0xbf, `club::DIRECTORS` +0xc3) **não são lidos** por nenhuma das 7
rotinas desta seção [provado para as 4 em Rust e para a 0x8c0810; inferido para a 0x83cb40 e a 0x83f3a0 pelo
pseudocódigo]. Do técnico entram só `non_player::YOUNGSTERS` (0x83f3a0) e o julgamento dele na nota
(`match_man_judge_player_ability` 0x6e86d0 recebe `club::MANAGER`); dos olheiros, só a habilidade atual (§3.2).

## 5. Constantes (lidas do exe)

| Endereço | Valor | Uso |
|---|---|---|
| 0x968928 | 1,25 (f64) | 0x84af00: reputação do comprador × 1,25; 0x8c0810: (v − 2,5 mi) × 1,25 |
| 0x969040 | 0,75 | 0x84af00 (reputação no caso 6/7); 0x83cb40 (nota × 0,75) |
| 0x968e58 | 3,15 | 0x83f3a0: vontade mínima |
| 0x968e38 | 3,5 | 0x83f3a0 (3,5/vontade); 0x842740 vontade mínima do empréstimo |
| 0x968de8 | 1,3 | 0x83f3a0: preço × 1,3 ≤ teto da vaga; 0x84a020: salário ≤ 1,3 × teto |
| 0x968978 | 1,2 | 0x83f3a0: salário ≤ 1,2 × teto salarial |
| 0x9689d8 / 0x96a32c | 0,9 | 0x83f3a0: mínimo de 3,5/vontade |
| 0x96f898, 0x968e48 | 4e-9, 0,6 | 0x83f3a0: virar vaga de empréstimo |
| 0x96a618, 0x96f8a8, 0x96f8b0, 0x96f8b8 | 0,0008, 1/1175, 1/1125, 1/1050 | 0x83f3a0: fator de reputação |
| 0x968940 | 1,5 | 0x8c0810 (nível × 1,5); 0x84a020 (moral) |
| 0x968f10, 0x96aff8, 0x968e90 | 8, 2.000.000, 6.000.000 | 0x8c0810: nível |
| 0x96b000, 0x96aff0 (f32) | 500.000, 2.500.000 | 0x8c0810: faixas do nível |
| 0x96a32c, 0x96b014, 0x968e70, 0x96a33c, 0x96b010, 0x96b004, 0x96b00c, 0x9689a8, 0x96b008 (f32) | 0,9 0,95 1 1,1 1,25 1,5 1,75 2 2,5 | 0x8c0810: fator do humor |
| imediatos | 5500 (0x157c), 5 clubes | 0x842c00 |
| imediatos | 6250 (0x186a), % 7, 30 + sorteio(70), sorteio(5) | laço do dia |
| imediatos | −1000 (0xfc18), 4500 (0x1194), 3250 (0xcb2), −50 (0xce), −500 − sorteio(300) | 0x83cb40: empréstimos |
| imediatos | −433 (0x1b1), −400 (0x190), 18/20/24 anos | 0x83cb40: empréstimo por felicidade |
| imediatos | 17.500.000, 250.000, 1250 | 0x8c0810 |

## 6. Defeitos do original

1. **Lances só de 2/7 dos clubes** fora das ligas carregadas (ou sem liga, ou na liga da chave 0x9cf764) com reputação
   ≤ 6250: (sorteio(2) + id) % 7 sem a data [provado].
2. **0x83cb40, empréstimo da lista 4:** `WORK_RATE` (byte) < 3250 sempre verdadeiro [provado; intenção inferida].
3. **0x83f3a0, olheiros:** `chance_to_know` com as constantes 6 e 5; os atributos de julgamento do olheiro nunca entram
   [provado o código; intenção inferida].
4. **0x84af00:** com contrato do vendedor de tipo ≠ 2 o empréstimo é sempre recusado e a compra sempre aceita (oferta
   qualquer) [provado; se é intenção, não se sabe].
5. **0x8c63a0, estados 0x11/0x12:** a previsão "negociar" (1) vira recusa [provado; intenção inferida].
6. **A pessoa repetida no elenco** (o pedido diz que nasce na 0x83cb40): **não nasce lá**. A cópia é escrita pela
   `toggle_squad_placement` 0x4cbd20 (via `database_add_staff_to_club` 0x53bd10, sem conferir a presença), chamada por
   `move_to_first_team` 0x4cc0b0 e `move_to_reserves` 0x4cc0f0. A 0x83cb40 é o **gatilho** mais frequente: a cada
   avaliação manda todo jogador do clube da IA ao time principal e depois parte às reservas (0x83d2a5, 0x83d2d7,
   0x83d3f4, 0x83dc66, 0x83dcbc, 0x83e456, 0x83e60e, 0x83e770, 0x83e80f, 0x83e84a) [provado]; com time B ligado
   (lugar 3/4) a pessoa troca de clube e a cópia aparece (`docs/DEFEITOS_DO_ORIGINAL.md` linha 102) [provado lá].
   `docs/COMISSAO_REPETIDA.md` é outra repetição (vagas de treinador, 0x7a65d0), sem relação com a 0x83cb40.

## 7. Pontos de extensão (um gancho pequeno muda a política)

| Onde | O que muda | Como |
|---|---|---|
| 0x84af00 (Rust; 7 chamadas) | quem empresta e vende | envolver: chamar a original e trocar 2 → 0 pela regra nova (o `loan_rule = squad` do Tweaks já faz) |
| `assess_club_squad` 0x83cb40, depois da chamada em 0x83a763 (0x83a730 é Rust) | quem vai às listas de transferência e de empréstimo | no fim de `update_club_shortlist`, percorrer o elenco e pôr/tirar `LOAN_LISTED` (jovens com PA ≫ CA e poucos jogos) |
| 0x842c00 (Rust) | quem é oferecido | o limiar 5500 e o máximo de 5 listas |
| laço do dia 0x5cdd27..0x5cdd6a | quem faz lance e com que frequência | corrigir o % 7 (usar o dia) ou a reputação 6250 |
| 0x5cdcee..0x5cdd15 | ritmo da lista (30 + sorteio(70), sorteio(5)) | encurtar para clubes de sucesso |
| 0x83f3a0 divisor k (0x83f4d8..0x83f5b8) | quantos jogadores o clube conhece | passar `JUDGEMENT`/`JUDGING_POTENTIAL` do olheiro |
| 0x83f3a0 vontade 3,15 (0x968e58) e 1,3/1,2 | quem entra na lista | as constantes do `.rdata` (cópia do exe) ou o gancho |
| 0x84a020 (Rust) sorteios | ritmo dos lances | `2 × reputação + 1000` e `reputação/10` |
| 0x8c0810 (mkb_1) | quanto o comprador aceita pagar depois da contraproposta | `w > valor` |

## 8. O que não foi portado

- **0x83cb40** (10.336 bytes até a fronteira, 3.136 instruções, quadro de 0x8a4 bytes, 45 chamadas, 2 ainda
  originais) e **0x83f3a0** (6.624 bytes, 2.048 instruções, recursiva, quadro SEH): grandes demais para portar e provar
  nesta rodada; cada uma pede um item sozinha e um leitor com tempo para a prova. O mapa acima (§3.1, §3.2) é o ponto
  de partida; a 0x83f3a0 tem os fragmentos 0x840000 e 0x840010 (mapa da comunidade) e o pedaço 0x840084 que não são
  rotinas.
- **0x8c0810**: item `mkb_1` (compila; comparação, carreiras e mutações **não** rodaram: as vagas do laboratório
  estavam ocupadas; ver `levas/fila/mkb_1/PRONTO.txt`).
