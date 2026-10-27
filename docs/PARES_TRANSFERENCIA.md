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
| `club.dat[A]`, `[B]` | i16 em +128 | ±1 (A −1, B +1; varia com a reputação do jogador: ver 2ª rodada) | — | (e) +2; (c) nada | — |
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

## Segunda rodada: vaga ocupada, jogador-técnico, clube B e Bosman

26/10/2026, mesmo método, mesma base e mesma cópia do jogo; o "antes" continua `antes1.sav` (uma 4ª gravação sem
operação com a DLL recompilada, `antes4.sav`, saiu **idêntica**, MD5 `35448cd3…`) **[observado]**. O `FeriasTesteSave`
foi conferido antes e depois de cada carga e no fim: MD5 `33fda465…` intocado. Candidatos escolhidos pelo
`ferramentas\analise2.py`; contagens C1–C8 em `coerencias2.txt`; diferenças completas nos `*.diff.txt`.

A DLL de diagnóstico ganhou quatro operações (o `S` e o `D` viraram funções, sem mudar o que fazem): `R` (substituição:
`D` no ocupante e `S` no novo), `B` (pré-contrato Bosman), `A` (passar para o time B) e `K` (contrato aceito com cargo
novo; não precisou ser usado, o `C` bastou).

### Os pares

| Caso | Arquivo "depois" | Bytes | MD5 | Itens mudados | Operação |
|---|---|---|---|---|---|
| (f1) auxiliar com a vaga ocupada | `f1_vaga_ocupada.sav` | 137.618.621 | `db35a7f2da67e97613ad6244ea5e180c` | 2 | auxiliar 487, sem clube, para o 4504, que já tem o auxiliar 24253 |
| (f2) substituição do auxiliar | `f2_substituicao.sav` | 137.618.924 | `95c7743e60b3317a2ebd87e0a0dc595f` | 10 | dispensa do 24253 e contratação do 487 no mesmo dia |
| (g1) jogador vira jogador-treinador | `g1_jogador_treinador.sav` | 137.618.532 | `354e33d9df980aca6363082fe4cb42ca` | 8 | jogador 3304 do 4504 renova com o cargo 15 |
| (g2) jogador contratado como auxiliar | `g2_jogador_auxiliar.sav` | 137.619.014 | `66e392706c099f700e21dd9b87c23eaa` | 12 | jogador 25549, sem clube, contratado como auxiliar (6) pelo AFC Bournemouth (983, de IA) |
| (h1) do time principal para o B | `h1_para_o_b.sav` | 137.618.548 | `45317be87f6e6461fbd3d5f145639361` | 2 | jogador 40851 do Real Madrid (4834) para o Real Madrid B (4835) |
| (h2) compra pelo time B | `h2_compra_pelo_b.sav` | 137.619.864 | `6d94b080f4121171f1a582af26f570e3` | 13 | jogador 1366, sem clube, contratado pelo 4835 sem valor |
| (h3) venda de quem está no B | `h3_venda_do_b.sav` | 137.618.995 | `4e25861b219e3d3a20d3774c3f8c260d` | 15 | jogador 40740 (contrato com o 4834, no elenco do 4835) para o 4504, valor 95.125 |
| (i1) pré-contrato Bosman | `i1_precontrato.sav` | 137.618.740 | `ecbb07f7f6ff4a66b4fe17b9cb4a70a6` | 3 | jogador 26657 do Colchester (1451), contrato até 272/2001, acerta ir para o 4504 no dia 273/2001 |
| (i2) saída Bosman concluída | `i2_bosman_saida.sav` | 137.619.038 | `e200bdd992e12f949656e97ab66ae430` | 15 | o mesmo 26657 sai para o 4504, valor 0, ainda com contrato |

### As rotinas e os argumentos

| Caso | Chamadas |
|---|---|
| (f1) | O `S` da 1ª rodada: 0x8bf060 (thiscall `TM`; pessoa 487, clube 4504, `&hoje`, 0, −1, 0) com o cargo 11 fixo trocado por **6**, e 0x8cbbe0. |
| (f2) | `R`: os passos do *Terminate Contract* do caso (b) no ocupante 24253 (0x4d0b50, 0x53c190 (4504, 24253, 0, 1, 0), 0x8c1af0 (24253, 4504, 0, 0, 1), treino 0x8b2880 (4504, 0)) e depois o mesmo de (f1). |
| (g1) | `C`: `STAFF_CONTRACT` 0x85e410 + `set_to_desired_contract` 0x85fc50 (pessoa 3304, clube 4504, −1, 0, cargo **15**, 0, 0) + `add_contract_offer` 0x4cc490 (thiscall `CM`; oferta, −1, −1, 15, 0, 0, 0) + `resolve_contract_offers` 0x4ce4e0 (thiscall `CM`; pessoa). É a renovação da tela *Offer Contract* com o cargo "Player/Coach"; o jogador **aceitou**, e o `sign_contract` 0x4cf720 seguiu o ramo de troca de cargo (vaga conferida pela 0x53b9a0, tira do clube pela 0x53c190 e põe de novo pela 0x53bd10 com o cargo novo) **[observado: desmontagem e save]**. |
| (g2) | O `S` com cargo **6** num clube de IA. Quem transforma em 13 é a `database_add_staff_to_club` 0x53bd10: clube não humano, pessoa com ficha de jogador (+0x61) e a ficha +5 (i16) acima de 0x55 → o cargo 6 vira 13 (e o 8 vira 15), com vaga de comissão **e** vaga no elenco **[observado na desmontagem]**. |
| (h1) | `A`: `CONTRACT_MANAGER::move_to_reserves` 0x4cc0f0 (thiscall `CM`; pessoa 40851, clube 4834) → `toggle_squad_placement` 0x4cbd20. É a ação do elenco que passa o jogador para o time B ligado. |
| (h2), (h3), (i2) | `T`: 0x8bf060 (pessoa, clube comprador, `&hoje`, valor, −1, 0) + 0x8cbbe0, como (a)/(e): valor 0 em (h2) e (i2), −1 (o valor do jogador) em (h3). |
| (i1) | `B`: 0x8bf060 (thiscall `TM`; pessoa 26657, clube 4504, `&(fim do contrato + 1 dia)` = 273/2001, valor **0**, −1, 0), **sem** 0x8cbbe0 (a data ainda não chegou). É a data que o jogo usa: `transfer_manager_get_offer_completion_date` 0x8cee70 dá, para o tipo 3, o fim do contrato + 1 dia, e o `confirm_offer_by_club` 0x8c3310 marca a transferência futura (`add_future_transfer` 0x8cb9a0) para esse dia **[observado na desmontagem]**. Com valor 0, contrato atual e nação do clube com regra Bosman (`nation_allows_free_transfers` 0x544670), a 0x8bf060 marca o contrato +0x49 = 1. Não é chamada a notícia de acordo (`post_transfer_agreed_news` 0x8e05d0) que a confirmação da tela posta. |

### O que mudou, item a item

**(f1) vaga ocupada: o jogo não substitui.** Só `news.dat` (1 notícia, tipo 0xfb0, não a 0xfa8 das transferências
concluídas) e `transfer.dat` (+73) mudaram; `club.dat`, `staff.dat`, `contract.dat` e `general.dat` **idênticos**: o
487 continua sem clube e o 24253 continua auxiliar **[observado]**. Pela desmontagem, a `process_due_future_transfers`
chama a `check_offer_restrictions` 0x8cfc90 → `transfer_manager_check_squad_limits` 0x8cd450, que recusa quando o
cargo da oferta é 6/13 e o clube (ou o B ligado) já tem auxiliar (motivo 0x22/0x23: "too many staff employed"); a
oferta é desfeita (`collapse_offer` 0x8c44e0) **[inferência sobre a desmontagem]**. O mesmo vale para treinadores (5),
olheiros (7) e fisios (3) cheios; e a `database_add_staff_to_club` devolve 0 para vaga única ocupada (presidente,
técnico, auxiliar) sem tocar no ocupante. **Não há caminho do jogo em que o novo empurre o antigo:** a substituição é
dispensar e depois contratar, que é o (f2).

**(f2) substituição (dispensa + contratação):**

| Arquivo | Campo | Antes → depois |
|---|---|---|
| `club.dat[4504]` | auxiliar (+211) | 24253 → 487 (a mesma vaga) |
| `staff.dat[24253]` (o ocupante) | +57 | 4504 → −1; **+61 (cargo 6), +62, +70, +78, +82 não mudam** |
| `staff.dat[487]` | +57; +62; +70 (+74 bissexto) | −1 → 4504; → 205/2001; → 204/2005 (+78 e +82 não mudam) |
| `general.dat` | 24253 +0; 487 +0, +8 | 18321 → −1; −1 → **18321**, +8 +1 |
| `contract.dat` | registro 18321 | **reaproveitado**: era o do 24253, vira o do 487 (clube 4504, salário 450, 205/2001 → 204/2005, tipo 0x2); `n` não muda (41.974) |
| `finance.dat[4504]` | +12, +20 | folha salarial |
| `transfer.dat` | +151 | = 37 (registro de transferência passada do dispensado) + 114 (oferta do contratado) |
| `training.dat` | 7 bytes | o auxiliar novo no treino do clube humano |
| `news.dat`, `staff records.dat`, `staff history.tmp`, cauda do `contract.dat` (+111) | | como nos casos da 1ª rodada |

O registro de contrato que a dispensa marca livre (+0 = −1) é o **primeiro livre** que a contratação seguinte usa
(`sign_contract`): confirma a regra 3 da coerência na prática **[observado]**.

**(g1) jogador vira jogador-treinador (mesmo clube):**

| Arquivo | Campo | Antes → depois |
|---|---|---|
| `club.dat[4504]` | elenco | **sai da vaga 16 e entra na vaga 1** (a primeira livre): o jogo tira e põe de novo; o elenco não é compactado e a ordem muda |
| | treinadores[2] (+423) | −1 → 3304 (a primeira livre) |
| `staff.dat[3304]` | +61 cargo | 11 → **15**; +62 (entrada) **não muda** (mesmo clube); +70 → 167/2007 (fim novo); +78 salário 650 → 1.094; +82 valor recalculado; **+105 (i32) −1 → 5533** (não decodificado) |
| `contract.dat` | registro 2496 | o mesmo, reescrito: +8 5.758, salário 1.094, início 205/2001, fim 167/2007, +79 0x4a → 0x0a; tamanho e `n` iguais, sem oferta pendente na cauda |
| `general.dat` | — | não muda |
| `player.dat` | +69 | 10 → 20 |
| `finance.dat[4504]` | +0 saldo, +12, +20, +156/+316 | luvas (= contrato +8) e folha |
| `retire.dat` | −16 bytes | sai um registro (o jogador tinha aposentadoria anunciada) **[inferência]** |
| `training.dat`, `news.dat` | | o treinador novo; 1 notícia |

**(g2) jogador sem clube contratado como auxiliar por clube de IA → jogador-auxiliar (13):**

| Arquivo | Campo | Antes → depois |
|---|---|---|
| `club.dat[983]` | auxiliar (+211); elenco[26] (+319) | −1 → 25549 nas **duas** vagas (a do elenco é a primeira livre) |
| | i16 +128 | 2.999 → 3.051 |
| `staff.dat[25549]` | +57; +61; +62; +70; +78; +82 | −1 → 983; 11 → **13**; → 205/2001; → 204/2005; 10.000 → 684 (salário do contrato); 0 → 105.815 |
| `contract.dat` | | **novo no fim** (41974; `n` +1), clube 983, salário 684, multa 250.000, tipo 0x2 |
| `general.dat` | +0, +8 | −1 → 41974; +1 |
| outros | | `player.dat`, `player stats`, `staff records`, `finance.dat[983]`, `transfer.dat` +114, `man_conf.dat`, históricos, cauda +111 |

**(h1) do time principal para o B (`move_to_reserves`): o mínimo de todos os pares.** Só 2 itens:

| Arquivo | Campo | Antes → depois |
|---|---|---|
| `club.dat[4834]` | elenco[5] (+235) | 40851 → −1 (sem compactar) |
| `club.dat[4835]` | elenco[8] (+247) | −1 → 40851 (a primeira livre) |
| `staff.dat[40851]` | +57 | 4834 → **4835** |

O **contrato não muda** (continua com o clube 4834, mesmo registro, mesmo tipo 0x2, sem bit 0x40), nem o
`general.dat`, nem as datas e o salário da pessoa **[observado]**. É exatamente o padrão dos 186 casos de "contrato com
o principal, pessoa no B" do save (C5 186 → 187).

**(h2) compra feita pelo time B:** núcleo igual ao de (e) (jogador livre): `club.dat[4835]` elenco[8] −1 → 1366;
+57 −1 → 4835; +62/+70/+78/+82; contrato **novo no fim** (41974) **com o clube 4835** (o B, não o principal), salário
2.802, +8 3.434; `general +0` −1 → 41974. Mas o dinheiro sai do **principal**: `finance.dat[4834]` +0 saldo −3.434 e
+156/+316 luvas (o 4835 não tem mudança em `finance.dat`) **[observado]**. `club records.dat` cresce 907 bytes.

**(h3) venda de quem está no B (contrato com o principal):**

| Arquivo | Campo | Antes → depois |
|---|---|---|
| `club.dat[4835]` | elenco[3] | 40740 → −1 (sai do **B**, onde está) |
| `club.dat[4504]` | elenco[1] | −1 → 40740 |
| `staff.dat[40740]` | +57, +62, +70, +78, +82 | 4835 → 4504; → 205/2001; → 204/2006; 0 → 1.216; recalculado |
| `contract.dat` | registro 30537 | o mesmo, reescrito: clube **4834 → 4504**, salário 1.216, início 205/2001, fim 204/2006, +79 0x6a → 0x20 |
| `general.dat` | +34 (clube anterior) | −1 → **4834** (o principal, o do contrato), não o 4835 |
| `finance.dat` | 4834 (principal) recebe; 4504 paga | 4834: +0 saldo +95.125, +120/+280 = 95.125; 4504: +0, +12, +20, +112/+272 = 95.125, +156/+316 |

C5 186 → 185 (some um caso de "contrato ≠ +57"). Quem recebe o valor e fica como "clube anterior" é o dono do contrato.

**(i1) pré-contrato Bosman (o acordo, antes da data):** o jogador **não sai** do clube.

| Arquivo | Campo | Antes → depois |
|---|---|---|
| `contract.dat` | registro 19956, +73 (+0x49, "sai pela Bosman") | 0 → **1**; o resto do registro igual (fim 272/2001, clube 1451) |
| | cauda | +111 (a oferta de contrato com o 4504, pendente até o dia) |
| `general.dat` | +8 | +1 |
| `transfer.dat` | +81 | a oferta (tipo 3, "no fim do contrato") e a transferência futura para 273/2001 |

Nada em `club.dat`, `staff.dat`, `finance.dat` **[observado]**. O byte +73 do contrato é o `staff_contract_is_leaving_on_bosman`
(0x8857e0 lê +0x49) e o "Leaving the club under the Bosman ruling on <data>" do perfil.

**(i2) saída Bosman concluída (contrato ainda em vigor, valor 0):** o mesmo núcleo de uma compra com contrato (a):
`club.dat[1451]` elenco[6] → −1, `club.dat[4504]` elenco[1] → 26657, +128 −5/+5; +57 1451 → 4504; +62 → 205/2001;
+70 → 204/2005 (e o bissexto +66 da entrada); +78 → 1.016; o **mesmo registro** 19956 reescrito (clube 4504, salário 1.016,
+8 1.565, início 205/2001, fim 204/2005, **+73 volta a 0**); `general +34` = 1451, +38 = 1; `tactics.dat` tira o
jogador da tática do 1451. **Sem dinheiro para o 1451** (só o 4504 paga as luvas, `finance.dat[4504]` +0/+156/+316 e a
folha) **[observado]**. No jogo a conclusão acontece no dia 273/2001 (contrato já vencido); aqui foi no próprio dia
205, com o contrato ainda valendo: a diferença é só a data.

### Coerência obrigatória (acréscimos desta rodada) [inferência sobre o observado]

7. **Vaga única ocupada:** o editor **não** deve empurrar o ocupante. O jogo recusa; a substituição é dispensa (caso b)
   + contratação. O ocupante dispensado fica com +57 = −1, `general +0 = −1`, registro de contrato livre, e **mantém**
   +61 (cargo), +62, +70, +78, +82 (confirma TRANSFERENCIA §3b, item 7; o CMRTE zerar o cargo é erro).
8. **Jogador-técnico/auxiliar/treinador (12/13/15):** duas vagas no **mesmo** clube (a de comissão e a do elenco).
   Na troca de cargo no mesmo clube, o jogo tira a pessoa do elenco e a põe de novo na **primeira vaga livre** (a
   posição muda); +62 não muda; o contrato é o mesmo registro reescrito.
9. **Clube B:** passar do principal para o B (e voltar) mexe **só** nas duas vagas de elenco e no +57; o contrato
   continua com o principal. Contratação feita pelo B: contrato com o **B** (+4 = B), dinheiro do principal. Venda de
   quem está no B: sai da vaga do **B**, o contrato (do principal) é reescrito para o comprador, o principal recebe e
   fica como clube anterior (`general +34`).
10. **Bosman:** o acordo grava só o contrato +73 = 1 (e as sobras de oferta); a mudança no dia é igual a uma
    transferência com contrato e valor 0 (mesmo registro reescrito, +73 volta a 0, nenhum dinheiro ao clube antigo).

### Cruzamento com as C1–C9 (2ª rodada)

`coerencias2.txt` **[observado]**: C1, C2, C3, C6, C7 e C8 **iguais ao "antes"** nos nove pares (o jogador-auxiliar
de (g2) e o jogador-treinador de (g1) estão em duas vagas do **mesmo** clube, que a C1 aceita). C4: `n` +1 em (g2) e
(h2) (contrato novo no fim); (f2) reaproveita o registro livre, então 0 vagas livres depois. C5: (h1) +1 (186 → 187, o
novo caso "contrato no principal, +57 no B") e (h3) −1 (185) — os dois **conferem** com a exceção do clube B do §2.1.
C9: o jogo muda de 2 a 15 itens; (h1) é o único par cujo "depois" é praticamente o núcleo.

Respostas ao §7 do TRANSFERENCIA **[observado]**: (4) substituído: +61 **fica**, +78/+82 ficam. (5) jogador-técnico:
vaga de comissão + vaga no elenco do mesmo clube; virar jogador-técnico no mesmo clube reposiciona o elenco. (6) clube B:
ver coerência 9. **Diverge do §1.2/§2.1** em um ponto: a contratação **pelo** B cria contrato com o próprio B (+4 = B),
então nem todo jogador do B tem contrato com o principal. E a nota da 1ª rodada sobre o club +128 (i16) "±1" estava
estreita: ele varia com a reputação do jogador (a 0x8c9720 soma a reputação/100 no clube novo e tira do antigo;
(i2) ±5, (g2) +52, (h2) +20).

## Limites

- Os casos (a), (b2), (c) e (e) usam o caminho das transferências **combinadas de antemão** (0x8bf060 + 0x8cbbe0), não a
  negociação de vários dias da tela (oferta → resposta do clube → contrato → confirmação); o fim é o mesmo
  (`finalise_transfer`), mas datas de acordo, cláusulas e a oferta pendente podem diferir de uma compra feita na
  tela **[inferência]**. O empréstimo (d) é o do arquivo de configuração (salário 0 %).
- O caso (c) troca, só durante a chamada, o cargo fixo da 0x8bf060 (11 → 9) em três imediatos do código.
- O "valor" de (a) e (b2) é o valor do jogador (+0x52), sem parcelas nem cláusulas.
- 2ª rodada: (g2) e (h2)/(h3)/(i2) usam o mesmo caminho combinado de antemão; (f1)/(f2) trocam o cargo fixo para 6;
  (i1) não posta a notícia de acordo da tela; (i2) conclui no dia do acordo, não no dia seguinte ao fim do contrato.
  (g1) é o caminho da tela (oferta + decisão do jogador), que aceitou.
- Não cobertos: elenco a 50; transferência de quem já é jogador-técnico (12/13/15) para outro clube; jogador-técnico
  (12) por nomeação de técnico (`MANAGER_MANAGER`); clube B com o principal **humano**; reservas de clubes ingleses (não
  são clubes à parte: só o bit 0x40 do contrato, `toggle_squad_placement` caso 1/2); a chegada real do dia 273/2001.
- `transfer.dat`, `news.dat`, `staff records.dat` e os históricos não foram decodificados (só tamanho e posição).
