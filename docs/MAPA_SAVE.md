# Mapa do bloco do save

Levantamento para as levas do save (ordem combinada: match_screens/match_day, **save**, Database, competições). Só leitura: desmontagem do `cm0102.exe` 3.9.68, o mapa da comunidade (`referencias/cm0102-function-map`), `docs/mapa_funcoes.csv` e o projeto irmão `D:\Sistemas\CM3SaveEditor`. O formato do contêiner e a correção acima de 2 GB estão em [SAVE_2GB.md](SAVE_2GB.md).

Tamanhos em bytes, medidos do início ao último byte antes do enchimento (o "size" do mapa da comunidade não é em bytes).

## 1. As funções

### si/code/zipdir.cpp (o contêiner)

Nenhuma tem SEH. Todas usam a CRT de arquivo do jogo (`fopen` 0x945b00, `fread` 0x945bfe, `fwrite` 0x945ce6, `fseek` 0x946a22, `fclose` 0x945a09). A estrutura aberta (`ZFILE`, 0xa3191 bytes, `new` 0x944cf1) guarda o arquivo em +0, modo +4, posição lógica +8, item +0xc, estado do RLE +0x10/+0x14/+0x18, cabeçalho e índice em +0x19/+0x25, buffer de leitura +0x41705 e de escrita +0x72445 (0x30d40 bytes cada).

| Endereço | Nome | Bytes | O que faz | Quem chama | Rust | Aloca / arquivo / UI |
|---|---|---|---|---|---|---|
| 0x9326e0 | `zopen` | 1051 | abre um item para ler, ou acrescenta um para gravar (cria o contêiner com o cabeçalho) | 114 chamadas em ~70 rotinas: `load`/`save` dos subsistemas, `load_database`, `save_database`, `zextract`, `zadd` | **sim** (158ª, `conteiner.rs`) | aloca 0xa3191; arquivo |
| 0x932b00 | `zclose` | 294 | gravando: esvazia o buffer, fecha a repetição pendente do RLE e regrava cabeçalho + índice inteiros (0x416ec bytes) no começo; fecha e libera | as mesmas, 288 chamadas | não | libera; arquivo; efeito único |
| 0x932c30 | `zread` | 468 | `fread` dentro do item; no tipo 4 decodifica o RLE | 521 chamadas: subsistemas, `db_files_read_*`, `load_game_data`, `zextract`, `zseek` | não | arquivo |
| 0x932e10 | `zwrite` | 611 | `fwrite` no item; soma ao tamanho no índice; no tipo 4 codifica o RLE | 327 chamadas: subsistemas, `db_files_write_*`, `save_game_data`, `zadd` | não | arquivo |
| 0x933080 | `zseek` | 239 | `fseek` dentro do item; no tipo 4 volta ao começo e relê | `load_database` | **sim** | arquivo |
| 0x933170 | `ztell` | 76 | `ftell` dentro do item | `save_database` | **sim** | arquivo |
| 0x9331c0 | `zget_archive_id` | 342 | lê o cabeçalho e devolve o dword +4 (0x16 `.sav`, 0x17 `.qst`; 0x14 se falhar) | `check_saved_game_file`, `check_quick_start_file` | não | aloca 0x416ec; arquivo; caixa de erro |
| 0x933320 | `zextract_matching_files` (desempacotar) | 746 | para cada item cujo nome casa com o padrão: `zopen` + `zread` em blocos de 0x30d40 + `fwrite` num arquivo da pasta | `load_game_data` (`*.tmp`), `load_database` (`*.lng`) | não | aloca 0x3f7a0 (0x944e46) + 0x416ec; cria arquivos; caixa de erro |
| 0x933610 | `zadd_matching_files` (empacotar) | 582 | `file_find_first/next` (0x4c0aa0/0x4c0b50) na pasta; cada arquivo vira item: `zopen` gravando + `fread`/`zwrite` em blocos de 0x30d40 + `zclose` | `save_game_data` (`*.tmp`, tipo 3 ou 4), `save_database` (`*.lng`) | não | arquivo; caixa de erro |
| 0x933860 | `zget_matching_size` | 405 | soma os tamanhos dos itens que casam com o padrão (`int` com sinal: passa de 2 GB negativo) | `load_game_data` (espaço em disco antes de desempacotar) | não | aloca 0x416ec; arquivo; caixa de erro |
| 0x933a00 | `zipdir_match_filename_pattern` | 79 | casa nome com padrão com `*`, sem diferenciar maiúsculas | `zextract`, `zget_matching_size` | não | pura |

O RLE do tipo 4 não tem rotina própria: está dentro de `zread`, `zwrite` e `zclose` (e `zseek`, já em Rust, relê pela `zread`).

### cm3/code/Setup.cpp e auxiliares (gravar e carregar)

| Endereço | Nome | Bytes | O que faz | Quem chama | SEH | Carreira automática |
|---|---|---|---|---|---|---|
| 0x82e430 | `save_game_data(caminho, a4)` | 6927 | grava o save inteiro (fluxo na §2) | menu (`menubar_handle_command`), `create_game_from_setup_file` (`.qst`) | sim | só pelo `quick_start.cfg` (`save_grande.py`), tipo 3 |
| 0x82a9c0 | `load_game_data(caminho)` | 14950 | carrega o save inteiro (§2) | tratador da tela Setup Game (0x81bd70, sem nome no mapa: Restore e `-load`), rede | sim (40 desenrolamentos) | não; sem cliques só com `-load <save>` |
| 0x82ff40 | `free_game_data(flag)` | 1500 | apaga os objetos de todos os subsistemas, os `*.tmp` e zera o estado do jogo | `setup_run_front_end`, `create_game_from_setup_file`, `game_main_day_loop`, `game_fatal_cleanup`, rede | não | **sim** (abertura) |
| 0x827110 | `create_game_from_setup_file(cfg, qst)` | 3129 | lê `auto_setup.cfg`/`quick_start.cfg`, cria o jogo e grava o `.qst` ou acrescenta o técnico | `setup_run_front_end` | não | **sim** |
| 0x838e10 | `check_saved_game_file` | 152 | filtro da caixa de arquivos: aceita id 0x16 | caixa de arquivos (ponteiro) | não | não (cliques) |
| 0x839290 | `check_quick_start_file` | 165 | filtro: aceita id 0x17 e versão igual | caixa de arquivos (ponteiro) | não | não |
| 0x838eb0 | `valid_saved_game_version` | 977 | abre `general.dat`, lê versão e resumo, compara com a do exe | `check_quick_start_file` | sim | não |
| 0x8395b0 | `save_fifa_transfer_rules(zip)` | 421 | grava `fifa_transfer_rules.zip` (2 bytes) no save | `save_game_data` | não | não |
| 0x839760 | `load_fifa_transfer_rules(zip)` | 436 | lê o mesmo item; regra em vigor = estado 5 | `load_game_data` | não | não |
| 0x591f80 | `build_save_game_summary` (file_screens.cpp) | 712 | monta o resumo do `general.dat` (nações, técnicos, data, versão, tempo de jogo) | `save_game_data` | não | não |
| 0x524d30 | `save_database_for_game` (Database.cpp) | 26 | chama `save_database` 0x52dad0 | `save_game_data` | não | não |
| 0x524d10 | `load_database_from_stream` (Database.cpp) | 21 | chama `load_database` 0x528540 | `load_game_data` | não | não |
| 0x90cd50 | `utils_clear_temp_folder` (utils.cpp) | 453 | apaga os arquivos de `CM3_TEMP` em grupos de 100 | `create_game`, `load_game_data`, `free_game_data`, `database_free_all_tables` | não | **sim** |

Nenhuma destas está em Rust. As rotinas `load`/`save` de cada subsistema (~30 pares, listados na §2) ficam com o arquivo de cada um; só `Match_Stats::load/save` (0x72aa80/0x72adf0) já estão em Rust.

**Carreira automática e zipdir:** a base das cadeias não registra os ramos `conteiner lido/gravado/criado` da 158ª: a carreira de férias não abre nenhum item. O contêiner só roda ao gravar (menu, `.qst`) e ao carregar.

## 2. Os fluxos

### Gravar (`save_game_data`)

1. Data local, `[0xdd7110] = 0xfffc1fb9`. Menos de 25 MB livres liga `[0xdd7134]` (caixa Disk Full); memória livre < 1 liga `[0xdd7138]`.
2. Em rede (bit 0x20000000 de `[0xae3138]+4`) grava em `temp_save_file` e renomeia no fim; senão apaga o save antigo (`remove`).
3. Tipo do contêiner: bit 0x8000 de `[0xae3138]+4` (o dword +4 do `game.cfg`) ligado = **3** (sem compressão), desligado = **4** (RLE). O `a4` do cabeçalho é o 2º argumento: 0x16 `.sav`, 0x17 `.qst`.
4. Passo 1: `save_database_for_game` → `save_database` grava as tabelas do banco (um item por tabela, `index.dat`) e empacota os `*.lng` (`zadd`).
5. Passo 2: `general.dat` (16 `zwrite`: versão, resumo de `build_save_game_summary`, id único, tamanho do banco, data e hora, última data e hora, ligas, nação padrão, dados de execução de pessoas e clubes...). Depois, cada subsistema, nesta ordem (entre um e outro: espaço em disco, memória e mensagem de rede 0x802f80):

| Ordem | Rotina | Vai para |
|---|---|---|
| 1 | `HUMAN_MANAGER::save` 0x5ff9e0 | `human_manager.dat` |
| 2 | `SCOUT_MANAGER::save` 0x7f8020 | `scout_manager.dat` |
| 3 | `News::save_to_zip` 0x787c30 | `news.dat` |
| 4 | método +0x1c de `[0xae2c74]` (Friendly, 0x5c9650) | `Friendly.tmp` |
| 5 | `UefaSeeding` 0x905510, `ConmebolSeeding` 0x4c6b70 | `uefa_seeding.tmp`, `conmebol_seeding.tmp` |
| 6 | `Staff_Records` 0x86ffd0, `Match_Stats` 0x72adf0, `PlayerStats` 0x7c55a0, `Club_History` 0x443f50, `Club_Records` 0x451ff0, `Tactics` 0x8995d0 | `staff records.dat`, `match_stats.dat`, `player stats.dat`, `club history.dat`, `club records.dat`, `tactics.dat` |
| 7 | `HostCountry` 0x5fb320 | `HostCountry.tmp` |
| 8 | `Discipline` 0x555dd0, `AWARD_MANAGER` 0x4141a0, `Fine` 0x5a53d0, `AWOL` 0x418980, `DISPUTE` 0x569ae0, `Notes` 0x794f00, `Injury` 0x62b2d0, `NationalTeams` 0x776a20 | `discipline.dat`, `awards.dat`, `fine.dat`, `awol.dat`, `dispute.dat`, `notes.dat`, `injury.dat`, `national_teams.dat` |
| 9 | `Stadium` 0x85bea0, `FIXTURE_MANAGER::save_fixtures` 0x5ac060, `key_nation_save_all_comps` 0x6692f0 | `Stadium.tmp`, `fixtures_<ano>.tmp`, `club_dates.tmp`, `check_fixture_legs.tmp`, os `.tmp` das competições |
| 10 | `CONTRACT_MANAGER` 0x4dc3a0, `FinanceTable` 0x59b3b0, `TRANSFER_MANAGER` 0x8bd4b0, `PLAYER_REGEN` 0x7ad7c0, `Media` 0x72b5f0, `FOG_OF_WAR` 0x5afad0, `MANAGER_MANAGER` 0x68e920, `OfficialsManager` 0x79bb80, `SimulatedStats` 0x84c680, `TRAINING_MANAGER` 0x8b1450, `SQUAD_MANAGER` 0x859cc0 | `contract.dat`, `finance.dat`, `transfer.dat`, `retire.dat`, `media.dat`, `fog.dat`, `man_conf.dat`, `off.dat`, `simulated_stats.dat`, `training.dat`, `squad_manager.dat` |

6. `zadd_matching_files(save, CM3_TEMP, "*.tmp", a4, tipo)`: todos os `.tmp` da pasta temporária (os do passo 5 e os que o jogo foi escrevendo: `history.tmp`, `transfers_<ano>.tmp`, `ban_history.tmp`, `injury_history.tmp`, `events*.tmp`...), na ordem do `FindFirstFile` (nome, no NTFS).
7. `HallOfFame::save` 0x5f00c0 (fora do save), `save_fifa_transfer_rules` (o último item), renomeação em rede, data do último save em 0xdd70a0.

Ordem real conferida num save de 248 itens (CM3SaveEditor, `docs/FORMATOS.md` §4): 20 tabelas do banco, `index.dat`, 10 `.lng`, `general.dat`, os 29 `.dat` dos subsistemas, 186 `.tmp`, `fifa_transfer_rules.zip`.

### Carregar (`load_game_data`)

1. Pasta `CM3_TEMP`, CD (`Insert CD`), espaço e memória.
2. `database_free_all_tables` 0x528080 e `load_database_from_stream` → `load_database`: cada tabela pelo `zopen`/`zread`/`zseek` e `zextract("*.lng")`.
3. `utils_clear_temp_folder`, `zget_matching_size(save, "*.tmp")` contra o disco livre ("unable to unpack the temporary files"), `zextract_matching_files(save, CM3_TEMP, "*.tmp")`.
4. `general.dat` com `zread`, na ordem da gravação, com uma mensagem de erro por campo.
5. Tabela das nações-chave (0x667150), e os construtores dos subsistemas, que leem o próprio `.dat` do save (recebem o `ZipDir`) ou o `.tmp` da pasta: `Tactics`, `HUMAN_MANAGER`, `SCOUT_MANAGER`, `News`, `Friendly`, `UefaSeeding`, `ConmebolSeeding`, `Staff_Records`, `Match_Stats`, `Club_Records`, `PlayerStats`, `Club_History`, `HostCountry`, `Discipline`, `AWARD_MANAGER`, `Fine`, `AWOL`, `DISPUTE`, `Notes`, `Injury`, `NationalTeams`, `VIRTUAL_STAFF`, `Stadium`, `FIXTURE_MANAGER`; `key_nation_create_all_comps` 0x669180 (competições, dos `.tmp`) e `build_comp_object_table` 0x8386f0; depois `FinanceTable`, `CONTRACT_MANAGER`, `TRANSFER_MANAGER`, `PLAYER_REGEN`, `MANAGER_MANAGER`, `Media`, `FOG_OF_WAR`, `OfficialsManager`, `SimulatedStats`, `Weather`, `TRAINING_MANAGER`, `PunditList`.
6. `load_fifa_transfer_rules`; sem `auto_setup.cfg`, a caixa "CD Not Required".

### O RLE (tipo 4)

| Byte gravado | Significado |
|---|---|
| `b ≤ 0x80` | o próprio byte (literal) |
| `b > 0x80` | `b − 0x80` cópias do byte seguinte (1 a 0x7f) |

- **Codificar (`zwrite`):** guarda o byte da repetição em +0x18 e a contagem em +0x14, através das chamadas. Byte diferente: repetição de 1 com byte ≤ 0x80 sai literal; senão sai o par (`0x80 + n`, byte). Ao chegar a 0x7f, sai o par e recomeça. `zclose` fecha a repetição pendente do mesmo jeito.
- **Decodificar (`zread`):** uma repetição pode atravessar chamadas (+0x10 já entregues de +0x14).
- O índice guarda a posição no arquivo e o tamanho **descomprimido**; a posição lógica (+8) conta bytes descomprimidos.
- Buffers de 0x30d40 bytes: cheio, grava e recomeça no byte 0.
- O cabeçalho e o índice ficam crus (sem RLE) no começo do arquivo.

## 3. O que o CM3SaveEditor oferece (só leitura, regra de dono)

| O quê | Onde | Serve para |
|---|---|---|
| Leitura do contêiner (cabeçalho, 1000 entradas de 268 bytes, nome) | `rust/cm3sav/src/lib.rs`: `Sav::new`, `n_arquivos`, `entradas` | conferir a saída do `zadd`/`zclose` nos testes (formato idêntico ao do jogo) |
| Ida e volta byte a byte (extrair os 248 itens e remontar só pelo manifesto: SHA-256 igual) | `testes/teste_container.py --viagem`, `engenharia-reversa/sav_lista.py` (valida o modelo), `sav_extrai.py`, `sav_info.py` | prova de que a tabela tem 1000 vagas, dados contíguos, sem cauda: oráculo do empacotar tipo 3 |
| Ordem real dos itens e tamanhos | `docs/FORMATOS.md` §2–§4 | conferir a ordem da §2 |
| RLE: só recusa o tipo 4 (`teste_recusa_comprimido.py`) | — | nada; o decodificador está no CMScout: `D:\Sistemas\CMScout\rust\olheiro\src\rle.rs` (`descomprimir`, provado em `tests/comprimido.rs` contra o save não comprimido do mesmo jogo). Não há codificador em nenhum irmão: a referência do `zwrite` é só o jogo |
| Estruturas dos registros do banco e dos `.dat` | `pessoa.rs`, `clube.rs`, `contrato.rs`, `nacao.rs`, `competicao.rs`, `arbitro.rs`, `preferencias.rs`, `scout.rs`, `aposentadoria.rs`; `docs/CAMPOS.md`, `docs/REGISTROS.md`; `extrair_finance/contract/injury/retiring` em `lib.rs` | bloco Database e os `.dat` dos subsistemas, não este |
| Não serve | `achar` (51 caracteres em maiúsculas), `extrair_como_editor` (blocos de 0xFFFA), `gravar_como_editor` (erro E2), arredondamento do VB | são do editor, não do jogo (`zopen` compara o nome inteiro com `strcmp`) |

O dword +4 que o editor chama de desconhecido é o `a4` do `zopen`: 0x16 `.sav`, 0x17 `.qst` (`zget_archive_id`, `check_saved_game_file`).

## 4. Levas propostas

Tipo novo de item (**save**), ao lado de telas e simulação. 21 rotinas em 7 itens e 4 levas.

| Leva | Item | Rotinas | Comparador previsto |
|---|---|---|---|
| S1 | C1 contêiner: consultas | 0x933a00 padrão, 0x9331c0 id, 0x933860 tamanho | padrão: `lado_a_lado` puro (+ autoteste com nomes, `*`, maiúsculas); id e tamanho: as duas no mesmo arquivo (só leem), compara a resposta |
| S1 | C2 empacotar/desempacotar | 0x933320 `zextract`, 0x933610 `zadd`, 0x90cd50 limpar `CM3_TEMP` | **arquivo** (novo): original e Rust sobre cópias (pastas A e B em `jogo_testes`), compara resposta, lista e bytes dos arquivos gerados |
| S1 | R1 RLE | 0x932c30 `zread`, 0x932e10 `zwrite`, 0x932b00 `zclose` | **arquivo**: dois `ZFILE` sobre cópias do mesmo contêiner; compara buffer, resposta, a estrutura sem o `FILE*` (+0) e, no `zclose`, o arquivo inteiro. Autoteste com tipos 3 e 4, repetições de 1 a 300, bytes > 0x80, viradas do buffer de 0x30d40, repetição entre chamadas; leitura de um save comprimido real (cópia de `TesteTelasComprimido.sav`) comparando cada item |
| S2 | S1 verificações | 0x838e10, 0x839290, 0x838eb0 | `lado_a_lado` sobre um `.sav`/`.qst` fixo (só leitura); autoteste, porque só a caixa de arquivos chama |
| S2 | S2 itens avulsos | 0x8395b0, 0x839760 regras FIFA; 0x591f80 resumo | regras: **arquivo**; resumo: `lado_a_lado` do bloco (com a máscara do tempo de jogo, se for relógio) |
| S2 | S3 ligações e limpeza | 0x524d30, 0x524d10, 0x82ff40 `free_game_data` | ligações: repasse (validadas pelo save e pela carga); `free_game_data`: orquestrador, carreira |
| S3 | S4 gravar | 0x82e430 `save_game_data`, 0x827110 `create_game_from_setup_file` | orquestrador sem comparador: `.qst` gravado pela original e pela Rust no mesmo estado, comparado byte a byte (tipos 3 e 4); `create_game_from_setup_file` também pela carreira |
| S4 | S5 carregar | 0x82a9c0 `load_game_data` | orquestrador: `-load <qst>` + carreira de férias, impressão Rust = original; mutações pela cadeia |

**Banco de provas do bloco:** `engenharia-reversa/prova_save.py` (`docs/COMO_CONTINUAR.md`, "Prova do save"): `quick_start.cfg` com a compressão ligada (bit 0x8000 do `game.cfg` desligado, cópia e restauração) grava um `.qst` tipo 4 nos dois modos e compara item a item. A carga não pode ser do `.qst` (sem técnico, o `-load` para no "Enter Name"): usa um `.sav` com técnico de férias, feito uma vez pelo Lucas. A carreira atual não passa por nenhuma rotina de zipdir.

## 5. Riscos

| Risco | Onde | Cuidado |
|---|---|---|
| Efeito único | `zclose` (fecha e libera), `zextract`/`zadd`/limpar temporários (criam e apagam arquivos), `save_game_data` (apaga o save antigo), `load_game_data` e `free_game_data` (constroem e destroem tudo) | comparar sobre cópias; orquestradores só pela carreira |
| Caixas de erro (0x5e8290) | `zget_archive_id`, `zextract`, `zadd`, `zget_matching_size`, Setup | o autoteste não pode cair nos caminhos de erro, ou trava esperando clique |
| Lixo de heap no arquivo | `zopen` criando grava 0x416ec bytes de cabeçalho + índice de uma alocação não zerada (as entradas além da contagem); `zclose` regrava | hoje sai zero (bloco grande da heap); o comparador confere o cabeçalho inteiro |
| Ordem do `FindFirstFile` | `zadd`, limpar temporários | a Rust usa as mesmas 0x4c0aa0/0x4c0b50 |
| `int` com sinal | `zget_matching_size` acima de 2 GB dá negativo e a conferência de espaço passa sempre | manter igual; correção é outra decisão (como o 2 GB) |
| Lixo de pilha | `save_game_data` (quadro de 0x175c), `load_game_data`, `valid_saved_game_version` (SEH) | sonda de zeragem (`ZERADAS`) antes de comparar |
| Não determinismo | resumo (tempo de jogo), data e hora no `general.dat`, espaço em disco, CD | conferir na 1ª prova; máscara se for relógio |
| DLL do save | `save/` usa as três rotinas de `conteiner.rs` e a assinatura de 0x9326e0–0x933860 | as novas rotinas não entram em `save/`; o repositório público só muda nas releases |
| Alocação grande | cada `zopen` aloca ~650 KB, `zextract` mais ~530 KB | muda a heap e a impressão: comparar só no passo do save |

Fronteira com o bloco Database: `save_database` 0x52dad0, `load_database` 0x528540, `database_free_all_tables` 0x528080 e as `db_files_read_*`/`db_files_write_*` (Db_files.cpp, 0x54d5e0–0x550950), todas sobre `zread`/`zwrite`.

## Pista: ids fora da tabela no elenco gravado (23/10/2026, do estudo do validador do Save Editor)

Fonte: CM3SaveEditor\docs\VALIDADOR_ESTUDO.md (commit bd7959e), repassado pela sessão orquestradora; marcado como inferência lá. O próprio jogo grava no elenco do club.dat ids ≥ N (fora da tabela de pessoas): 51 vagas no TesteTelas.sav (clube 1109, ids 159578..159597; clube 3397, 158727..158734) e 22 no Valencia.sav. Entre o TesteTelas e o TesteTelasComprimido o club.dat muda só nessas vagas (viram −1) e o staff.dat fica igual: um recarregamento entre os dois, com o `resolve_database_indexes` esvaziando as vagas. Hipótese: jovens criados nas linhas de reserva (as 23.000 de tipo 6 ou as 1.325 não contadas) que o `save_database` (0x52dad0) não leva ao staff.dat, mas cujo id fica no elenco. É um candidato à causa da corrupção de ids que o Save Game Editor conserta (PROJETO.md, item 8). Conferir quando o save_database e a carga estiverem em Rust: a cópia mostra exatamente que pessoas entram no staff.dat e quais ids ficam nos elencos.

## 6. O `.tmp` de uma copa (30/10/2026)

Cada competição grava o seu estado num `.tmp` da pasta `CM3_TEMP` (`docs/.md` §2.11), e esses arquivos entram no save como itens. A ordem abaixo é a de `Cup::save_cup_to_disk` (0x5210F0), lida na desmontagem. A carga, `Cup::load_from_disk` (0x51FC00), lê na mesma ordem. Deslocamentos com `+` são do objeto da copa (0xB2 bytes) e "u16"/"u32" são inteiros com sinal, little-endian.

**Prova** [observado]: `engenharia-reversa/tmp_copa.py <save>` segue esta ordem e confere que cada arquivo acaba exatamente onde a leitura acaba.
- Fecham 47 dos 186 `.tmp` do `Testes Claude 2.sav`, e 48, 47 e 9 em outros três saves (`Roteiro`, `TesteTelas`, `FeriasTesteSave`). Todos são copas sem estágios: as supertaças (1781 bytes) e as copas nacionais de 1 a 9 rodadas, até a Taça de França com 222 inscritos.
- Os ids dos inscritos são clubes do `club.dat`: a Supertaça de Inglaterra dá Arsenal × Liverpool, a Intercontinental Bayern × Boca, a Supertaça de Portugal Sporting × Leixões.
- Três mutações no leitor (tirar o +0x562 das estatísticas, confronto de 0x34 em vez de 0x35, tirar o +0xab) derrubam a contagem para 0.
- Não fecham, e o leitor separa: as ligas, as copas com estágios (as 51 que recusa por ter estágio atual ≥ 0) e os 20 globais.

| Ordem | Campo | Tamanho |
|---|---|---|
| 1 | nome do arquivo (+0x53) | 0x50 |
| 2 | há bloco de estatísticas (+0x08 ≠ 0) | u32 |
| 3 | o bloco, se houver: `Comp_Stats::save` (0x4AC4E0), ver abaixo | variável |
| 4 | dados próprios da classe derivada: slot 10 da vtable (+0x24); na base é vazio (0x48CEA0 = `ret 4`) | variável |
| 5 | fundo de prêmios +0x28 | u32 |
| 6 | +0x52, +0x51, +0x4B | 3 × u8 |
| 7 | +0x4C, +0x24 (o id da regra de mando, §2.1 da referência) | 2 × u32 |
| 8 | banco +0x49, substituições +0x4A | 2 × u8 |
| 9 | +0x45 (o sorteio só roda com ele ≠ 0, 0x51C431) | u32 |
| 10 | +0xB1 | u8 |
| 11 | rodadas +0x3C, inscritos +0x3E, ano +0x40 | 3 × u16 |
| 12 | +0xAB, +0xAC, tipo +0x42 | 3 × u8 |
| 13 | código do estágio principal +0x3A | u16 |
| 14 | liga de cima +0x1C, de baixo +0x20 | 2 × u32 |
| 15 | **o ponteiro** da tabela de média de pontos +0x18 (o valor de memória, gravado como está) | u32 |
| 16 | linhas da média +0x34 e, se o ponteiro não é nulo, 0x13 bytes por linha | u16 + variável |
| 17 | id da competição (registro +0, o mesmo do nome do arquivo) | u32 |
| 18 | rodadas: 0x68 bytes cada (leiaute na referência, §2.4) | 0x68 × +0x3C |
| 19 | inscritos (+0xA7): 6 bytes cada; o ponteiro do clube vira o **id** na gravação e volta a ser ponteiro depois | 6 × +0x3E |
| 20 | confrontos de cada rodada (+0xAD): 0x35 bytes por jogo, quantos diz o +0x1A da rodada | variável |
| 21 | estágios +0x2C, estágio atual +0x30, +0x44 | u32 + u32 + u8 |
| 22 | para cada estágio de 0 até o atual: tipo (+0x43 do estágio); se 1, estágio de copa: +0x3E, +0x3C, o +0x1A de cada rodada (u16) e `Cup_Stage::save_to_file` (0x523EB0); se 2, estágio de liga: +0x3E, +0xA9 e `League_Stage::save_to_file` (0x68CA40) | variável |
| 23 | livro de inscritos: capacidade +0x38, usados +0x36 (u16 cada) e, se existe, 6 bytes por vaga da capacidade, com os clubes em id | variável |

**Bloco de estatísticas** (`Comp_Stats::save`, objeto de 0x5CE bytes), na ordem: +0x10 (u32); +0x14 (u32, n) e n × 0x76 de +0x0C; +0x1D (u32); +0x21 (u32, n) e n × 0x2D de +0x19; 0x64 de +0x56A; 0x269 de +0x26 e 0x269 de +0x28F (os dois conjuntos de recordes); +0x4F8, +0x4FC (u32); +0x500 (u16); 0x5C de +0x506; +0x562 (u32); +0x04 (u32, n); +0x08 (u32); n × 0x1C de +0x00. Sem tabelas, são 0x5B8 bytes.

**O que isso diz** [inferência, a conferir]:
- O `.tmp` não é cópia do objeto: é uma lista de campos escolhidos, e os ponteiros para clubes viram ids. A exceção é o +0x18, gravado como endereço de memória; é ele o "ponteiro" que a outra sessão achou em alguns `.tmp`.
- As tabelas só são gravadas se o ponteiro existir, e o arquivo não guarda essa marca: o leitor supõe tabela vazia quando a contagem é 0, o que fechou em todos os casos acima.
- Para o editor: numa copa simples, inscritos e confrontos estão em posições calculáveis, e trocar um clube é trocar um id. Mudar a quantidade de rodadas ou de inscritos muda os tamanhos e exige regravar o arquivo inteiro.
- Faltam: os estágios (o próximo passo natural, com `Cup_Stage::save_to_file` e `League_Stage::save_to_file`), o `.tmp` das ligas (`League::save_to_temp_file`, 0x6891C0), o slot 10 das copas derivadas e o leiaute do confronto de 0x35 bytes.
