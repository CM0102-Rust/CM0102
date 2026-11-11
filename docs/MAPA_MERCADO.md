*Versão em inglês: [MAPA_MERCADO.en.md](MAPA_MERCADO.en.md).*

# Mapa do mercado de transferências (com os empréstimos)

Feito em 11/11/2026 para a release 1.5 do Rust Tweaks (o rebalanceamento do mercado), por quatro leitores da desmontagem do original e do código Rust. Cada afirmação está marcada **[provado]** (desmontagem ou teste) ou **[inferido]**. As rotinas que ainda não eram Rust foram portadas como itens da fila (`levas/fila/mka_*`, `mkb_*`, `mkc_*`, `mkd_*`); a prova de cada um está no `PRONTO.txt` dele. Fontes: `levas/reorganizacao/mercado/`.

**Atualizado em 11/11/2026: fechamento dos inferidos** (`levas/reorganizacao/mercado/fechamento_inferidos.md`): as marcas [inferido] fora das rotinas ainda em leitura (0x4d1600, 0x85fc50, 0x864550, 0x4ce4e0, 0x4c7ac0, 0x862720, 0x844780) foram provadas na desmontagem ou ficaram com o motivo. Principais: o empréstimo preso no estado 8 é um defeito provado do original, sem férias (seção C §8); bit 4 de `contract::TRANSFER_STATUS` = preço pedido automático, bit 8 = listado a pedido; os estados do elenco (nibble alto de +0x4f); o `edi` do filtro de jovens é id da pessoa + id do clube; +0x10 das transferências futuras = 1 quando a pessoa sai; as regras da 0x83cb40 e da 0x83f3a0 pelos itens `mkb_2`/`mkb_3`; o `get_max_wage` não é invertido.

**Atualizado em 11/11/2026: fechamento do leitor D2** (`levas/reorganizacao/mercado/fechamento_mkd2.md`): a 0x4ce4e0 (o jogador escolhe a oferta; agora item `mkd_4`) e a 0x4c7ac0 (fim de contrato, renovações da IA, livres; não portada) provadas na desmontagem (seção D §2.2, §2.4, §2.6, §5, §7); o fim natural do empréstimo também (seção C §9).

Correções achadas depois das seções: a 0x83f3a0 é chamada pela `allocate_transfer_budget` 0x83b6e0 (0x83bab7), pela `update_staff_status` 0x843e10 (0x844108) e por ela mesma, não pela 0x83cb40; a 0x83cb40 virou o item `mkb_2` e a 0x83f3a0 o `mkb_3`; na 0x83cb40 a base do alvo (+0x74, da 0x7bca90) decide os estados das vagas e o corte (base/68 − 35), e a nota (+0x64) dá o preço das vagas.

Seções: [A. O lado do dinheiro](#a) · [B. Como a IA compra e vende](#b) · [C. Empréstimos, janelas e limites](#c) · [D. O lado do jogador e quem decide](#d)


<a id="a"></a>

## Mercado, seção A: o lado do dinheiro (leitor A, 11/11/2026)

Quanto o clube pode gastar, quanto pede por um jogador e quanto vale um jogador para quem compra. Marcas:
**[provado]** = lido na desmontagem (ou no código Rust já comparado com o jogo); **[inferido]** = pelo pseudo-C do
Ghidra, pelo uso ou pelo mapa da comunidade, sem prova.

### As rotinas

| Endereço | Nome (mapa de maxevans60) | Em Rust? | Item |
|---|---|---|---|
| 0x598800 | `Finance::get_transfer_budget` | não → **item `mka_1`** | `get_transfer_budget` |
| 0x4d42d0 | `contract_manager_get_human_asking_price` | não → **item `mka_1`** | `get_human_asking_price` |
| 0x4d1600 | `contract_manager_calc_asking_price` | não; **fica de fora desta rodada** (11,3 KB de x87, ver abaixo) | — |
| 0x840d80 | `Shortlist_Manager::value_player_for_club` | sim (`value_player_for_club`, shortlist_manager.rs) | — |
| 0x4d0c60 | `CONTRACT_MANAGER::calc_player_value` | sim (`calc_player_value`, contract_manager.rs) | — |
| 0x544410 | `non_player_attribute_rating` | sim (`chance_to_know`, fog_of_war.rs) | — |
| 0x5a0590 | `FinanceTable::get_club_finance` | sim | — |
| 0x5986b0 | `Finance::get_wealth_band` | sim | — |
| 0x598370, 0x59cf50 | `update_wage_bill`, `refresh_all_wage_bills` (a folha que a verba lê) | sim (fin_prm.rs) | — |
| 0x59a5d0, 0x59af40, 0x5987c0, 0x599a20, 0x59b7e0, 0x5999a0 | as receitas: bilheteria, TV, carnês, produtos, prêmios | sim (fin_prm.rs) | — |
| 0x599800, 0x599890 | `add_players_sold`, `pay_players_bought` (os totais da temporada que a verba lê) | sim | — |
| 0x8d0780, 0x8ca8a0, 0x8cd240 | a compensação, o pagamento, o "vendedor deve aceitar" | na fila (`cpq_6`) | — |
| 0x8ccb80 | `TRANSFER_MANAGER::get_club_future_transfer_list` | não (outra seção: transferências) | — |
| 0x4ca300 | `CONTRACT_MANAGER::get_current_contract_offers` | não (outra seção: contratos) | — |

### 1. A verba de transferências: `Finance::get_transfer_budget` (0x598800)

`thiscall(finanças, jogador, somar_listados, *ajustada, conhecimento, pendentes)`, `ret 0x14`, devolve **i64**
(edx:eax) **[provado]**. 37 pontos de chamada (`refs.py`): os lances da IA (`TRANSFER_OFFER::build_bid` 0x8e5d60 e
vizinhos 0x8e6330, 0x8e74a2, 0x8e839c, 0x8ea2af, 0x8ecf7f..0x8ee61d), as listas de observação (verba por vaga,
`shortlist_manager.rs` 2486), a diretoria (`manager_manager.rs` 837 e 1820: pedido de verbas, o veto), os eventos
financeiros (0x59e329..0x59ed38), o contrato (0x4d753c, 0x4de3a6), as telas (0x463af8, 0x6a3b87, 0x7b6d5a...). As
chamadas Rust de hoje passam `(fin, 0, 0, 0, 1, 0)`: sem jogador, sem listados, sem pendentes. A verba é calculada a
cada lance montado (`TRANSFER_OFFER::build_bid` 0x8e5d60, que chama a 0x598800 em 0x8e6330) e na conferência das
restrições de cada oferta (0x8cfe2d). Os lances novos da IA saem só de `make_club_bids` 0x84a020 (chamada única em
0x5cdd6d, no laço do dia), no ritmo da seção B §2 (todo dia ou só quando (sorteio(2) + id) % 7 = 0), **sem olhar a
janela**: o motivo 9 (fora da janela) de `can_club_bid_for_player` é aceito (as ofertas fora da janela ficam guardadas
pela 0x8c4e10) **[provado: refs da 0x598800 e da `add_transfer_offer` 0x8be300 (0x84a728 na 0x84a020; as outras são
revisão 0x8c0810/0x8c0f40, empréstimo 0x8bed60, 0x8bf060 e telas); 0x84a020 em Rust comparado (cqe_3); o ritmo em
0x5cdc6e..0x5cdd6d]**.

A regra, na ordem **[provado]** (desmontagem 0x598800..0x5997f9, 1240 instruções):

1. Começa no saldo (`finance::BALANCE` +0, i64). Com `somar_listados`: soma o valor (`staff::VALUE` +0x52) de cada
   uma das 50 vagas do elenco (`club::SQUAD` +0xd7) com pessoa listada para venda (0x5444a0).
2. `*ajustada` = 0.
3. Com `pendentes` e clube humano (0x600a50 com 1, 1): as transferências futuras do clube (0x8ccb80, linhas de 27
   bytes: pessoa +8, venda +0x10, taxa +0x14, estágio +0x19; ver a nota do +0x10 abaixo): compra soma a taxa ao comprometido (menos a do próprio
   `jogador`); venda de estágio 3 a 6 tira a taxa (uma vez por pessoa) e o salário do contrato com o clube (0x4cf5f0,
   `contract::WAGE` +0xc) dos salários. Depois as ofertas de contrato abertas (0x4ca300, cópias de 111 bytes: pessoa
   +0, taxa +8, salário +0xc, estado +0x50; o estado 4 não conta): cada uma soma salário e taxa; quem já tem contrato
   com o clube (contrato principal 0x4cf5a0, guardado em `G_CONTRACT_LOOKUP_RESULT` 0xae1080) tira o salário atual. As
   duas listas são liberadas (`free`).
4. Clube sem liga (`club::DIVISION` +0x57), ou liga sem objeto em `G_COMP_OBJECT_TABLE` (0xadadfc): devolve o valor
   do passo 1 (sem nenhuma reserva).
5. **O multiplicador da diretoria m** (byte com sinal): presidente (`club::CHAIRMAN` +0xbf) com ficha
   (`staff::NON_PLAYER` +0x69): `non_player::BUSINESS` (+0xf) + 6, menos (`staff::AMBITION` +0x57 − 15) se a Ambição
   passa de 15; sem presidente ou sem ficha: 16. **+5** se a nação do clube (`club::NATION` +0x53) é a Alemanha
   (`KEY_NATION_GERMANY` [0x9cf314]). O pedido falava em "Business + 6, 16 sem presidente, +5 pela nação [0x9cf314],
   ajuste se staff+0x57 > 15": confere; o ajuste é a Ambição do presidente e só reduz.
6. **A reserva:** sai m × (`finance::WAGE_BILL` +0xc + os salários do passo 3 − `finance::INSTALMENT_BALANCE` +0x1c ×
   ⅔ [0x96a670]) (_ftol de 64 bits). Folha semanal × (Negócios + 6): um presidente de Negócios 20 guarda 26 semanas
   de folha; um de Negócios 1, 7 semanas.
7. **O estádio** (`club::STADIUM` +0x69, `stadium::CAPACITY` +0x3c): próprio (`club::OWN_STADIUM` +0x6d): reputação
   (`club::REPUTATION` +0x80) acima de 5750 → sai 1,25 × lugares × m [0x96a5c8 = −1,25]; acima de 4250 → 1,15
   [0x96a668]; acima de 1250 e profissional (`club::PROFESSIONAL_STATUS` +0x64 = 1) → 1,05 [0x96a660]; senão 1 ×.
   Alugado: 2 × lugares × m. Sem estádio: 2500 × m.
8. **O treino** (`club::TRAINING` +0x7f, t): 200·t (t < 6), 500·(t − 2) (t < 16), 2000·(t − 12), × m × 0,25
   [0x9689e0], sai.
9. **A reputação r:** sai r × m; entra público (`club::ATTENDANCE` +0x73) × r × m × 0,0005 [0x96a658] (0,001
   [0x96a350] com r ≥ 4000).
10. Com nação: entra (reputação da liga, `competition::REPUTATION` +0x69)² × 5000 (Inglaterra, Itália, Espanha), 2500
    (Alemanha, França) ou 750; entra r² × m × 0,001 (estádio próprio) ou 0,0005.
11. Em competição continental (`club::CONTINENTAL_COMP_ID` +0x1db ≠ −1) e profissional: entra 300·r × 1 (Inglaterra,
    Alemanha, Itália, França, Escócia, Espanha), 0,65 (Holanda, Suécia, Bélgica, Portugal, Turquia), 0,35 (Dinamarca,
    Noruega) ou 0,5; sem nação, 0.
12. **O comprometido** S = max(taxas do passo 3, 0) + max(0, gasto líquido da temporada
    (`finance::SEASON_PLAYERS_BOUGHT` +0x110 − `SEASON_PLAYERS_SOLD` +0x118, só o dword baixo) − 8 milhões); sai o dword
    baixo de S. Se ainda positivo: abaixo de 10% do saldo [0x967880] vira esses 10% (no máximo 2,5 milhões [0x96a650]);
    senão o **teto suave** Z = r² × 0,5, × q² quando q = reputação do técnico (`non_player::CURRENT_REPUTATION` +0xa do
    `club::MANAGER` +0xcf) / r é menor que 1 (q pelo menos 0,5 [0x968f18, f32]); Z pelo menos saldo − 10 milhões (− 15
    milhões na Alemanha ou sem nação); num clube PLC (`club::PLC` +0x82), sem nação ou na Suécia, Noruega, EUA,
    Portugal, Dinamarca, Finlândia, Escócia, Turquia ou Alemanha, no máximo 40 milhões − S (35 na Alemanha ou sem
    nação); Z pelo menos 50000. Acima de Z fica só 10% do excesso.
13. Entram as verbas que a diretoria concedeu (`G_MANAGER_MANAGER` 0xb67a44 → `board_record::EXTRA_FUNDS_GRANTED`
    +0x28 da linha do clube, 73 bytes).
14. Reputação acima de 5000: pelo menos (200·r − 1 milhão) × reputação da liga / 17 [0x96a640] (e então pelo menos
    50000). Abaixo de 3500: no máximo 500·r (profissional) ou 250·r. Nunca negativa.
15. **Por nação:** Suécia, Noruega, Dinamarca, País de Gales, EUA, Finlândia, as duas Irlandas e Argentina: metade
    (`_alldiv` 0x945e80), e acima de 2,5 milhões fica 10% do excesso; Bélgica e Portugal: 25% acima de 15 milhões;
    Brasil: 25% acima de 2,5 milhões; Alemanha, França, Escócia e Holanda: 25% acima de 17,5 milhões. Inglaterra,
    Itália e Espanha: sem teto.
16. **O conhecimento do humano:** com `jogador`, `conhecimento`, máscara de atributos (`G_ATTRIBUTE_MASKING_ENABLED`
    0x9cc270), névoa (`G_FOG_OF_WAR` 0xae2a60) e clube humano (0x600a50 com 0, 1): o nível de conhecimento do técnico
    sobre o jogador (0x5afde0, 0x5b02f0) dá V: nível 3, nada muda; 2: r² × 0,04; 1: r² × 0,01 (liga de padrão abaixo
    de 9 e r abaixo de 4250) ou 0,02; 0: r² × 0,005 ou 0,01. V × (reputação do técnico / r, entre 0,1 e 1) × (padrão da
    liga × 0,05, entre 0,25 e 1); com saldo negativo, ver o defeito abaixo. Acima de V, a verba vira V + um oitavo do
    excesso [0x96a628 = −0,125], no máximo 1,75 V se passar de 125000, e `*ajustada` = 1.

**O campo +0x10 das transferências futuras [provado]:** 1 quando a pessoa **sai** do clube consultado, 0 quando
**chega**. Em `get_club_future_transfer_list` 0x8ccb80, para cada oferta aberta do clube (0x8ccbf3..0x8ccc1b, as que ele
fez; mais as recebidas pela 0x8c6100 em 0x8ccc4d; puladas as de empréstimo, `offer::OFFER_TYPE` +0x1f = 1, em
0x8ccc78), a linha do alvo (0x8ccdb7..0x8cce4d): +8 = o alvo (`offer+0xc`); se o clube é o comprador (`[clube] =
offer+8`, 0x8ccdca..0x8ccdcf), +0xc = o clube atual do alvo (−1 sem) e +0x10 = 0 (0x8ccdda/0x8cce02); senão +0xc = o
comprador e +0x10 = 1 (0x8cce0b..0x8cce0e); +0x14 = a taxa (`offer+0x1b`); +0x18 = 4 pré-contrato sem taxa
(completamento 3/4), 1 com troca, 0 senão; +0x19 = o estágio pela tabela 0x8cd134/0x8cd114 (1 a 6, 0 fora); +0x1a =
`offer+0x2d`; +0 a data. As linhas dos jogadores de troca (0x8cce89..0x8ccf33) invertem: com o clube comprador, +0x10 =
1 (ele cede o jogador de troca), senão 0; taxa 0, +0x18 = 1. No passo 3, +0x10 = 0 soma a taxa às compras; ≠ 0 entra no
ramo da venda nos estágios 3 a 6.

#### Defeitos do original
- **[provado] Saldo negativo no passo 16:** com ficha de jogador, V vira `_ftol(razão × 0,65)` (a razão da reputação
  do jogador pela do clube, entre 0,15 e 0,85): o V se perde e fica 0. A verba vira um oitavo dela se isso for até
  125000, senão **0** (o teto 1,75 × 0). Sem ficha de jogador, V × 0,65 **[provado]**: 0x59970b (+0x61 nulo) →
  0x599759 `fild V` → 0x59975d `fmul [0x96a5d0]` (= 0,65, lido do exe) → `_ftol` 0x599763; com ficha, a razão de
  reputação (0,15 [0x968f20] a 0,85 [0x969520]) entra no lugar de V (0x599712..0x599757) e é multiplicada pelo mesmo
  0,65. Que a intenção fosse V × razão × 0,65 segue **[inferido]** (intenção não se prova pelo código).
- **[provado] Passo 3, pessoa repetida:** depois de achar a pessoa numa linha anterior, o salário descontado é o do
  contrato da pessoa da linha seguinte à primeira ocorrência, não o desta linha.
- **[provado] Passo 12:** o gasto líquido da temporada entra só pelo dword baixo (acima de 2,1 bilhões vira
  negativo e some); o comprometido S sai também pelo dword baixo.
- **[provado]** O passo 4 devolve a verba sem nenhuma reserva para clube sem liga: `club::DIVISION` (+0x57) nulo
  (0x598af3..0x598afc) ou sem objeto em `G_COMP_OBJECT_TABLE` (0x598b02..0x598b0d) salta para o retorno 0x5997ec com o
  valor do passo 1; é o mesmo teste que a lista de compras usa para "liga não carregada" (0x83cc68..0x83cc86). Quais
  clubes são esses numa carreira dada segue **[inferido]** (depende das ligas carregadas; não contado).

#### Pontos de extensão
- **O multiplicador m (passo 5):** o único lugar onde a diretoria (Negócios, Ambição) entra. Na cópia Rust, uma linha;
  no exe, os bytes `add al, 6` (0x598b2a) e `add al, 5` (0x598b56). Mexer em m muda a reserva de folha, estádio,
  treino e reputação de uma vez.
- **O retorno (0x5997de / 0x5997ec):** um desvio que chame a original e escale o i64 por grupo de clube (nação,
  reputação, PLC) muda a política sem reescrever nada. Cuidado: as listas de observação dividem a verba entre as vagas
  e a diretoria compara com 250 × reputação (manager_manager.rs 700).
- **Os tetos por nação (passo 15) e o teto suave (passo 12):** constantes imediatas (2,5M, 15M, 17,5M, 40M, 35M) e
  doubles da `.rdata` lidos do exe (a cópia Rust lê do exe; um patch na `.rdata` vale nas duas).
- **O piso de 10% do saldo:** `0x967880` é o 0,1 genérico (usado em centenas de lugares): mudar só aqui exige a
  cópia Rust.

### 2. O preço pedido pelo jogador do clube humano: `contract_manager_get_human_asking_price` (0x4d42d0)

`thiscall(gerente de contratos, vendedor, comprador, jogador)`, `ret 0xc` **[provado]**. 9 chamadores: o lance da
IA (0x8e64df, depois de testar que o vendedor é humano), as listas (0x8402e6, 0x840be3, 0x843387, 0x843551,
0x843afd, 0x84b03e), a mídia/tela (0x7b6dd5) e 0x8f4b8c. O mapa da comunidade chama o 1º argumento de
`buying_club`: é o **vendedor** (é ele que 0x8e64c3 testa como humano e que vai como `param_1` da 0x4d1600)
**[provado]**.

Regra **[provado]**: o contrato principal do jogador (em 0xae1080) dá o clube; clube que não é humano: a caixa
"Error" (contract_manager.cpp linha 0x1f28) e devolve `staff::VALUE`. Contrato com esse clube (0x4cf5f0): nenhum, tipo
(`contract::CONTRACT_TYPE` +0x35 & 0x3f) ≠ 2 ou saindo pela Bosman (`contract::LEAVING_ON_BOSMAN` +0x49): 0. O piso =
o valor guardado (`staff::VALUE`), limitado pela multa rescisória (`contract::RELEASE_FEE` +0x21) se a cláusula está
ativa (0x867b50). O preço = 0x4d1600(vendedor, comprador, jogador, 0, 1, 1), elevado ao piso e limitado pela multa.
Listado ou pedindo saída (0x544520), ou sem o bit 4 de `contract::TRANSFER_STATUS` (+0x4e): o piso. **Bit 4 = preço
pedido automático [provado]**: sem ele o técnico fixou o preço à mão, e `staff::VALUE` (+0x52) é esse preço. Pelo papel
no elenco (`contract::SQUAD_STATUS` +0x4f & 0xf0): **0x10 Key e 0x50 Prospect** o preço; **0x20 First Team e 0x30
Rotation** a média (com sinal, truncada); 0 Undefined, 0x40 Backup, 0x60 Youngster e 0x70+ Not Needed, o piso
**[provado]**.

- **Prova do bit 4:** a tela "Set Staff Transfer Status" (0x8f6770, transfer_screens.cpp): o controle 0xa nasce do
  bit 4 (0x8f6923..0x8f6936: `and edx, 4; shr edx, 2`); o botão "Auto<…automatic asking price>" (texto em 0x8f6fe9) é
  o evento 5 do tratador 0x8f7640, que inverte o controle 0xa e, ao ligar, põe no controle 1 (o preço) o
  `calc_player_value` 0x4d0c60 (0x8f772b..0x8f77bb); mexer no preço (evento 0xb) desliga o controle 0xa
  (0x8f780d..0x8f781e); na confirmação o controle 0xa vira o bit 4 (0x8f80b0..0x8f80da). O contrato novo nasce com
  +0x4e = 4 (0x866f14, em 0x866ee0, chamado por 0x4cfb13/0x4cfccb/0x4cfef4). Com o bit 4, o dia (0x4c7f9e..0x4c7fbe) e
  0x84ae17..0x84ae30 recalculam `staff::VALUE` pela 0x4d0c60; sem ele, não. `set_player_squad_status` 0x8ce0e0 (só
  telas: 0x8efc88, 0x8f08b5, 0x8f0902, 0x8f0af6, 0x8f7fbc) grava o preço em +0x52 (0x8ce1e9) e apaga o bit 4 com o 8º
  argumento ≠ 0 e o clube fora do estado de crise (0x8ce8af..0x8ce8bf).
- **Prova dos estados (nibble alto de +0x4f):** 0x867f6d..0x867ffa (staff_contracts.cpp) lê `+0x4f >> 4` e salta pela
  tabela 0x868000 (0x867f81 Undefined, 0x867f90 Key, 0x867f97 First Team, 0x867fa6 Rotation, 0x867fb5 Backup, 0x867fbc
  Prospect, 0x867fcb Youngster; acima de 6, "Not Needed" em 0x867fda). Logo o "estatuto 6 ou 7" da 0x84af00 é
  Youngster/Not Needed, e a 0x83cb40 põe Youngster (6) abaixo de 20 anos e Not Needed (7) no resto.

Nota: dentro da 0x4d1600, vendedor humano com o técnico **de férias** (0x5fd210, 0x5fea50 com a opção 0) e sem o bit
4 devolve logo o valor guardado **[provado: 0x4d16xx]**: na carreira de testes (técnico de férias) o preço pedido do
clube humano é o valor guardado.

**Ponto de extensão:** o retorno; ou o piso (o `staff::VALUE` +0x52, que o próprio jogo atualiza).

### 3. O preço pedido: `contract_manager_calc_asking_price` (0x4d1600) — mapa, sem item

`thiscall(gerente de contratos, vendedor, comprador, jogador, *motivos, ajustar, marca)`, `ret 0x18`, 0x4d1600..
0x4d4258 (11,3 KB, ~3200 instruções, x87 com dezenas de `_ftol` cujo argumento o Ghidra esconde) **[provado:
tamanho e convenção]**. 20 chamadores: o pedido de transferência (0x4d8705, já Rust), o veto da diretoria (0x696ec5,
Rust), o preço do humano (0x4d4429), os lances da IA (0x8e6533, 0x8e6554, 0x8c8c6e, 0x8c8291...), a verificação de
restrições (0x8cd391), as listas e as telas. Fica **fora desta rodada**: portar com fidelidade de bit exige a
desmontagem inteira (os `_ftol` do pseudo-C não dizem o que truncam) e não cabe no tempo; o mapa abaixo é
**[inferido]** pelo pseudo-C, salvo onde marcado.

A regra, em linhas gerais:
1. Sem vendedor, sem contrato com ele, contrato que não é tipo 2 ou saída pela Bosman: 0 **[provado]**. Vendedor
   humano de férias sem o bit 4: o valor guardado **[provado]**.
2. **Fator da liga** (sem uso visível depois; provavelmente num `_ftol`): pelo `nation::STATE_OF_DEVELOPMENT` (+0x7e,
   1 a 3), o `club::PROFESSIONAL_STATUS` (2 → ×2 ou ×4, 3 → ×7,5) e (reputação da liga² × 25 + reputação do clube) /
   (fator × 1000); dá 1 − (20 − x)² × 0,2 / 60, entre 0,1 e 1,1.
3. **Idade** acima de 28: um fator por potência (0x945ff0).
4. **Felicidade e papel** (`contract::HAPPINESS` +0x3d, `SQUAD_STATUS` +0x4f >> 4): a falta até 75/50/40/25/0 por
   papel 1, 2 e 5, 3, 4, 6; papel 7 → felicidade −100. Jogador ausente sem licença (0x418f60): a disciplina do
   técnico do vendedor zera os termos.
5. **Moral** (`contract::MORALE` +0x44): × 0,00746 (negativa) ou 0,00333; × 0,8333 sem queixas (`GRIEVANCES` +0x45 =
   0) ou 1,2 com.
6. **O valor** = `calc_player_value(jogador, vendedor, 0, habilidade vista pelo técnico (0x6e86d0), 0)` (0x4d1a5a);
   0 → devolve 0 **[provado]**.
7. **Contrato a terminar** (com `ajustar`, vendedor do computador, não listado): fator por papel (7,5; 6,0; 5,5;
   3,5 em f32) menos dias até o fim (121 a 365) / (365 / (f − 1)); acima de 2 marca o motivo 0x100.
8. **Felicidade alta** (≥ 61 motivo 0x200; 21..60 motivo 0x400): (falta/2 + felicidade) × 0,01167 + 1,4165, e acima
   de 49 no computador × (… × 0,006 + 0,7).
9. **As finanças do vendedor** (0x5a0590): o saldo contra a tabela 0x9a7f48 por reputação / 500 (50 mil a 25
   milhões): rico, saldo / tabela; pobre, amortecido. Somado à felicidade × 0,002 e ao nibble de observação² × 0,05
   (registro de 79 bytes +0xb): ≤ 1 → × 0,2125 + 0,7875, senão × 0,25 + 0,75; pelo menos felicidade × 0,0025 + 0,5,
   no máximo 2,5.
10. **O comprador:** Celtic × Rangers (`g_key_club_glasgow_celtic`/`rangers` 0x9d0118/0x9d011c): 2,3 + …; dérbi
    (0x4b02e0) ou rivais (`club::RIVAL1..3` +0xb3/+0xb7/+0xbb), mesma cidade do estádio ou não: 1,3 a 2; mesma
    divisão (fora de 0x9cf764): 1 + … × 0,005 (motivos 8 e 0x10).
11. **Importância no elenco:** habilidade acima de reputação/50 × 17/20, idade < 25, papel < 7: 1 + … × 0,04
    (motivo 0x20).
12. **O preço** = valor + valor × (produto dos fatores, com tetos 4,25..10 e 6 × o fator de finanças), entre −3 e 50
    milhões; mínimo 1000. Permissão de trabalho (0x8c7560) e listado: os motivos ficam só com 0x800. Clube rico
    (0x5986b0) com reputação > 4250 e idade > 18: ajuste pela faixa de riqueza.
13. **Queixas** (`GRIEVANCES` & 0x6b ou 0x100) com contrato a menos de um ano: o preço desce para perto do valor.
14. **Nações grandes** (Inglaterra, Itália, Espanha, Alemanha, Escócia, Japão, EUA, Holanda, Portugal...) com
    reputação ≥ 4001: multiplicadores por papel (1,25; 1,1; 1,4; 1,15; 1,5; 1,75; ×2 até +2M; ×5 até +5M) e mais
    1,025..1,1 se o comprador está na mesma divisão; mais as flags do contrato (0x8676f0) e da comissão (Lealdade +0x59,
    Profissionalismo +0x5b, moral).
15. **Teto por idade** **[provado no pseudo-C, constantes]**: 29 anos (26 para goleiro, `player_sheet::GOALKEEPER`
    = 20, −3): valor + 5M; 30: +4M; 31: +3,25M; 32: +2,5M; 33: +1,75M; 34: +1M; 35: +500k; 36: +200k; 37: +100k;
    38 a 45: o valor.
16. **Portugal** acima do valor: < 6M → × (preço × 1,667e−7 × 0,6 + 0,3); senão × 0,9. **Coreia do Sul** acima do
    valor e de 50000: um ajuste.
17. **A multa rescisória ativa** (0x867b50) limita o preço **[provado]**.

**Pontos de extensão:** o retorno da 0x4d1600 (todos os 20 chamadores passam por ele; os motivos em `*motivos`
explicam a recusa); a chamada do valor em 0x4d1a5a (o preço é valor + prêmio: escalar o valor escala tudo); o teto por
idade (passo 15) e a multa (passo 17).

### 4. O valor do jogador para o clube: `value_player_for_club` (0x840d80, em Rust)

Verificado no Rust (shortlist_manager.rs 1807, comparado com o jogo) **[provado]**: o observador escolhido para a nota
é o técnico (`.manager()`) ou o melhor dos 7 olheiros (`club::SCOUTS` +0x1b3) por
`non_player_attribute_rating(ficha, 6 ou 5)` (0x544410 = `chance_to_know`), que é **só a habilidade geral** do
observador (`non_player::CURRENT_ABILITY` +4: x = (CA + 2 × nível) × 0,05 + 10, depois (x + 0,1x²)/3 + 0,5, entre 1 e
20). O nível é a constante 6 (marca 0x100) ou 5 (marca 0x80, o técnico com +5), não os atributos de julgamento
(`JUDGEMENT` +0x17, `JUDGING_POTENTIAL` +0x18); o escopo da reputação tira 5 ou 10, e com escopo 3/4 e reputação do
clube < 3500 o olheiro nem entra. Confere com o pedido. Os atributos de julgamento só entram na névoa
(`know_person`, fog_of_war.rs, com `judgement()`).

O preço que o clube pagaria: a nota (0x6e8xxx `match_man_score_player_for_selection` com o observador), × 1,05 para
listado de outro clube; a névoa (nibble do +0xb do registro de 79 bytes) limita; o teto por posição (6 a 13 milhões)
e a fração; o valor base; idade acima de 28/32/19, estrangeiro (0x53d980); propensão a lesão; acima de 1,5 milhão, o
potencial visto abaixo de 145 puxa para 1,5 milhão; a divisão do clube; Portugal e França; com `ceiling`, no máximo
o +7 da vaga.

**Ponto de extensão:** a chamada `non_player_attribute_rating(…, 6/5)` é onde trocar o critério do olheiro (por
exemplo usar `JUDGING_POTENTIAL`); o retorno, para escalar o que a IA paga.

### 5. O valor do jogador: `calc_player_value` (0x4d0c60, em Rust)

Ver o cabeçalho em contract_manager.rs 902: base por posição (6 a 13 milhões), idade de pico 28/29, as notas da
ficha, a habilidade vista pelo técnico do clube × 50 + a reputação, a tabela 0x9a7f30 por idade, a nação do clube
(desenvolvimento, padrão da liga), os dias de contrato e a regra Bosman, Portugal/França/Coreia, a multa. É a base
de tudo acima: 0x4d1600 soma um prêmio a ele e 0x4d42d0 o usa como piso (via `staff::VALUE`, que o jogo grava
com ele). 58 chamadores.

### 6. Reputação nos preços

- **Reputação do clube** (`club::REPUTATION` +0x80): na verba, r em quase todos os passos (r × m, público × r, r²,
  300·r, os limites 5000/3500, os tetos por nação); no preço pedido, a tabela de riqueza (r / 500), os limiares 4001,
  4250, 6751 e o "clube grande"; no valor para o clube, os cortes de reputação < 3500 dos olheiros.
- **Reputação da liga** (`competition::REPUTATION` +0x69): na verba (passos 10 e 14) e no fator da liga da 0x4d1600.
- **Reputação do jogador** (`player_sheet::CURRENT_REPUTATION` +0xb, `WORLD_REPUTATION` +0xd): no valor
  (0x4d0c60) e na habilidade vista (0x6e86d0); na verba só no defeito do saldo negativo.
- **Reputação do técnico** (`non_player::CURRENT_REPUTATION` +0xa): na verba, o q do teto suave e do conhecimento.

### 7. Item

- **`mka_1`** (`levas/fila/mka_1/`): `get_transfer_budget` (0x598800) e `get_human_asking_price` (0x4d42d0),
  comparadores lado a lado pelo estado do sorteio; mutações: Negócios + 1 no lugar de + 6, metade do excesso no teto
  suave, o piso do preço do humano dobrado. A prova está em `PRONTO.txt`.


<a id="b"></a>

## Mercado, seção B: como a IA compra e vende (11/11/2026, leitor B)

Parte do mapa do mercado pedido pelo Rust Tweaks (release 1.5; [BRIEF.en.md](BRIEF.en.md)). Endereços em VA do
`cm0102.exe` 3.9.68. **[provado]** = lido na desmontagem (`desmonta.py`) ou no código Rust já comparado com a original;
**[inferido]** = lido só no pseudocódigo do Ghidra (`pacotes/0x83cb40`, `pacotes/0x83f3a0`) ou concluído sem prova
direta. Os nomes de campo são os de `rust/src/motor/fields.rs` e `records.rs`.

### 1. As rotinas

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

### 2. Quem chama e quando

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
  clubes **nunca** fazem lance pela lista [provado: 0x5cdc6e..0x5cdc7b, sorteio(2) uma vez por dia antes do laço;
  0x5cdd32..0x5cdd42 e 0x5cdd58.., (r + id) % 7. Efeito provado para a lista: os lances novos e os pedidos de
  empréstimo da lista só saem da 0x84a020 (a 0x842740 tem um chamador só, 0x84ab5a, dentro da 0x84a020; os lances novos
  da IA, 0x84a728); esses clubes ainda podem receber/responder ofertas e rever as suas. A intenção ("uma vez por
  semana") segue inferida: não se prova pelo código].
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
  `move_player_to_club` 0x8c9720, `set_player_squad_status` 0x8ce0e0 e as telas de pessoa (2). A que o mapa da
  comunidade põe em `CONTRACT_MANAGER::save_to_disk` 0x4dc3a0 (0x4dd0a1) é a fronteira do mapa [provado]: o intervalo
  da 0x4dc3a0 (3408 bytes) cobre várias rotinas; 0x4dd0a1 está no tratador 0x4dcd70 do diálogo **"Transfer Request"**
  (construtor 0x4dcb30; os dois registrados em 0x4dcab5/0x4dcaba; textos "Put <…> on the transfer list at his own
  request ?", "Yes", "No"). No "Yes": tira a lista de empréstimo (0x4dd060..0x4dd069), **liga o bit 8** de +0x4e
  (0x4dd06c..0x4dd075; bit 8 = listado a pedido do jogador), evento 0xc (0x544710), vitrine forçada (0x842c00 com 1, em
  0x4dd0a1) e 0x4d71b0.
- **A lista de transferência pela IA**: `transfer_list_player` 0x842320 tem 3 chamadas: 0x83dc42 (dentro da
  `assess_club_squad`, a única da IA) e as telas 0x8f8030/0x8f8607 [provado]. **A lista de empréstimos**: os únicos
  `or byte [r+0x4e], 2` do `.text` são 0x83e49c e 0x83e651 (`assess_club_squad`) e 0x8442c4 (`add_to_loan_list`, que
  só as telas chamam: 0x8f806c, 0x8f8538); mais dois por registrador na `assess_club_squad` (0x83dd42, 0x83dd98)
  [provado pela varredura de todo o `.text` por escritas de **byte** em `[reg + 0x4e]` (mov/or/and/add, imediato ou
  registrador): o bit 2 só é ligado nesses cinco pontos; as outras escritas por registrador são cópias de contrato
  (0x4cc6dc, 0x4cc903, 0x4ce39f, 0x4cf9d8, 0x4cfc10, 0x4cfdb9, 0x4d000f, 0x4d0546, 0x4d06a5, 0x5fbb51), limpezas
  (0x83d287 `& 0xfc`, 0x8442ef `& 0xfd`, 0x4d8747/0x4dd069/0x784848/0x8f80a1 `& 0xfd`, `& 0xf7`, `& 0xfe`), zeros de
  construtores (0x85e458, 0x85e4f4, 0x85e6a4, 0x85e9db, 0x866eeb) ou mexem nos bits 1, 4 e 8. Segue inferido: as
  escritas mais largas que cobrem +0x4e (265 `mov dword/word` em +0x4b..+0x4e em todas as estruturas) não foram olhadas
  uma a uma].

### 3. As regras

#### 3.1 A avaliação do elenco (0x83cb40, original)

`assess_club_squad(this, entrada, clube, daily, reset)`, thiscall, ret 0x10. Ordem [provado: desmontagem
0x83cb40..0x83f3a0 (3168 linhas), lida inteira para o item `mkb_2`; os pontos citados conferidos; onde não há endereço,
o texto é o cabeçalho do `mkb_2`]:

1. DAYS (+8 da entrada) = 0 (0x83cbc4); o reset é forçado quando o +0x23 da vaga 0 é 0xff (0x83cbb4..0x83cbbb). Sai
   com ACTIVE 0 (0x83cc16) se o clube é seleção (0x53b900), sem nação, humano com daily (0x600a50 com 0, 1),
   `club::PROFESSIONAL_STATUS` = 3, ou quando `club_get_linked_b_team` 0x540a50 acha o par e diz que este clube **é o
   B** (0x83cc04..0x83cc14: retorno ≠ 0 e o indicador de "é o A" 0): quem cuida da lista é o A [provado].
2. Sem lance em vaga: se ACTIVE era 2, vira 1 e reset = 1. Se a liga da nação não está carregada ou a liga do clube não
   está ativa em `[0xadadfc]`, e a reputação < 4000: se sorteio(reputação + 1000) < sorteio(5000), ACTIVE = 0 e sai
   (clubes pequenos de ligas não carregadas desligam a lista) [provado: 0x83cc6b..0x83ccab].
3. Com reset: as 40 vagas refeitas (`shortlist_set_up_formation_slots` 0x83c090; o estatuto de cada vaga pelo índice
   e pela reputação-alvo `local_840` de `player_search_get_club_target_rating` 0x7bca90: até 6000 ou 8500 e com/sem
   competição continental). Sem reset: só ordena as vagas, `qsort(vagas, 40, 0x25, 0x83b6c0)` em 0x83cfe5..0x83cff1
   (ordem pelo índice da vaga) [provado].
4. A vaga sem alvo, ou com alvo sem oferta (`OFFER_REF` −1) e com a data passada ou aposentado (cargo 0x10), perde
   alvo, substituto e teto (o alvo, se está nos pares da comissão, recebe o evento 3 na 0x843e10); a posição vai para a
   lista do seu estatuto (2 e 3, 4, 5) [mkb_2, 0x83cff9..0x83d1c7].
5. **O elenco** (`database_collect_club_squad` 0x541f90, 50): para cada jogador com ficha, cargo ≠ 0x10, fora da lista
   do clube, com contrato no clube, sem transferência futura (0x8cca10) e sem `contract::LEAVING_ON_BOSMAN`: num clube da
   IA sem técnico de férias, **apaga as marcas de transferência e de empréstimo** (`contract::TRANSFER_STATUS` & 0xfc),
   põe o estatuto (`contract::SQUAD_STATUS` nibble alto) 6 abaixo de 20 anos ou 7 (clube da IA) e manda ao time
   principal (`move_to_first_team` 0x4cc0b0) [provado: 0x83d281..0x83d2d7]. Lesão (`Injury::get_player_injury`
   0x62ed70): o limite é **150 dias** (0x83d32d); **30** quando sorteio(nível de recaída 0x62ece0 + 1) ≠ 0
   (0x83d33e..0x83d34b); senão, lesionado com nota (0x6e6340) ≥ alvo: **400** (0x83d37f..0x83d387); nota ≥ ¾ do alvo
   (0x969040 = 0,75): **300** (0x83d391..0x83d3a8). Dias restantes ≥ limite, ou anúncio de aposentadoria (0x7a7ed0), vai
   às reservas (`move_to_reserves` 0x4cc0f0, em 0x83d3f4; com time B só acima de 300 dias, 0x83d3d8..0x83d3ea) e, no
   clube humano, perde o estatuto (0x83d40e). Os outros vão para o resto ou para a lista dos emprestados a outro clube
   (0x4cf6d0 em 0x83d41b), e com `reset` a felicidade vai a 0 [provado].
6. Escalação por estatuto (`match_man_assign_players_to_positions` 0x6e2000, chamadas em 0x83d512, 0x83d636,
   0x83d7f9): o resto vai para as posições de estatuto 2 e 3, depois 4; os emprestados a outro clube entram no resto;
   os mais velhos que `player_sheet::GOALKEEPER` / 10 + 22 (24 para um goleiro nota 20, 22 para os outros) saem dele (o
   trocado é olhado de novo); o resto vai às posições de estatuto 5; os velhos voltam [mkb_2, 0x83d474..0x83d8ac].
7. **Quem sobra** (o fim do elenco), para cada jogador do clube, sem contrato de empréstimo e sem fila de transferência
   livre (0x8c1870) [mkb_2, 0x83d8bf..0x83de6b]: o limite é ftol((idade × 0,13 − 1,98) × base × 0,02), o corte
   base/68 − 35 e o nível ftol(reputação × 0,25 + PA × 12,5 + CA × 12,5 + reputação do clube × 0,25); "sobra" = julgado
   abaixo do limite (0x6e86d0 modo 1, 0x83da02) e contrato tipo 2 com menos de profissionalismo + pressão + 450 dias no
   clube ou fim a 450 dias ou mais, ou `staff::VALUE` ≥ reputação × 50, ou o valor ≥ os salários que faltam (dias ×
   salário / 30). Com time B ou no clube humano, abaixo de 27 (goleiro 20) ou 23, o estatuto 5 quando ¾ do nível passa
   o limite + 20, senão 6. Os pontos já provados:
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
8. As vagas recebem os escolhidos [mkb_2, 0x83e185..0x83e66f]: o substituto é o escolhido; sem técnico de férias o
   contrato recebe o estatuto da vaga, menos na lista 2/3 sobre os estatutos 1 e 2; a data da vaga sai da chegada dele
   (+120 dias, com `reset` e técnico). Estatuto 4: na vaga 24, com time B ou no humano e fora dos grandes, o escolhido
   (não o técnico) vai para os reservas. Nas listas 4 e 5 (reservas, sobra):
   - lista 4: **empréstimo** se a fadiga (`Injury::get_fatigue` 0x62c3d0) < −1000, `CURRENT_REPUTATION` < 4500,
     `player_sheet::WORK_RATE` (+0x44) < 3250 e felicidade < −50 [provado: 0x83e46a..0x83e49c]. **Defeito:** +0x44 é um
     byte com sinal comparado com 3250 (0xcb2): sempre verdadeiro; provavelmente era `WORLD_REPUTATION` (+0xd, palavra)
     [provado o código: `movsx edx, byte [eax+0x44]; cmp edx, 0xcb2` em 0x83e482..0x83e486; a intenção segue
     inferida: que fosse `WORLD_REPUTATION` não se prova pelo código];
   - lista 5: **empréstimo** se a fadiga < −500 − sorteio(300) e felicidade < 0 [provado: 0x83e613..0x83e651].
9. Preço das vagas de jogador (`shortlist_slot::VALUATION`, `shortlist_slot::PRICE`) [mkb_2, 0x83e66f..0x83ebfb]:
   p = min(alvo, 36000) (× 0,75 numa vaga de goleiro sem substituto). Sem substituto: valor p e preço p × 2,5 +
   reputação, 2p + rep., p × 1,8 + rep., p × 0,8 ou p × 0,5 pelos estatutos 1, 2, 3, 4/5, outro (× 3 numa vaga de
   goleiro). Com substituto: vai ao time (2, 3, 4; com `reset` felicidade + 50, 10 jogos de fora) ou aos reservas (5) e
   recebe nota para a posição (nos primeiros 200 dias × (1,25 − dias × 0,00125)), entre −10000 e 60000; nota ≥ p, sem
   queixas 0x100 ou 1 e mais de 364 dias de contrato: ele fica como alvo. Senão a felicidade muda ftol((1 − nota/p) ×
   −100) (com `reset`; estatutos 2 e 3 ao menos 25 e a vaga o perde; os outros ao menos 0; estatuto 5 recebe o
   estatuto 6), e o preço (rep. × f + p × k) − nota × k.
10. As vagas de comissão (cargos 6 a 10) com a pessoa do cargo (auxiliar +0xd3, `club::COACHES`, `club::PHYSIOS`,
    `club::SCOUTS`): a demissão só acontece quando o indicador local [esp+0x60] está ligado (zerado em 0x83cb6c), e ele
    só liga em 0x83cc44..0x83cc64: **a lista estava em ACTIVE 2, sem lances, e o clube não é humano** (0x600a50 com
    1, 1). Aí, nota de `score_staff_for_job` 0x69d1b0 abaixo de min(alvo/5, 5000) e sem oferta de contrato (0x4cc9e0):
    **demitida** (`MANAGER_MANAGER::remove_staff_from_club` 0x690890; 0x83ee28..0x83ee72, igual para os outros 4 cargos
    em 0x83ef46, 0x83f056, 0x83f15f, 0x83f275). Sem isso, preço (limite − nota) × 2,5 [provado].

#### 3.2 A escolha do alvo (0x83f3a0, original)

`fill_shortlist_slot(this, vaga, clube, excluído)`, thiscall, ret 0xc, recursiva [provado: desmontagem
0x83f3a0..0x840d80 (2048 linhas), lida inteira para o item `mkb_3`; os pontos citados conferidos; onde não há endereço,
o texto é o cabeçalho do `mkb_3`]:

1. Sai: sem nação; humano (0x600a50 com 0, 1); humano (1, 1) de férias com a regra 4 = 1; vaga com oferta; seleção.
2. **O divisor de conhecimento k** [provado: 0x83f4d8..0x83f5b8]: 100; reputação < 3500 → (6025 − reputação) × 4;
   Bélgica, Holanda, Suécia, Dinamarca, Noruega → 50. Cada uma das 7 vagas de olheiro (`club::SCOUTS`): vazia +5 (+10
   com a liga ativa); com olheiro, + 30 − 2 × `chance_to_know`(olheiro, 6) − `chance_to_know`(olheiro, 5). k = k/10,
   mínimo 2. **Defeito provável:** a 0x544410 recebe as constantes 6 e 5, não atributos do olheiro; o resultado depende
   só da habilidade atual dele (+4 da ficha de não jogador): `JUDGEMENT`/`JUDGING_POTENTIAL` não entram [provado o
   código (mkb_3, 0x83f4d8..0x83f5b8); a intenção segue inferida: não se prova pela desmontagem].
3. Fator de reputação: reputação × 0,0008; na ITA, ENG, GER, FRA e ESP com finanças, pela faixa de riqueza (0x5986b0):
   faixa 1, × 1/1175 (× 1/1125 acima de 6250); faixa 2, × 1/1125 (× 1/1050 acima de 4750). Teto da vaga
   (`max_fee_for_squad_status` 0x84b230).
4. Percorre as listas de posição (`POSITION_LISTS_BY_PERSON`) da posição da vaga. O candidato passa por: não excluído,
   id < número de pessoas − 16, cargo ≠ 5, sem cargo 5 na seleção (`job_for_club`/`job_for_nation`), sem vaga de
   técnico (0x69fc60); jogador com ficha, sem anúncio de aposentadoria (0x83fb53..0x83fb5d), não saudita
   (0x83fb63..0x83fb73); estatuto 5 (`(+0x24 & 0x1e) = 0xa`) e idade > 23 sai (0x83fb79..0x83fb8d); compra: teto (+7)
   menor que **preço × 1,3** (0x968de8) sai (0x83fb97..0x83fbad); empréstimo: passa com clube e preço ≤ 0, senão
   precisa estar listado para empréstimo (0x83fbb8..0x83fbd7) [provado]; não do próprio clube; em menos de 10 listas;
   chegou há mais de 200 dias (ou estatuto 7, queixas, listado); fora de outra lista do clube; permissão de trabalho
   (0x84b180, 0x8c7560); no empréstimo, pode emprestar (0x8d0040); empréstimo de humano só listado (bit 2, 0x83feb8). Um
   humano de férias com a regra 4 = 2 só considera a lista de observação dele. Não-jogador: o resto de um contrato tipo
   2 (dias × salário / 7) ≥ teto ou > (reputação + 500) × 10 sai.
5. **Quem o clube conhece** [provado: 0x83fed8..0x8400d4]: conhecido quando (0x83fed8) a lista do olheiro humano o
   tem, ou (0x83feea..0x83fefd) (id da pessoa + id do clube) % k = 0. Senão, de **outra nação** (0x83ff03) sem jogos
   pela seleção (+0x22, 0x83ff11) e cujo clube não é da nação do clube (0x83ff1c..0x83ff26): um jogador de menos de 23
   anos estrangeiro para o clube (0x53d980 com 1) e reputação mundial (+0xd) < 3250 sai (0x83ff2c..0x83ff4f); de outra
   região (+0x75) precisa reputação do clube ≥ 2750 e (clube dele sem nação ou da mesma região, ou reputação do clube ≥
   4250, ou `league_standard` +0x85 da nação ≥ 12) (0x83ff55..0x83ffa3); e então a reputação vista pelo técnico
   (0x5407e0) ≥ 2750 + sorteio(2000) nas vagas de estatuto 5/6 (com reputação mundial ≥ 1750) ou ≥ 3250 +
   sorteio(2000) nas outras (mundial ≥ 2750) (0x83ffa9..0x840058). Da **mesma nação** (ou com jogos pela seleção, ou
   num clube da nação; 0x840060): não-jogador, vaga de empréstimo, sem ficha, reputação em casa (+9) ≥ 3250, clube dele
   ≥ 3250 ou 20 anos ou mais: conhecido; senão o filtro de jovens (seção D §3, 0x8400ae: fixo por par pessoa-clube).
6. A nota e o preço: abaixo da avaliação ou da nota a bater − 10000 sai; compra num vendedor em crise: no máximo o
   valor do vendedor; troca por compensação (0x84b6f0): a taxa dela; senão o preço pedido (0x4d42d0 humano em
   0x8402e6, 0x4d1600 IA em 0x8402fe); estatuto 4 abaixo de 28 ou estatuto 5: o excesso sobre o valor soma
   min(pedido − valor, (rep./13000 + 0,5) × valor, 2.500.000) à oferta; **a previsão 0x84af00** = 2 sai; preço acima do
   teto sai; nota × y, y = 1 − (pedido − valor)/(preço máx. × fator) (acima de 1 → y/3 + 0,67; 0,01..3); lesão > 15
   dias: − (dias − 15)²; empréstimo: previsão do dono 2 sai, lesão > 10 dias − (dias − 10)² × 200; sem
   `can_club_bid_for_player` sai; por fim nota × (92 + sorteio(17))/100.
7. **Vontade** (0x846920; não-jogador: 0x844780 em 0x840682); notícia de interesse com sorteio(79 − rep./150) = 0 e
   (rep. > 5000 + sorteio(2000) ou clube humano); vontade ≥ 3,15 (0x968e58): salário desejado (0x864550 em 0x840893,
   como f32) × max(3,5/vontade, 0,9) acima de 1,2 × o teto salarial (0x5968d0 em 0x84091d) sai; senão é o melhor até
   ali. Notícia do olheiro (0x587220) por sorteio.
8. O escolhido vira o alvo (`shortlist_slot::TARGET_STAFF_ID`) e entra na lista (`add_shortlist_entry` 0x8423f0).
   Sem ninguém, vaga de estatuto 2/3 vira **vaga de empréstimo** (bit 0) e recomeça se (reputação² × 4e-9 + 0,6) ×
   valor < preço, ou teto < 50000, ou Itália com sorteio e reputação 3000..6000; só com o indicador da temporada
   (nação-chave: entre o início − 28 e o fim − 28 da temporada; outras nações sempre; vaga de não-jogador nunca), e a
   liga da nação carregada **ou** sorteio(reputação) > sorteio(20000). Senão o alvo fica −1.

#### 3.3 Os lances e os pedidos (0x84a020, Rust)

O cabeçalho em `rust/src/motor/shortlist_manager.rs` (linhas 38-97) está completo [provado: Rust comparado, cqe_3].
Resumo: clube da IA (não seleção, não humano, nem humano de férias com regra 4 = 1), lista ativa, DAYS < 100, com
finanças; a conta = vagas com lance + elenco; vaga de jogador pulada com conta ≥ 50. **Compra**: valor da vaga
(`shortlist_slot::PRICE`, ×2 com a marca 0x20, mínimo 20000), ×3/×2 na liga ativa por reputação > 4750 e conta < 25/35;
substituto incapaz, lesionado (dias² × reputação/500), moral, em oferta; salário desejado ≤ 1,3 × teto; **sorteio(valor) >
sorteio(2 × reputação + 1000)** faz o lance. **Empréstimo**: não França amadora; pode emprestar; vendedor humano só com
o jogador listado; **sorteio(PRICE) > sorteio(reputação/10)**, ou no segundo par > sorteio(2 × reputação): o pedido
(0x842740, que exige vontade ≥ 3,5).

#### 3.4 A previsão do vendedor (0x84af00, Rust)

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

#### 3.5 O comprador revendo o lance (0x8c0810, original; item mkb_1)

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

#### 3.6 A resposta do vendedor (0x8c8090, Rust)

Cabeçalho completo em `rust/src/motor/transfer_manager.rs` (linhas 520-565) [provado: Rust comparado]. Resumo:
empréstimo recusado sem olhar nada (código praticamente morto, ver §2); "tem de aceitar"
(`transfer_manager_selling_club_must_accept`) aceita; humano de férias com regra 0 recusa; piso = preço pedido
(0x4d1600) ou a melhor outra oferta; opções em dinheiro, troca e cláusula; a primeira com valor × 0,85 abaixo do pedido
aceita; a melhor × 0,75 abaixo do pedido vira **contraproposta** (salvo tipos 1 e 2); senão recusa (motivo 10 se o
total < 0,55 do valor com 2 milhões de diferença, ou < 0,1 com 100 mil).

#### 3.7 A vitrine (0x842c00, Rust)

[provado: Rust comparado, cpp_6] Só jogador com ficha e `CURRENT_REPUTATION` ≥ 5500 (0x157c), salvo force; sem
aposentadoria anunciada; cargo de jogador; não técnico de seleção; não saudita; sem transferência futura; quem chegou há
menos de 200 dias só com estatuto 7 ou queixas. Percorre os clubes por reputação (`[G_CLUB_REPUTATION_ORDER]`) e oferece
(0x842e10, que usa a 0x84af00) até o jogador estar em 5 listas (a conta é o nibble baixo de +0xb do registro de 79
bytes), pulando seleções, o clube do contrato e o próprio. **Por isso o jogador abaixo de 5500 de reputação nunca é
oferecido**, e os clubes de reputação alta ficam com ele primeiro.

### 4. Campos lidos e gravados

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

O presidente e os diretores (`club::CHAIRMAN` +0xbf, `club::DIRECTORS` +0xc3) **não são lidos diretamente** por
nenhuma das 7 rotinas desta seção [provado: para as 4 em Rust e a 0x8c0810; e nenhum acesso a `[clube + 0xbf]` ou
`[clube + 0xc3]` em 0x83cb40..0x83f3a0 nem em 0x83f3a0..0x840d80]. **Mas entram pelas chamadas:** a 0x83cb40 chama
`update_club_target_reputation` 0x83bf40 (0x83cda7), que lê a Paciência (+0x1d) e os Recursos (+0x20) do presidente
(seção D §3); a 0x83f3a0 chama `get_max_wage` 0x5968d0 (0x84091d: Negócios +0xf e Recursos do presidente) e o preço
pedido 0x4d1600 (0x8402fe, 0x840bfb). Do técnico entram só `non_player::YOUNGSTERS` (0x83f3a0) e o julgamento dele na nota
(`match_man_judge_player_ability` 0x6e86d0 recebe `club::MANAGER`); dos olheiros, só a habilidade atual (§3.2).

### 5. Constantes (lidas do exe)

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

### 6. Defeitos do original

Nos itens 2 a 5 o código está provado; a intenção segue **[inferido]** em todos: se era o pretendido não se prova pela
desmontagem.

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

### 7. Pontos de extensão (um gancho pequeno muda a política)

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

### 8. O que não foi portado

- **0x83cb40** (10.336 bytes até a fronteira, 3.136 instruções, quadro de 0x8a4 bytes, 45 chamadas, 2 ainda
  originais) e **0x83f3a0** (6.624 bytes, 2.048 instruções, recursiva, quadro SEH): grandes demais para portar e provar
  nesta rodada; cada uma pede um item sozinha e um leitor com tempo para a prova. O mapa acima (§3.1, §3.2) é o ponto
  de partida; a 0x83f3a0 tem os fragmentos 0x840000 e 0x840010 (mapa da comunidade) e o pedaço 0x840084 que não são
  rotinas.
- **0x8c0810**: item `mkb_1` (compila; comparação, carreiras e mutações **não** rodaram: as vagas do laboratório
  estavam ocupadas; ver `levas/fila/mkb_1/PRONTO.txt`).


<a id="c"></a>

## Mercado, seção C: empréstimos, janelas e limites (leitor C, 11/11/2026)

*English version: [secao_c.en.md](secao_c.en.md)*

Parte do mapa do mercado pedido pelo Tweaks 1.5 ([BRIEF.md](BRIEF.md)). **[provado]** = lido na desmontagem ou no
Rust já comparado (0 divergências); **[inferido]** = conclusão do código, não reproduzida no jogo.

### 1. As rotinas

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

### 2. O pedido de empréstimo da IA (0x842740) [provado, Rust]

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

### 3. O período do empréstimo (0x8d0590) [provado, Rust]

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

### 4. Pode emprestar? (0x8d0040) [provado, Rust]

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

### 5. Janelas [provado]

Regras de transferência: +4 a tabela das janelas (12 bytes: +0 abertura, +6 fechamento; cada data 6 bytes: +0 regra,
+1 número, +2 dia da semana para onde anda (0xff nenhum) [provado: `transfer_window_get_date` 0x40a7d0 passa o byte
+2 com sinal (0x40a7d1 `movsx ax, byte [ecx+2]`) como 4º argumento de `Date::Date` 0x549f70, que com valor ≠ 0xffff
chama `Date::get_nearest_weekday` 0x54a2d0 (0x54a0ea..0x54a0fc): a data vai para o dia da semana mais próximo], +3 dia, +4 mês (0 = janeiro), +5 1 abre / 0
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

### 6. Limites que barram a contratação [provado]

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

### 7. Defeito suspeito 1: o empréstimo em fim de contrato

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
  - o humano recebe um período de zero meses (volta neste mês) [o que a tela faz com ele: inferido; só tela, não muda
    a carreira automática: o construtor 0x8f9690 põe o período nos controles 3-6 e 0xc/0xd (0x8f980f..0x8f98ac), o
    tratador é 0x8fac30 (registrado em 0x8f9747) e a confirmação passa pela 0x8fb5e0; não foi seguido até o ponto em
    que o fim 1972 ou o período zero viram (ou não) recusa].
- Contrato que termina **no mês seguinte**: um mês de empréstimo, a IA pede se a volta ficar a 3 semanas ou mais.

**É defeito do original?** Sim, de consistência [provado]: o `get_loan_period` usa 1972 para dizer "não dá", mas o
`can_loan_player` só barra pela regra de encerrar do dono e aceita início = fim, então o humano é liberado a abrir
uma proposta com fim impossível (ou de zero meses), enquanto a IA recusa calada. Que isso é exatamente o "não consigo
pegar emprestado jogador em fim de contrato" do fórum (58.2) é **inferido**: só se prova reproduzindo no jogo.

**Correção pequena:** um desvio na 0x8d0040 (aqui já é Rust; no Tweaks, prólogo `6a ff 68 1b 40 96 00`, 7 bytes)
que, depois do período, devolve 0 com uma mensagem clara quando o ano do fim é o da data padrão (1972) ou o fim fica
antes do início; ou, como política mais generosa, um desvio na 0x8d0590 (prólogo `83 ec 10 32 d2`) que chama a
original e, quando o sentinela veio do contrato (tipo ≠ 2 ou recuo abaixo deste ano), põe o fim no mês do fim do
contrato e o início no mês seguinte a hoje, se couber.

### 8. Defeito suspeito 2: ofertas presas com o técnico de férias

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
  Prova: o `process_due_offer` não compara 8, 9, 0xb, 0xc, 0xd (as comparações de estado são as de 0x8c6708..0x8c6972
  e não há tabela de saltos); a varredura pula 0x13, 7, 8 e 0xa em 0x8c5964..0x8c5973. Escritores (varredura de
  `mov byte [reg+0x2c], imm`): 9 só em 0x8c6a8c (`process_due_offer`, comprador humano); 0xb em 0x8c31e7 (0x8c2f50);
  0xc/0xd na 0x8c3310 (0x8c336e/0x8c335b). Escritas por registrador não varridas.

**O defeito [mecanismo provado; efeito no jogo inferido, não reproduzido]:** uma oferta que esperava o humano (9: o comprador humano tem de oferecer o
contrato; 0xb–0xd: as confirmações dos clubes) quando ele entra de férias só anda se a notícia correspondente ainda
estiver na caixa; se saiu do anel de 100 (ou ele apagou), fica parada as férias inteiras, e o jogador continua "com
oferta" (bit 0 do registro de 8 bytes em `PERSON_FLAGS`), o que barra outras ofertas por ele. É defeito do original:
o despacho depende da notícia, não do estado da oferta.

**E os empréstimos?** O empréstimo não passa por 9 nem por 0xb–0xd: completa na hora em que o jogador aceita
(0x8c2180). Os estados de empréstimo que esperam gente são 0 (dono humano: estado 4 em 14 dias, caduca), 1 (comprador
humano: estado 4 em 10 dias, caduca), 0xf (removido na volta com rodada 3) e 8 (decisão do jogador, no gerente de
contratos 0x4cc930/0x4cd9c0). O único que **não caduca** é o 8, e a varredura semanal o exclui.

**Defeito do original: o empréstimo preso no estado 8 [provado, caminho de código; não reproduzido no jogo].** Um
empréstimo pode ficar em 8 para sempre, **sem férias**:

1. O dono (IA) aceita: `contract_manager_schedule_decision_for_transfer` 0x4cc930, estado 8 e **nenhum
   reagendamento** (0x8c6577..0x8c6592 no `process_due_offer`; 0x8c0680/0x8c068b no `accept_loan_offer`). A 0x4cc930
   marca a oferta de contrato (pessoa, clube) que tem +0x62 = 0 e agenda a decisão (0x4cc975 é a única escrita de 1 em
   +0x62 fora da `add_contract_offer`).
2. A oferta de contrato do empréstimo é tipo 6 (`init_from_current_contract` 0x85e590: `& 0xc6 | 6`), e a
   `add_contract_offer` 0x4cc490 grava +0x62 = (tipo ≠ 6): 0 para o empréstimo (0x4cc597..0x4cc5b7 na oferta nova;
   0x4cc741..0x4cc761 na atualizada).
3. A decisão do jogador (0x4ce4e0) só olha ofertas com +0x62 ≠ 0 (0x4ce63c..0x4ce641). Se ele escolhe **outra**
   proposta ligada a uma transferência, a do empréstimo vira estado 1 e `handle_player_rejected_terms` 0x8c28c0
   (0x4ced1b..0x4ced32: a transferência vai a 0xf, rodada 3, volta em 10 dias); e a oferta de contrato do empréstimo é
   **guardada** na lista +0x38 (0x4cf0b5..0x4cf13d, clube diferente do escolhido e escolhido com vaga ≠ −1) e sai da
   lista viva (0x4cf35d..0x4cf383).
4. Se a transferência escolhida é **retirada** com estado ≥ 0xb (`withdraw_transfer_offer` 0x8bfcb0, 0x8bfd3a; único
   chamador da 0x4cf4a0, em 0x8bfd4a) enquanto o empréstimo ainda existe (os 10 dias do passo 3),
   `resubmit_held_offers` 0x4cf4a0 põe a transferência de 0xf em **8** (0x4cf4fb..0x4cf501) e reenvia a oferta com
   `add_contract_offer(…, decide 1, combinado 0)` (0x4cf505..0x4cf52a): tipo 6, então +0x62 = 0.
5. A decisão agendada pula essa oferta (passo 3); a 0x4cc930 não volta a ser chamada (só nas aceitações do estado 0);
   o `process_due_offer` não tem ramo para 8; a varredura pula 8. **O empréstimo fica em 8 para sempre** e o jogador
   segue "sob oferta" (o bit 0 do registro de 8 bytes só é limpo com as ofertas resolvidas, e 8 não conta).

A decisão também é adiada de 2 em 2 dias enquanto o jogador tiver outra oferta em 0, 1, 5 ou 6
(`player_has_pending_offer` 0x8cd150 em 0x4ce5ce), mas isso termina quando a outra caduca. **As férias não entram**
nesse caminho (nem a 0x4ce4e0 nem a 0x4cf4a0 olham o humano). Que é isso o item 88 do xeno segue **[inferido]** (não
reproduzido no jogo).

**Correção pequena (oferta presa nas férias):** no `process_due_offer` (aqui Rust; no Tweaks, prólogo
`6a ff 68 00 3d 96 00`), para o estado 9/0xb/0xc/0xd com o humano envolvido de férias (`news_club_has_human_manager` ≠
0 e `human_manager_is_human_club(…, 0, 1, 0, 0)` = 0), fazer o que o `auto_respond` faria: 9 →
`offer_contract_to_offer_player`; 0xb–0xd → `check_offer_restrictions` e `confirm_offer_by_club` ou
`club_rejects_offer`.

**Correção pequena (empréstimo preso):** na `resubmit_held_offers` 0x4cf4a0, para uma transferência de empréstimo
(`offer_type` 1), chamar a 0x4cc930 depois do reenvio (ou gravar +0x62 = 1); ou, no `process_due_offer` (Rust), tratar
o 8 vencido de empréstimo como recusa (`reject_loan_offer`).

### 9. Como o empréstimo termina e volta [provado, Rust]

- **Assinatura:** `handle_player_accepted_terms` (0x8c2180) → `sign_loan_contract` (0x4d01e0): o contrato vai para a
  vaga de empréstimo (+4 do registro de 79 bytes), o jogador entra no elenco de quem pega (função 0xb), bit 0x40 do
  contrato principal = reservas; histórico `add_loan_records`; `record_loan` da nação de quem pega (Inglaterra conta
  a temporada); `add_past_transfer` tipo 2; oferta 0x14 e removida em 5 dias.
- **Fim natural:** `CONTRACT_MANAGER::daily_update` 0x4c7ac0 (na fatia 2) chama `end_loan` (0x4d07e0, em 0x4c8114)
  na data do fim: o contrato de empréstimo (índice +4 do registro de 79 bytes) que vence hoje; sem clube, só o índice
  vira −1; senão notícia 6 (0x4da590, fora do tipo 7) e `end_loan` [provado: 0x4c8077..0x4c8114; seção D §2.4].
- **Fim antecipado:** a tela (0x8fbcc0) pergunta `can_end_loan` (0x8d0530: só quando o clube dono do jogador é da
  mesma nação que quem pede; senão 1) → `schedule_loan_end` (0x4ccca0: o fim do contrato de empréstimo vira amanhã,
  tipo 7, notícia 0xfb8 se houver humano) → `end_loan` no dia seguinte.
- **Transferência definitiva:** `move_player_to_club` (0x8c9d5d) encerra o empréstimo antes.
- **`end_loan`:** tira do elenco e das táticas de quem pegou, libera o contrato de empréstimo, refaz as folhas, devolve
  ao dono com a marca de elenco (com time reserva: 2 se o principal tem o bit 0x40, senão 1), inscrição europeia 0x10
  → 4.

### 10. O histórico de transferências (0x8cb810) [provado]

Registro de 37 bytes (0x25): +0 pessoa, +4/+8/+0xc ids dos nomes, +0x10 clube destino, +0x14 clube origem, +0x18
valor, **+0x1c o tipo**, +0x1d a data. Os tipos, pelos gravadores: **0 pago**, **1 com jogadores na troca**
(`move_player_to_club` 0x8c99a5: `exchange_players` ≠ 0), **2 empréstimo** (0x8c2180), **3 dispensa** (`free_transfer_staff`
0x8c1af0), **4 livre** (pré-contrato com valor 0 e conclusão 3 ou 4, 0x8c9996), **5 quem não é jogador**
(técnico/comissão: `move_player_to_club` 0x8c99bf e `MANAGER_MANAGER::daily_update` 0x690432; ou só registro em
`free_transfer_staff`). Vetor com capacidade +0x866, conta `past_transfer_count()`, cresce de 100 em 100.

### 11. Pontos de extensão (para o Tweaks)

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

### 12. Dúvidas

- O ponto exato da tela do humano que transforma o fim 1972 (ou o período de zero meses) em recusa [segue inferido,
  ver §7].
- ~~Qual estado prende o empréstimo nas férias~~: respondido em §8 (estado 8, sem férias, pela retirada de uma
  transferência guardada).
- O tipo 3 e 4 de contrato (N/A na tela) existem nos dados? [segue inferido: precisa varrer os contratos de um save
  ou do banco, fora da desmontagem]
- O laço do `ENGLAND_RULES::can_loan_player` testa o id do `player` alvo, não o do jogador do elenco, para pular
  vagas inválidas (o Rust reproduz; efeito nulo na prática) [provado: em 0x579ea8..0x579ec2 o teste lê `[esp+0x84]`
  (o 2º argumento, o jogador-alvo, também lido em 0x579d65) e não `esi` (o jogador da vaga); com um alvo de id válido o
  teste sempre passa].


<a id="d"></a>

## Mercado, seção D: o lado do jogador e quem decide (leitor D, 11/11/2026)

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

### 1. As rotinas

| Endereço | Nome | Arquivo | Em Rust | Item | Quem chama / quando |
|---|---|---|---|---|---|
| 0x846920 | `Shortlist_Manager::staff_interest_in_joining_club` (a vontade) | shortlist_manager.cpp | sim | — | lances (0x84a020, 0x842e10, 0x843800, 0x840084), empréstimos (0x842740), aceitar lance (0x8c0080), prazo de resposta (0x8cc5b0), jogador desestabilizado (0x8c1290), troca (0x8c8860), orçamento (0x83b6e0), decisão de contrato (0x4ce4e0), salário desejado (0x85fc50), olheiros (0x7b0050, 0x7b62a0), filtros e telas |
| 0x844780 | `staff_interest_in_job_at_club` (a vontade da comissão) | shortlist_manager.cpp | não | — | 0x4ce4e0, 0x85fc50, `is_staff_suitable_for_job` (0x69f810), 0x840084, filtros |
| 0x4ce4e0 | `CONTRACT_MANAGER::resolve_contract_offers` (o jogador escolhe a oferta) | contract_manager.cpp | não | **mkd_4** | `process_due_decision_dates` (0x4cd9c0, chamador único em 0x4cda4c), as datas de decisão tipo 3 |
| 0x85fc50 | `STAFF_CONTRACT::set_to_desired_contract` (salário pedido e duração) | staff_contracts.cpp | não | — | `offer_contract` (0x4cda70), `lowest_wage_demand` (0x4ce450), 0x4ce4e0, `make_club_bids` (0x84a020), `accept_transfer_bid` (0x8c0080), `move_player_to_club` (0x8c9720), `get_contract_satisfaction` (0x866620), `calc_contract_value` (0x866830), telas |
| 0x864550 | `STAFF_CONTRACT::default_staff_contract` (salário padrão) | staff_contracts.cpp | não | — | 0x846920 (portão do salário), 0x85fc50 (×3), 0x844780, 0x842e10, 0x840084, `update_wage_bill` (0x598370), 0x85e960 |
| 0x862720 | `staff_contract_calc_unset_default_wage` | staff_contracts.cpp | não | — | `set_default_wage` (0x8626e0), `create_initial_shortlists` (0x83a1b0) |
| 0x8626e0 | `STAFF_CONTRACT::set_default_wage` | staff_contracts.cpp | sim | — | regens (0x7a70e4) |
| 0x866830 | `STAFF_CONTRACT::calc_contract_value` (valor de um contrato contra outro) | staff_contracts.cpp | não | **mkd_1** | `get_contract_satisfaction` (0x866620, ×2), 0x4ce4e0 (×2), 0x4d4830 (tela) |
| 0x866620 | `get_contract_satisfaction` (1..3) | staff_contracts.cpp | sim | — | passe diário das pessoas (0x5ce2d0, random(50) = 0), tela |
| 0x4cda70 | `contract_manager_offer_contract` (a oferta do clube) | contract_manager.cpp | sim | — | 0x4c7ac0 (×9: renovações), telas |
| 0x4ce450 | `contract_manager_lowest_wage_demand` | contract_manager.cpp | sim | — | tela de contrato |
| 0x4c7ac0 | `CONTRACT_MANAGER::daily_update` (fim de contrato, renovação, livres) | contract_manager.cpp | não | — (grande demais; ver §2.4) | `game_main_day_loop` (0x5cd406) a cada fatia do dia: fatia 0 as datas fixas, fatia 2 as pessoas; `game_run_to_start_date` (0x5cc9ef) |
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

### 2. As regras

#### 2.1 A vontade de ir ou ficar (0x846920, Rust) [provado]

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

#### 2.2 O jogador escolhe entre as ofertas de contrato (0x4ce4e0, original; item **mkd_4**) [provado]

`thiscall(gerente de contratos, pessoa)`, `ret 4` (0x4cf493), quadro SEH; chamador único 0x4cda4c, na
`process_due_decision_dates` (0x4cd9c0), quando vence uma data de decisão do tipo 3; igual no cm0102-1080.exe (0xfb6
bytes). Constantes lidas do exe: 1/300 (0x968e60), 3,15 (0x968e58), 0,99 (0x968e50), 0,6 (0x968e48), 0,7 (0x968e40),
3,5 (0x968e38), 0,9 (0x9689d8), 0,8 (0x9689b0), 0,5 (0x968968), 10 (0x9688b0), 40 (0x968e30), 5 (0x9688a0); 2,75
(0x4ce51b) e 3,15 (0x4ceaab) são imediatos, fora da `.rdata`. Ordem [provado, desmontagem]:

1. k = (profissionalismo − ambição − 9 + lealdade) × 1/300: o int (+0x5b − +0x57 + +0x59 − 9) passa por `fild` e
   `fmul` 0x968e60 (0x4ce5a4..0x4ce5ca).
2. Com uma oferta de transferência pendente (`player_has_pending_offer` 0x8cd150): hoje + 2 (0x54c5b0) e uma data de
   decisão do tipo 3 na fatia atual (0x4cd6d0 com [0xae2c88]); nada mais (0x4ce5ce..0x4ce606).
3. Para cada oferta da pessoa com +0x62 ≠ 0 (0x4ce630..0x4ce6fa): oferta sem transferência (+0x59 = −1) de um clube
   que não é o do contrato (procurado com a gravação em [0xae1080]) e tipo ≠ 5: removida (sai da lista e desconta o
   byte +8 do registro de 79 bytes). Estado +0x50 ≠ 0 pula; = 2 liga o "acertada" e no fim nada é escolhido (0x4ce701,
   0x4cec69..0x4cec6e). Estado 0:
   - tipo 5: vontade = 0x846920(…, empréstimo 1, situação 8); tipo 6: com a situação de `get_squad_status_for_club`.
     A vontade é a nota, escolhida **na hora** se for maior que a melhor e ≥ 3,15, antes das recusas
     (0x4ce70c..0x4ce78c, 0x4ce791..0x4ce827);
   - os outros: a situação (nibble alto de +0x4f) vai antes para o byte +5 da entrada da pessoa na oferta de
     transferência (0x8c61e0, 0x8e82f0); o contrato desejado no clube (0x85fc50 com −1, a situação e o cargo +0x61) é
     **um só** contrato local reaproveitado de oferta em oferta; razão = valor(oferta contra desejado) / valor(desejado
     contra desejado) − k (`calc_contract_value` 0x866830); valor do desejado ≤ 0 dá nota 0 (0x4ce82c..0x4ce8f1);
   - a vontade: 0x846920 sem ficha de comissão (+0x69 = 0) ou com cargo 11; senão 0x844780 e, se `is_player_job`
     (0x53b620), (0x846920 + 0x844780) × 0,5 (0x4ce8d8..0x4ce9b8);
   - **renovação** (o clube do contrato, procurado de novo em [0xae1080], é o da oferta): razão ≥ 0,99 e vontade ≤ 3,15
     → vontade = 3,15 (0x4ce9bc..0x4ceaab); o corte razão < 0,6 → 0 vale para **todas** as ofertas comuns, não só a
     renovação (0x4ceac5..0x4cead0);
   - recusa (0x4ceae2..0x4ceb70), para todos os tipos (empréstimo e teste inclusive): (a) cargo que não é de jogador
     simples (0x53b610) para quem não tem ficha de comissão (+0x69 = 0) e não sobra registro de comissão
     (`PLAYER_REGEN::has_spare_non_player_record` 0x7a6e20 = 0); (b) razão > 0,5, precisa de permissão de trabalho
     (0x8c7560) e o fim do contrato **da oferta** a menos de 90 dias (0x54cdb0 de +0x2d até hoje < 0x5a);
   - nota (0x4ceb9b..0x4cec67): razão ≥ 0,9 → vontade = max(vontade, 3,15); razão > 0,8 e vontade > 3,15 → mantém;
     razão > 0,7 e vontade > 3,5 → max(vontade, 3,15); senão recusada. Nota = vontade × razão; maior que a melhor
     (começa em 2,75) → escolhida; a renovação não escolhida (ou recusada) fica lembrada. Empréstimo/teste (desejado =
     0) não passam por aqui.
   Uma oferta já acertada, ou uma pessoa com ficha de comissão candidata a uma vaga de técnico
   (`manager_manager_is_vacancy_candidate` 0x69fc60), não escolhe nada (0x4cec90..0x4cecab).
4. A escolhida vira estado 2 (notícia 0x7549b0, `set_agreed_contract_flag` 0x8c5410); as outras estado 1 (notícia
   0x4d49a0, ou `handle_player_rejected_terms` 0x8c28c0, que recebe a escolhida como 3º argumento, se ligadas a
   transferência); a notícia 0x4d49a0 recebe 1 só para a renovação recusada de um jogador (cargo 11) quando nada foi
   escolhido (0x4cecaf..0x4ced73). Sem escolha, uma renovação **recusada** liga o bit 0x80 do byte +0xb do registro de
   79 bytes; com escolha, o bit é **apagado** no fim, salvo quando a conclusão falha (0x4cf071..0x4cf094,
   0x4cf42d..0x4cf447).
5. **Os sorteios pela lealdade** (escolhida de tipo ≠ 5, 6; 0x4cef64..0x4cf06f): nota > 10 → n = max(40 − nota, 5);
   `random(ftol(n))` < lealdade, clube da oferta com técnico, e reputação do clube > 4500 ou `random(4)` = 0 →
   `random(2)` ≠ 0: passa a gostar do técnico (`staff_add_favoured_staff`); = 0: do clube (`staff_add_favoured_club`).
6. **Terceira passada** (0x4cf096..0x4cf3a2): com escolha, as ofertas do mesmo clube sem transferência **concluem** o
   contrato (`complete_accepted_contract_offer` 0x8c38b0 com a cópia local; sucesso: registro de comissão para cargo
   de comissão sem ficha, técnicos e listas de olheiros se o clube do contrato for humano, valor do jogador
   recalculado); saem da lista e limpam a marca de acerto; com transferência ficam. As de outro clube saem (para a
   lista guardada +0x38 se a escolhida tiver transferência; o clube vai para a lista que a 0x8c2180 recebe). Sem
   escolha, sai toda oferta fora do estado 3. Conclusão que falhou termina aí.
7. **Fim** (0x4cf3a8..0x4cf42d): escolhida com transferência → `handle_player_accepted_terms` (0x8c2180: pessoa, vaga,
   cópia, o dword do cargo +0x61, a lista dos clubes), antes um registro de comissão se for cargo de comissão sem
   ficha; sem transferência → notícia 0x4d49a0 da cópia.

#### 2.3 Salário pedido e duração [provado no que está em Rust; inferido no resto]

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

#### 2.4 Renovações e fim de contrato (0x4c7ac0, original) [provado, desmontagem; não portada]

`thiscall(gerente de contratos, &data, fatia (byte), sinal (byte))`, `ret 0xc` (0x4c9e2e). Chamadores:
`game_main_day_loop` (0x5cd406: hoje, a fatia atual [0xae2c88], 1) a **cada fatia** do dia, e `game_run_to_start_date`
(0x5cc9ef). Constante: 0,004 = 0x968d60 (passos da barra de progresso). Ordem [provado]:

- **As fatias:** roda em cada fatia; a **fatia 0** só recalcula as datas fixas (hoje + 7, 30, 60, 91, 121, 152, 182,
  365, 550, 1095 em 0xae1048, 0xae1030, 0xae1058, 0xae1010, 0xae1060, 0xae1040, 0xae1090, 0xae1068, 0xae1008,
  0xae1020); a **fatia 2** faz o passe das pessoas (só com o sinal = 1); toda fatia termina nas datas de decisão
  (0x4cd9c0 com a data e a fatia) (0x4c7b2e..0x4c7c74, 0x4c7c79, 0x4c7de0, 0x4c9e0e..0x4c9e15).
- **Conversas de contrato** (fatia 2): cada registro de conversa (21 bytes, +0x28/+0x2c) vence em (25 − lealdade do
  técnico) × 2 dias desde a data +0xc; vencido sai (troca pelo último) e o vetor é reordenado (qsort 0x4dc970). É o
  primeiro uso da lealdade (0x4c7c81..0x4c7d7b).
- **Barra "Updating game data"**: em 31/12 (0x549f70 dia 31, mês 11), a cada 250 pessoas (0x4c7b07..0x4c7b29,
  0x4c7d7e..0x4c7e2f).
- **Antes da carga completa** ([0xdd711c] = 0, `G_GAME_DATA_LOADED_FLAG`): o contrato que vence hoje só vira tipo 1
  (+0x35 = (x & 0xc1) | 1) (0x4c7e34..0x4c7ec0).
- **Valor do jogador recalculado** (0x4c7f65..0x4c8070): clube humano e cargo de jogador: com o bit 4 de +0x4e e
  ficha, valor recalculado (0x4d0c60 sinal 1); clube da IA e cargo de jogador: **nos dias ímpares do ano**, bit 4
  ligado e valor recalculado (sinal 0); comissão: todo dia; nos dois primeiros, limitado pela multa de rescisão ativa
  (0x867b50). Sem contrato ou clube: valor 0.
- **Fim do empréstimo** (0x4c8077..0x4c8114): o contrato de empréstimo (índice +4 do registro de 79 bytes) que vence
  hoje: sem clube, só o índice vira −1; senão notícia 6 (0x4da590, fora do tipo 7) e `end_loan` 0x4d07e0.
- **Renovação antes da mudança das regras de transferência** (0x4c8152..0x4c847a): antes de [0xae2330]
  (`G_TRANSFER_RULE_CHANGE_DATE`) e com ficha, quando essa data cai em hoje + 7/30/42/60/70/91 (recalculadas **por
  pessoa**, em 0xae1070..0xae1088), o contrato começou há mais de 180 dias e acaba antes de hoje + 1095: clube que não
  o rejeita e cujo técnico não o rejeita (0x5477d0, 0x547840), da IA, idade < 32 → pelo estatuto: 1 oferta (fator 1,0);
  2 e 3 com reputação atual > 6750, ou idade < 24 e potencial > 100; 5 com potencial > 100; 4 nunca.
- **Período protegido da FIFA** (0x868210; 0x4c847f..0x4c85fe): contrato tipo 2, sem empréstimo, que acaba **depois**
  de hoje + 182, fora do período protegido (0x868210 = 0 e a saída ≠ 0), sem aposentadoria (0x7a7eb0), cargo 11: a IA
  (idade < 32) renova pelos mesmos estatutos (fator 1,0); clube humano: notícia 7 (0x4d6790; Bosman pela idade em
  hoje + 182 ≥ 24 e 0x8d0a70).
- **Tipo 1 (fim de contrato vencido)** (0x4c8603..0x4c883b): sem empréstimo, passados 31 dias do fim, querendo sair
  (0x866f60) ou saindo pelo Bosman (+0x49), sem ofertas nem trocas nem fila: notícia 1 ao clube humano; cargos 12, 13
  (IA) e 15 (IA) voltam a 5, 6 e 8 e saem do elenco; os outros vão à **fila de transferência livre** (0x8c16b0 com
  1, −1).
- **A faixa** (`get_expiry_band` 0x867d30) contra +7..+182 (0x4c8876..0x4c88a5, 0x4c8a79..0x4c8abb): as faixas
  ímpares (o dia exato de 182, 152, 121, 91, 60, 30 e 7 dias) põem o jogador nas listas dos clubes (0x842c00 com 1); a
  faixa 14 só com `random(20)` = 0; quem já sai pelo Bosman também, nas ímpares.
- **A saída pelo Bosman** (+0x49 = 1; 0x4c88b5..0x4c8a74): tipo 2, contrato começado há mais de 93 dias, na janela das
  faixas (0x867d30 ≠ 0 e ≠ 14), sem aposentadoria, cargo 11, contrato atual fora de empréstimo, clube com nação;
  reputação atual > 3750, mundial > 1750, **lealdade < 15**, idade 19..30, a nação permite (0x544670), e não (clube >
  4750 e idade < 23); e querer sair (0x866f60) ou o bit 8 de +0x4e. Então +0x49 = 1, bit 8 apagado, listas dos clubes
  (0x842c00) e a notícia 0x4d6d70. É o segundo uso da lealdade.
- **A oferta de renovação da IA** (`offer_contract` 0x4cda70, ×9; pessoa, clube 0, saída 0, vaga −1, fator, modo 2),
  só para clube da IA e jogador sem aposentadoria; o fator (f64) é um imediato empilhado: faltando **550 dias**
  (0xae1008), estatuto 1..3 e na lista do próprio clube (0x849ce0): 1,0; **365 dias** (0xae1068, ou no primeiro dia do
  jogo com fim antes disso): notícia 6 e, estatuto 1..3 ou 5 na lista: 1,0; faixas **3 e 5** (152/121 dias): estatuto
  1..3 na lista 1,2, fora 1,0; 4 e 5 → 1,0; faixa **1** (182): 1,1 na lista / 1,0; faixa **7** (91): 1,3 na lista /
  1,0, 1,1 para 3..5; faixas **9, 11** (60/30): 1,4 (1..2) e 1,2 (3..5) na lista, fora 1,0; faixa **13** (7 dias): o
  mesmo, com notícia ao clube humano. Quando a 0x4cda70 devolve 0 nos caminhos de 365 dias e das faixas 1 e 7, uma
  oferta local de tipo 2 vira a notícia de recusa (0x4d49a0 com 1); nas faixas 1 e 7 liga também o bit 0x80 do
  registro de 79 bytes (0x4c910e, 0x4c91e3, 0x4c93d0, 0x4c94a5). Prova: 0x4c8ac0..0x4c9a8b, 0x4c9b5f..0x4c9b71; os
  fatores 1,1 = 0x3ff19999_9999999a, 1,2 = 0x3ff33333_33333333, 1,3 = 0x3ff4cccc_cccccccd, 1,4 = 0x3ff66666_66666666.
- **Listado sem ofertas** (0x4c9a90..0x4c9b10): jogador (cargo 11) listado para transferência (0x5444a0) sem ofertas
  nem trocas nem fila: entra na fila de transferência livre.
- **O fim do contrato no dia** (0x4c9b76..0x4c9d21): tipo 5 (empréstimo) que acaba hoje: notícia 5, estatísticas
  arquivadas (0x7c5fe0), o contrato apagado (os dois índices −1), folha salarial (0x598370), fora das escalações
  (0x896c60 ×2) e do clube (0x53c190). Outro tipo: vira tipo 1; saindo pelo Bosman, sem empréstimo e sem ofertas:
  notícia 2 e **liberado na hora** (`free_transfer_staff` 0x8c1af0).
- **Datas de decisão** (0x4c9d40..0x4c9e07): nos dias 36 e 200 do ano (fatia 2), as já processadas saem do vetor; em
  toda chamada, se houve datas novas (+0x1c), a parte pendente é reordenada (qsort com 0x4dc910) e o +0x1c zera.

Atributos: só `lealdade` (+0x59, ×2): conversas (0x4c7cc3) e Bosman (0x4c8a01).

**Por que não virou item:** 9.376 bytes (2.700 instruções, 10 tabelas de saltos), ~45 rotinas chamadas (a barra de
progresso inclusive), 9 caminhos de renovação e o fim de contrato; grande demais para provar com uma comparação (não
roda duas vezes: notícias, sorteios, filas). Proposta: portar em pedaços pelos chamadores já em Rust (o valor do
jogador, o fim do empréstimo, a regra do Bosman e as renovações por faixa como ajudantes), com o laço por último.

#### 2.5 Cláusulas [provado onde em Rust]

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

#### 2.6 Jogadores livres [provado onde em Rust]

Quem fica livre: o fim do contrato em 0x4c7ac0 [provado, §2.4]: no dia do fim, só quem sai pelo Bosman (+0x49, a regra
da nação 0x8d0a70) é liberado na hora (`free_transfer_staff` 0x8c1af0); os outros viram tipo 1 e, 31 dias depois, só vão
à fila se quiserem sair (0x866f60) ou saírem pelo Bosman; e os dispensados pela fila (0x8c16b0 →
`process_free_transfer_queue`, no dia das transferências: no clube humano a diretoria pode barrar, um sorteio põe o
técnico nos desafetos, compensação). Quem os contrata: as listas dos clubes (`add_player_to_club_shortlists`,
`make_club_bids` 0x84a020; sem clube a vontade ganha × 1,75 e a base 2,75 + mín(12, dias² / 900)). Por quanto: o
contrato desejado (0x85fc50) contra o teto (`get_max_wage`).

### 3. Atributos do presidente e da comissão no mercado (a pergunta do pedido)

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
| Youngsters (+0x22), técnico | 0x8400ae | `fill_shortlist_slot` 0x83f3a0 (resto em 0x840084) | não | vaga que não é de empréstimo, menor de 20 com reputação doméstica (+9) < 3250 e clube atual < 3250: só segue se edi mod (25 − jovens) = 0 (sem ficha de não-jogador no técnico, mod 15) [provado: 0x8400ae..0x8400d4]; `edi` = id da pessoa + id do clube (0x83fee7..0x83feee, a mesma soma do teste % k; nenhuma escrita em `edi` até 0x8400b9 nos três caminhos 0x83ff0b, 0x83ff16, 0x83ff26) [provado]: **fixo por par (pessoa, clube)**, o mesmo jovem é sempre aceito ou sempre ignorado pelo mesmo clube enquanto o técnico for o mesmo; com jovens 20 passam os pares de soma múltipla de 5. Só chega aqui quem já falhou no % k |
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

### 4. Constantes (lidas do exe)

0x846920: 3,5 (0x968e38), 2,75 (0x9690a8), 1,75 (0x968f98), 3,15 (0x968e58). 0x4ce4e0: 1/300 (0x968e60), 3,15
(0x968e58), 0,99 (0x968e50), 0,6 (0x968e48), 0,9 (0x9689d8), 0,8 (0x9689b0), 0,7 (0x968e40), 0,5 (0x968968). 0x866830:
0,8 / 0,3 / 0,2 / 0,4 / 0,7 / 0,5 (f64), 0,2222222 (f32 0x96ff84), multiplicadores f32 1,01 (0x96ff7c), 1,05
(0x96d3b8), 1,1 (0x96a33c), 1,15 (0x96d390), 1,2 (0x96d7bc), 1,25 (0x96b010), 1,45 (0x96ff74), 1,5 (0x96b004), 1,7
(0x96ff78), 1,75 (0x96b00c), 1,8 (0x96d7b4), 1,95 (0x96ff80), 1,25 f64 (0x968928). 0x5968d0 (presidente): 1,05
(0x9689b8), 1,1 (0x9689c0), 0,95 (0x9689d0), 1,125 (0x96a5a8). Quase todas são compartilhadas por dezenas de rotinas:
mudar a `.rdata` muda o jogo inteiro.

### 5. Defeitos do original

1. `calc_contract_value` (0x866830): sem oferta, lê as cláusulas da oferta por ponteiro nulo (0x866df6); nenhum
   chamador passa nulo [provado]. Sem ficha de jogador a idade entra no lugar de "faixa − idade" (a = idade, 5 depois
   do limite) [provado]. A cláusula de não promoção (caso 0) olha a cláusula de multa mínima da oferta e usa a faixa
   de 7,5 milhões, as outras 5 milhões [provado (mkd_1); a intenção segue inferida: não se prova pelo código].
2. `staff_interest_in_joining_club`: no próprio clube divide por um float da pilha que ninguém escreveu (sem efeito,
   o caminho não usa o resultado; já anotado no Rust).
3. `get_max_wage`: o presidente com **pouco** tino para negócios (< 6) paga **mais** (× 1,05/× 1,1) e o bom (> 15)
   paga menos (× 0,95) [provado]. **Não é invertido** pelo código: nas duas rotinas do dinheiro um presidente de
   Negócios alto gasta menos: no orçamento guarda mais semanas de salário de reserva (Negócios + 6, 0x598b24..0x598b2a)
   e no teto salarial paga × 0,95 (0x597a0d). É um presidente prudente, coerente nas duas [provado]. Se era o
   pretendido segue sem prova [inferido].
4. Documentação Rust: o cabeçalho de `young_player_adjustment` (shortlist_manager.rs) diz "current ability seen", mas
   chama `perceived_ability` com `kind` 1, que é a potencial (lê JudgingPotential +0x18) [provado].
5. 0x4ce4e0: o registro de comissão sobrando é aceito até o **próprio total** (`cmp eax, [0xae2368]; jg` é o erro,
   devia ser `jge`) [provado: 0x4cf1b5].
6. 0x4ce4e0: com várias ofertas do **mesmo clube** e a escolhida sem transferência, `complete_accepted_contract_offer`
   roda **uma vez por oferta** (o laço chama de novo a cada uma) [provado: 0x4cf0c9..0x4cf15c].
7. 0x4ce4e0: empréstimo e teste são escolhidos antes das recusas: uma recusa depois (permissão de trabalho, falta de
   registro) zera a nota guardada, mas não desfaz a escolha [provado: 0x4ce75c..0x4ce764 contra 0x4ceb68].
8. 0x4ce4e0: o marcador "renovação" e a vontade são variáveis da pilha que atravessam as ofertas: o primeiro valor do
   marcador é o numerador de k (0x4ce5bc) e uma oferta com valor desejado negativo usaria os de uma oferta anterior
   [provado]; sem efeito na prática (o valor nunca é negativo) [inferido].
9. 0x4c7ac0: as seis datas da mudança de regras (0xae1070, 0xae1018, 0xae1038, 0xae1028, 0xae1078, 0xae1088) são
   recalculadas **para cada jogador** do laço, não uma vez por dia (mesmo resultado, trabalho à toa) [provado:
   0x4c81a3..0x4c827c].
10. 0x4c7ac0: o valor do jogador da IA só é recalculado nos dias **ímpares** do ano (`movsx eax, word [data]; and
    0x80000001`), o do humano todo dia [provado: 0x4c7ffe..0x4c8010]; a intenção [inferido].

### 6. Pontos de extensão (o menor gancho que muda a política)

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
8. **0x4ce4e0 (mkd_4, Rust quando integrado)**: o limiar 2,75, os cortes 0,9/0,8/0,7, o piso 3,15 da renovação e o k
   da personalidade ficam numa função só (`choose_contract_offer`): o lugar para mexer na disposição de renovar ou de
   trocar de clube por grupos (idade, reputação), sem tocar a `staff_interest_in_joining_club`.
9. **0x4cda70 nas renovações da IA**: o fator (1,0..1,4) é um imediato empilhado em cada uma das 9 chamadas da
   0x4c7ac0; um desvio na `contract_manager_offer_contract` (Rust) que ajuste o fator pelo jogador muda todas sem
   portar a 0x4c7ac0. A regra do Bosman (lealdade < 15, idade 19..30, reputações 3750/1750) está num trecho só
   (0x4c89ea..0x4c8a4a).

### 7. Itens na fila

- **mkd_1** — `STAFF_CONTRACT::calc_contract_value` (0x866830): `levas/fila/mkd_1/`. Compila com o motor atual (cópia
  própria, sem avisos do item). **Não provado no jogo**: a trava do laboratório estava ocupada e a vigia de memória
  recusou (2,3 GB livres, mínimo 4). Ver `PRONTO.txt`.
- **mkd_4** — `CONTRACT_MANAGER::resolve_contract_offers` (0x4ce4e0, §2.2): `levas/fila/mkd_4/`. Compilado; a prova
  fica com a sessão principal.

Não portados por tamanho (para outra rodada): 0x85fc50 (10.896 bytes), 0x864550 (8.400), 0x862720 (7.728), 0x844780
(8.608), 0x4c7ac0 (9.376; ver o fim do §2.4). Só de tela (sem efeito na carreira automática): 0x4d4660, 0x4d4830,
0x4d44e0, 0x867ad0, 0x85f7a0.
