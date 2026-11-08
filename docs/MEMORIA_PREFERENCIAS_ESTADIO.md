# Preferências e estádio na memória (para o editor em tempo real)

Levantamento para o editor gravar, com o jogo rodando, as **preferências das pessoas** (clubes e pessoas favoritos e rejeitados) e os **campos de estádio** (cidade e atração, estádio reserva). Só leitura: desmontagem do `cm0102.exe` 3.9.68 (`resolve_database_indexes` 0x5315a0, `save_database` 0x52dad0, `database_trim_staff_tables` 0x524d70, `staff_interest_in_job_at_club` 0x844780 e `Shortlist_Manager::staff_interest_in_joining_club` 0x846920), as cópias Rust de `rust/src/motor.rs` (`resolver_indices_do_banco`, 233ª; `cortar_tabela_de_clubes`; as rotinas de preferência 0x547810..0x5480f0, 46ª, 72ª, 234ª, 235ª e a fila `dbp_*`/`dbq_*`), `docs/FORMATOS.md` e o inventário de criação do Bosman Edition (§2d). O jogo não foi aberto.

Cada afirmação leva **[observado]** (lido na desmontagem ou no código) ou **[inferência]** (dedução sem prova direta). Endereços em VA.

## Resumo

- **Na memória, todo campo de ligação é ponteiro; o id só existe no +0 de cada registro.** Preferências: os 6 clubes (+0x04..+0x18) apontam para a tabela de clubes, as 6 pessoas (+0x1c..+0x30) para a tabela de pessoas; o +0x00 é o id da própria linha [observado].
- **Vaga vazia na memória = 0 (NULL). No disco = −1.** Nunca gravar −1 na memória [observado].
- **Gravar ponteiro = `[tabela] + id × tamanho`**, com `0 ≤ id < [quantidade]`, conferindo que o `+0` do alvo é o próprio `id` (é o que o save grava de volta) [observado].
- **O save troca ponteiro por id lendo o +0 do alvo** (`[ptr]`), NULL vira −1, grava e depois chama a `resolve` de novo [observado]. Durante a gravação os campos ficam com **ids** na memória: o editor não deve ler nem gravar nesse intervalo [observado o vaivém; o risco é inferência].
- **Estádio:** `clube+0x69` (estádio) e `clube+0x6e` (estádio reserva) são ponteiros para a tabela de estádios; `estádio+0x38` é ponteiro para a cidade e a atração é `cidade+0x33` (byte). A atração é lida pela cadeia `clube+0x69 → +0x38 → +0x33` na vontade de uma pessoa de ir para o clube [observado].

## 1. As tabelas

| Tabela | Ponteiro | Quantidade | Bytes | Fonte |
|---|---|---|---|---|
| pessoas | `[0xae23bc]` | `[0xae2364]` | 110 (0x6e) | [observado] |
| preferências | `[0xae23c8]` | `[0xae2370]` | 52 (0x34) | [observado] |
| clubes (e seleções) | `[0xae23b4]` | `[0xae235c]` | 581 (0x245) | [observado] |
| estádios | `[0xae23b0]` | `[0xae2358]` | 78 (0x4e) | [observado] |
| cidades | `[0xae23ac]` | `[0xae2354]` | 56 (0x38) | [observado] |
| nações | `[0xae23a8]` | `[0xae2350]` | 290 (0x122) | [observado] |

**Invariante `+0 == linha`.** Depois dos cortes da criação, o `+0` de cada registro é o índice da linha: o corte de clubes (0x5267c0, cópia `cortar_tabela_de_clubes`) grava `+0 = linha` e refaz os ponteiros de clube das preferências (+4..+0x18) pelo novo índice; o corte de pessoas 0x524d70 copia pessoa, jogador, não jogador e preferências para a frente com `+0 = contador` (0x525ed9, 0x525f50) e refaz os ponteiros de pessoa das preferências (+0x1c..+0x30, 0x525d30..0x525dde) pela tabela de remapeamento [observado]. Por isso `ponteiro = [tabela] + id × tamanho` e `id = [ponteiro]` são a mesma relação.

**Endereço fixo na carreira.** As tabelas são cortadas com `realloc` no lugar (com `assert` de mesmo endereço) e trazem linhas de reserva (1325 pessoas "virtuais", cada uma já com a sua linha de preferências, 0x526554..0x526636) [observado]. Não achei `realloc` dessas tabelas durante a carreira [inferência: não varri todas as chamadas a 0x945667]. Mesmo assim, ler `[0xae23c8]` etc. a cada gravação, não guardar o endereço.

## 2. Preferências (`[0xae23c8]`, 52 bytes)

| Desl. | Tipo na memória | Campo | Na carga (0x5315a0) | Vazio |
|---|---|---|---|---|
| +0x00 | `i32` **id** | id da linha (= índice) | fica id | — |
| +0x04, +0x08, +0x0c | `Club*` | clubes favoritos 1..3 | id → `[0xae23b4] + id × 581` | 0 |
| +0x10, +0x14, +0x18 | `Club*` | clubes rejeitados 1..3 | idem | 0 |
| +0x1c, +0x20, +0x24 | `Staff*` | pessoas favoritas 1..3 | id → `[0xae23bc] + id × 110` | 0 |
| +0x28, +0x2c, +0x30 | `Staff*` | pessoas rejeitadas 1..3 | idem | 0 |

Fontes [observado]: `resolver_indices_do_banco` (as 12 com a regra "sem teto": negativo → NULL; **id ≥ quantidade vira ponteiro além da tabela**, sem caixa), `save_database` 0x52e1db..0x52e324 (as 12: `ptr ? [ptr] : −1`), as rotinas de consulta e de escrita (abaixo).

Quem aponta para a linha: `pessoa+0x65` (`Prefs*`). Na carga, id negativo → NULL; id ≥ `[0xae2370]` → registro no log, caixa de erro (linha 0x1907 do Database.cpp) e NULL [observado]. Pessoa sem preferências = `+0x65 == 0`; todas as rotinas testam isso e não fazem nada [observado].

### Como o jogo usa as vagas [observado, cópias Rust]

- **Consulta** (`gosta_de` 0x547810, `nao_gosta_de` 0x547840, `gosta_do_clube` 0x547d60, `nao_gosta_do_clube` 0x547d30, contagens 0x547c90/0x547ce0): compara **ponteiro** com ponteiro nas 3 vagas; vaga 0 nunca casa.
- **Acrescentar** (0x547a10 pessoa favorita, 0x547e70 clube rejeitado e irmãs): se já está na lista oposta, tira de lá e para; senão a primeira vaga 0; sem vaga, com `forcar`, uma vaga sorteada (0x90cfc0, 3). Grava o **ponteiro** recebido.
- **Tirar** (0x547f50, 0x547c10, 0x547c50...) e **esfriar** (0x547f90, diário): grava 0 na vaga.
- **Listar** (0x5480f0, 0x548090): copia os ponteiros não nulos na ordem.

Consequências para o editor: buracos são normais (vaga 1 vazia com a 2 cheia é estado que o jogo produz); repetição é tolerada (0x547e70 não confere repetição nos rejeitados); o jogo não exige simetria com o clube (`clube+0x9b..+0xaf`, favoritas e rejeitadas do clube, também ponteiros) [observado].

### Gravar com o jogo rodando

1. Pessoa `p` (índice), linha `L = [p_ptr + 0x65]`; se `L == 0`, a pessoa não tem preferências: **não criar** (não há linha livre; as linhas são uma por pessoa) [observado o 1:1 no corte; inferência sobre não haver sobra].
2. Clube de id `c`: conferir `0 ≤ c < [0xae235c]`, calcular `P = [0xae23b4] + c × 581` e conferir `[P] == c`. Pessoa de id `s`: `0 ≤ s < [0xae2364]`, `P = [0xae23bc] + s × 110`, `[P] == s`.
3. Gravar os 4 bytes de `P` (ou 0 para esvaziar) em `L + desl`. **Nunca o id, nunca −1.** Um id cru seria lido como ponteiro pelas consultas (não casa nunca) e, pior, pelo save (`[id]` = leitura em endereço baixo, falha de acesso) [observado: o save desreferencia todo valor não nulo].
4. Para não repetir o que o jogo evita, ao pôr um clube entre os favoritos tirá-lo dos rejeitados (e vice-versa), como 0x547870..0x547e70 fazem [observado nas rotinas; opcional para o jogo].
5. **Quando:** com o jogo parado esperando o usuário (tela aberta, sem "continuar" em andamento). Não durante a gravação do jogo: entre o início do `save_database` e a `resolve` final os campos estão com ids; uma gravação de ponteiro nesse intervalo seria tratada como id pela `resolve` (sem teto → ponteiro absurdo) [observado o vaivém; inferência sobre a janela]. A gravação de 4 bytes não é atômica entre processos se o campo cruza linha de cache (registros de 52 e 110 bytes não são alinhados); o caminho mais seguro é gravar de dentro do processo, na linha de execução do jogo (ponto de extensão `rust/src/extension.rs`) [inferência].

### Como volta para o save [observado, 0x52dad0]

O `save_database` percorre a tabela e, campo a campo, grava `[ptr]` (o +0 do alvo) ou −1; escreve a tabela 22 com `gravar_preferencias` 0x54fd00 (0x530bab); depois chama `resolve_database_indexes` para voltar aos ponteiros. Ou seja: o que vai para o `.sav` é o id que estiver no +0 do registro apontado. Por isso basta apontar para o **início** de um registro válido; ponteiro para o meio de um registro grava 4 bytes de lixo como id e, na próxima carga, vira ponteiro fora da tabela (as 12 vagas não têm teto) [observado].

## 3. Estádio na memória

### Os campos de ligação

| Registro | Desl. | Na memória | Alvo | Regra na carga (0x5315a0) | No save |
|---|---|---|---|---|---|
| clube | +0x69 | `Stadium*` (estádio) | `[0xae23b0] + id × 78` | negativo → NULL; **id > `[0xae2358]`** → log e NULL (teto inclusivo: id == quantidade passa) | `[ptr]` / −1 |
| clube | +0x6d | `u8` | dono do estádio | não é ligação | cópia |
| clube | +0x6e | `Stadium*` (estádio reserva) | idem | como +0x69, sem log | `[ptr]` / −1 (0x52dc2e) |
| estádio | +0x00 | `i32` **id** | — | fica id | — |
| estádio | +0x38 | `City*` | `[0xae23ac] + id × 56` | negativo → NULL; id > `[0xae2354]` → NULL | `[ptr]` / −1 (0x52e1a1) |
| estádio | +0x48 | `Stadium*` (estádio vizinho) | `[0xae23b0] + id × 78` | negativo → NULL; id > `[0xae2358]` → **zera a cidade (+0x38) e +0x48 fica com o id cru** (erro do original) | `[ptr]` / −1 (0x52e1ba) |
| cidade | +0x00 | `i32` **id** | — | fica id | — |
| cidade | +0x1f | `Nation*` | `[0xae23a8] + id × 290` | negativo → NULL | `[ptr]` / −1 |
| cidade | +0x33 | `i8` | **atração** | dado, não ligação | cópia |

Fontes: `resolver_indices_do_banco` e a desmontagem do `save_database` [observado]. Capacidade, sentados, teto, coberto e aquecimento (+0x3c..+0x4d) são dados: ver [OBRAS_DO_ESTADIO.md](OBRAS_DO_ESTADIO.md).

### De onde vem a atração da cidade

`staff_interest_in_job_at_club` 0x844780 (0x845452..0x845469, 0x845c5c) e `Shortlist_Manager::staff_interest_in_joining_club` 0x846920 (0x84797c..0x847993, 0x8489c2) fazem `estadio = [clube+0x69]`; se não nulo, `cidade = [estadio+0x38]`; se não nula, lêem o byte `cidade+0x33`; com 0 entra um ramo pela reputação do clube (`clube+0x80`) [observado]. Varredura do `.text` inteiro (carga de +0x38 seguida, em até 40 bytes, de leitura de byte +0x33 no mesmo registrador): só esses 4 pontos [observado]. Não achei leitura da atração pelo estádio reserva [observado na varredura].

Logo, para mudar a atração "do clube" o editor tem três caminhos: (a) mudar o byte `cidade+0x33` (vale para todos os estádios daquela cidade); (b) apontar `estádio+0x38` para outra cidade (vale para todos os clubes daquele estádio); (c) apontar `clube+0x69` para outro estádio. O (a) é o único que não troca ponteiro [observado].

### O estádio reserva (`clube+0x6e`)

É ponteiro (ou NULL) desde a carga e volta a id no save [observado]. Leitores fora da carga e do save: a varredura das leituras de dword em +0x6e achou candidatos em 0x41c317 (passa o ponteiro a 0x41cdd0 com "CM3_PICTURES": foto do estádio, se o registro for clube) e 0x49f8b1/0x49f90c, 0x6e62d8, 0x7e65c7, 0x7e69cd, sem confirmar que o registro é um clube [inferência]. A **capacidade** do estádio reserva, quando usada, viria de `[clube+0x6e]+0x3c` [inferência: não achei a leitura encadeada +0x6e → +0x3c/+0x40 na varredura de 40 bytes].

### Gravar com o jogo rodando

- **Atração:** gravar o byte `cidade+0x33` (`[0xae23ac] + id × 56 + 0x33`); não é ligação, nada a converter; o save copia [observado]. Faixa de valores do banco não conferida aqui.
- **Estádio do clube / reserva:** id `e` com `0 ≤ e < [0xae2358]` (**não** `≤`: o teto inclusivo da carga deixa passar `e == quantidade`, que aponta para depois da tabela), `P = [0xae23b0] + e × 78`, `[P] == e`; gravar `P` ou 0. Dois clubes podem dividir o mesmo estádio (o jogo não exige exclusividade) [observado no acesso por ponteiro].
- **Cidade do estádio:** id `k`, `0 ≤ k < [0xae2354]`, `P = [0xae23ac] + k × 56`, `[P] == k`; gravar `P` ou 0.
- **Estádio vizinho (+0x48):** só ponteiro válido ou 0. Um id cru ali (o caso do erro do original) faz o save desreferenciar o id como endereço [observado: o save lê `[ptr]` de todo não nulo; inferência sobre a falha].
- Mesmas regras de horário e de linha de execução da §2.

## 4. Prova sugerida no laboratório (não rodada)

Com uma cópia do save de teste, carregar pelo `simular.py` e, de dentro da DLL (sem cliques): (1) conferir em todas as linhas de preferências que cada vaga é 0 ou um ponteiro para o início de um registro da tabela certa com `[P] == (P − base) / tamanho`, e o mesmo para `clube+0x69/+0x6e` e `estádio+0x38/+0x48`; (2) gravar um clube favorito, um estádio reserva e uma atração pelas regras acima; (3) gravar o jogo, comparar os itens `staff.dat` (tabela 22), `club.dat`, `stadium.dat` e `city.dat` do `.qst` com o esperado (o id no lugar do ponteiro); (4) carregar de novo e repetir (1). A ferramenta próxima disso é `engenharia-reversa/prova_save.py`.
