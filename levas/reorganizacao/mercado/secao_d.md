*Versão em inglês: [secao_d.en.md](secao_d.en.md). O português é o original.*

# Mercado, seção D: o lado do jogador e quem decide (leitor D, 11/11/2026)

Escopo: a vontade do jogador de ir ou ficar (empréstimos incluídos), o salário pedido e a duração, as renovações, as
cláusulas, o mercado de jogadores livres e, no mercado inteiro, onde se leem os atributos do presidente e da comissão
(treinador, auxiliar, olheiros, diretores). Marcas: **[provado]** = desmontagem, varredura do exe ou código Rust já
comparado com 0 divergências; **[inferido]** = pseudocódigo do Ghidra ou leitura estrutural, sem conferir instrução a
instrução.

Fontes: `rust/src/motor/{shortlist_manager,contract_manager,staff_contracts,transfer_manager,match_man,fog_of_war}.rs`
(as rotinas já em Rust, lidas pelo código e pelo cabeçalho), `pacotes/0x4ce4e0`, `pacotes/0x866830`, o pseudocódigo de
0x85fc50, 0x4c7ac0, 0x864550, 0x862720 e 0x844780 (`descompilar.py`), `refs.py` (chamadores) e uma varredura do
`.text` inteiro (capstone) pelas leituras de byte em `[ficha + 0x0e..0x22]` depois de `mov r, [x + 0x69]`
(`staff::NON_PLAYER`).

## 1. As rotinas

| Endereço | Nome | Arquivo | Em Rust | Item | Quem chama / quando |
|---|---|---|---|---|---|
| 0x846920 | `Shortlist_Manager::staff_interest_in_joining_club` (a vontade) | shortlist_manager.cpp | sim | — | lances (0x84a020, 0x842e10, 0x843800, 0x840084), empréstimos (0x842740), aceitar lance (0x8c0080), prazo de resposta (0x8cc5b0), jogador desestabilizado (0x8c1290), troca (0x8c8860), orçamento (0x83b6e0), decisão de contrato (0x4ce4e0), salário desejado (0x85fc50), olheiros (0x7b0050, 0x7b62a0), filtros e telas |
| 0x844780 | `staff_interest_in_job_at_club` (a vontade da comissão) | shortlist_manager.cpp | não | — | 0x4ce4e0, 0x85fc50, `is_staff_suitable_for_job` (0x69f810), 0x840084, filtros |
| 0x4ce4e0 | `CONTRACT_MANAGER::resolve_contract_offers` (o jogador escolhe a oferta) | contract_manager.cpp | não | — | `process_due_decision_dates` (0x4cd9c0), as datas de decisão tipo 3 |
| 0x85fc50 | `STAFF_CONTRACT::set_to_desired_contract` (salário pedido e duração) | staff_contracts.cpp | não | — | `offer_contract` (0x4cda70), `lowest_wage_demand` (0x4ce450), 0x4ce4e0, `make_club_bids` (0x84a020), `accept_transfer_bid` (0x8c0080), `move_player_to_club` (0x8c9720), `get_contract_satisfaction` (0x866620), `calc_contract_value` (0x866830), telas |
| 0x864550 | `STAFF_CONTRACT::default_staff_contract` (salário padrão) | staff_contracts.cpp | não | — | 0x846920 (portão do salário), 0x85fc50 (×3), 0x844780, 0x842e10, 0x840084, `update_wage_bill` (0x598370), 0x85e960 |
| 0x862720 | `staff_contract_calc_unset_default_wage` | staff_contracts.cpp | não | — | `set_default_wage` (0x8626e0), `create_initial_shortlists` (0x83a1b0) |
| 0x8626e0 | `STAFF_CONTRACT::set_default_wage` | staff_contracts.cpp | sim | — | regens (0x7a70e4) |
| 0x866830 | `STAFF_CONTRACT::calc_contract_value` (valor de um contrato contra outro) | staff_contracts.cpp | não | **mkd_1** | `get_contract_satisfaction` (0x866620, ×2), 0x4ce4e0 (×2), 0x4d4830 (tela) |
| 0x866620 | `get_contract_satisfaction` (1..3) | staff_contracts.cpp | sim | — | passe diário das pessoas (0x5ce2d0, random(50) = 0), tela |
| 0x4cda70 | `contract_manager_offer_contract` (a oferta do clube) | contract_manager.cpp | sim | — | 0x4c7ac0 (×9: renovações), telas |
| 0x4ce450 | `contract_manager_lowest_wage_demand` | contract_manager.cpp | sim | — | tela de contrato |
| 0x4c7ac0 | `CONTRACT_MANAGER::daily_update` (fim de contrato, renovação, livres) | contract_manager.cpp | não | — | `game_main_day_loop` (0x5cccd0), diário; `game_run_to_start_date` (0x5cc850) |
| 0x4cd9c0 | `process_due_decision_dates` | contract_manager.cpp | sim | — | 0x4c7ac0, diário |
| 0x4cd230 | `schedule_offer_decision` | contract_manager.cpp | sim | — | ofertas de contrato |
| 0x4cc490 | `add_contract_offer` | contract_manager.cpp | sim | — | 0x4cda70 |
| 0x866f60 | `STAFF_CONTRACT::wants_to_leave` (`staff_criterion`) | staff_contracts.cpp | sim | — | 0x4c7ac0 (×3) |
| 0x867240 | `check_transfer_request_withdrawal` | staff_contracts.cpp | sim | — | diário |
| 0x4d8610 | `handle_transfer_request` | contract_manager.cpp | sim | — | pedido de transferência |
| 0x867700 | `STAFF_CONTRACT::apply_event_flags` (queixas, pedido de saída) | staff_contracts.cpp | não (só um desvio de diagnóstico) | — | eventos |
| 0x4d4660 | `will_negotiate_offer` | contract_manager.cpp | não | — | só telas (0x4de5d0, 0x4e06e0, 0x4e30f0) |
| 0x4d4830 / 0x4d44e0 | `prefill_offer_from_pending` / `get_offer_job` | contract_manager.cpp | não | — | só telas |
| 0x4cd030 | `trigger_relegation_clauses` | contract_manager.cpp | sim | — | rebaixamento (0x689c80 e outros) |
| 0x4cced0 | `trigger_non_promotion_clauses` | contract_manager.cpp | não | — | `league_promote_and_relegate` (0x689c80), ArgPrm (0x4067f0) |
| 0x4cd120 | `reset_division_clauses` | contract_manager.cpp | não | — | `League::move_club_to_division` (0x6830b0) |
| 0x867b50 | `has_active_release_clause` | staff_contracts.cpp | sim | — | 0x4c7ac0, 0x4d0c60, 0x4d2070, 0x84af00, 0x8cd240, 0x8e5d60 |
| 0x868210 | `check_fifa_protected_period` | staff_contracts.cpp | sim | — | 0x4c7ac0, lance, troca |
| 0x867d30 | `get_expiry_band` (0..14) | staff_contracts.cpp | sim | — | 0x4c7ac0 (×11) |
| 0x8cd240 | `transfer_manager_selling_club_must_accept` | transfer_manager.cpp | sim | — | resposta ao lance |
| 0x8caaf0 / 0x8d0dd0 / 0x8cb0f0 | percentagem da próxima venda: gravar / achar / pagar | transfer_manager.cpp | sim | — | transferência concluída (0x8c91e0) |
| 0x8cac40 / 0x8cad90 / 0x8cb340 / 0x8c39f0 | bônus por jogos (clube, seleção), pagar, depois do jogo | transfer_manager.cpp | sim | — | fim de cada jogo |
| 0x867ad0 / 0x85f7a0 | `is_clause_allowed` / `add_release_clause` | staff_contracts.cpp | não | — | só telas |
| 0x8c16b0 / 0x8c18c0 / 0x8c1af0 | fila da transferência livre: pôr / processar / liberar | transfer_manager.cpp | sim | — | 0x4c7ac0; o dia das transferências (0x8c5880) |
| 0x8d0a70 | `player_club_uses_bosman` | transfer_manager.cpp | sim | — | 0x4c7ac0, valor, ~31 milhões de chamadas |
| 0x842740 / 0x8d0590 / 0x8d0040 | empréstimo: tentar, período, pode emprestar | shortlist/transfer | sim | — | 0x84a020 |
| 0x8c9720 | `TRANSFER_MANAGER::move_player_to_club` | transfer_manager.cpp | não | — | transferência concluída |
| 0x6e86d0 | `perceived_ability` (habilidade vista por um observador) | match_man.cpp | sim | — | valor (0x4d0c60), lista (0x840d80), regens, motor |

## 2. As regras

### 2.1 A vontade de ir ou ficar (0x846920, Rust) [provado]

`thiscall(lista, pessoa, clube, salário, empréstimo, situação, &motivos)`, ret 0x18, devolve um double em st0.
Ordem das condições (código em `shortlist_manager.rs`, comparado com 0 divergências):

1. 0 sem ficha de jogador, sem nação do clube ou da pessoa, com transferência futura marcada, ou se aposentando.
2. O contrato atual (`contract_manager_get_contract`, em `G_CONTRACT_LOOKUP_RESULT`); clube sem nação: **3,5**; contrato
   tipo 5 (empréstimo): conta como sem clube.
3. **Atalho de ficar** (o clube é o próprio): sem queixas nos grupos 0x161a8, faltando menos de (id mod 35) + 20 dias
   para o fim, sem estar em listas de clubes da IA e reputação atual + ambição × 20 < (lealdade + 170) × 25 + s (s =
   (reputação do clube − a dele)/2, metade com a queixa 0x10, teto lealdade × 10 + 1125): **3,5**.
4. Base: no próprio clube 3,5 (0 com queixas que não sejam 2); em outro, **2,75**; saindo pelo Bosman,
   (1,1 − dias × 0,002) × 2,75; sem clube, 2,75 + mín(12, dias sem clube² / 900).
5. Fatores, nesta ordem: reputações em pedido de empréstimo (×1,025 a ×1,5); clube detestado ×0,33; clube atual
   detestado ×1,25/×1,5/×2 (por pressão + lealdade + profissionalismo); clube atual favorito ×0,5; clube novo favorito
   ×3; acima de 34 anos e outra nação ×0,75; rivais (Celtic×Rangers 0,025/lealdade; Croácia×Iugoslávia; rival direto
   0,2/lealdade, Itália 1 − lealdade × 0,02; mesma liga 0,75/lealdade); o técnico novo gosta ×2 / detesta ×0,33; o
   técnico atual detesta ×2 / gosta ×0,5; a moral no contrato (`contract::MORALE`); o clube humano pela lealdade do
   técnico (lealdade × 0,025 + 0,5); a situação no elenco merecida (`get_deserved_squad_status`) e a oferecida
   (`p5`, 1..7: ×1,54 a ×0,002); a reputação comparada (estádio, divisão, competição, técnico, finanças, por
   `ambição`, `profissionalismo`); fluência no idioma; nações e distância (cidades a menos de 40/80 km), com a
   **adaptabilidade** e a **ambição**; empréstimo de outra nação ×0,45/×0,08; o tempo no clube pela **lealdade**
   (lealdade + 8) × 15 dias sem queixa: 0; o desenvolvimento das nações; o jovem com contrato até o fim da temporada
   não sai.
6. **Portão do salário** (só com `salário` ≠ 0, sem empréstimo e vontade ≥ 1,75): o salário padrão
   (`default_staff_contract`, 0x864550) × 3,5 / vontade (mínimo 0,9) contra o teto do clube
   (`finance_get_max_wage`, 0x5968d0): pedido × 0,65 acima do teto → 0; entre o teto e o pedido, mistura pela
   insatisfação (queixas, lista, felicidade, moral, 0..20).

Atributos lidos: da pessoa `ambição` (+0x57), `lealdade` (+0x59), `pressão` (+0x5a), `profissionalismo` (+0x5b),
`adaptabilidade` (+0x56), `int_apps` (+0x22); do técnico novo `lealdade` (clube humano). **Nenhum atributo de
`non_player`** (só `CURRENT_REPUTATION` do técnico atual). Constantes: 3,5 = `rdata::F64_3_5` (0x968e38, usada em 25
lugares do exe, inclusive o motor de jogo: não remendar a `.rdata`); 2,75 = `F64_2_75` (0x9690a8); 1,75 =
`F64_1_75` (0x968f98).

Limiares dos chamadores: 0x4ce4e0 escolhe acima de 2,75 e de 3,15 (`F64_3_15`, 0x968e58); `try_loan_offer_for_player`
exige 3,5; `make_club_bids` usa 3,15 (`F64_3_15`).

### 2.2 O jogador escolhe entre as ofertas de contrato (0x4ce4e0, original) [inferido]

Por pessoa, quando a data de decisão tipo 3 vence (`process_due_decision_dates`, diário):

1. k = (profissionalismo − ambição − 9 + lealdade) × 1/300 (0x968e60).
2. Com uma oferta de transferência pendente (0x8cd150): adia a decisão 2 dias e sai.
3. Para cada oferta da pessoa na lista (+0x34) pronta para decidir: oferta sem transferência (+0x59 = −1) de um
   clube que não é o do contrato e tipo ≠ 5: removida. Estado 0:
   - tipo 5: vontade = 0x846920(…, empréstimo = 1, situação 8); tipo 6: com a situação de `get_squad_status_for_club`;
   - os outros: o contrato desejado no clube (0x85fc50 com −1, a situação da oferta e o cargo +0x61); razão =
     valor(oferta contra desejado) / valor(desejado contra desejado) − k (`calc_contract_value`, 0x866830); a vontade
     por 0x846920 (jogador) ou 0x844780 (comissão; média das duas se o cargo for de jogador); **renovação** (oferta do
     clube atual): razão ≥ 0,99 (0x968e50) e vontade ≤ 3,15 → vontade = 3,15; razão < 0,6 (0x968e48) → 0;
   - recusa: cargo de jogador sem ficha de jogador (0x7a6e20 falso), ou razão > 0,5 precisando de permissão de
     trabalho (0x8c7560) a menos de 90 dias;
   - nota = vontade × razão, com os cortes 0,9 (0x9689d8), 0,8, 0,7 e 3,5; fica a maior acima de 2,75.
   Estado 2 (já aceita): nenhuma outra.
4. A escolhida vira estado 2 (notícia 0x7549b0, `set_agreed_contract_flag` 0x8c5410); as outras estado 1 (notícia
   0x4d49a0, ou `handle_player_rejected_terms` 0x8c28c0 se ligadas a transferência). Sem escolhida e com oferta do
   próprio clube: marca 0x80 no byte +0xb do registro de 79 bytes. Na escolhida (tipos ≠ 5, 6) com nota > 0x9688b0,
   sorteios pela lealdade (0x90cfc0) [inferido].

### 2.3 Salário pedido e duração [provado no que está em Rust; inferido no resto]

- **0x85fc50 `set_to_desired_contract`** (original, 10.896 bytes): o contrato que a pessoa quer no clube. Lê
  `ambição`, `lealdade`, `pressão`, `profissionalismo`, `int_apps`; o valor do jogador (`calc_player_value`
  0x4d0c60, ×7, que usa a `JudgingPotential` do técnico, §3); o salário padrão (0x864550, ×3); a vontade (0x846920 e
  0x844780); o teto (`get_max_wage`, ×4); a janela (0x8cf470, 0x8cf0e0); o mês do fim da temporada (0x4d45f0); um
  sorteio. Grava o tipo 2 em +0x35, a situação em +0x4f (nibble alto), o salário/valor em +8/+0xc, cláusulas e a data
  de fim. [inferido] Teto absoluto 250.000 no ramo de comissão (`fVar27 = 250000.0`). É o melhor ponto para mudar o
  salário pedido sem portar: um desvio que chama a original e ajusta +0xc/+8 na saída.
- **0x864550 `default_staff_contract`** e **0x862720 `calc_unset_default_wage`** (originais): o salário padrão pela
  ambição, lealdade, pressão, a nação grande (0x549e50), a temporada (0x547770) e o teto (0x5968d0). [inferido]
- **0x4cda70 `offer_contract`** (Rust) [provado]: a oferta do clube = o desejado × fator; multa mínima pela oferta
  (faixas 50 mil / 1 milhão / 5 milhões); teto = `get_max_wage` × (fator + 1) × 0,5 (situação 11) ou
  `get_max_staff_wage`; **duração pela idade** (idade em dias × k − peso de condição física/profissionalismo: acima de
  34 fim no ano seguinte, acima de 31 em 2 anos, acima de 29 pelo ano da idade); total limitado acima de 30 anos;
  lesão acima de 70 dias: 0; com as regras FIFA, no máximo 5 anos; acima do teto × 1,4: 0.
- **0x866830 `calc_contract_value`** (item **mkd_1**) [provado pela desmontagem]: salário + bônus pela posição
  (máscara 0x5b7d20: goleiro jogo sem gol × 0,8; defensor gol × 0,3 e assistência × 0,2; meio × 0,4 e × 0,7; atacante
  × 0,8 e × 0,5) + `contract::UNKNOWN` / (dias × 0,2222); reputação mundial > 4000 e reputação² × 0,01 > +8: metade;
  cada cláusula ativa multiplica (1,01..2 pelas multas, abaixo de 1 M, 5 M (7,5 M no caso 0) ou acima, e pela idade:
  < 24, 24..28, > 28); fator de duração (0,75 / 0,875 / 1, × (1 − a × 0,01) para quem passou da faixa de idade da
  posição, faixas 33/31/29/28) elevado à diferença dos anos de término, entre 0,5 e 2.
- **0x866620 `get_contract_satisfaction`** (Rust) [provado]: 100 × valor(atual) / valor(desejado), com desconto do
  recém-contratado (15 × lealdade + 100 dias); < 50 → 1, < 80 → 3, senão 2. Diário com random(50) = 0.

### 2.4 Renovações e fim de contrato (0x4c7ac0, original) [inferido pela lista de chamadas]

Diário. Para cada contrato: a faixa do fim (`get_expiry_band`, 0..14, ×11) contra as datas +7..+182 dias; aposentadoria
(0x7a7eb0, ×10); clube humano (×12); oferta de renovação da IA (`offer_contract`, ×9); `wants_to_leave` (0x866f60);
ofertas de transferência pendentes (0x8c5cf0, 0x8c5e40); Bosman (0x8d0a70, ×5; nação com transferência livre,
0x544670); período protegido FIFA (0x868210); cláusula ativa (0x867b50); notícias de fim de contrato (0x4d6790 ×12,
0x4da590 ×5, 0x4d6d70); o jogador livre vai às listas dos clubes (`add_player_to_club_shortlists` 0x842c00, ×4) e à fila
da transferência livre (0x8c16b0) ou é liberado na hora (`free_transfer_staff` 0x8c1af0); o fim do empréstimo
(0x4d07e0); e, no fim, as datas de decisão (0x4cd9c0). Atributos: só `lealdade` (+0x59, ×2).

### 2.5 Cláusulas [provado onde em Rust]

- Rebaixamento: `trigger_relegation_clauses` (Rust) passa `RELEGATION_RC` 1 → 2 nos 50 do elenco do clube rebaixado e
  do time B, com notícia. Não promoção: 0x4cced0 (original), chamado na subida/descida (0x689c80) e na Argentina.
  `reset_division_clauses` 0x4cd120 (original) na troca de divisão.
- Ativa: `has_active_release_clause` = não promoção 2, multa mínima 1 ou rebaixamento 2. O vendedor tem de aceitar
  (`selling_club_must_accept`) com taxa ≥ multa (motivo 0x1b) ou vendedor em crise (0x1c). O valor do jogador é
  limitado pela multa (`cap_value_at_release_fee`).
- Percentagem da próxima venda (13 bytes, vetor ordenado por 0x8e54d0): paga a partir de 1000, na transferência
  concluída. Bônus por jogos: 1, 10, 20… 50 jogos pelo clube ou pela seleção, depois de cada jogo.
- Telas só: `is_clause_allowed` (pela regra de acesso/descenso da liga) e `add_release_clause` (multa mínima exclui as
  outras).

### 2.6 Jogadores livres [provado onde em Rust]

Quem fica livre: o fim do contrato em 0x4c7ac0 (Bosman pela nação, 0x8d0a70) e os dispensados pela fila (0x8c16b0 →
`process_free_transfer_queue`, no dia das transferências: no clube humano a diretoria pode barrar, um sorteio põe o
técnico nos desafetos, compensação). Quem os contrata: as listas dos clubes (`add_player_to_club_shortlists`,
`make_club_bids` 0x84a020; sem clube a vontade ganha × 1,75 e a base 2,75 + mín(12, dias² / 900)). Por quanto: o
contrato desejado (0x85fc50) contra o teto (`get_max_wage`).

## 3. Atributos do presidente e da comissão no mercado (a pergunta do pedido)

A busca do pedido **não estava completa**: Patience, Resources, Youngsters, Judgement, JudgingPotential e Interference
são lidos no mercado. ManHandling não. Varredura do `.text` inteiro (leitura de byte de `[ficha_non_player + d]`), cada
acerto do mercado conferido na desmontagem [provado]:

| Atributo (+) | Onde (endereço da instrução) | Rotina | Em Rust | Efeito |
|---|---|---|---|---|
| Patience (+0x1d), técnico e presidente | 0x83bf92, 0x83bfd8 | `update_club_target_reputation` 0x83bf40 | sim | reputação-alvo das contratações × (1 + (ambição − paciência) × 0,0125 [técnico] e / 60 [presidente]) |
| Resources (+0x20), presidente | 0x83c016 | idem | sim | × (1 + (recursos − 10) × 0,025) |
| Business (+0xf), presidente | 0x597a0d | `Finance::get_max_wage` 0x5968d0 | sim | < 6: teto do salário × 1,05 (× 1,1 com técnico de ambição > 15 e reputação > 7250); > 15: × 0,95 |
| Resources (+0x20), presidente | 0x597a6e | idem | sim | > 15 com ambição do presidente > 15 (e +0x82 = 0): × 1,1 / × 1,125 / × 1,05 pela reputação do clube (2750, 5750) e do jogador (7250) |
| Business (+0xf), presidente | 0x598b24 | `Finance::get_transfer_budget` 0x598800 | não | divisor do orçamento = negócios + 6 (+ (15 − ambição) com ambição > 15; 16 sem presidente; +5 numa nação) |
| Youngsters (+0x22), técnico | 0x8400ae | `fill_shortlist_slot` 0x83f3a0 (resto em 0x840084) | não | vaga que não é de empréstimo, menor de 20 com reputação doméstica (+9) < 3250 e clube atual < 3250: só segue se edi mod (25 − jovens) = 0 (sem técnico, mod 15); com jovens 20 passa 1 em 5, com 1, 1 em 24 [provado o teste; o que é edi, inferido: um contador ou id] |
| Youngsters (+0x22), técnico | 0x4d3ccb | `contract_manager_calc_asking_price` 0x4d1600/0x4d2070 | não | clube da IA (0x600a50 = 0): soma (jovens − 10) a um limiar comparado com a habilidade potencial do jogador (+7 da ficha); abaixo dela, (potencial − limiar) × 0,5 entra no preço [provado o termo; o papel no preço, inferido] |
| JudgingPotential (+0x18), técnico | via `perceived_ability` 0x6e86d0 (ramo `kind` ≠ 2) | `calc_player_value` 0x4d0c60; `value_player_for_club` 0x840d80 (`young_player_adjustment`, menores de 27) | sim | a habilidade potencial vista pelo técnico entra no valor do jogador e na lista |
| Judgement (+0x17) | 0x6e86d0 (`kind` = 2); `know_person` 0x5b0a28 | `perceived_ability`; névoa (`FOG_OF_WAR_MAP::raise_knowledge`) | sim | habilidade atual vista; o conhecimento do observador (olheiro) sobe |
| Interference (+0x16), presidente | 0x8c9a26 | `move_player_to_club` 0x8c9720 | não | venda acima de 2,5 milhões e acima de (recursos + ambição + 750) × reputação do clube: com interferência > random(30), o presidente põe o jogador nos favoritos (0x547a10) |
| Resources (+0x20), presidente | 0x8c99f6 | idem | não | o limiar acima |
| Interference (+0x16), presidente | 0x867941 | `apply_event_flags` 0x867700 | não | com interferência > random(30), o presidente passa a detestar (0x547af0) |
| Interference (+0x16), presidente | 0x58772c | `news_create_not_for_sale_story` 0x587600 | sim | só a notícia "não está à venda" |
| ManHandling (+0x19) | — | nenhuma no mercado | — | só AWOL, multas, mídia, telas, `score_staff_for_job` |

Fora do mercado de jogadores, no **mercado da comissão** (`score_staff_for_job` 0x69d1b0, contratação de técnicos e
auxiliares, `manager_manager.cpp`): Judgement, JudgingPotential, Youngsters, ManHandling, Interference e Patience
(leituras em 0x69d1b0..). E `game_daily_staff_update` 0x5ce2d0 lê Youngsters, ManHandling, Patience, Judgement e
Interference (moral diária; chama a satisfação com o contrato).

## 4. Constantes (lidas do exe)

0x846920: 3,5 (0x968e38), 2,75 (0x9690a8), 1,75 (0x968f98), 3,15 (0x968e58). 0x4ce4e0: 1/300 (0x968e60), 3,15
(0x968e58), 0,99 (0x968e50), 0,6 (0x968e48), 0,9 (0x9689d8), 0,8 (0x9689b0), 0,7 (0x968e40), 0,5 (0x968968). 0x866830:
0,8 / 0,3 / 0,2 / 0,4 / 0,7 / 0,5 (f64), 0,2222222 (f32 0x96ff84), multiplicadores f32 1,01 (0x96ff7c), 1,05
(0x96d3b8), 1,1 (0x96a33c), 1,15 (0x96d390), 1,2 (0x96d7bc), 1,25 (0x96b010), 1,45 (0x96ff74), 1,5 (0x96b004), 1,7
(0x96ff78), 1,75 (0x96b00c), 1,8 (0x96d7b4), 1,95 (0x96ff80), 1,25 f64 (0x968928). 0x5968d0 (presidente): 1,05
(0x9689b8), 1,1 (0x9689c0), 0,95 (0x9689d0), 1,125 (0x96a5a8). Quase todas são compartilhadas por dezenas de rotinas:
mudar a `.rdata` muda o jogo inteiro.

## 5. Defeitos do original

1. `calc_contract_value` (0x866830): sem oferta, lê as cláusulas da oferta por ponteiro nulo (0x866df6); nenhum
   chamador passa nulo [provado]. Sem ficha de jogador a idade entra no lugar de "faixa − idade" (a = idade, 5 depois
   do limite) [provado]. A cláusula de não promoção (caso 0) olha a cláusula de multa mínima da oferta e usa a faixa
   de 7,5 milhões, as outras 5 milhões [provado; intenção inferida].
2. `staff_interest_in_joining_club`: no próprio clube divide por um float da pilha que ninguém escreveu (sem efeito,
   o caminho não usa o resultado; já anotado no Rust).
3. `get_max_wage`: o presidente com **pouco** tino para negócios (< 6) paga **mais** (× 1,05/× 1,1) e o bom (> 15)
   paga menos (× 0,95) [provado]; parece invertido [inferido].
4. Documentação Rust: o cabeçalho de `young_player_adjustment` (shortlist_manager.rs) diz "current ability seen", mas
   chama `perceived_ability` com `kind` 1, que é a potencial (lê JudgingPotential +0x18) [provado].

## 6. Pontos de extensão (o menor gancho que muda a política)

1. **0x846920 (Rust)**: um fator na saída de `staff_interest_in_joining_club` muda a vontade em todos os caminhos
   (lances, empréstimos, renovação, listas da IA). Para grupos (idade, reputação, nação), multiplicar o retorno; os
   limiares dos chamadores (2,75, 3,15, 3,5) ficam.
2. **0x85fc50 (original)**: um desvio que chama a original e ajusta o salário (+0xc) e o valor (+8) na saída muda o
   salário pedido sem portar as 10.896 bytes.
3. **0x5968d0 (Rust)**: o teto do salário do clube (e onde o presidente pesa por Business/Resources).
4. **0x866830 (mkd_1)** e **0x866620 (Rust)**: a satisfação (50/80) e a razão da decisão de renovação.
5. **0x4cda70 (Rust)**: a oferta da IA (fator, duração pela idade, teto × 1,4).
6. **0x83bf40 (Rust)**: a reputação-alvo, onde entram a paciência e os recursos do presidente.
7. **0x6e86d0 (Rust)**: o quanto o técnico enxerga (Judgement/JudgingPotential).

## 7. Itens na fila

- **mkd_1** — `STAFF_CONTRACT::calc_contract_value` (0x866830): `levas/fila/mkd_1/`. Compila com o motor atual (cópia
  própria, sem avisos do item). **Não provado no jogo**: a trava do laboratório estava ocupada e a vigia de memória
  recusou (2,3 GB livres, mínimo 4). Ver `PRONTO.txt`.

Não portados por tamanho (para outra rodada): 0x85fc50 (10.896 bytes), 0x864550 (8.400), 0x862720 (7.728), 0x844780
(8.608), 0x4c7ac0 (9.376), 0x4ce4e0 (4.032). Só de tela (sem efeito na carreira automática): 0x4d4660, 0x4d4830,
0x4d44e0, 0x867ad0, 0x85f7a0.
