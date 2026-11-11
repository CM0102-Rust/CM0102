*Versão em inglês: [fechamento_mkc.en.md](fechamento_mkc.en.md). As duas são mantidas juntas; o português é o original.*

# Fechamento do mapa do mercado, leitor mkc: as regras de empréstimo das nações fora das 8 ligas (11/11/2026)

Rotinas: as regras de empréstimo e de janela que a seção C do mapa (`docs/MAPA_MERCADO.md`, §1, §3, §4) dava como
"só mapeadas": Finlândia 0x5a9a80/0x5a9af0/0x5a9ba0, Noruega 0x794760, Turquia 0x902240, Irlanda do Norte
0x793870/0x793900, Rússia 0x7ec8e0, País de Gales 0x90f1a0, Alemanha 0x5e0210, Irlanda 0x63c530/0x63c4e0. Tudo abaixo
vem da desmontagem do `jogo/cm0102.exe` (`desmonta.py`), com o endereço da instrução. Para cada afirmação do mapa: o
texto antigo, o novo e a prova. A sessão principal troca [inferido] / "só mapeado" por [provado] e corrige a coluna
"Item" do §1.

Itens: **mkc_3** (0x90f1a0, 0x7ec8e0, 0x902240; LIGAS das 8 ligas, porque **rodam nelas**), **mkc_4** (Finlândia),
**mkc_5** (0x793870, 0x793900, 0x794760), **mkc_6** (0x5e0210, 0x63c530, 0x63c4e0); os três últimos com LIGAS =
`ENG FIN NOR NIR GER IRL`. Todos compilados numa cópia (com mkc_1 e mkc_2), mutações compiladas, nada rodado no jogo.

## 0. A lista conferida contra o código: as 29 tabelas virtuais das regras

A lista do mapa vinha do mapa da comunidade (nomes `*_RULES::`). Conferida pelo código: toda tabela virtual de regras
tem 0x40a5f0 no slot +0x14; a varredura da `.rdata` por esse dword acha **29 tabelas** (28 nações + a base), e cada
uma é gravada por um único construtor (`refs.py`; 15 delas pelos construtores já em Rust, `construct_nation_rules`,
as outras pelos nomes do mapa da comunidade nos construtores 0x5c1570 França, 0x7ec770 Rússia, 0x555460 Dinamarca
etc.). Os slots dos empréstimos (+0 `daily_update`, +8 `record_loan`, +0x20 `CAN_LOAN_PLAYER`, +0x24 `CAN_END_LOAN`,
+0x28 `GET_LOAN_PERIOD`; "base" = 0x40a600 / 0x40a610 / 0x8e8bd0, "nada" = 0x583470):

| Tabela | Nação | +0 | +8 | +0x20 | +0x24 | +0x28 |
|---|---|---|---|---|---|---|
| 0x9674a0 | Argentina | 0x40a710 | nada | base | base | base |
| 0x967830 | Austrália | nada | nada | 0x412dd0 | base | **0x90f1a0** |
| 0x967c10 | Bélgica | nada | nada | base | base | 0x90f1a0 |
| 0x968838 | Brasil | nada | nada | base | base | **0x7ec8e0** |
| 0x969394 | Croácia | nada | nada | 0x5ed060 | base | 0x90f1a0 |
| 0x969860 | Dinamarca | nada | nada | 0x793900 | base | base |
| 0x969f4c | Inglaterra | 0x57a020 | 0x579fd0 | 0x579d60 | base | 0x579c70 |
| 0x96a188 | Europa (genérica) | nada | nada | base | base | base |
| 0x96a75c | Finlândia | nada | nada | 0x5a9af0 | 0x5a9ba0 | 0x5a9a80 |
| 0x96ae48 | França | 0x5c1710 | nada | **0x412dd0** | 0x5c1ed0 | **0x90f1a0** |
| 0x96b3cc | Alemanha | nada | nada | base | 0x5e0210 | 0x90f1a0 |
| 0x96b784 | Grécia | nada | nada | 0x5ed060 | base | 0x90f1a0 |
| 0x96ba88 | Holanda | nada | nada | 0x794760 | base | 0x90f1a0 |
| 0x96c190 | Irlanda | **0x57a020** | 0x63c4e0 | 0x63c3d0 | 0x63c530 | 0x90f1a0 |
| 0x96c96c | Itália | nada | nada | 0x661770 | 0x6618f0 | base |
| 0x96ccfc | Japão | nada | nada | base | base | base |
| 0x96cfec | Coreia | nada | nada | 0x793900 | base | 0x90f1a0 |
| 0x96df9c | Irlanda do Norte | nada | nada | 0x793900 | base | 0x793870 |
| 0x96dfe4 | Noruega | nada | nada | 0x794760 | base | base |
| 0x96e61c | Polônia | nada | nada | 0x5ed060 | base | 0x7ec8e0 |
| 0x96e9c0 | Portugal | nada | nada | base | base | base |
| 0x96f340 | Rússia | nada | nada | 0x412dd0 | base | 0x7ec8e0 |
| 0x96f838 | Escócia | nada | nada | 0x7f6650 | base | 0x7f6530 |
| 0x96fdec | Espanha | nada | nada | 0x859630 | base | base |
| 0x970368 | Suécia | nada | nada | base | base | base |
| 0x970458 | base `TRANSFER_RULES` | nada | nada | base | base | base |
| 0x97075c | Turquia | nada | nada | 0x902240 | base | 0x90f1a0 |
| 0x970a38 | EUA | nada | nada | **0x902240** | base | base |
| 0x970a9c | País de Gales | nada | nada | 0x793900 | base | 0x90f1a0 |

O que muda: as rotinas "de uma nação" são, várias delas, de várias; e **três rodam nas 8 ligas**.

## 1. A tabela das rotinas (§1 da seção C)

| Afirmação do mapa | Texto novo | Prova |
|---|---|---|
| 0x5a9a80 / 0x5a9af0 / 0x5a9ba0 Finlândia: "fora: não roda nas 8 ligas" | **[provado]**: só a tabela finlandesa 0x96a75c aponta para elas; como regra de quem pega emprestado não rodam nas 8 ligas (a `can_end_loan`, regra do dono, só se a IA pedir emprestado um jogador de clube finlandês). Item **mkc_4** | §0 |
| 0x794760 / 0x902240 Noruega, Turquia: "fora (idem)" | **corrigido [provado]**: 0x794760 é da Noruega **e da Holanda** (item **mkc_5**); 0x902240 é da Turquia **e dos EUA**: **roda nas 8 ligas** (clube americano pegando emprestado). Item **mkc_3** | §0 |
| 0x793870 / 0x793900 / 0x7ec8e0 / 0x90f1a0: "fora (idem)" | **corrigido [provado]**: 0x793870 só da Irlanda do Norte; 0x793900 da Dinamarca, Coreia, Irlanda do Norte e País de Gales (**mkc_5**); 0x7ec8e0 do **Brasil**, Polônia e Rússia e 0x90f1a0 de **11 nações**, entre elas **França e Austrália**: as duas **rodam nas 8 ligas** (**mkc_3**) | §0 |
| 0x5e0210 / 0x63c530 / 0x63c4e0 Alemanha, Irlanda: "fora (idem)" | **[provado]**: só das próprias tabelas (0x96b3cc, 0x96c190). Item **mkc_6** | §0 |
| (novo) | 0x412dd0 (`AUSTRALIA_RULES::can_loan_player`, mkc_2) é também o `CAN_LOAN_PLAYER` da **França** e da Rússia: na carreira de 8 ligas ele roda também pelos clubes franceses **[provado]** | tabelas 0x96ae48 +0x20, 0x96f340 +0x20 |
| (novo) | 0x5ed060 (Grécia, também Croácia e Polônia) é `mov eax, 1; ret 0x10` (8 bytes): nada a portar **[provado]** | 0x5ed060..0x5ed065 |
| (novo) | A Irlanda usa como `daily_update` (+0) a inglesa 0x57a020 (já em Rust): a conta de empréstimos da temporada dos clubes irlandeses também é zerada em 20 de junho **[provado]** | tabela 0x96c190 +0 |

## 2. O período (§3)

Assinatura comum (a da 0x8d0590): thiscall(regra, clube, a2, &mês_início, &ano_início, &mês_fim, &ano_fim, jogador),
`ret 0x1c`. Parte comum: início = mês de hoje (0x54cfb0 sobre 0xae2c90) + k; ano = [0xae2c92]; início > 11 (com
sinal): −12 e ano + 1; mês do fim = `contract_manager_season_end_month(clube)` (0x4d45f0 com ecx = [0xae1050]); início
< fim (com sinal): ano do fim = ano do início, senão ano de hoje + 1. `a2` e `jogador` não são lidos.

| Afirmação do mapa | Texto novo | Prova |
|---|---|---|
| Finlândia 0x5a9a80: "como a base" | **[provado]**: k = 1, a mesma conta da 0x8e8bd0 (que zera o mês em vez de subtrair 12: mesmo resultado, o máximo é 12) | 0x5a9a80..0x5a9aeb |
| Rússia 0x7ec8e0: "início em 3 meses, fim no fim da temporada" | **[provado]**: k = 3; vale para **Brasil**, Polônia e Rússia. **Defeito latente [provado no código]**: com o início virando o ano (hoje outubro a dezembro) e o mês do fim da temporada ≤ o mês do início (temporada acabando em janeiro ou fevereiro), o fim fica no mesmo ano do início e antes dele (ou igual). Se acontece com os dados destas nações não foi conferido | 0x7ec8f4 (`add al, 3`), 0x7ec924..0x7ec947 |
| Irlanda do Norte 0x793870: "o mesmo, e o início nunca passa do fim" | **[provado]**: k = 3 e depois, início depois do fim (ano sem sinal, depois o mês com sinal), o início vira o fim. É a Rússia com a correção do defeito acima. Só a Irlanda do Norte | 0x793885, 0x7938d7..0x7938f3 |
| País de Gales 0x90f1a0: "início = fim (fim da temporada)" | **[provado]**: k = 1 e depois início = fim (mês e ano). Vale para **11 nações**: Austrália, Bélgica, Croácia, França, Alemanha, Grécia, Holanda, Irlanda, Coreia, Turquia e País de Gales. Consequência **[provado]**: no `can_loan_player` (0x8d0040, §4 passo 5) início = fim devolve 1 na hora, sem perguntar o `CAN_END_LOAN` do dono, a não ser que a 0x8d0590 mude o fim pelo contrato (§3 passos 2–4); na IA (0x842740) os meses até o início e até o fim são os mesmos | 0x90f1b4, 0x90f1ff..0x90f206 |

## 3. Pode pegar emprestado? (§4 passo 3)

Assinatura: thiscall(regra, clube, jogador, mensagem, &motivo), `ret 0x10`.

| Afirmação do mapa | Texto novo | Prova |
|---|---|---|
| "base 0x40a600 / Grécia 0x5ed060: 1; Irlanda do Norte 0x793900: 1 (grava [0xae1080])" | **[provado]**, com as nações: base para Argentina, Bélgica, Brasil, Europa, Alemanha, Japão, Portugal, Suécia; 0x5ed060 para Croácia, Grécia, Polônia; 0x793900 para **Dinamarca, Coreia, Irlanda do Norte e País de Gales**: procura o contrato principal do jogador (0x4cf5a0), guarda em [0xae1080], calcula o clube dele e não usa; 1 | 0x793900..0x793934 |
| "Austrália 0x412dd0, Noruega 0x794760, Turquia 0x902240: fora da janela, 9" | **[provado]**: as três: `find_open_window(hoje, 0)` (0x8e8e50); fechada: com mensagem o texto com o adjetivo da nação da regra (+0xa, forma 2), o motivo 9 (mesmo sem mensagem) e 0; aberta: 1. Diferenças: a norueguesa (também **Holanda**) usa o texto 0x9cf088 (exemplo "Dutch", mesmo formato); a turca (também **EUA**) com a janela aberta procura o contrato e grava [0xae1080] antes do 1 | 0x794760..0x7947cb; 0x902240..0x9022da (0x9022a2..0x9022b4) |
| "Finlândia 0x5a9af0: o mesmo e, para um clube da IA, só entre os dias 90 e 300 do ano" | **[provado]**: janela fechada como acima (texto 0x987d38). Aberta: com clube e não humano (`human_manager_is_human_club(clube, 1, 1, 0, 0)` = 0, cdecl), o dia do ano de hoje (palavra com sinal em 0xae2c90; 0 = 1º de janeiro, tabela 0x9b8448) > 300 ou < 90: **0 sem texto e sem motivo**. 90 = 1º de abril, 300 = 28 de outubro (31 de março e 27 de outubro no ano bissexto). Somada à janela (16/11–15/8), a IA finlandesa só pega emprestado de 1º de abril a 15 de agosto | 0x5a9b52..0x5a9b89 |

## 4. Pode terminar? (`CAN_END_LOAN`, §4 passo 5)

Assinatura: thiscall(regra, clube, jogador, mensagem, data, flag), `ret 0x14`. Chamadores da 0x8d0530 conferidos: tela
0x8fa341 (flag 1, sem mensagem), tela de encerrar 0x8fbcf9 (**flag 0**, mensagem 0xde1f64, data de hoje, clube =
`club_job` do jogador), IA 0x8429ae (flag 1); a 0x8d0040 sempre com flag 1.

| Afirmação do mapa | Texto novo | Prova |
|---|---|---|
| "Irlanda 0x63c530: a data dentro de uma janela da nação" | **[provado]**: data nula = hoje; `find_open_window(data, 0)` com o ecx da entrada; fechada: 0 e, com mensagem, o adjetivo da Irlanda ([0x9cf2dc], forma 10) em 0x9cd030 ("sent back", quando `clube` é o `club_job` do jogador) ou 0x9ccfbc ("recalled"). O mesmo código da França (0x5c1ed0) com a chave irlandesa. Janela irlandesa: 10 de maio a 31 de janeiro | 0x63c530..0x63c5d9 |
| "Finlândia 0x5a9ba0: recusa entre 18 de agosto e 15 de novembro" | **[provado]**, preciso: recusa com 18/8 do ano da data **antes** da data e a data **antes** de 15/11 (estrito nas duas pontas; ano sem sinal, depois o dia com sinal; datas por 0x549f70(18, 7, ano, −1) e (15, 10, ano, −1)). Textos 0x9cc1a4 ("cannot be sent back until 15th November") / 0x9cc170 ("recalled"), sem adjetivo. A data **não é conferida** (nula quebraria; nenhum chamador passa nula). A flag não é lida | 0x5a9bb9..0x5a9ca5 |
| "Alemanha 0x5e0210: sem a flag, recusa encerrar empréstimos entre clubes alemães (com a flag, 1)" | **[provado]**, preciso: flag ≠ 0: 1. Senão recusa quando o clube do jogador (`club_job`, +0x39) é alemão ([0x9cf314]) **ou não tem clube ou nação**; outro país: 1. Com mensagem, o adjetivo alemão (forma 2) em 0x9cd5cc ("Players on loan from other German clubs cannot be sent back until the end of the loan.", quando `clube` = `club_job`, relido) ou 0x9cd574 ("...cannot be recalled."). Como a 0x8d0530 só chama a regra com o dono e o clube na mesma nação, é "entre clubes alemães". **Só a tela de encerrar (flag 0) recusa**: na conferência do empréstimo (0x8d0040) e no pedido da IA a Alemanha nunca recusa. A data não é lida | 0x5e0210..0x5e02bf; flag 0 em 0x8fbce2 |
| (novo) Irlanda 0x63c4e0 `record_loan` | **[provado]**: o clube da oferta (id em +8 na tabela [0xae23b4], 0x245 bytes) com nação de id [0x9cf2dc] (Irlanda): byte 0 da linha de 16 bytes `[[0xde1f54]+0x91c] + id·16` + 1. O mesmo código da inglesa 0x579fd0 com a chave irlandesa; é a conta que o `can_loan_player` irlandês (0x63c3d0) limita a 5, zerada em 20 de junho pela 0x57a020 (o `daily_update` irlandês) | 0x63c4e0..0x63c521 |

## 5. §7 (o defeito do empréstimo no fim do contrato)

| Afirmação do mapa | Texto novo | Prova |
|---|---|---|
| "Com dono francês/italiano/irlandês/finlandês fora da janela, 'This loan deal cannot go ahead'" | **corrigido [provado]**: o finlandês não é por janela, é o outono (18/8–15/11, §4); o alemão nunca recusa ali (flag 1). E, para quem pega emprestado sob o período galês (11 nações, França e Austrália inclusive), o início vem igual ao fim e o dono só é consultado se a 0x8d0590 mexer no fim pelo contrato | §2, §4 |

## 6. §11 (pontos de extensão)

Acrescentar à linha "tabelas das regras (+0x20, +0x24, +0x28)": a tabela completa do §0 (as 29, com a nação). Trocar
o ponteiro do slot numa **tabela** muda só aquela nação (trocar o +0x28 da tabela da França muda só a França); desviar
a rotina compartilhada (0x90f1a0) muda as 11. A conta irlandesa da temporada fica na mesma tabela da inglesa
(`[[0xde1f54]+0x91c] + id·16`).

## 7. Defeitos achados

1. **Período russo (Brasil, Polônia, Rússia)** — com o início no ano seguinte e a temporada acabando em janeiro ou
   fevereiro, o fim cai antes do início (ou nele). A cópia norte-irlandesa da mesma rotina corrige com o "início vira o
   fim"; a russa não. Latente: depende do mês do fim da temporada dessas nações (não conferido). **[provado no código]**
2. **Alemanha sem clube** — o jogador sem `club_job` (ou com clube sem nação) cai na recusa alemã com o texto "recalled".
   Na prática a 0x8d0530 não chama a regra nesse caso (exige o dono na mesma nação). **[provado no código]**
3. Não é defeito, mas é surpresa: o período galês dá início = fim para 11 nações, então para França e Austrália (e
   as outras) o `can_loan_player` aceita sem a regra do dono. **[provado]**
