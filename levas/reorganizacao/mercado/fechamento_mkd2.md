*Versão em inglês: [fechamento_mkd2.en.md](fechamento_mkd2.en.md). As duas são mantidas juntas; o português é o original.*

# Fechamento do mapa do mercado, leitor D2: 0x4ce4e0 e 0x4c7ac0 (11/11/2026)

Rotinas: `CONTRACT_MANAGER::resolve_contract_offers` (0x4ce4e0, a pessoa escolhe entre as ofertas e a renovação) e
`CONTRACT_MANAGER::daily_update` (0x4c7ac0, fim de contrato, renovações da IA, agentes livres). Tudo abaixo vem da
desmontagem do `jogo/cm0102.exe` (`desmonta.py`), com o endereço da instrução; as constantes lidas do exe. Para cada
afirmação do mapa (`docs/MAPA_MERCADO.md`, seção D, §1, §2.2, §2.4, §2.6, §4, §5, §7): o texto antigo, o novo e a
prova. A sessão principal troca [inferido] por [provado].

## 1. 0x4ce4e0 `resolve_contract_offers` — item **mkd_4** (portado)

Convenção: thiscall(gerente de contratos, pessoa), `ret 4` (0x4cf493), quadro SEH. Chamador único: 0x4cda4c, dentro
da `process_due_decision_dates` (0x4cd9c0), quando vence uma data de decisão do tipo 3. Igual no cm0102-1080.exe
(0xfb6 bytes).

Constantes (lidas do exe): 1/300 = 0x968e60 (0.0033333333333333335); 3,15 = 0x968e58; 0,99 = 0x968e50; 0,6 =
0x968e48; 0,7 = 0x968e40; 3,5 = 0x968e38; 0,0 = 0x9689a0; 0,9 = 0x9689d8; 0,8 = 0x9689b0; 0,5 = 0x968968; 10 =
0x9688b0; 40 = 0x968e30; 5 = 0x9688a0. Os imediatos 2,75 (`mov [esp+0x40], 0x40060000`, 0x4ce51b) e 3,15
(`0x40093333_33333333`, 0x4ceaab) não estão na `.rdata`.

| Afirmação do mapa (§2.2) | Texto novo | Prova |
|---|---|---|
| 1. k = (profissionalismo − ambição − 9 + lealdade) × 1/300 | igual, **[provado]**: o int (+0x5b − +0x57 + +0x59 − 9) passa por `fild` e `fmul` 0x968e60 | 0x4ce5a4..0x4ce5ca |
| 2. Com oferta de transferência pendente adia 2 dias e sai | **[provado]**: `player_has_pending_offer` (0x8cd150); hoje + 2 (0x54c5b0) e uma data de decisão do tipo 3 na fatia atual (0x4cd6d0 com [0xae2c88]); nada mais | 0x4ce5ce..0x4ce606 |
| 3. Oferta sem transferência de clube que não é o do contrato e tipo ≠ 5: removida | **[provado]**, só as ofertas da pessoa com +0x62 ≠ 0; o contrato é procurado com a gravação em [0xae1080]; a remoção tira da lista e desconta o byte +8 do registro de 79 bytes | 0x4ce630..0x4ce6fa |
| Estado 0 (o resto pula; estado 2 marca "já acertada") | **[provado]**: +0x50 ≠ 0 pula; = 2 liga o "acertada", e no fim nada é escolhido | 0x4ce701, 0x4cec69..0x4cec6e, 0x4cec90..0x4cecab |
| Tipo 5: vontade = 0x846920(…, empréstimo 1, estatuto 8); tipo 6 com `get_squad_status_for_club` | **[provado]**; e mais: a vontade é a nota da oferta, escolhida **na hora** se for maior que a melhor e ≥ 3,15 (antes das recusas: uma recusa depois zera a nota, mas a oferta continua escolhida) | 0x4ce70c..0x4ce78c, 0x4ce791..0x4ce827 |
| As outras: desejado (0x85fc50 com −1, estatuto, cargo); razão = valor(oferta)/valor(desejado) − k | **[provado]**: o estatuto (nibble alto de +0x4f) vai antes para o byte +5 da entrada da pessoa na oferta de transferência (0x8c61e0, 0x8e82f0); o desejado é **um só** contrato local reaproveitado de oferta em oferta; os dois valores são contra o desejado (`ecx` = desejado e depois a oferta); valor do desejado ≤ 0 dá nota 0 | 0x4ce82c..0x4ce8f1 |
| A vontade por 0x846920 (jogador) ou 0x844780 (comissão; média num cargo de jogador) | **[provado]**: 0x846920 sem ficha de comissão (+0x69 = 0) ou com cargo 11; senão 0x844780 e, se `is_player_job` (0x53b620), (0x846920 + 0x844780) × 0,5 | 0x4ce8d8..0x4ce9b8 |
| Renovação: razão ≥ 0,99 e vontade ≤ 3,15 → 3,15; razão < 0,6 → 0 | **[provado]**; **corrigido**: o corte em 0,6 vale para **todas** as ofertas comuns, não só a renovação. Renovação = o clube do contrato (procurado de novo, [0xae1080]) é o da oferta | 0x4ce9bc..0x4ceaab (renovação), 0x4ceac5..0x4cead0 (corte) |
| Recusa: cargo de jogador sem ficha de jogador (0x7a6e20 falso), ou razão > 0,5 com permissão e menos de 90 dias | **corrigido [provado]**: (a) cargo **que não é de jogador simples** (`0x53b610`) para quem **não tem ficha de comissão** (+0x69 = 0) e **não sobra registro de comissão** (`PLAYER_REGEN::has_spare_non_player_record` 0x7a6e20 = 0); (b) razão > 0,5, precisa de permissão (0x8c7560) e o fim do contrato **da oferta** a menos de 90 dias (0x54cdb0 de +0x2d até hoje < 0x5a). Vale para todos os tipos (empréstimo e teste inclusive) | 0x4ceae2..0x4ceb70 |
| Nota = vontade × razão, com os cortes 0,9, 0,8, 0,7 e 3,5; a maior acima de 2,75 | **[provado]**: razão ≥ 0,9 → vontade = max(vontade, 3,15); razão > 0,8 e vontade > 3,15 → mantém; razão > 0,7 e vontade > 3,5 → max(vontade, 3,15); senão recusada. Nota = vontade × razão; maior que a melhor (começa em 2,75) → escolhida; renovação não escolhida (ou recusada) fica lembrada. Ofertas de empréstimo/teste (desejado = 0) não passam por aqui | 0x4ceb9b..0x4cec67 |
| (novo) | Uma oferta já acertada, ou uma pessoa com ficha de comissão candidata a uma vaga de técnico (`manager_manager_is_vacancy_candidate` 0x69fc60), não escolhe nada **[provado]** | 0x4cec90..0x4cecab |
| 4. A escolhida vira estado 2 (0x7549b0, 0x8c5410); as outras 1 (0x4d49a0 ou 0x8c28c0) | **[provado]**; `handle_player_rejected_terms` recebe a escolhida como 3º argumento; a notícia 0x4d49a0 recebe 1 só para a renovação recusada de um jogador (cargo 11) quando nada foi escolhido | 0x4cecaf..0x4ced73 |
| Sem escolha e com oferta do próprio clube: marca 0x80 no byte +0xb | **[provado]**, mais preciso: sem escolha, uma renovação **recusada** liga o bit 0x80; com escolha, o bit é **apagado** no fim (salvo quando a conclusão falha) | 0x4cf071..0x4cf094, 0x4cf42d..0x4cf447 |
| Para a escolhida (tipos ≠ 5, 6) com nota acima de 0x9688b0, sorteios pela lealdade [inferido] | **[provado]**: nota > 10 → n = max(40 − nota, 5); `random(ftol(n))` < lealdade, clube da oferta com técnico, e reputação do clube > 4500 ou `random(4)` = 0 → `random(2)` ≠ 0: passa a gostar do técnico (`staff_add_favoured_staff`); = 0: do clube (`staff_add_favoured_club`) | 0x4cef64..0x4cf06f |
| (novo) Terceira passada | **[provado]**: com escolha, as ofertas do mesmo clube sem transferência **concluem** o contrato (`complete_accepted_contract_offer` 0x8c38b0 com a cópia local; sucesso: registro de comissão para cargo de comissão sem ficha, técnicos e listas de olheiros se o clube do contrato for humano, valor do jogador recalculado); saem da lista e limpam a marca de acerto; com transferência ficam. As de outro clube saem (vão para a lista guardada +0x38 se a escolhida tiver transferência; o clube vai para a lista que a 0x8c2180 recebe). Sem escolha, sai toda oferta fora do estado 3. Conclusão que falhou termina aí | 0x4cf096..0x4cf3a2 |
| (novo) Fim | **[provado]**: escolhida com transferência → `handle_player_accepted_terms` (0x8c2180: pessoa, vaga, cópia, o dword do cargo +0x61, a lista dos clubes), antes um registro de comissão se for cargo de comissão sem ficha; sem transferência → notícia 0x4d49a0 da cópia | 0x4cf3a8..0x4cf42d |

### Defeitos achados (0x4ce4e0)

1. O registro de comissão sobrando é aceito até o **próprio total** (`cmp eax, [0xae2368]; jg` é o erro, devia ser
   `jge`) **[provado]** 0x4cf1b5.
2. Com várias ofertas do **mesmo clube** e a escolhida sem transferência, `complete_accepted_contract_offer` roda
   **uma vez por oferta** (o laço chama de novo a cada uma) **[provado]** 0x4cf0c9..0x4cf15c.
3. Empréstimo e teste são escolhidos antes das recusas: uma recusa depois (permissão de trabalho, falta de registro)
   zera a nota guardada, mas não desfaz a escolha **[provado]** 0x4ce75c..0x4ce764 contra 0x4ceb68.
4. O marcador "renovação" e a vontade são variáveis da pilha que atravessam as ofertas: o primeiro valor do
   marcador é o numerador de k (0x4ce5bc) e uma oferta com valor desejado negativo usaria os de uma oferta anterior
   **[provado]**; sem efeito na prática (o valor nunca é negativo) **[inferido]**.

### Pontos de extensão

- **0x4ce4e0 (mkd_4, Rust quando integrado)**: o limiar 2,75, os cortes 0,9/0,8/0,7, o piso 3,15 da renovação e o
  k da personalidade ficam numa função só (`choose_contract_offer`): o lugar para o Tweaks mexer na disposição de
  renovar ou de trocar de clube por grupos (idade, reputação), sem tocar a `staff_interest_in_joining_club`.

## 2. 0x4c7ac0 `daily_update` — **não portado** (ver §3)

Convenção: thiscall(gerente de contratos, &data, fatia (byte), sinal (byte)), `ret 0xc` (0x4c9e2e). Chamadores:
`game_main_day_loop` (0x5cd406: hoje, a fatia atual [0xae2c88], 1) a **cada fatia** do dia, e `game_run_to_start_date`
(0x5cc9ef). Constante: 0,004 = 0x968d60 (passos da barra de progresso).

| Afirmação do mapa (§2.4, §2.6) | Texto novo | Prova |
|---|---|---|
| "Diária" | **corrigido [provado]**: roda em cada fatia; **fatia 0** só recalcula as datas fixas (hoje + 7, 30, 60, 91, 121, 152, 182, 365, 550, 1095 em 0xae1048, 0xae1030, 0xae1058, 0xae1010, 0xae1060, 0xae1040, 0xae1090, 0xae1068, 0xae1008, 0xae1020); **fatia 2** faz o passe das pessoas (só com o sinal = 1); toda fatia termina nas datas de decisão (0x4cd9c0 com a data e a fatia) | 0x4c7b2e..0x4c7c74, 0x4c7c79, 0x4c7de0, 0x4c9e0e..0x4c9e15 |
| (novo) Conversas de contrato | **[provado]**: na fatia 2, cada registro de conversa (21 bytes, +0x28/+0x2c) vence em (25 − lealdade do técnico) × 2 dias desde a data +0xc; vencido sai (troca pelo último) e o vetor é reordenado (qsort 0x4dc970). É o **primeiro uso da lealdade** | 0x4c7c81..0x4c7d7b |
| (novo) Barra "Updating game data" | **[provado]**: em 31/12 (0x549f70 dia 31, mês 11) a cada 250 pessoas | 0x4c7b07..0x4c7b29, 0x4c7d7e..0x4c7e2f |
| (novo) Antes da carga completa | **[provado]**: com [0xdd711c] = 0 (`G_GAME_DATA_LOADED_FLAG`), o contrato que vence hoje só vira tipo 1 (+0x35 = (x & 0xc1) \| 1) | 0x4c7e34..0x4c7ec0 |
| (novo) Valor do jogador recalculado | **[provado]**: clube humano e cargo de jogador: com o bit 4 de +0x4e e ficha, valor recalculado (0x4d0c60 sinal 1); clube da IA e cargo de jogador: **nos dias ímpares do ano**, bit 4 ligado e valor recalculado (sinal 0); comissão: todo dia; nos dois primeiros, limitado pela multa de rescisão ativa (0x867b50). Sem contrato ou clube: valor 0 | 0x4c7f65..0x4c8070 |
| Fim do empréstimo (0x4d07e0) | **[provado]**: o contrato de empréstimo (índice +4 do registro de 79 bytes) que vence hoje: sem clube, só o índice vira −1; senão notícia 6 (0x4da590, fora do tipo 7) e `end_loan` | 0x4c8077..0x4c8114 |
| (novo) Renovação antes da mudança das regras de transferência | **[provado]**: antes de [0xae2330] (`G_TRANSFER_RULE_CHANGE_DATE`) e com ficha, quando essa data cai em hoje + 7/30/42/60/70/91 (recalculadas **por pessoa**, em 0xae1070..0xae1088), o contrato começou há mais de 180 dias e acaba antes de hoje + 1095: clube que não o rejeita e cujo técnico não o rejeita (0x5477d0, 0x547840), da IA, idade < 32 → pelo estatuto: 1 oferta (fator 1,0); 2 e 3 com reputação atual > 6750, ou idade < 24 e potencial > 100; 5 com potencial > 100; 4 nunca | 0x4c8152..0x4c847a |
| Período protegido da FIFA (0x868210) | **[provado]**: contrato tipo 2, sem empréstimo, que acaba **depois** de hoje + 182, fora do período protegido (0x868210 = 0 e a saída ≠ 0), sem aposentadoria (0x7a7eb0), cargo 11: a IA (idade < 32) renova pelos mesmos estatutos (fator 1,0); clube humano: notícia 7 (0x4d6790; Bosman pela idade em hoje + 182 ≥ 24 e 0x8d0a70) | 0x4c847f..0x4c85fe |
| (novo) Tipo 1 (fim de contrato vencido) | **[provado]**: sem empréstimo, passados 31 dias do fim, querendo sair (0x866f60) ou saindo pelo Bosman (+0x49), sem ofertas nem trocas nem fila: notícia 1 ao clube humano; cargos 12, 13 (IA) e 15 (IA) voltam a 5, 6 e 8 e saem do elenco; os outros vão à **fila de transferência livre** (0x8c16b0 com 1, −1) | 0x4c8603..0x4c883b |
| Bosman (0x8d0a70, nação com transferência livre 0x544670) | **[provado]**, a regra da **saída pelo Bosman** (+0x49 = 1): tipo 2, contrato começado há mais de 93 dias, na janela das faixas (0x867d30 ≠ 0 e ≠ 14), sem aposentadoria, cargo 11, contrato atual fora de empréstimo, clube com nação; reputação atual > 3750, mundial > 1750, **lealdade < 15**, idade 19..30, a nação permite (0x544670), e não (clube > 4750 e idade < 23); e querer sair (0x866f60) ou o bit 8 de +0x4e. Então +0x49 = 1, bit 8 apagado, listas dos clubes (0x842c00) e a notícia 0x4d6d70. É o **segundo uso da lealdade** | 0x4c88b5..0x4c8a74 |
| A faixa (0x867d30) contra +7..+182 | **[provado]**: as faixas ímpares (o dia exato de 182, 152, 121, 91, 60, 30 e 7 dias) põem o jogador nas listas dos clubes (0x842c00 com 1); a faixa 14 só com `random(20)` = 0; quem já sai pelo Bosman também, nas ímpares | 0x4c8876..0x4c88a5, 0x4c8a79..0x4c8abb |
| A oferta de renovação da IA (`offer_contract`, ×9) | **[provado]**, pelos imediatos do fator (f64) empilhados antes de 0x4cda70 (pessoa, clube 0, saída 0, vaga −1, fator, modo 2), só para clube da IA e jogador sem aposentadoria: faltando **550 dias** (0xae1008), estatuto 1..3 e na lista do próprio clube (0x849ce0): 1,0; **365 dias** (0xae1068, ou no primeiro dia do jogo com fim antes disso): notícia 6 e, estatuto 1..3 ou 5 na lista: 1,0; faixas **3 e 5** (152/121 dias): estatuto 1..3 na lista 1,2, fora 1,0; 4 e 5 → 1,0; faixa **1** (182): 1,1 na lista / 1,0; faixa **7** (91): 1,3 na lista / 1,0, 1,1 para 3..5; faixas **9, 11** (60/30): 1,4 (1..2) e 1,2 (3..5) na lista, fora 1,0; faixa **13** (7 dias): o mesmo, com notícia ao clube humano. Quando a 0x4cda70 devolve 0 (a oferta não foi feita) nos caminhos de 365 dias e das faixas 1 e 7, uma oferta local de tipo 2 vira a notícia de recusa (0x4d49a0 com 1); nas faixas 1 e 7 liga também o bit 0x80 do registro de 79 bytes (0x4c910e, 0x4c91e3, 0x4c93d0, 0x4c94a5) | 0x4c8ac0..0x4c8b9f, 0x4c8ba4..0x4c8e0a, 0x4c8e0f..0x4c8ee3, 0x4c8ee8..0x4c91f1, 0x4c91f6..0x4c9550, 0x4c9555..0x4c9a8b, 0x4c9b5f..0x4c9b71; fatores 1,1 = 0x3ff19999_9999999a, 1,2 = 0x3ff33333_33333333, 1,3 = 0x3ff4cccc_cccccccd, 1,4 = 0x3ff66666_66666666 |
| O fim do contrato no dia | **[provado]**: tipo 5 (empréstimo) que acaba hoje: notícia 5, estatísticas arquivadas (0x7c5fe0), o contrato apagado (os dois índices −1), folha salarial (0x598370), fora das escalações (0x896c60 ×2) e do clube (0x53c190). Outro tipo: vira tipo 1; saindo pelo Bosman, sem empréstimo e sem ofertas: notícia 2 e **liberado na hora** (`free_transfer_staff` 0x8c1af0) | 0x4c9b76..0x4c9d21 |
| (novo) Listado sem ofertas | **[provado]**: jogador (cargo 11) listado para transferência (0x5444a0) sem ofertas nem trocas nem fila: entra na fila de transferência livre | 0x4c9a90..0x4c9b10 |
| (novo) Datas de decisão | **[provado]**: nos dias 36 e 200 do ano (fatia 2), as já processadas saem do vetor; em toda chamada, se houve datas novas (+0x1c), a parte pendente é reordenada (qsort com 0x4dc910) e o +0x1c zera | 0x4c9d40..0x4c9dd5, 0x4c9dd8..0x4c9e07 |
| "Atributos: só lealdade (+0x59, ×2)" | **[provado]**: conversas (0x4c7cc3) e Bosman (0x4c8a01) | — |

### Defeitos achados (0x4c7ac0)

1. As seis datas da mudança de regras (0xae1070, 0xae1018, 0xae1038, 0xae1028, 0xae1078, 0xae1088) são recalculadas
   **para cada jogador** do laço, não uma vez por dia (mesmo resultado, trabalho à toa) **[provado]** 0x4c81a3..0x4c827c.
2. O valor do jogador da IA só é recalculado nos dias **ímpares** do ano (`movsx eax, word [data]; and 0x80000001`),
   o do humano todo dia **[provado]** 0x4c7ffe..0x4c8010; intenção [inferido].

### Pontos de extensão

- O fator das renovações da IA (1,0..1,4) é um imediato empilhado em cada uma das 9 chamadas da 0x4cda70: um desvio
  na `contract_manager_offer_contract` (Rust) que ajuste o fator pelo jogador muda todas sem portar a 0x4c7ac0.
- A regra do Bosman (lealdade < 15, idade 19..30, reputações 3750/1750) está num trecho só (0x4c89ea..0x4c8a4a).

## 3. Por que 0x4c7ac0 não virou item

São 9.376 bytes (2.700 instruções, 10 tabelas de saltos), ~45 rotinas chamadas (a barra de progresso da interface
inclusive), 9 caminhos de renovação e o fim de contrato; uma rotina por item, tudo de uma vez, é grande demais para
provar com uma comparação (não roda duas vezes: notícias, sorteios, filas). Proposta para outra rodada: portar em
pedaços pelos chamadores já em Rust (a parte do valor do jogador, o fim do empréstimo, a regra do Bosman e as
renovações por faixa como ajudantes), com o laço por último.

## 4. O que muda no mapa

- §1: 0x4ce4e0 → item **mkd_4**; 0x4c7ac0 → "fatia 0 datas, fatia 2 pessoas", sem item (§3 acima).
- §2.2: trocar [inferido] por [provado] com as correções da tabela 1 (o corte 0,6 para todas; a recusa é de cargo de
  comissão sem ficha e sem registro; o sorteio pela lealdade provado; o bit 0x80 é limpo na escolha).
- §2.4: trocar [inferido da lista de chamadas] pela tabela 2 (as fatias, a regra do Bosman, os fatores 1,0..1,4 por
  janela, os 550/365 dias, o tipo 1 e a fila depois de 31 dias, o valor nos dias ímpares).
- §2.6: quem fica livre: no dia do fim, só quem sai pelo Bosman é liberado na hora; os outros ficam tipo 1 e, 31
  dias depois, só vão à fila se quiserem sair.
- §5: acrescentar os defeitos 1–4 de 0x4ce4e0 e 1–2 de 0x4c7ac0.
- §7: mkd_4 (compilado; prova pela sessão principal).
