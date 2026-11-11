# Mercado, seção C: empréstimos, janelas e limites (leitor C, 11/11/2026)

*English version: [secao_c.en.md](secao_c.en.md)*

Parte do mapa do mercado pedido pelo Tweaks 1.5 ([BRIEF.md](BRIEF.md)). **[provado]** = lido na desmontagem ou no
Rust já comparado (0 divergências); **[inferido]** = conclusão do código, não reproduzida no jogo.

## 1. As rotinas

| Endereço | Nome | Em Rust | Item |
|---|---|---|---|
| 0x842740 | `Shortlist_Manager::try_loan_offer_for_player` | sim (`try_loan_offer_for_player`) | — |
| 0x8d0590 | `TRANSFER_MANAGER::get_loan_period` | sim (`transfer_manager_get_loan_period`) | — |
| 0x8d0040 | `TRANSFER_MANAGER::can_loan_player` | sim (`transfer_manager_can_loan_player`) | — |
| 0x8d0530 | `TRANSFER_MANAGER::can_end_loan` | sim (`can_end_loan`) | — |
| 0x8c63a0 | `TRANSFER_MANAGER::process_due_offer` | sim (`process_due_offer`) | — |
| 0x8c5880 | `TRANSFER_MANAGER::daily_update` (a varredura semanal das ofertas) | sim | — |
| 0x8c2180 | `TRANSFER_MANAGER::handle_player_accepted_terms` (o empréstimo se completa aqui) | sim | — |
| 0x8cb810 | `TRANSFER_MANAGER::add_past_transfer` | sim (`add_past_transfer`) | — |
| 0x8cd450 | `TRANSFER_MANAGER::check_squad_limits` | sim | — |
| 0x8c6bc0 | `TRANSFER_MANAGER::update_transfer_window_news` | sim | — |
| 0x8cf470 | `TRANSFER_MANAGER::find_next_window_open_date` | sim | — |
| 0x8c4e10 | `TRANSFER_MANAGER::hold_offers_at_window_close` | sim | — |
| 0x4d01e0 / 0x4d07e0 / 0x4ccca0 | `CONTRACT_MANAGER::sign_loan_contract` / `end_loan` / `schedule_loan_end` | sim | — |
| 0x8e8bd0 | `TRANSFER_RULES::get_loan_period` (o período padrão) | não | **mkc_1** |
| 0x8e8e50 | `TRANSFER_RULES::find_open_window` | não | **mkc_1** |
| 0x8e8c40 | `transfer_window_contains_date` | não | **mkc_1** |
| 0x5c1ed0 | `FRANCE_RULES::can_end_loan` | não | **mkc_2** (8 ligas) |
| 0x6618f0 | `ITALY_RULES::can_end_loan` | não | **mkc_2** (8 ligas) |
| 0x412dd0 | `AUSTRALIA_RULES::can_loan_player` | não | **mkc_2** (8 ligas) |
| 0x579c70 / 0x579d60 / 0x579fd0 / 0x57a020 | Inglaterra: período, `can_loan_player`, `record_loan`, `daily_update` | sim | — |
| 0x7f6530 / 0x7f6650 | Escócia: período, `can_loan_player` | sim | — |
| 0x661770 / 0x63c3d0 / 0x859630 | `can_loan_player` da Itália, Irlanda, Espanha | sim | — |
| 0x40a600 / 0x40a610 | base `can_loan_player` / `can_end_loan` (`mov eax, 1; ret`) | não | não dá para desviar (2 bytes) |
| 0x5a9a80 / 0x5a9af0 / 0x5a9ba0 | Finlândia: período, `can_loan_player`, `can_end_loan` | não | fora: não roda nas 8 ligas |
| 0x794760 / 0x902240 | Noruega, Turquia `can_loan_player` | não | fora (idem) |
| 0x793870 / 0x793900 / 0x7ec8e0 / 0x90f1a0 | Irlanda do Norte (período, `can_loan_player`), Rússia e País de Gales (período) | não | fora (idem) |
| 0x5e0210 / 0x63c530 / 0x63c4e0 | Alemanha `can_end_loan`, Irlanda `can_end_loan` e `record_loan` | não | fora (idem) |
| 0x8e8560 / 0x8e8800 | base `can_sign_player` / `can_make_offer` (limites de estrangeiros) | não | mapeadas aqui, não portadas (são das contratações, leitor B) |
| 0x784100 / 0x7874b0 / 0x5fd2f0 | `NewsItem::auto_respond`, `News::auto_respond_all`, `HUMAN_MANAGER::set_on_holiday` | não | mapeadas (defeito das férias) |

## 2. O pedido de empréstimo da IA (0x842740) [provado, Rust]

Chamado por `make_club_bids` (0x84a020, a passada da IA pelas 40 vagas da lista), e o `can_loan_player` antes, pelo
`fill_shortlist_slot` (0x83f3a0) e pelo próprio `make_club_bids` (0x84aa2b). Ordem:

1. Sem dono (contrato principal 0x4cf5a0 nulo): a vaga da lista volta ao estado 1; −1.
2. Interesse do jogador no clube (`staff_interest_in_club`, 0x846920, com o estado da vaga) abaixo de **3,5**
   (`.rdata` 0x968e38): −1.
3. O período (`get_loan_period`, §3): meses até o fim `(ano_fim − ano)·12 − mês + mês_fim`, até o início idem, no
   mínimo 1; início depois do fim: −1. **É aqui que o empréstimo de fim de contrato morre para a IA** (§7).
4. Clube da Noruega (`KEY_NATION_NORWAY`) com o mês de hoje em novembro a março: −1.
5. Interesse sem salário ≥ **2,75** (0x9690a8) e nação do clube fora de Espanha, Portugal e Itália: conta os meses
   até o início; senão até o fim.
6. A data de volta: dia de hoje (no máximo 28) no mês hoje + meses. `can_end_loan` (0x8d0530, flag 1) = 0: −1. Volta
   depois do fim do contrato principal (+0x2d/+0x2f): −1. Volta − 4 semanas antes da data da janela da nação-chave do
   clube (`[0xb63d8a + 72·key_nation_index_of_club]`): −1. Volta antes de hoje + 3 semanas: −1.
7. Salário pago: 100% se a reputação do clube (`club.reputation()`) passa a do dono em mais de 2500, 0% se fica
   1500 ou mais abaixo, senão 50%. A oferta: `make_loan_offer` (0x8bed60).

Quem decide a resposta do dono é a 0x84af00 (`predict_selling_club_response`, leitor de outra seção).

## 3. O período do empréstimo (0x8d0590) [provado, Rust]

`TRANSFER_MANAGER::get_loan_period(club, clube_do_contrato, jogador, &ini_mês, &ini_ano, &fim_mês, &fim_ano)`,
thiscall ret 0x1c. Chamadores: 0x842740 (IA), 0x8d0040 (`can_loan_player`), 0x8f9690 (tela do empréstimo do humano),
0x8f1c30 (responder ao lance de empréstimo), 0x888260/0x889c30 (perfil e comparação). 203.886 chamadas até
2002-06-01 na carreira inglesa (contadores do item cqe_6).

1. A regra da nação do **clube que pede** (das 28 em `TRANSFER_RULES` +0x8ac; sem ela, a primeira) dá o período pelo
   virtual +0x28 (`GET_LOAN_PERIOD`):
   - base 0x8e8bd0 (**mkc_1**): do mês seguinte ao mês do fim da temporada da nação do clube (0x4d45f0), neste ano se
     ainda à frente do início, senão no seguinte;
   - Inglaterra 0x579c70: do mês seguinte a 3 meses depois; depois de 19 de junho e jogador com menos de 24 anos, a base;
   - Finlândia 0x5a9a80: como a base; Rússia 0x7ec8e0: início daqui a 3 meses, fim no fim da temporada; Irlanda do
     Norte 0x793870: idem, e o início não passa do fim; País de Gales 0x90f1a0: o início = o fim (fim da temporada).
2. O contrato principal do jogador (0x4cf5a0). **Sem contrato, tipo (+0x35 & 0x3f) diferente de 2 ou hoje depois do
   fim do contrato: o ano do fim vira 1972** (o ano da data padrão 0x54a110) — um sentinela "impossível".
   Os tipos (tela 0x87cee0, tabela de saltos 0x87dd58) [provado]: 0 inválido, 1 mensal, **2 contrato normal**,
   5 experiência, 6/7 empréstimo; bit 0x80 = meio período.
3. Fim mais de 5 anos à frente: 1972.
4. Enquanto a data (dia do mês de hoje, mês do fim, ano do fim) passa o fim do contrato (+0x2d dia do ano, +0x2f
   ano), o fim recua um mês, o início acompanha se ficar depois do fim; o ano do fim abaixo do atual: 1972.

## 4. Pode emprestar? (0x8d0040) [provado, Rust]

`can_loan_player(clube, jogador, mensagem)`, thiscall ret 0xc, 1 pode. Chamadores: 0x83f3a0 e 0x84a020 (IA, sem
mensagem), 0x8f9690 (tela do humano, com mensagem em 0xde1f64).

1. Sem clube atual (contrato fora o tipo 5): 0. Já emprestado (contrato de empréstimo 0x4cf6d0): 0. Não é jogador
   (`job_for_club() != PLAYER`): 0.
2. `can_club_bid_for_player` (motivos 0x21 em negociação, 0x27, 7, 1, 0x12/0x13, 0x19/0x28/0x1a nação): 0 com o texto.
3. A regra da nação do **clube que pede**, virtual +0x20 (`CAN_LOAN_PLAYER`) = 0: 0. Por nação:
   - base 0x40a600 / Grécia 0x5ed060: 1; Irlanda do Norte 0x793900: 1 (grava `[0xae1080]`);
   - Inglaterra 0x579d60: mesma divisão (só a Premier) 0x1d; **5 por temporada** (byte 0 de
     `[[0xde1f54]+0x91c] + id·16`, zerado em 20 de junho por 0x57a020, somado por 0x579fd0) 0x1e; **mais de 1** já
     emprestado de fora no elenco (o 3º é recusado) 0x26; fora da janela (0x8e8e50) 9;
   - Austrália 0x412dd0 (**mkc_2**), Noruega 0x794760, Turquia 0x902240: fora da janela, 9 ("The deadline for loans to
     ... clubs has passed."); Finlândia 0x5a9af0: idem e, para clube da IA, só entre os dias 90 e 300 do ano;
   - Itália 0x661770, Espanha 0x859630, Escócia 0x7f6650, Irlanda 0x63c3d0: em Rust (janelas e limites por divisão).
4. A regra da nação do **clube atual** (o que empresta): sem regra, 1.
5. O período (§3). **Início = fim: 1** (sem mais nada). Senão, mês a mês do início ao fim (inclusive), o virtual +0x24
   (`CAN_END_LOAN`, flag 1) da regra do dono; o primeiro mês que deixa encerrar dá 1; passou do fim: 0, "This loan
   deal cannot go ahead". Por nação (+0x24):
   - base 0x40a610: 1 (Inglaterra, Escócia, Espanha...: o laço responde 1 no primeiro mês);
   - França 0x5c1ed0 (**mkc_2**) e Irlanda 0x63c530: a data numa janela da nação;
   - Itália 0x6618f0 (**mkc_2**): com flag, junho; senão a data na 1ª ou 2ª janela italiana;
   - Finlândia 0x5a9ba0: recusa entre 18 de agosto e 15 de novembro;
   - Alemanha 0x5e0210: sem flag, recusa encerrar entre clubes alemães (com flag, 1).

## 5. Janelas [provado]

Regras de transferência: +4 a tabela das janelas (12 bytes: +0 abertura, +6 fechamento; cada data 6 bytes: +0 regra,
+1 número, +2 dia da semana para onde anda (0xff nenhum) [inferido], +3 dia, +4 mês (0 = janeiro), +5 1 abre / 0
fecha), +8 quantas, +0xa a nação, +0x13 limite de estrangeiros, +0x14 limite de não comunitários.

- `transfer_window_contains_date` 0x8e8c40 (**mkc_1**): as duas datas no ano da data, pontas incluídas; a janela que
  vira o ano (mês de abertura > mês de fechamento) está aberta da abertura a dezembro e de janeiro ao fechamento.
- `find_open_window` 0x8e8e50 (**mkc_1**): a primeira janela com a data; **a nação sem janela nenhuma (Dinamarca) está
  sempre aberta**; com janelas e nenhuma aberta, 0.
- Datas (construtores, Rust): Inglaterra 2 de junho (segunda) a 26 de março (quinta); EUA 7 de janeiro a 15 de agosto;
  Austrália 11 de junho a 17 de abril; França 30/5–31/8, 1/9–20/12, 21/12–10/1; Finlândia 16/11–15/8; Croácia
  1/6–30/3; Dinamarca nenhuma.
- Avisos (0x8c6bc0, todo dia, fatia 0): no dia, a abertura/fechamento; no dia seguinte à abertura, as ofertas que
  esperavam a janela daquela nação (0x8c4e10); uma semana antes.

## 6. Limites que barram a contratação [provado]

- **Elenco** (`check_squad_limits` 0x8cd450, Rust, pelas restrições da oferta 0x8cfc90 para quem compra e quem
  vende): 50 jogadores (clube + o B), 5 técnicos, 3 fisioterapeutas, 7 olheiros e o auxiliar; acima, a IA compradora
  sem troca dispensa de graça o mais barato que pode sair; senão o motivo 4 ou 0x22 (+1 para o vendedor).
  No empréstimo, a conclusão (0x8c2180) recusa se o elenco de quem pega + 1 passa de 50 (motivo 4) e pergunta de novo
  o `CAN_LOAN_PLAYER` da nação de quem pega.
- **Estrangeiros** (base `can_sign_player` 0x8e8560, virtual +0x10, não portada): conta no elenco (50 vagas +0xd7) os
  estrangeiros (`database_is_staff_foreign` 0x53d980) até o limite +0x13 (padrão 50; 0xff sem limite): atingido,
  motivo 0x14. Clube turco fora da Premier turca: limite 0 (nenhum estrangeiro). Com a nação da regra na Europa
  (`KEY_CONTINENT_EUROPE`), os não comunitários (0x53d740) até +0x14 (padrão 30): motivo 0x15; Espanha: 4 na
  Primeira, 3 na Segunda, 0 nas outras. Depois a janela: fechada, 9. `can_make_offer` 0x8e8800 (+0xc) faz a mesma conta
  com os jogadores da troca e só olha a janela quando quem decide é o comprador. Sobrescrevem: Japão (5 estrangeiros),
  Portugal, Espanha, Itália, Alemanha, Bélgica, França, Argentina.

## 7. Defeito suspeito 1: o empréstimo em fim de contrato

**O caminho [provado]:** §3 passo 2–4 e §4 passo 5, com §2 passo 3.

- Contrato que **não é do tipo 2** (mensal 1, experiência 5) ou já vencido: o período sai com o ano 1972.
  - A IA (0x842740) calcula meses até o fim negativos: nunca pede.
  - O `can_loan_player` **não reconhece o sentinela**: início ≠ fim, então testa o mês do início pelo `CAN_END_LOAN` do
    dono; com o dono inglês (base = 1) **devolve 1**. A tela do humano (0x8f9690) abre com os campos 5/6 (fim) = mês e
    **1972** [provado: os valores vão para os campos 3–6 e 0xc/0xd da tela]. Com dono francês/italiano/irlandês/
    finlandês fora da janela, sai "This loan deal cannot go ahead".
- Contrato tipo 2 que **termina no mês corrente**: o fim recua até este mês e o início acompanha (início = fim):
  - `can_loan_player` devolve 1 (início = fim);
  - a IA calcula meses até o fim 0 < 1: nunca pede;
  - o humano recebe um período de zero meses (volta neste mês) [o que a tela faz com ele: inferido].
- Contrato que termina **no mês seguinte**: um mês de empréstimo, a IA pede se a volta ficar a 3 semanas ou mais.

**É defeito do original?** Sim, de consistência [provado]: o `get_loan_period` usa 1972 para dizer "não dá", mas o
`can_loan_player` só barra pela regra de encerrar do dono e aceita início = fim, então o humano é liberado a abrir
uma proposta com fim impossível (ou de zero meses), enquanto a IA recusa calada. Que isso é exatamente o "não consigo
pegar emprestado jogador em fim de contrato" do fórum (58.2) é **inferido** (não reproduzido no jogo).

**Correção pequena:** um desvio na 0x8d0040 (aqui já é Rust; no Tweaks, prólogo `6a ff 68 1b 40 96 00`, 7 bytes)
que, depois do período, devolve 0 com uma mensagem clara quando o ano do fim é o da data padrão (1972) ou o fim fica
antes do início; ou, como política mais generosa, um desvio na 0x8d0590 (prólogo `83 ec 10 32 d2`) que chama a
original e, quando o sentinela veio do contrato (tipo ≠ 2 ou recuo abaixo deste ano), põe o fim no mês do fim do
contrato e o início no mês seguinte a hoje, se couber.

## 8. Defeito suspeito 2: ofertas presas com o técnico de férias

**O que o código diz [provado]:**

- Em férias, `human_manager_is_human_club(clube, 0, 1, 0, 0)` (0x600a50, conferindo as férias) diz **não**: as ofertas
  novas seguem os ramos da IA no `process_due_offer` (0 → resposta da IA; 1 → `submit_loan_contract_offer`; 5/6 →
  contrato ao jogador). O estado 0x11/0x12 usa `news_club_has_human_manager` (0x7875e0, **ignora** as férias): manda a
  notícia 0xfa0 e espera 28 dias.
- Notícia entregue com o técnico ausente é respondida na hora (`News::deliver_to_manager` 0x785d40 →
  `NewsItem::auto_respond` 0x784100). Ao **entrar** de férias, `set_on_holiday` (0x5fd2f0, em 0x5fd3d9/0x5fd3ed)
  responde tudo o que está na caixa (`auto_respond_all` 0x7874b0) — só os itens que ainda estão no anel de 100.
- O `auto_respond` cobre: 0xfa0 (oferta; empréstimo → 0x84af00 → aceita 0x8c0660 / recusa 0x8bfa30), 0xfa2, 0xfa4 (só
  põe `RESPONSE_ROUND` = 3), 0xfa5 (confirmação → 0x8cfc90 → `club_rejects_offer`/`confirm_offer_by_club`), 0xfa8,
  0xfab, 0xfac (conversa de contrato → `offer_contract_to_offer_player`), 0xfb4, 0xfb5 (pedido de empréstimo),
  0xfb6, 0xfc0 (negociação de empréstimo → `submit_loan_contract_offer`).
- O `process_due_offer` **não tem ramo** para 8, 9, 0xb, 0xc, 0xd e 0x13 (a 0x13 é das transferências futuras; a 8 é
  do gerente de contratos; a 0xa das licenças). A varredura semanal do `daily_update` (dia múltiplo de 7) reagenda
  para amanhã toda oferta vencida, **menos** 0x13, 7, 8 e 0xa: as 9, 0xb, 0xc e 0xd voltam todo dia e nada acontece.

**O defeito [inferido, não reproduzido]:** uma oferta que esperava o humano (9: o comprador humano tem de oferecer o
contrato; 0xb–0xd: as confirmações dos clubes) quando ele entra de férias só anda se a notícia correspondente ainda
estiver na caixa; se saiu do anel de 100 (ou ele apagou), fica parada as férias inteiras, e o jogador continua "com
oferta" (bit 0 do registro de 8 bytes em `PERSON_FLAGS`), o que barra outras ofertas por ele. É defeito do original:
o despacho depende da notícia, não do estado da oferta.

**E os empréstimos?** O empréstimo não passa por 9 nem por 0xb–0xd: completa na hora em que o jogador aceita
(0x8c2180). Os estados de empréstimo que esperam gente são 0 (dono humano: 14 dias e caduca), 1 (negociação com
comprador humano: caduca) e 8 (decisão do jogador, no gerente de contratos 0x4cc930/0x4cd9c0). O único que **não
caduca** é o 8, e a varredura semanal o exclui; um empréstimo cuja decisão do jogador não chega fica preso. Que é isso
o item 88 do xeno é **inferido**; o caminho exato que perde a decisão não foi achado.

**Correção pequena:** no `process_due_offer` (aqui Rust; no Tweaks, prólogo `6a ff 68 00 3d 96 00`), para o estado
9/0xb/0xc/0xd com o humano envolvido de férias (`news_club_has_human_manager` ≠ 0 e
`human_manager_is_human_club(…, 0, 1, 0, 0)` = 0), fazer o que o `auto_respond` faria: 9 →
`offer_contract_to_offer_player`; 0xb–0xd → `check_offer_restrictions` e `confirm_offer_by_club` ou
`club_rejects_offer`. Para o 8, um prazo: oferta de empréstimo em 8 há mais de N dias (a data `DUE_DATE` já vencida há
uma semana) → `reject_loan_offer`.

## 9. Como o empréstimo termina e volta [provado, Rust]

- **Assinatura:** `handle_player_accepted_terms` (0x8c2180) → `sign_loan_contract` (0x4d01e0): o contrato vai para a
  vaga de empréstimo (+4 do registro de 79 bytes), o jogador entra no elenco de quem pega (função 0xb), bit 0x40 do
  contrato principal = reservas; histórico `add_loan_records`; `record_loan` da nação de quem pega (Inglaterra conta
  a temporada); `add_past_transfer` tipo 2; oferta 0x14 e removida em 5 dias.
- **Fim natural:** `CONTRACT_MANAGER::daily_update` (0x4c8114) chama `end_loan` (0x4d07e0) na data do fim
  [inferido do ponto de chamada].
- **Fim antecipado:** a tela (0x8fbcc0) pergunta `can_end_loan` (0x8d0530: só quando o clube dono do jogador é da
  mesma nação que quem pede; senão 1) → `schedule_loan_end` (0x4ccca0: o fim do contrato de empréstimo vira amanhã,
  tipo 7, notícia 0xfb8 se houver humano) → `end_loan` no dia seguinte.
- **Transferência definitiva:** `move_player_to_club` (0x8c9d5d) encerra o empréstimo antes.
- **`end_loan`:** tira do elenco e das táticas de quem pegou, libera o contrato de empréstimo, refaz as folhas, devolve
  ao dono com a marca de elenco (com time reserva: 2 se o principal tem o bit 0x40, senão 1), inscrição europeia 0x10
  → 4.

## 10. O histórico de transferências (0x8cb810) [provado]

Registro de 37 bytes (0x25): +0 pessoa, +4/+8/+0xc ids dos nomes, +0x10 clube destino, +0x14 clube origem, +0x18
valor, **+0x1c o tipo**, +0x1d a data. Os tipos, pelos gravadores: **0 pago**, **1 com jogadores na troca**
(`move_player_to_club` 0x8c99a5: `exchange_players` ≠ 0), **2 empréstimo** (0x8c2180), **3 dispensa** (`free_transfer_staff`
0x8c1af0), **4 livre** (pré-contrato com valor 0 e conclusão 3 ou 4, 0x8c9996), **5 quem não é jogador**
(técnico/comissão: `move_player_to_club` 0x8c99bf e `MANAGER_MANAGER::daily_update` 0x690432; ou só registro em
`free_transfer_staff`). Vetor com capacidade +0x866, conta `past_transfer_count()`, cresce de 100 em 100.

## 11. Pontos de extensão (para o Tweaks)

| Onde | O que muda | Como |
|---|---|---|
| 0x8d0590 `get_loan_period` | duração padrão; o sentinela de fim de contrato | desvio, chama a original e ajusta as 4 saídas |
| 0x8d0040 `can_loan_player` | quem pode pegar emprestado | desvio: original + regra nova (ex. recusar o sentinela; liberar fora da janela) |
| vtables das regras (+0x20, +0x24, +0x28) | a regra de uma nação | trocar o ponteiro do slot na tabela da nação (Inglaterra 0x969f4c, Escócia 0x96f838, Itália 0x96c96c, Irlanda 0x96c190, Espanha 0x96fdec) |
| regras +4/+8 (memória) | as datas das janelas | dados, sem código: reescrever a tabela depois dos construtores |
| regras +0x13/+0x14 (memória) | limites de estrangeiros/não comunitários | dados |
| `[[0xde1f54]+0x91c] + id·16` | a conta dos empréstimos ingleses da temporada | dados (os limites 5 e 2 são imediatos em 0x579d60) |
| 0x842740 (3,5 / 2,75 / 2500 / −1500 / 3 semanas) | quanto a IA pede emprestado | desvio; não mexer nos doubles da `.rdata` (compartilhados) |
| 0x8cb810 `add_past_transfer` | medir | todo movimento concluído passa aqui com o tipo |
| 0x8c63a0 `process_due_offer` | as ofertas presas nas férias | desvio (§8) |

## 12. Dúvidas

- O ponto exato da tela do humano que transforma o fim 1972 (ou o período de zero meses) em recusa.
- Qual estado prende o empréstimo nas férias (o 8 é o candidato; o caminho que perde a decisão não foi achado).
- O tipo 3 e 4 de contrato (N/A na tela) existem nos dados?
- O laço do `ENGLAND_RULES::can_loan_player` testa o id do `player` alvo, não o do jogador do elenco, para pular
  vagas inválidas (o Rust reproduz; efeito nulo na prática) [inferido].
