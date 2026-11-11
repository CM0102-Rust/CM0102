*Versão em inglês: [secao_a.en.md](secao_a.en.md).*

# Mercado, seção A: o lado do dinheiro (leitor A, 11/11/2026)

Quanto o clube pode gastar, quanto pede por um jogador e quanto vale um jogador para quem compra. Marcas:
**[provado]** = lido na desmontagem (ou no código Rust já comparado com o jogo); **[inferido]** = pelo pseudo-C do
Ghidra, pelo uso ou pelo mapa da comunidade, sem prova.

## As rotinas

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

## 1. A verba de transferências: `Finance::get_transfer_budget` (0x598800)

`thiscall(finanças, jogador, somar_listados, *ajustada, conhecimento, pendentes)`, `ret 0x14`, devolve **i64**
(edx:eax) **[provado]**. 37 pontos de chamada (`refs.py`): os lances da IA (`TRANSFER_OFFER::build_bid` 0x8e5d60 e
vizinhos 0x8e6330, 0x8e74a2, 0x8e839c, 0x8ea2af, 0x8ecf7f..0x8ee61d), as listas de observação (verba por vaga,
`shortlist_manager.rs` 2486), a diretoria (`manager_manager.rs` 837 e 1820: pedido de verbas, o veto), os eventos
financeiros (0x59e329..0x59ed38), o contrato (0x4d753c, 0x4de3a6), as telas (0x463af8, 0x6a3b87, 0x7b6d5a...). As
chamadas Rust de hoje passam `(fin, 0, 0, 0, 1, 0)`: sem jogador, sem listados, sem pendentes. Os lances da IA rodam
todo dia na janela, para cada clube do computador que compra **[inferido]** pelos chamadores.

A regra, na ordem **[provado]** (desmontagem 0x598800..0x5997f9, 1240 instruções):

1. Começa no saldo (`finance::BALANCE` +0, i64). Com `somar_listados`: soma o valor (`staff::VALUE` +0x52) de cada
   uma das 50 vagas do elenco (`club::SQUAD` +0xd7) com pessoa listada para venda (0x5444a0).
2. `*ajustada` = 0.
3. Com `pendentes` e clube humano (0x600a50 com 1, 1): as transferências futuras do clube (0x8ccb80, linhas de 27
   bytes: pessoa +8, venda +0x10, taxa +0x14, estágio +0x19): compra soma a taxa ao comprometido (menos a do próprio
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

### Defeitos do original
- **[provado] Saldo negativo no passo 16:** com ficha de jogador, V vira `_ftol(razão × 0,65)` (a razão da reputação
  do jogador pela do clube, entre 0,15 e 0,85): o V se perde e fica 0. A verba vira um oitavo dela se isso for até
  125000, senão **0** (o teto 1,75 × 0). Para não jogador, V × 0,65 (o que parece ser a intenção) **[inferido]**.
- **[provado] Passo 3, pessoa repetida:** depois de achar a pessoa numa linha anterior, o salário descontado é o do
  contrato da pessoa da linha seguinte à primeira ocorrência, não o desta linha.
- **[provado] Passo 12:** o gasto líquido da temporada entra só pelo dword baixo (acima de 2,1 bilhões vira
  negativo e some); o comprometido S sai também pelo dword baixo.
- **[inferido]** O passo 4 devolve a verba sem nenhuma reserva para clube sem liga (clubes de fundo sem competição).

### Pontos de extensão
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

## 2. O preço pedido pelo jogador do clube humano: `contract_manager_get_human_asking_price` (0x4d42d0)

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
Listado ou pedindo saída (0x544520), ou sem o bit 4 de `contract::TRANSFER_STATUS` (+0x4e) **[inferido: o técnico
fixou o preço]**: o piso. Pelo papel no elenco (`contract::SQUAD_STATUS` +0x4f & 0xf0): 0x10 e 0x50 o preço; 0x20 e
0x30 a média (com sinal, truncada); os outros, o piso.

Nota: dentro da 0x4d1600, vendedor humano com o técnico **de férias** (0x5fd210, 0x5fea50 com a opção 0) e sem o bit
4 devolve logo o valor guardado **[provado: 0x4d16xx]**: na carreira de testes (técnico de férias) o preço pedido do
clube humano é o valor guardado.

**Ponto de extensão:** o retorno; ou o piso (o `staff::VALUE` +0x52, que o próprio jogo atualiza).

## 3. O preço pedido: `contract_manager_calc_asking_price` (0x4d1600) — mapa, sem item

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

## 4. O valor do jogador para o clube: `value_player_for_club` (0x840d80, em Rust)

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

## 5. O valor do jogador: `calc_player_value` (0x4d0c60, em Rust)

Ver o cabeçalho em contract_manager.rs 902: base por posição (6 a 13 milhões), idade de pico 28/29, as notas da
ficha, a habilidade vista pelo técnico do clube × 50 + a reputação, a tabela 0x9a7f30 por idade, a nação do clube
(desenvolvimento, padrão da liga), os dias de contrato e a regra Bosman, Portugal/França/Coreia, a multa. É a base
de tudo acima: 0x4d1600 soma um prêmio a ele e 0x4d42d0 o usa como piso (via `staff::VALUE`, que o jogo grava
com ele). 58 chamadores.

## 6. Reputação nos preços

- **Reputação do clube** (`club::REPUTATION` +0x80): na verba, r em quase todos os passos (r × m, público × r, r²,
  300·r, os limites 5000/3500, os tetos por nação); no preço pedido, a tabela de riqueza (r / 500), os limiares 4001,
  4250, 6751 e o "clube grande"; no valor para o clube, os cortes de reputação < 3500 dos olheiros.
- **Reputação da liga** (`competition::REPUTATION` +0x69): na verba (passos 10 e 14) e no fator da liga da 0x4d1600.
- **Reputação do jogador** (`player_sheet::CURRENT_REPUTATION` +0xb, `WORLD_REPUTATION` +0xd): no valor
  (0x4d0c60) e na habilidade vista (0x6e86d0); na verba só no defeito do saldo negativo.
- **Reputação do técnico** (`non_player::CURRENT_REPUTATION` +0xa): na verba, o q do teto suave e do conhecimento.

## 7. Item

- **`mka_1`** (`levas/fila/mka_1/`): `get_transfer_budget` (0x598800) e `get_human_asking_price` (0x4d42d0),
  comparadores lado a lado pelo estado do sorteio; mutações: Negócios + 1 no lugar de + 6, metade do excesso no teto
  suave, o piso do preço do humano dobrado. A prova está em `PRONTO.txt`.
