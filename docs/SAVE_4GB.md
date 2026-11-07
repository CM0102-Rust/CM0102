# Saves acima de 4 GB (modelo, sem implementação)

Planejamento pedido pelo Lucas (25/10/2026): modelar as mudanças na DLL e no arquivo para o save passar de 4 GB **sem recompilar o jogo em 64 bits**. Nada aqui está implementado. A correção até 4 GB, já em uso, está em [SAVE_2GB.md](SAVE_2GB.md). As funções e o fluxo do save estão em [MAPA_SAVE.md](MAPA_SAVE.md).

Marcas: **[observado]** = visto na desmontagem do `cm0102.exe` 3.9.68, no código do projeto ou nos arquivos; **[inferência]** = conclusão ou estimativa ainda não provada.

Fontes: desmontagem (`desmonta.py`, `refs.py`, `textos.py`), mapa da comunidade (`referencias/cm0102-function-map`; o "size" de lá não é em bytes), `cm0102-base/src/conteiner.rs`, `save/src/lib.rs` e a leitura **só do índice** (cabeçalho e entradas, mais os 4 bytes da data do jogo no `general.dat` + 0xF68) de 8 saves do Lucas. Nada foi copiado, gravado nem executado.

## Resumo

1. **O primeiro limite de verdade não é o formato, é a memória** [inferência]. O exe não tem LARGEADDRESSAWARE [observado], então o processo tem 2 GB de endereços. Os `.dat` do save ficam inteiros na memória. Se a proporção medida se mantiver, eles ocupam ~1,3 GB quando o arquivo chega a 4 GB.
2. **O arquivo passa de 4 GB antes de qualquer item passar de 2 GB** [inferência]. O maior item que cresce (`player stats history.tmp`) é ~25% do save. Ele só chega a 2 GB com o arquivo entre ~6 e 8 GB.
3. Para o **arquivo** acima de 4 GB, com itens abaixo de 2 GB, basta mudar as 3 rotinas que já estão em Rust e portar mais 2 ou 3 do contêiner (fase A, ~1 leva). Os `.tmp` e o código dos subsistemas não mudam.
4. Para **itens** entre 2 e 4 GB é preciso portar o `FILE_LLIST` (si/code/file_llist.CPP), o `zread` e o `utils_copy_file` (fase B, ~2 levas). Item acima de 4 GB pede mudança nos `.dat` de 8 subsistemas: não recomendado.
5. **Ritmo** [inferência]: 45 a 100 MB por temporada sem compressão. O arquivo chega a 4 GB depois de 40 a 90 temporadas (o dobro com a compressão ligada). Nenhum save real medido aqui passa de 280 MB. Os de "2,5 GB" são enchimento.

## 1. Os limites, um a um

### (a) O formato do contêiner [observado]

| Trecho | Conteúdo | Limite |
|---|---|---|
| +0, 12 bytes | tipo (3 = sem compressão, 4 = RLE), `a4` (0x16 `.sav`, 0x17 `.qst`), quantidade de itens | os 3 são u32 |
| +12, 1000 × 0x10c | por item: posição u32 (+0), tamanho u32 (+4), nome de 260 bytes (+8) | **posição até 4 GB − 1**; tamanho até 4 GB − 1 (descomprimido no tipo 4) |
| total fixo | 0x416ec bytes de cabeçalho e índice, regravados inteiros pelo `zclose` (0x932b00) | no máximo 999 itens: o `zopen` recusa a partir de 0x3e7 |

- No tipo 4 a posição é no arquivo comprimido e o tamanho é o descomprimido. O limite de 4 GB vale para a posição comprimida. Compressão ligada = o mesmo limite com o save ~2× menor.
- **Folga no índice** [observado nos 8 saves]: o nome mais longo tem 55 caracteres, e os bytes +0x104..+0x10b de todas as entradas usadas são zero, assim como as entradas livres. Há espaço para uma parte alta da posição sem mudar o tamanho da entrada.
- Quantidade de itens: 125 a 250 nos saves medidos. Cresce ~1 por temporada (`fixtures_<ano>.tmp` fica um por ano: o Valencia tem 2025 a 2029). O limite de 999 só chegaria em centenas de temporadas [inferência].

### (b) As contas do contêiner e as APIs de arquivo

| Rotina | Endereço | Situação hoje | Acima de 4 GB no arquivo |
|---|---|---|---|
| `zopen` | 0x9326e0 | Rust (158ª): posição sem sinal, busca por `SetFilePointer` com a parte alta (`buscar_64`, `posicao_64`) [observado] | lê e grava `pos` como u32; **recusa** o item quando o fim do arquivo passa de u32 (linha "item alem de 4 GB recusado") [observado]. Vai ao fim com o `fseek(f, 0, SEEK_END)` da CRT (0x946a22) |
| `zseek` | 0x933080 | Rust | `alvo`, `pos` e `tam` em u32 (`wrapping_add`) [observado]: precisa de u64 |
| `ztell` | 0x933170 | Rust | `posicao_64(...) as u32 - pos` [observado]: precisa de u64 antes de subtrair |
| `zread` | 0x932c30 | original | compara a posição no item (+8) com `tamanho − n` com sinal (`jle`/`jg` em 0x932c78/0x932c80) [observado]: **item limitado a 2 GB**; não usa a posição no arquivo |
| `zwrite` | 0x932e10 | original | soma ao tamanho (u32), sem comparar [observado]: item até 4 GB |
| `zclose` | 0x932b00 | original | `fseek(f, 0, 0)` e regrava o bloco de 0x416ec da memória [observado]: sem limite |
| `zextract_matching_files` | 0x933320 | original | lê o próprio cabeçalho e índice, exige tipo 3/4 (0x933442/0x933447) e abre cada item pelo `zopen` [observado]: herda o `zopen` |
| `zadd_matching_files` | 0x933610 | original | `fread` do `.tmp` + `zwrite` em blocos de 0x30d40 [observado]: sem limite próprio |
| `zget_matching_size` | 0x933860 | original | soma os tamanhos em `int` (0x9339c9) [observado]: negativo a partir de 2 GB de `.tmp`, dá a volta em 4 GB. A conferência de espaço do `load_game_data` passa sempre ou falha por engano |
| `zget_archive_id` | 0x9331c0 | original | devolve o dword +4 do cabeçalho; `check_saved_game_file` (0x838e10) só lista o save com 0x16 [observado em MAPA_SAVE] |

**APIs da CRT** [observado]: o `fseek` (0x946a22) e o `ftell` (0x946aae) são `long` de 32 bits com sinal. O `_lseek` (0x94c552) chama `SetFilePointer(h, desloc, NULL, método)`, sem a parte alta. Portanto:
- `ftell` devolve −1 a partir de 2 GB;
- `fseek(f, 0, SEEK_END)` num arquivo acima de 4 GB pede ao Windows uma posição que não cabe em 32 bits sem o ponteiro alto. Segundo a documentação do `SetFilePointer`, a chamada falha [inferência: não testado]. O `zopen` em Rust usa esse caminho hoje e teria de ir ao fim por `SetFilePointer` com a parte alta (ou `SetFilePointerEx`);
- `fread`/`fwrite` andam pelo ponteiro do Windows (64 bits) e não limitam, desde que ninguém chame o `_lseek` da CRT no meio [inferência; o `buscar_64` já refaz o que o `fseek` faz no `FILE`].

### (c) Os itens extraídos para `CM3_TEMP\*.tmp`

Ao carregar, `zextract("*.tmp")` grava cada item `.tmp` como um arquivo na pasta. Daí em diante, o jogo lê e grava esses arquivos com a CRT (`fopen`/`fseek`/`ftell`, posições `long`). Os `.dat` não vão para a pasta: são lidos pelo `zread` direto para a memória (item (d)).

**O mecanismo dos que crescem: `FILE_LLIST`** (si/code/file_llist.CPP, 0x58e280–0x58f240) [observado]. É uma lista encadeada num arquivo: cada nó = registro de tamanho fixo + deslocamento do próximo (`int`, −1 = fim). As cabeças (primeiro/último deslocamento, `int`) ficam nos objetos de cada jogador ou clube e são gravadas nos `.dat` do save.

| Função | Endereço | O que faz com a posição |
|---|---|---|
| `FILE_LLIST::FILE_LLIST` | 0x58e280 | guarda o caminho e o tamanho do registro (0x2b, 0x1b...) |
| `add_node` | 0x58e340 | `fopen("ab")`, `fseek(fim)` + **`ftell`** (0x58e3f3/0x58e3f9) e grava o resultado como primeiro e último **sem conferir −1**; depois `fopen("r+b")` + `fseek(último + tamanho)` para ligar o nó |
| `add_node_list` | 0x58e9b0 | o mesmo para uma lista inteira (5 `ftell`) |
| `reset_last_offset` | 0x58f110 | último = −1 |
| `get_nodes` | 0x58f120 | `fseek(deslocamento)` e segue os "próximo" até o último |

**Quebra:** com o arquivo a partir de 2 GB, o `ftell` devolve −1. O nó novo fica com primeiro = último = −1, que é a marca de "lista vazia". O histórico daquele jogador some sem erro e o arquivo continua crescendo com nós órfãos [observado no código; o efeito no jogo é inferência]. Ler nós acima de 2 GB também não funciona, porque o deslocamento é `int` com sinal.

| Arquivo | Dono (construtor) | Tamanho medido | Cresce? |
|---|---|---|---|
| `player stats history.tmp` (+ `new_history.tmp`) | `PlayerStats` 0x7bfd50; recompactado a cada carga em `load_stats` 0x7c48b0 (`get_nodes` → `add_node_list` em `new_history.tmp` → `utils_copy_file` 0x90cc50 → apaga) | 66,5 MB (Valencia, 2028) | **sim, o maior** |
| `club records history.tmp` | `Club_Records` 0x444520 | 9,1 MB | sim |
| `staff history.tmp`, `club history.tmp` | `Staff_Records` 0x868390 (dois `FILE_LLIST`; ~60 chamadas de `add_node`) | 6,4 MB / 0,1 MB | sim |
| `injury_history.tmp` | `Injury` 0x62a680 | 5,7 MB | sim |
| `ban_history.tmp` | `Discipline` 0x5554a0 | 0,4 MB | sim |
| `club cup/league history.tmp` | `Club_History` 0x442ab0 | 0,1 MB | pouco |
| `comp records history.tmp` | `Comp_Stats` 0x49ee70 | 0,1 MB | pouco |
| `past_team_awards.tmp`, `past_year_awards.tmp` | `AWARD_MANAGER` 0x413180 | < 30 KB | pouco |

**Outros `.tmp` com posição de 32 bits que não crescem** [observado nos tamanhos]:
- `match_stats_matches.tmp` (`Match_Stats` 0x729000, `read_match_record` 0x729600, `write_match_record` 0x729820): tamanho fixo, 34.500.000 bytes em todos os saves do banco original e 69.000.000 nos do CLZ;
- `human/computer events1/2.tmp` (`Match_Events::save_event_log` 0x6d98b0, `load_event_log` 0x6d9c40): até ~2 MB, em rodízio;
- `fixtures_<ano>.tmp` (`FIXTURE_MANAGER::save_fixtures` 0x5ac060), `transfers_<ano>.tmp`, os `%s_%d.tmp` das competições, `Friendly.tmp` (0x5c9650): ~1,7 MB por ano no máximo. Crescem em quantidade, não em tamanho.

**`utils_copy_file`** (0x90cc50, usa `ftell`/`fseek`) [observado]: chamado na recompactação do `player stats history.tmp` e no `load_database` (0x5286b1). Quebra quando o arquivo copiado passa de 2 GB.

**A partir de que tamanho cada um quebra:** todos os `FILE_LLIST` e o `utils_copy_file` quebram com o **arquivo `.tmp` em 2 GB** (posição 0x80000000). No contêiner, o `zread` limita o item a 2 GB. O primeiro a chegar lá é o `player stats history.tmp`.

### (d) A memória do processo [observado o cabeçalho; o resto é inferência]

| Exe | Characteristics | LARGEADDRESSAWARE |
|---|---|---|
| `cm0102.exe` 3.9.68 | 0x10f | não |
| `cm0102-1080.exe`, `cm0102-portatil.exe` | 0x10f | não |
| `clz_teste\cm0102.exe` (NickPatcher) | 0x10f | não |

Sem a marca, o processo tem **2 GB** de endereços. A marca fica no cabeçalho do exe e o Windows a lê antes de carregar qualquer DLL: a nossa DLL não consegue ligá-la.

- **Fica inteiro na memória:** o banco (as tabelas do `save_database`) e os `.dat` dos subsistemas (`transfer.dat`, `player stats.dat`, `staff.dat`, `club records.dat`, `contract.dat`...). O `load` de cada um lê o item inteiro para as estruturas.
- **Fica no disco, lido aos pedaços:** os `.tmp` em `CM3_TEMP`. O `FILE_LLIST::get_nodes` lê só a lista pedida, e o `Match_Stats` lê um registro por vez.
- **Medido nos índices:** os itens que não são `.tmp` somam 108 MB no Valencia (39% do save) e 186 MB no TesteTelas. Entre o Maior2GBPatcher (2026-004) e o Valencia (2028-329), ~2/3 do crescimento foi em `.tmp` e ~1/3 em `.dat`.
- **Estimativa** [inferência]: com 4 GB de arquivo, ~1,3 GB de `.dat` na memória, mais o jogo, as DLLs, as superfícies de vídeo e a fragmentação da heap. Isso está perto ou acima dos 2 GB. **A memória pode ser a parede antes dos 4 GB**, e isso precisa ser medido (fase M, §3.2).

## 2. Medição (só o índice)

Saves lidos (só leitura: cabeçalho, índice e os 4 bytes da data; o `FeriasTesteSave.sav` continua com MD5 33fda465193b5e8e6d7ace93ecd44559, conferido depois):

| Save | Data do jogo | Tipo | Itens | Tamanho | Sem enchimento | `.tmp` | `FILE_LLIST` | `player stats history.tmp` |
|---|---|---|---|---|---|---|---|---|
| `jogo\FeriasTesteSave.sav` | 2001-205 | 3 | 142 | 137,6 MB | 137,6 MB | 47,3 MB | 11,0 MB | 10,3 MB |
| `jogo\Lucas.sav` | 2001-354 | 3 | 125 | 124,3 MB | 124,3 MB | 44,3 MB | 8,7 MB | 7,9 MB |
| `jogo\Roteiro.sav` | 2001-213 | 3 | 248 | 240,0 MB | 240,0 MB | 64,5 MB | 24,9 MB | 22,7 MB |
| `jogo\Testes Claude 2.sav` | 2001-275 | 3 | 248 | 252,8 MB | 252,8 MB | 70,1 MB | 27,6 MB | 23,7 MB |
| `jogo\TesteMais2GB.sav` (o "2,55 GB") | 2001-272 | 3 | 249 | 2.550,9 MB | **252,4 MB** | 69,7 MB | 27,5 MB | 23,7 MB |
| `jogo\TesteTelas_Descomprimido.sav` | 2001-332 | 3 | 250 | 259,5 MB | 259,5 MB | 73,0 MB | 29,9 MB | 24,7 MB |
| `clz_teste\Maior2GBPatcher.sav` (CLZ) | 2026-004 | 3 | 138 | 2.445,8 MB | **147,3 MB** | 81,3 MB | 10,9 MB | 10,4 MB |
| `clz_teste\Valencia.sav` (CLZ) | 2028-329 | 3 | 201 | 279,0 MB | 279,0 MB | 171,0 MB | 88,6 MB | 66,5 MB |

[observado] Os dois saves "acima de 2 GB" são o enchimento do teste da 158ª (`0000_enchimento.tmp`, 2.298.478.592 bytes, o primeiro `.tmp`). O conteúdo real tem 252 MB e 147 MB. **Não há carreira real acima de 2 GB entre os arquivos do Lucas.**

[observado] Maiores itens do Valencia: `match_stats_matches.tmp` 69,0 MB (fixo), `player stats history.tmp` 66,5 MB, `transfer.dat` 14,9 MB, `club records.dat` 9,8 MB, `club records history.tmp` 9,1 MB, `player stats.dat` 6,9 MB, `staff.dat` 6,9 MB, `staff history.tmp` 6,4 MB, `injury_history.tmp` 5,7 MB. Os 10 `.lng` (1,9 a 2,0 MB cada) e os arquivos de nomes não mudam.

[observado] Compressão: `TesteTelas_Comprimido.sav` (tipo 4) tem 131,7 MB contra 259,5 MB do `TesteTelas_Descomprimido.sav` (os dois com 250 itens, gravados com 16 s de diferença; mesmo estado [inferência]): 51%.

**Ritmo** [inferência; poucos pontos e carreiras diferentes]:
- CLZ, carreira do Valencia (~3,4 temporadas desde ~julho de 2025): `FILE_LLIST` ~23 MB por temporada (o `player stats history.tmp` ~17); save inteiro ~45 MB por temporada, supondo uns 130 MB no começo.
- Banco original com muitas ligas (carreira do TesteTelas, 2001-213 → 2001-332): +19,4 MB em 119 dias, ~60 MB/ano. O trecho é curto e dentro da temporada.
- Os dois pontos dão uma faixa de **45 a 100 MB por temporada** sem compressão. O valor depende mais das ligas ativas e do tamanho do banco do que do tempo. O crescimento tende a ser linear (registros por jogador e por temporada), mas os regens que se acumulam no `staff.dat` podem acelerá-lo.

| Limite | Quando [inferência] |
|---|---|
| 2 GB no arquivo (resolvido pela 158ª) | 20 a 45 temporadas |
| memória de 2 GB do processo | a medir; pela estimativa, com o arquivo entre ~3 e 4 GB |
| **4 GB no arquivo** | 40 a 90 temporadas sem compressão; ~80 a 180 com compressão |
| 2 GB num item (`player stats history.tmp`, 17 a 35 MB por temporada) | 55 a 115 temporadas; o arquivo estaria com ~6 a 8 GB |
| 999 itens | centenas de temporadas |

## 3. O modelo da mudança

### 3.1 Formato novo do contêiner

O tamanho da entrada (0x10c) e o bloco de 0x416ec continuam iguais. Assim a estrutura `ZFILE` na memória (índice em +0x25) não muda, e `zread`, `zwrite`, `zadd` e o RLE seguem sem mudança.

**Modelo M (mínimo):**
- marca de versão: `a4` do cabeçalho = **0x18** ("save estendido"). Hoje é 0x16. O `zclose` regrava o cabeçalho da memória, então a marca persiste sozinha;
- parte alta da posição: u32 em **+0x108** da entrada (os 4 últimos bytes do campo do nome). Hoje é zero em todos os saves medidos. Com a marca, o nome fica limitado a 255 caracteres (o `zopen` recusa nome maior);
- o tipo continua 3/4, porque `zread`, `zwrite`, `zclose`, `zextract` e `zget_matching_size` testam o tipo.

**Modelo R (robusto, recomendado):** M e mais o seguinte. Nas entradas além de 4 GB, o disco guarda posição = tamanho = **0xFFFFFFFF**, e a posição verdadeira (baixa +0x100, alta +0x104) e o tamanho (+0x108) vão para a cauda do nome. O `zopen` em Rust converte para o leiaute normal na memória, e o `zclose` (portado) converte de volta ao gravar. Leitores antigos que só conhecem o u32 veem um tamanho absurdo e falham em vez de ler lixo calado.

**Compatibilidade: gravar no formato antigo enquanto couber.** O `zopen` (modo 1) só liga a marca e escreve a parte alta quando a posição de um item passa de u32. Um save de até 4 GB continua idêntico ao de hoje e abre em qualquer jogo, com ou sem a DLL (até 2 GB).

**Ao abrir sem a DLL ou nas ferramentas:**

| Quem | O que acontece | Como recusar com clareza |
|---|---|---|
| jogo sem DLL (ou com a save2gb 1.0) | `check_saved_game_file` só aceita 0x16: **o save nem aparece na lista** [inferência a partir de MAPA_SAVE]; sem estrago | não há código para mostrar mensagem. O jeito é a DLL gravar `<save>.LEIA.txt` ao lado ("este save passou de 4 GB: precisa da DLL 2.0") |
| save2gb 1.0 / ddraw atual | idem, não lista | — |
| SaveChanger/Scouter do nckstwrt (`referencias/CM0102Patcher`) | leem o tipo (`== 4` → comprimido, qualquer outro → cru), pulam o +4 e usam a posição u32 [observado em `SaveReader2.cs`, `Scouter/SaveReader.cs`]. No **M**, leem lixo dos itens além de 4 GB sem avisar; no **R**, `new byte[-1]` gera exceção antes de qualquer gravação [inferência] | modelo R |
| CM3SaveEditor, CMScout (irmãos) | leem o índice u32 | pedido via `ORQUESTRACAO.md` §4 (regra de dono): reconhecer `a4 = 0x18` e recusar com mensagem, ou ler a parte alta |

### 3.2 As rotinas, por fase

**Fase A: arquivo acima de 4 GB, itens abaixo de 2 GB** (o caso dos próximos ~50 anos de jogo pela estimativa).

| Rotina | Endereço | Mudança |
|---|---|---|
| `zopen` | 0x9326e0 (Rust) | posição u64 (parte alta em +0x108 com a marca); ir ao fim com `SetFilePointer` e o ponteiro alto em vez do `fseek` da CRT; ligar a marca ao passar de u32 em vez de recusar; no R, converter o leiaute |
| `zseek` | 0x933080 (Rust) | `pos`, `alvo` e fim do item em u64 |
| `ztell` | 0x933170 (Rust) | `posicao_64 − pos` em u64 (o resultado, dentro do item, continua cabendo em i32) |
| `zget_archive_id` | 0x9331c0 (portar) | devolver 0x16 quando o cabeçalho tiver 0x18, para a caixa de arquivos listar o save (alternativa: `check_saved_game_file` 0x838e10) |
| `zget_matching_size` | 0x933860 (portar) | soma em u64 e saturação em `i32::MAX` na volta, para a conferência de espaço não passar por engano (decisão do Lucas, como a de 2 GB) |
| `zclose` | 0x932b00 (portar, só no R) | converter o leiaute antes de regravar o índice; inclui o fim do RLE (já previsto no item R1 do MAPA_SAVE) |

`zextract` e `zadd` não mudam: abrem cada item pelo `zopen`. `zread`/`zwrite` também não, enquanto os itens ficarem abaixo de 2 GB.

**Fase M: memória** (antes ou junto da A, conforme a medição):
- medir o pico de memória privada do jogo carregando saves de tamanhos diferentes, sem cliques (`-load` + `CM_FERIAS`, `psutil`), e extrapolar;
- se for a parede: ligar LARGEADDRESSAWARE **nas cópias geradas** (`patch_portatil.py`, `resolucao.py`; os originais não mudam) e provar com a DLL reservando a metade baixa do espaço de endereços no início, para a heap ir acima de 2 GB e expor ponteiros tratados com sinal. Carreira ampla Rust = original. Nos exes do NickPatcher, a DLL não alcança o cabeçalho: fica fora do nosso alcance.

**Fase B: itens entre 2 e 4 GB:**

| Rotina | Endereço | Mudança |
|---|---|---|
| `FILE_LLIST` (bloco) | 0x58e280–0x58f240 (`add_node`, `add_node_list`, `reset_last_offset`, `get_nodes` e vizinhas) | deslocamentos u32, com 0xFFFFFFFF como fim; posição por `buscar_64`/`posicao_64` |
| chamadores | `Staff_Records` (~60 `add_node`), `PlayerStats` 0x7c5fe0 e 0x7c3e40, `Injury`, `Discipline`, `Club_Records`, `Club_History`, `Comp_Stats`, `AWARD_MANAGER` | auditoria: comparações das cabeças com −1 ou com sinal (`jl`/`jg`). Só mudam se compararem |
| `zread` | 0x932c30 | comparações da posição no item sem sinal |
| `utils_copy_file` | 0x90cc50 | tamanho e posição em 64 bits |

**Fase C: item acima de 4 GB:** as cabeças e os "próximo" são `int` dentro dos `.dat` e dos `.tmp`. Seria preciso mudar o formato de 8 subsistemas (por exemplo, guardar o índice do nó em vez do byte) e o `load`/`save` de cada um. **Não recomendado**; pela estimativa, fica além de 100 temporadas.

### 3.3 O que muda na `save\` (DLL só do save, para os exes do NickPatcher)

- usa o mesmo `conteiner.rs` (`#[path]`): as mudanças da fase A entram sozinhas. `zget_archive_id`, `zget_matching_size` (e `zclose` no R) passam a ser desviados também;
- **assinatura nova:** o trecho conferido hoje é 0x9326e0–0x933860 e não cobre o `zget_matching_size` (começa em 0x933860). É preciso ampliar `TRECHOS` (DLL e `assinatura_save.py`) até 0x9339f5, além de 0x838e10 se esse for o desvio escolhido. Depois, conferir de novo o 3.9.68, as cópias e os exes do CLZ;
- versão 2.0 no `save2gb.log` ("saves acima de 4 GB"), release v2.0 no repositório público `CM0102-Rust/cm0102-save2gb` (fonte e doc só mudam na release), com aviso de que save acima de 4 GB não abre mais na 1.0 nem sem DLL.

### 3.4 O que muda na nossa `ddraw.dll`

- as mesmas rotinas, no `motor.rs` (tabela de prólogos) e no comparador, com os ramos novos ("conteiner estendido", "parte alta lida");
- a fase B entra como itens de leva do tipo save (FILE_LLIST: 3 a 6 rotinas). O comparador é o de **arquivo** previsto no MAPA_SAVE (original e Rust sobre cópias, comparando os bytes);
- a fase M, se aprovada, muda `patch_portatil.py`/`resolucao.py` (marca no cabeçalho da cópia) e ganha um modo de reserva baixa na DLL.

### 3.5 Como provar (sem cliques)

1. **Gravação:** `save_grande.py` no mesmo molde, com **três enchimentos de 1,5 GB** (`0000_a/b/c.tmp`, cada um abaixo de 2 GB, como na fase A) gera um `.qst` acima de 4,5 GB. O `conferir()` lê a parte alta quando houver a marca e confere a sequência das posições e o fim do arquivo. Na DLL atual, o item além de 4 GB é recusado; na nova, sai certo.
2. **Carga:** um montador em Python (novo, em `engenharia-reversa/`) cria um `.sav` sintético a partir de uma **cópia** do `FeriasTesteSave.sav`. Ele põe na frente um item de enchimento que não casa com `*.tmp` (por exemplo `zz_enchimento.bin`, que o `zextract` ignora) e marca o arquivo como **esparso** no NTFS, para os 4,5 GB não custarem E/S nem disco (a máquina compartilhada sofre com E/S pesada). A prova é o `prova_save.py --sav <sintético>` até 2001-08-28, que tem de dar a mesma impressão da carga normal (7a1167744a713c73).
3. **Mutações:** zerar a parte alta, não ligar a marca, `zseek` em u32. Todas têm de ser acusadas pela prova 2.
4. **Fase B:** um `FILE_LLIST` sintético de 2,5 GB (esparso, com nós só no fim), lido e acrescentado pela original e pela Rust sobre cópias.
5. **No jogo, com o Lucas:** só no fim, um save real com enchimento acima de 4 GB, gravado e carregado (como o teste de 2,55 GB da 158ª).

### 3.6 Riscos

| Risco | Cuidado |
|---|---|
| memória antes do formato | medir primeiro (fase M); sem LARGEADDRESSAWARE, o save de 4 GB pode não carregar mesmo com o contêiner certo |
| `fseek(SEEK_END)` da CRT acima de 4 GB | trocar no `zopen` por busca com a parte alta; procurar outros `fseek`/`ftell` no caminho do contêiner |
| a heap às vezes deixa lixo na cauda do nome | hoje sai zero (8 de 8 saves); a parte alta só é lida com a marca |
| ferramentas antigas lendo lixo | modelo R; aviso na release; pedido aos irmãos |
| E/S e disco: salvar copia todos os `.tmp` para o contêiner, carregar extrai tudo, e a carga recompacta o `player stats history.tmp` (o dobro dele em disco) | provas com arquivo esparso; trava do laboratório; um teste pesado por vez |
| a assinatura muda e a save2gb 2.0 deixa de agir num exe que altere 0x933860+ | conferir os exes conhecidos antes da release |
| salvar por cima com um jogo sem a DLL: o save estendido nem aparece, e o `save_game_data` só apaga o arquivo antigo se o nome for o mesmo | aviso no `.LEIA.txt` |

### 3.7 Custo estimado [inferência]

| Fase | Trabalho | Levas |
|---|---|---|
| A (arquivo > 4 GB) | 3 rotinas alteradas, 2 portadas (3 no R), prova 1 e 2, save2gb 2.0 | **1 leva** + 1 sessão de prova + release |
| M (memória) | medição; marca nas cópias, reserva baixa e carreira ampla | 1 sessão de medição; ~1 leva se for preciso |
| B (itens de 2 a 4 GB) | FILE_LLIST + `zread` + `utils_copy_file` + auditoria dos chamadores | **~2 levas** |
| C (item > 4 GB) | formato de 8 subsistemas | 5 ou mais; não recomendado |

**Ordem sugerida:** medir a memória (M) → fase A no modelo R → B só quando algum `player stats history.tmp` real passar de ~1 GB.

## Medição num save real longo (25/10/2026)

O Lucas trouxe `All Leagues - 3.9.68 - Compress.sav` (3.9.68 sem patch, base "All Leagues"), só lido no índice e no `general.dat` [observado]:

- **Data do jogo:** 2063, dia 182, ou seja, cerca de 62 temporadas.
- **Arquivo:** 1.431.514.996 bytes (tipo 4, compactado), com 564 itens.
- **Soma dos itens descompactados:** 2.147,8 MB, um pouco acima de 2^31 (2.147,48 MB). O mesmo save sem compressão passaria de 2 GB, e só abriria com o save2gb.
- **Maiores itens:** `player stats history.tmp` 646,4 MB; `club records history.tmp` 474,2 MB; `injury_history.tmp` 258,5 MB; `staff history.tmp` 186,6 MB; `match_stats_matches.tmp` 69,0 MB; `contract.dat` 40,9 MB; `transfer.dat` 26,4 MB. Os `.dat` somados ficam bem abaixo dos históricos.

Ritmo médio nesta carreira [inferência, um ponto só, com crescimento suposto linear]: ~35 MB por temporada descompactado (~23 MB compactado), abaixo da estimativa acima. Com esse ritmo:

- o arquivo descompactado chegaria a 4 GB perto da temporada 115 (~2116); compactado, perto da 175;
- o maior item (`player stats history.tmp`, ~10,4 MB por temporada) chegaria a 2 GB perto da temporada 190.

Este save é o melhor caso de teste real para as fases A e B: ele já tem um item de 646 MB e a soma no limite de 2 GB.
