# Mapa das estatísticas da competição e do elenco da pessoa

Levantamento para o editor de saves (pedido do Lucas via orquestrador, 26/10/2026): (A) onde o save guarda quem jogou por qual clube em cada competição (a regra "já atuou por outro clube nesta competição", *cup-tied*) e como liberar o jogador; (B) o valor de quem não é jogador e o byte `+0x5e` da pessoa (PlayingSquad).

Só leitura: desmontagem (`desmonta.py`, `refs.py`) e pseudocódigo do Ghidra (`descompilar.py`) do `cm0102.exe` 3.9.68, o mapa da comunidade (`referencias/cm0102-function-map`, nomes de funções e globais) e a leitura de uma **cópia** do `jogo/FeriasTesteSave.sav` (MD5 33fda465193b5e8e6d7ace93ecd44559, conferido antes da cópia; o original não foi tocado). O jogo não foi aberto. Data do save: 205/2001 (24/07/2001), começo de temporada.

Cada afirmação leva **[observado]** (lido na desmontagem, no pseudocódigo ou no arquivo) ou **[inferência]** (dedução sem prova direta). Endereços em VA. "Pessoa" = registro de 110 bytes do `staff.dat` (na memória, `[0xae23bc] + id * 0x6e`).

## A. Quem jogou por qual clube em cada competição

### A.1 Onde fica

Cada competição tem um objeto `Comp_Stats` em `comp + 8` (`comp = [[0xadadfc] + 4 * id_competição]`) [observado 0x57a0c0..0x57a0c9]. Ele guarda a tabela dos times, a **tabela dos jogadores** e os recordes da competição. Não há item próprio no save: o `Comp_Stats` vai dentro do `.tmp` da própria competição, `<nome da competição>_<id>.tmp` (ex.: `Taça de Inglaterra_351.tmp`, `Copa do Brasil_68.tmp`) [observado].

| Quem | Rotina | O que faz |
|---|---|---|
| grava o `.tmp` | `Cup::save_cup_to_disk` 0x5210f0, `League::save_to_temp_file` 0x6891c0 | arquivo em `CM3_TEMP`, que o `save_game_data` empacota (`zadd "*.tmp"`, `MAPA_SAVE.md` §2) [observado] |
| lê o `.tmp` | `Cup::load_from_disk` 0x51fc00, `League::load_from_temp_file` 0x687b10 | constrói o `Comp_Stats` (0x49ee70) e chama `Comp_Stats::load` 0x4ab890 [observado: chamadas em 0x51fd74/0x51fd8e e 0x687ca4/0x687cbe] |
| grava/lê o bloco | `Comp_Stats::save` 0x4ac4e0 / `Comp_Stats::load` 0x4ab890 | campos na tabela abaixo [observado] |

### A.2 O leiaute do `.tmp` da competição (começo)

| Deslocamento no item | Tamanho | Campo | Fonte |
|---|---|---|---|
| 0x00 | 0x50 | nome (`comp + 0x53`) | [observado 0x5210f0, 0x6891c0] |
| 0x50 | i32 | 1 = tem `Comp_Stats` (segue o bloco), 0 = não tem | [observado] |
| 0x54 | i32 | capacidade da tabela de times (`+0x10`) | [observado 0x4ab914] |
| 0x58 | i32 | quantidade de times (`+0x14`) | [observado] |
| 0x5c | 118 × qtd | times (registros de 0x76) | [observado] |
| … | i32 | **capacidade** da tabela de jogadores (`+0x1d`) | [observado 0x4abb3e] |
| … | i32 | **quantidade** de jogadores (`+0x21`) | [observado] |
| … | 45 × qtd | **jogadores** (registros de 0x2d) | [observado] |
| … | 100 | 25 totais i32 da competição (`+0x56a`, somados em 0x4a25f0) | [observado] |
| … | 0x269 + 0x269 | recordes (`+0x26` e `+0x28f`) | [observado] |
| … | i32, i32, i16 | `+0x4f8`, `+0x4fc` (primeiro/último nó no `comp records history.tmp`), `+0x500` | [observado; o significado dos dois i32, inferência pelo `FILE_LLIST::add_node` em 0x4a1c50] |
| … | 0x5c, i32 | `+0x506`, `+0x562` | [observado] |
| … | i32 n, i32, n × 0x1c | tabela de sequências (`+4`, `+8`, `[+0]`) | [observado] |
| … | resto | dados da copa/liga (método `+0x24` da vtable e os campos seguintes) | [observado] |

A carga aloca **capacidade × 45** e lê **quantidade** registros (o mesmo para os times) [observado 0x4abb3e..0x4abc52]: no arquivo, a capacidade tem de ser ≥ quantidade, senão a leitura passa do bloco alocado. O jogo cresce a tabela de 22 em 22 (0x4ad030); no save, toda capacidade é múltiplo de 22 e fica a menos de 22 da quantidade [observado]. Com a versão do save atual (`[0xdd7110]` = −0x3e047) o registro é lido inteiro (45 bytes) e o bloco de 100 bytes existe [observado nos testes de versão de 0x4ab890].

### A.3 O registro do jogador (45 bytes)

| Desl. | Tipo | Campo | Fonte |
|---|---|---|---|
| +0x00 | i32 | id do jogador (`player.dat`, = `staff +0x61` da pessoa); **chave de ordenação** | [observado 0x4ad030; no save, 1018 de 1018 conferem na Copa do Brasil] |
| +0x04 | i32 | id da pessoa (`staff.dat`) | [observado 0x4a0bbc..0x4a0bc7] |
| +0x08 | i32 | **id do clube pelo qual jogou a última partida nesta competição** (o vínculo) | [observado 0x4a0bce..0x4a0bdb: `[clube]` do lado do jogador na partida, `partida+0x1c` ou `+0x20`] |
| +0x0c | u8 | jogos | [observado: +1 por partida; o nome, inferência] |
| +0x0d | u8 | jogos entrando do banco (vaga 11..19 da escalação) | [observado o teste da vaga; o nome, inferência] |
| +0x0e | u8 | gols | [observado: soma `escalação+0x21`; inferência que são gols] |
| +0x1c | u8 | copiado de `partida+0x34` a cada jogo; 0xff ao criar | [observado; o significado, inferência] |
| demais | — | outras estatísticas somadas por `record_match_result` | não decodificadas |

`add_player_entry` 0x4ad030 zera o registro, grava +0 e +4, põe +0x1c = 0xff e marca a tabela como desordenada (`+0x25 = 0xfe`) [observado]. `find_player_entry` 0x4a4a30 ordena pela chave +0 quando `+0x25 ≠ 0xff` e faz busca binária (comparador 0x4ad250: `+0x25 = 0xff` = pela chave) [observado]. O `+0x25` não vai para o arquivo: o construtor põe 0xfe e a carga não mexe, então a primeira busca depois de carregar reordena [observado 0x49ee70, 0x4ab890]. No save a tabela já está ordenada [observado].

### A.4 Quem grava e quem lê

- **Grava:** só `Comp_Stats::record_match_result` 0x49f570 (chamado por `Cup::process_fixture` 0x51a150 e `League::add_result` 0x685d30): para cada jogador que entrou em campo, acha o registro (`find_player_entry`) ou cria (`add_player_entry`) e **regrava +8 com o clube da partida** a cada jogo [observado 0x4a0b66..0x4a0bdb].
- **Zera:** `Comp_Stats::archive_season_and_reset` 0x4a1c50, na virada da temporada da competição: libera a tabela e põe capacidade e quantidade em 0 [observado]. O vínculo dura uma temporada.
- **Lê (a regra):** o `get_ineligibility_code` das regras de transferência da nação. Os únicos que consultam a tabela são estes (os 7 chamadores de 0x4a4a30 são eles, `record_match_result` e a tela `get_player_stat`) [observado]:

| Regra | Rotina | Competições | Código |
|---|---|---|---|
| Inglaterra | 0x57a090 | FA Cup (`[0x9cf74c]`) e Copa da Liga (`[0x9cf750]`) | 1 |
| Escócia | 0x7f65d0 | Copa da Escócia (`[0x9cf75c]`) e Copa da Liga (`[0x9cf760]`) | 1 |
| Irlanda | 0x63c350 | `[0x9cf83c]`, `[0x9cf840]`, `[0x9cf844]` (League Cup, Senior Challenge Cup, Super Cup) | 1 |
| Irlanda do Norte | 0x7937f0 | `[0x9cf8c0]`, `[0x9cf8c4]`, `[0x9cf8c8]` (Irish Cup, Gold Cup, League Cup) | 1 |
| Brasil | 0x43f210 | toda competição, menos a de reservas (`[0x9cf684]`) e os amistosos (0x5c6710) | 4 |

Nomes das globais pelo `data-map` da comunidade. A regra é a mesma nas cinco [observado]:

```
se a partida tem competição C (partida+0x14) e C está na lista
   e a pessoa tem jogador (staff+0x61 ≠ 0)
   e e = find_player_entry(comp(C)+8, jogador) existe
   e a pessoa tem clube (staff+0x39 ≠ 0)
   e e+8 ≠ id do clube atual            → inelegível (código acima)
senão                                   → regra base (TRANSFER_RULES 0x8e8a60)
```

O código sai por `TRANSFER_MANAGER::get_ineligibility_code` 0x8cf5e0 para a escalação automática (`MATCH_MAN::pick_team`), a checagem da escalação (0x7260f0) e `is_player_available_for_match` 0x45cbe0 [observado: chamadores de 0x8cf5e0; o caminho até a regra da nação, pelo mapa da comunidade]. A regra europeia (0x5832e0) não usa esta tabela [observado].

No save de teste: 58 itens de competição, todos com `Comp_Stats`, somando 5.913 registros de jogador; 412 têm +8 diferente do clube atual da pessoa, mas 386 deles são de competições de seleções (o clube da partida é a seleção, e nenhuma regra olha essas). Os vínculos de clube de verdade estão nos estaduais e na Copa do Brasil (regra do Brasil), na Libertadores, na Intertoto e na Coreia. As copas inglesas ainda estão vazias (24/07) [observado].

### A.5 Como liberar o jogador numa competição

**Recomendado: regravar `+8` do registro com o id do clube atual da pessoa (`staff +0x39`)** [o efeito é observado na regra; escolher assim, inferência]. É o que o próprio jogo faz na próxima partida dele por esse clube. O item não muda de tamanho, a ordem continua (a chave é +0) e as estatísticas continuam lá (passam a aparecer no clube novo).

```
item = "<nome>_<id>.tmp";  se i32[0x50] == 0: não há tabela, nada a fazer
p = 0x54; nt = i32[p+4]; p += 8 + 118*nt
nj = i32[p+4]; base = p + 8
para k em 0..nj-1:  e = base + 45*k
    se i32[e+4] == pessoa: i32[e+8] = staff[pessoa].i32[0x39]
```

Alternativa: apagar o registro (deslocar os seguintes, `quantidade − 1`, manter a ordem pela chave e a capacidade ≥ quantidade). O item encolhe 45 bytes, o índice do contêiner tem de ser refeito, e somem os jogos e gols dele na competição. Não há outra cópia do vínculo para acertar: `player stats.dat` e `match_stats` não entram na regra [observado: só os 5 `get_ineligibility_code` leem +8].

Cuidados: um contêiner tipo 4 (RLE) precisa ser recomprimido ao regravar o item; a regra só vale na competição daquele `.tmp`; na temporada nova a tabela é zerada de qualquer jeito [observado].

## B. O valor de quem não é jogador e o byte `+0x5e`

### B.1 O valor (`staff +0x52`, i32)

Fica no registro da pessoa, `staff.dat +0x52` (o "+82 valor" do `PARES_TRANSFERENCIA.md`) [observado]. O jogo **recalcula todo dia**: `game_main_day_loop` chama `CONTRACT_MANAGER::daily_update` 0x4c7ac0 com a passada das pessoas ligada (0x5cd406), e ela regrava `+0x52` de cada pessoa [observado 0x4c7ec5..0x4c8070]:

| Situação (pelo contrato principal, `general.dat` +0 da pessoa → `contract.dat`) | Novo valor |
|---|---|
| sem contrato, ou o clube do contrato não existe | 0 |
| cargo de jogador (11, 12, 13 ou 15, 0x53b620), clube humano | recalcula só se o bit 0x04 de `contrato +0x4e` estiver ligado; depois limita à multa rescisória se houver (0x867b50, `contrato +0x21`) |
| cargo de jogador, clube do computador | dia ímpar: liga o bit 0x04, recalcula e limita à multa; dia par: não mexe |
| **outro cargo (não jogador)** | **todo dia:** liga o bit 0x04 e `valor = contract_manager_calc_player_value` 0x4d0c60 |

Para a pessoa sem jogador (`staff +0x61` nulo), 0x4d0c60 devolve 0 sem clube e, com clube, a compensação `transfer_manager_calc_compensation_fee` 0x8d0780 (com "ignorar regras novas" = 1) [observado]. Com `+0x69` (não jogador) presente, a conta é [observado 0x8d07cf..0x8d0849]:

```
c = contrato principal (CONTRACT_MANAGER::get_contract 0x4cf5a0)
se c nulo ou (c+0x35 & 0x3f) != 2:  valor = 0
dias  = Date::days_since(c+0x2d fim, hoje)          ; dias até o fim do contrato
valor = trunc(dias * salário(c+0x0c) / 7)            ; os salários que faltam
valor = limitar(valor, 1.000, 1.000.000)
```

No arquivo: `contract.dat` registro de 80 bytes, `+12` salário, `+45` fim (`i16 dia, i16 ano`), `+53` tipo (`PARES_TRANSFERENCIA.md`). **Conferido no save de teste:** das 11.179 pessoas com `+0x61 = −1` e `+0x69 ≠ −1`, 11.176 batem com a conta usando hoje = 204/2001 (o último dia passado antes da data gravada, 205); as 3 que não batem entraram ou perderam o contrato no dia 204/205 [observado]. Com hoje = 205 batem só 5.858.

Para gravar sem quebrar: o `+0x52` de não jogador é derivado e o jogo o sobrescreve no dia seguinte [observado]. Quem quiser outro valor muda o contrato (salário ou fim); ao editar o contrato, regravar `+0x52` com a conta acima mantém o save coerente até o próximo dia [inferência]. Pessoa sem `+0x61` e sem `+0x69` com contrato faria 0x8d0780 cair num caminho que volta a chamar 0x4d0c60 [observado 0x8d08f6..0x8d098a]: não criar essa combinação [inferência: repetição sem fim].

O valor de jogador (`+0x61` presente) é a conta de 0x4d0c60 por reputação, posição, idade e nação (resumo em `ENDERECOS_COMUNIDADE.md`); também é refeito pela passada diária (tabela acima).

### B.2 O byte `+0x5e` (PlayingSquad)

Dois pares de bits independentes [observado `staff_set_squad_flags` 0x540980: valor com bits 0–1 troca só os bits 0–1 (mantém `& 0x0c`); senão troca só os bits 2–3 (mantém `& 0x03`)]:

| Bit | Valor | Significado | Fonte |
|---|---|---|---|
| 0 | 0x01 | **time principal** do clube | [observado `get_squad_placement` 0x4cbb60: com clube que tem reservas, devolve 2 = principal se o bit 0 está ligado, 1 = reservas se não] |
| 1 | 0x02 | **reservas** do clube | [observado: a escalação principal (`pick_team` 0x6dcaa8) pula quem tem 0x02; o perfil (0x87090a) troca 0x02 por 0x01 em "passar ao principal"] |
| 2 | 0x04 | convocado para a seleção em `staff +0x24` (principal ou sub-21, conforme o time de `+0x24`) | [observado `national_teams_player_in_squad` 0x76da80: principal = `+0x24` é o time de 0x53b3d0 e bit 0x04; sub-21 = time de 0x53b4f0 e bit 0x04] |
| 3 | 0x08 | convocado para a seleção B | [observado 0x76da80 (vaga 1 = time principal e bit 0x08), `get_player_squad_slot` 0x76e070] |
| 4–7 | — | não usados | [observado: nenhum dos 62 leitores testa 0x10..0x80; alguns copiam o byte inteiro (carga, `Staff::operator=`, sorteio dos registros)] |

O clube não tem lista separada de reservas: o elenco do `club.dat` são 50 vagas, e principal/reserva é só este byte [inferência pelo leiaute do `club.dat` em `PARES_TRANSFERENCIA.md`].

Quem grava: `staff_set_squad_flags` (16 chamadas, sempre com 1, 2, ou o valor da seleção), as rotinas das seleções (0x76c120, 0x76d880, 0x775c92, 0x776366, que limpam os bits 2–3), a carga do banco e o `database_trim_staff_tables` (0x52651b, põe 2) [observado]. Na transferência (`move_player_to_club`, 0x8ca7d9..0x8ca81f): clube do computador, ou clube sem reservas, ou clube com par de time B → 0x01; clube humano com reservas → 0x01 se o status no elenco do contrato (`contrato +0x4f >> 4`) for 1..4, senão 0x02 [observado; nomes dos status, inferência].

A carga do banco (`load_database`, 0x52d29e..0x52d2c1) corrige 0x02 em clube que não pode ter reservas (`database_club_has_reserve_squad` 0x542740 falso) para 0x01 [observado; que esse trecho rode também ao carregar um save, inferência].

No save de teste [observado]: jogadores com clube: 0x01 (26.368), 0x02 (4.362), 0x05 (5.511 convocados), 0x06 (316); não jogadores quase todos 0x00/0x01 (o byte não tem papel para eles); 0x08 não aparece (nenhuma seleção B convocada).

**Para gravar sem quebrar** [inferência sobre o observado]:

1. Mexer só nos bits 0–1: `b = (b & 0x0c) | novo`, com novo = 0x01 (principal) ou 0x02 (reservas). Nunca 0x03, e 0x02 só se o clube pode ter reservas (0x542740: fora pares de time B, clubes espanhóis, alemães e portugueses e algumas seleções).
2. Não mexer nos bits 2–3: eles andam com `staff +0x24` e as listas de `national_teams.dat`. Para tirar alguém da seleção, o caminho do jogo é `NationalTeams::remove_player_from_squad` 0x7762f0 (limpa `+0x24`, os bits e as vagas).
3. A escalação do clube (`club.dat`, 20 vagas) não deve ter reserva no time principal: `pick_team` refaz a do computador; a do humano é dele [inferência].

## Pendências

- Nomes exatos dos campos +0x0d..+0x2c do registro de jogador da competição e do registro de time (118 bytes): não decodificados.
- `database_club_has_reserve_squad` 0x542740 não foi lida por dentro (descrição pelo mapa da comunidade).
- O trecho de 0x52d29e na carga de um save, e não só do banco novo: conferir com uma sonda quando a carga estiver em Rust.
