# Pares de saves de transferência feitos pelo próprio jogo

26/10/2026. Pedido do Lucas (via orquestrador), para o CMRTE implementar a transferência no save: pares
"antes/depois" com **uma** operação entre eles, feita pelas rotinas do próprio jogo, sem tela, no laboratório.
Cruzamento com `D:\Sistemas\CM3SaveEditor\docs\TRANSFERENCIA.md` (só leitura; outro repositório).

Convenção: **[observado]** = lido no save gravado, no `ddraw.log` ou na desmontagem; **[inferência]** = conclusão
a partir do observado.

## Método

- **Base:** cópia de `jogo\FeriasTesteSave.sav` (MD5 `33fda465193b5e8e6d7ace93ecd44559`, conferido antes e depois
  de cada carga e no fim: intocado) **[observado]**. Data do jogo 205/2001 (24/07/2001), técnico humano = pessoa
  54147, clube **4504** (Inglaterra, nação 61). Banco mínimo: 54.148 pessoas, 6.603 clubes, 41.974 contratos.
- **Jogo:** cópia própria do `jogo_testes` em `%TEMP%\cm0102_transf\jogo` (robocopy feito quando não havia jogo
  aberto no `jogo_testes`; sem saves, `.tmp` e despejos), só com a DLL de diagnóstico, `cm0102-1080.exe -windowed
  -seed 12345 -load <cópia>`, `CM_FERIAS=1`, `CM_JANELA=960x540`, `CM_OCULTO=1`, **modo original**
  (`CM_ORIGINAL=1`), sem cliques, dentro da trava do laboratório (bancada do `simular.py` apontada para a cópia:
  `game.cfg`, `*.tmp` e `hall_of_fame.bin` devolvidos). Tipo 3 (bit 0x8000 do `game.cfg`, já ligado na cópia).
- **DLL de diagnóstico:** `copias\transf\rust` (o `rust\` do HEAD + `src\transferir.rs`; fora do git, nada em
  `rust\src`). Na saída da 0x8387e0 (depois do `-load` e das férias do `CM_FERIAS`), o gatilho
  `CM_TRANSFERIR="X SSSSSSS CCCCC NNNNNNNNN"` (largura fixa: o ambiente do mesmo tamanho deixa a heap igual) executa a
  operação X com a pessoa S, o clube C e o número N, grava pelo `save_game_data` 0x82e430 (`par.sav`, id 0x16) e
  encerra o processo. `X = -` não faz nada: é o "antes" (mesmo dia, mesma gravação).
- **Determinismo:** três gravações "antes" (`antes1`, `antes2` e `antes3`, esta com a DLL já recompilada) saíram
  **idênticas** (MD5 `35448cd3…`) **[observado]**: nos pares, toda diferença é da operação; não precisou de máscara.
  O "antes" **não** é o `FeriasTesteSave`: só carregar e gravar já muda 40 itens (`general.dat`, `tactics.dat` +2.320,
  `fixtures_2001.tmp`, competições…) **[observado]**. Compare sempre com `antes1.sav`.
- **Ferramentas** (em `pares_transferencia\ferramentas`, fora do git): `pares.py` (roda os casos), `diff.py`
  (diferença item a item, decodificando `staff`, `club`, `contract`, `general`, `finance` e `player`), `verif.py`
  (contagens C1–C8), `analise.py` (escolha dos candidatos), `copiar.ps1` (a cópia do jogo). As diferenças completas
  estão nos `*.diff.txt`, e as contagens em `coerencias.txt`.

## Os pares

Pasta `D:\Sistemas\CM0102\pares_transferencia\` (no `.gitignore`). O "antes" de todos é `antes1.sav`.

| Caso | Arquivo "depois" | Bytes | MD5 | Operação |
|---|---|---|---|---|
| antes | `antes1.sav` (= `antes2`, `antes3`) | 137.618.548 | `35448cd3a6e8c5dcef97920e09b9581d` | nenhuma |
| (a) compra com valor | `a_compra.sav` | 137.618.995 | `e1d29378733c7b39252309e3d7e151cc` | jogador 28775 do Oldham (4221) para o 4504, valor 55.056 |
| (b) dispensa (rescisão) | `b_dispensa.sav` | 137.618.585 | `07f48166e770216d03cdd7666e89139b` | jogador 17198 do 4504 dispensado |
| (b2) venda com valor | `b2_venda.sav` | 137.619.046 | `ab476c2a70d6b9f47225ca751d48d5d3` | jogador 29006 do 4504 para o Liverpool (3439), valor 21.016 |
| (c) comissão técnica | `c_olheiro.sav` | 137.618.967 | `446c1853fb3fb0c8e2e14e5f386014ff` | olheiro 15, sem clube, contratado pelo 4504 |
| (d) empréstimo | `d_emprestimo.sav` | 137.618.722 | `4def2083f12d739b221e022217dd9129` | jogador 29141 do Luton (3529) emprestado ao 4504 por 150 dias |
| (e) jogador livre | `e_livre.sav` | 137.619.014 | `a8b72dd38040fd146b17b86e2ea5968d` | jogador 1, sem clube, contratado pelo 4504 sem valor |

Antes de dar certo, a venda do 29006 (e a do 24372) para o Oldham **desmoronou** (`check_offer_restrictions`
recusou, provavelmente pelo orçamento do comprador): o save saiu sem a transferência e foi descartado.

## As rotinas e os argumentos

Ponteiros: `TM = [0xde1f54]` (TRANSFER_MANAGER), `CM = [0xae1050]` (CONTRACT_MANAGER), pessoa = `[0xae23bc] + 0x6e·id`,
clube = `[0xae23b4] + 0x245·id`, hoje = `0xae2c90` (8 bytes). Nomes do `referencias\cm0102-function-map`;
convenções conferidas na desmontagem dos chamadores **[observado]**.

| Caso | Chamadas (nesta ordem) |
|---|---|
| (a), (b2), (e) | `TRANSFER_MANAGER::add_config_future_transfer` 0x8bf060 (thiscall `TM`; pessoa, clube comprador, `&hoje`, valor, −1, 0) com valor −1 = o valor do jogador (+0x52) nos casos (a) e (b2), e 0 no caso (e); em seguida `process_due_future_transfers` 0x8cbbe0 (thiscall `TM`), que confere permissões e restrições e chama `post_transfer_completed_news` 0x8dbef0 e **`finalise_transfer` 0x8c9390** (→ `move_player_to_club` 0x8c9720 → `CONTRACT_MANAGER::sign_contract` 0x4cf720). É o caminho das transferências combinadas de antemão (chamado pela 0x7a4cfc, as do arquivo de configuração de jogadores); termina na mesma `finalise_transfer` que conclui uma oferta aceita na tela. |
| (b) | Os passos do "Sim" do *Terminate Contract* (`contract_screens_handle_terminate_contract_screen` 0x4e5610): `CONTRACT_MANAGER::remove_contract` 0x4d0b50 (thiscall `CM`; pessoa); `database_remove_staff_from_club` 0x53c190 (cdecl; clube, pessoa, 0, 1, 0); `TRANSFER_MANAGER::free_transfer_staff` 0x8c1af0 (thiscall `TM`; pessoa, clube, 0, 0, 1 = só registro); clube humano: treino `0x8b2880` (thiscall `[0xde1f44]`; clube, 0) (e, se fosse olheiro, `0x7f7270` com `[0xb74448]`). |
| (c) | O mesmo 0x8bf060 + 0x8cbbe0 de (a), valor 0, com o **cargo 11 fixo no código trocado por 9** durante a chamada (os três imediatos em 0x8bf1f3, 0x8bf202 e 0x8bf2fc: o cargo da oferta (+0x11), o de `set_to_desired_contract` e o de `add_contract_offer`), e devolvido logo depois. Motivo: a contratação de comissão pelo humano (0x4e4ea0) monta uma oferta de transferência (a pessoa de fora do clube sempre passa por uma), e a `resolve_contract_offers` 0x4ce4e0 **apaga** a oferta de contrato sem oferta de transferência (+0x59 = −1) de clube que não é o do contrato atual: a 1ª tentativa (`set_to_desired_contract` + `add_contract_offer` + `resolve_contract_offers`) gravou um save idêntico ao "antes" **[observado]**. Toda entrada de pessoa nova num clube passa por `move_player_to_club` → `sign_contract` (os únicos chamadores de `sign_contract` são ela, `complete_accepted_contract_offer` (renovação) e um reparo da tela do perfil) **[observado: refs]**. |
| (d) | `TRANSFER_MANAGER::add_config_loan` 0x8bf400 (thiscall `TM`; pessoa, clube, `&hoje`, `&(hoje + 150 dias)`) → `sign_loan_contract` 0x4d01e0. É o empréstimo do arquivo de configuração: o salário pago é 0 % (`init_from_current_contract` com 0), não o de uma negociação. |

## O que mudou, item a item

Só os itens listados mudaram; os outros itens do contêiner (127 a 136 dos 142; mudaram 15 em (a), 6 em (b), 14 em (b2), 10 em (c), 8 em (d) e 13 em (e)) saíram **idênticos** byte a byte **[observado]**.
Nenhum item foi criado nem apagado (142 / 142). `general.dat` não muda no cabeçalho (data, contagens) em nenhum caso.

### Núcleo (o que todo caso de entrada ou saída de clube grava)

| Arquivo | Campo | (a) compra / (b2) venda | (b) dispensa | (c) olheiro / (e) livre | (d) empréstimo |
|---|---|---|---|---|---|
| `club.dat[A]` | vaga da pessoa no elenco | `p → −1` **sem compactar** (A 4221 vaga 34; 4504 vaga 5) | `p → −1` (vaga 34), sem compactar | — | **nada** (o dono 3529 mantém o jogador no elenco) |
| `club.dat[B]` | **primeira** vaga livre do grupo | elenco: 4504 vaga 1; Liverpool vaga 37 (as primeiras −1) | — | olheiros[3] (c), elenco[1] (e) | elenco[1] |
| `club.dat[A]`, `[B]` | i16 em +128 | ±1 (A −1, B +1) | — | (e) +2; (c) nada | — |
| `staff.dat[p]` | +57 clube | A → B | A → −1 | −1 → B | dono → **clube do empréstimo** |
| | +62..+69 entrada (data + i32 bissexto) | → 205/2001, bissexto 0 | não muda | → 205/2001 | não muda |
| | +70..+77 saída | → fim do contrato novo | não muda | → fim do contrato novo | não muda (fim do contrato do dono) |
| | +78 salário | → salário do contrato novo (250 → 959; 50 → 3.757) | não muda | 0 → salário do contrato (1.216; 384) | não muda |
| | +82 valor | recalculado (55.056 → 31.285) | não muda | (e) 0 → 28.059; (c) nada | não muda |
| | +94 (1 byte) | (a) 2 → 1; (b2) já era 1 | não muda | (e) 2 → 1 | não muda |
| | +61 cargo | não muda | **não muda** (11) | não muda (9 e 11) | não muda |
| `contract.dat` | registro | **o mesmo índice reescrito** (21368, 21527): +4 clube, +8, +12, bônus, +37 início = hoje, +45 fim, +53, +54..72, +79 | **+0 pessoa = −1** (o registro fica, vira vaga livre; `n` não muda; +4 continua o clube antigo) | **novo no fim** (índice 41974 = `n` antigo; `n` 41974 → 41975) | **novo no fim** (41974), tipo 0x46, salário 0, +28 = `01 01`; o do dono **não muda** (tipo 0x2) |
| `general.dat`, entrada de `p` (0xfa7 + 79·p) | +0 contrato | não muda (mesmo índice) | → −1 | −1 → 41974 | não muda |
| | +4 empréstimo | — | — | — | −1 → 41974 |
| | +8 (byte) ofertas pendentes | +1 (ver "sobras") | — | +1 | — |
| | +10 (byte) | 0 → 13 (a); 3 → 9 (b2); 0 → 9 (e) | — | — | — |
| | +34 i32 clube anterior | −1 → A | — | — | — |
| | +38 (byte) | 0 → 5 | — | — | — |

Datas: `i16 dia do ano, i16 ano, i32 bissexto` **[observado]**. Contratos novos: início = hoje (205/2001); fim = hoje
+ N·365 − 1 dia (204/2006, 204/2005); anos sorteados de 2 a 5 pela 0x8bf060 quando não informados.

### O que o jogo também grava (fora do núcleo)

| Item | (a) | (b) | (b2) | (c) | (d) | (e) |
|---|---|---|---|---|---|---|
| `finance.dat` (registros de 359 bytes, um por clube e seleção: 7.029) | A: +0 saldo +55.056, +120/+280 receita de transferências = 55.056; B: +0 saldo, +12 folha salarial, +20, +112/+272 gasto = 55.056, +156/+316 luvas (+3.126 = contrato +8) | B: +12 folha, +20 | os dois clubes, como (a) | B: +12, +20 | B: +12, +20 | B: +0 saldo −2.798 (= contrato +8), +12, +20, +156/+316 +2.798 |
| `transfer.dat` | +118 bytes; contagens em +0 e +4 +1 | **+37** (1 registro de transferência passada, `add_past_transfer`, `free_transfer_staff` com registro) | +122 | +114 | — | +114 |
| `contract.dat` cauda (depois dos registros) | +111 (1 oferta de contrato de 0x6f bytes) | — | +111 | +111 | — | +111 |
| `news.dat` (tamanho fixo 22.504) | 1 notícia (220 bytes) | — | 437 bytes | 1 notícia | — | 1 notícia |
| `staff records.dat`, `club records.dat` | sim | — | sim | `staff records` | — | `staff records` |
| `player.dat` (registro = staff +97, 70 bytes) | +69 | — | +9..+16, +69 | — | — | +11..+12, +69 |
| `player stats.dat` | sim | — | sim (bytes do jogador → 0) | — | sim (bytes do jogador → 0) | sim |
| `tactics.dat` | o jogador sai da tática de A (id → −1) | — | — | — | sai da tática do dono | — |
| `man_conf.dat` | sim | — | sim | — | — | sim |
| `scout_manager.dat` | — | — | — | 10 bytes (o olheiro novo) | — | — |
| `staff history.tmp` | +171 | — | +171 | +114 | — | +114 |
| `player stats history.tmp` | +47 | — | +94 | — | +94 | +47 |

**Sobras do caminho de configuração [inferência]:** nos casos que passam pela 0x8bf060, a oferta de contrato que ela
cria fica pendente (cauda do `contract.dat` +111 bytes, `general +8` +1) e a oferta de transferência fica no
`transfer.dat` marcada como concluída (estado 0x14, removida em 10 dias pela 0x8c6220). No fluxo normal (oferta aceita
na tela) a oferta de contrato é consumida quando o jogador aceita. Não é coerência: o editor não deve criá-las.

## Coerência obrigatória (o que o editor precisa gravar) [inferência sobre o observado]

1. **Clubes:** sair de A = a vaga vira −1, **sem compactar** a lista; entrar em B = a **primeira** vaga −1 do grupo do
   cargo. Empréstimo: o dono **mantém** a vaga.
2. **Pessoa:** +57 = clube novo (ou −1, ou o do empréstimo); na entrada, +62 = hoje, +70 = fim do contrato novo
   (+74 bissexto junto), +78 = salário do contrato; na dispensa e no empréstimo, só +57.
3. **Contrato:** com contrato atual, o **mesmo registro** é reescrito (o índice no `general.dat` não muda); sem
   contrato, o registro novo vai para a **primeira vaga livre (+0 = −1)** ou, sem vaga, para o **fim** (`n` + 1;
   `sign_contract` 0x4cf720 procura a vaga livre antes de crescer **[observado na desmontagem]**); empréstimo =
   registro novo, apontado por `general +4`. Dispensa = `+0 = −1` no registro e `general +0 = −1` (o registro **não
   some**, o tamanho do `contract.dat` não muda).
4. **Índice `general.dat`:** +0/+4 coerentes com 3. Os bytes +8, +10, +34, +38 mudam no jogo, mas fora do núcleo.
5. **Táticas:** o jogo tira o jogador da tática do clube antigo (`tactics.dat`, id → −1) **[observado em (a) e
   (d)]**; recomendável, para não deixar um jogador de fora na tática.
6. O resto (finanças, notícias, `transfer.dat`, históricos, `player.dat`, estatísticas, `man_conf.dat`, `+82`, `+94`,
   club +128) é o que o jogo **também** grava; decidir se o editor imita é do Lucas (TRANSFERENCIA §7 item 9).

## Cruzamento com as C1–C9 (TRANSFERENCIA.md §4)

Contagens do `verif.py` em `antes1` → cada "depois" (`coerencias.txt`) **[observado]**:

| Regra | Medida no "antes" | Resultado | Situação |
|---|---|---|---|
| C1 uma vaga por grupo, um clube | 54 pessoas em 2 clubes (exceções da base) | igual em todos; **(d) +1** (o emprestado nos dois elencos) | **confere** (exceção do empréstimo prevista no §2.1) |
| C2 +57 = c → vaga em c | 11 | igual em todos | **confere** |
| C3 +57 = −1 → sem vaga e sem contrato | 1 | igual (o dispensado sai da vaga e do índice) | **confere** |
| C4 índice ↔ contratos | 0 erros, 0 vagas livres | 0 erros em todos; **(b) cria 1 registro com +0 = −1 não apontado** | **diverge em parte:** "todo contrato é apontado" vale só para os registros ocupados; o jogo deixa registros livres (+0 = −1) e os reaproveita. O verificador tem de aceitar +0 = −1 |
| C5 contrato +4 = +57 | 186 (clube B, emprestados) | igual em todos | **confere**; no (b) o registro livre guarda o +4 antigo (não importa, +0 = −1) |
| C6 +70 = +45 do contrato | 0 | 0 em todos | **confere** (o jogo regrava +70 na entrada) |
| C7 escalação ⊂ elenco | 0 | 0 | **confere** (escalação vazia neste save) |
| C8 sem repetidos numa lista | 0 | 0 | **confere** |
| C9 diferença mínima | — | o jogo muda de 6 a 15 itens (tabela acima) | **diverge por definição**: o "depois" do jogo não é mínimo; a comparação do editor com o par deve ser só no núcleo (TRANSFERENCIA §6 passo 8) |

Respostas ao §7 do TRANSFERENCIA **[observado]**: (1) entra na 1ª vaga livre; não compacta. (2) com contrato: mesmo
registro reescrito, índice igual, tamanho igual; sem contrato: registro novo no fim (ou na 1ª vaga livre); dispensa:
registro marcado livre, não apagado. (3) +62 = hoje; +70 = +45 novo; +37 = hoje (nesta rota não há data de acordo
anterior); +45 = hoje + anos. (4) dispensado: +61, +78, +82 ficam. (6) emprestado: só +57, `general +4` e o registro
de empréstimo (tipo 0x46); o dono fica com a vaga. **Diverge do §1.1:** +78 (salário) **é** regravado com o salário
do contrato na entrada; +94 muda (2 → 1) na entrada do jogador; club +128 (i16) muda ±1. **Diverge do §1.3:** o tipo
0x42 (bit 0x40) aparece em contratos **novos** (c, e) e no da venda (b2), não só no do dono de um emprestado; e o
contrato do dono **não** ganhou o bit 0x40 no empréstimo (d).

## Limites

- Os casos (a), (b2), (c) e (e) usam o caminho das transferências **combinadas de antemão** (0x8bf060 + 0x8cbbe0), não a
  negociação de vários dias da tela (oferta → resposta do clube → contrato → confirmação); o fim é o mesmo
  (`finalise_transfer`), mas datas de acordo, cláusulas e a oferta pendente podem diferir de uma compra feita na
  tela **[inferência]**. O empréstimo (d) é o do arquivo de configuração (salário 0 %).
- O caso (c) troca, só durante a chamada, o cargo fixo da 0x8bf060 (11 → 9) em três imediatos do código.
- O "valor" de (a) e (b2) é o valor do jogador (+0x52), sem parcelas nem cláusulas.
- Não cobertos: auxiliar com a vaga ocupada (3b), jogador-técnico, clube B/reservas, elenco a 50, transferência
  Bosman de quem ainda tem contrato (a 0x8bf060 com valor 0 e contrato marca o contrato antigo para acabar).
- `transfer.dat`, `news.dat`, `staff records.dat` e os históricos não foram decodificados (só tamanho e posição).
