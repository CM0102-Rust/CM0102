# Mapa do bloco do banco de dados

*[English version](MAPA_DATABASE.en.md).*

Levantamento para as levas do banco de dados (ordem combinada: match_screens/match_day, save, **Database**, competições). Só leitura: desmontagem do `cm0102.exe` 3.9.68, o mapa da comunidade (`referencias/cm0102-function-map`, `map/Database.cpp.md`, `map/Db_files.cpp.md`, `map/index.cpp.md` e `data-map/Database.cpp.md`), os inventários do Bosman Edition (`referencias/cm0102-bosman-edition/docs/research/2026-10-13-database-inventory.md`, `2026-10-14-start-new-game-inventory.md`, `2026-10-14-game-creation-inventory.md`: pseudocódigo conferido na desmontagem, emulação do original no Unicorn), `docs/mapa_funcoes.csv` e os projetos irmãos (§3). O bloco do save está em [MAPA_SAVE.md](MAPA_SAVE.md).

Tamanhos em bytes, medidos do início ao último byte antes do enchimento (o "size" do mapa da comunidade não é em bytes). "Rust" = há `Prologo { endereco: ... }` em `rust/src/motor.rs`. Colunas de efeito: **SEH** (quadro com `fs:[0]`), **aloca** (`malloc` 0x944e46, `new` 0x944cf1, `realloc` 0x945667, `GlobalAlloc`), **arquivo** (`fopen`/`fread`/`fwrite` da CRT ou `zopen`/`zread`/`zwrite`), **sorteio** (0x90cfc0 ou `rand` 0x946a04), **caixa** (0x5e8290: as `assert` e os erros abrem caixa e esperam clique).

**Números:** `Database.cpp` tem 181 rotinas reais (184 entradas, 3 pedaços falsos), `Db_files.cpp` 40, `index.cpp` 11 (21 entradas, 10 pedaços). Em Rust: **19** (11 de `Database.cpp` mais 8 das preferências, 0x547790..0x547d90). Nenhuma de `Db_files.cpp` nem de `index.cpp`.

**Carreira automática:** a cobertura de 17/10 (`docs/cobertura_simulacao.txt`) não instrumentou `Database.cpp` nem `Db_files.cpp`; de `index.cpp` registrou os casadores de chaves (13.640 chamadas, uma por clube). A criação pelo `auto_setup.cfg` passa por `create_game_from_setup_file` 0x827110 → `load_database_from_files` (0x827430/0x8275d3, quando a tabela de nações está vazia) → `create_game` 0x828110 (0x82790b): toda a carga e a criação rodam **uma vez**. A coluna "Carreira" abaixo vem desse caminho; as consultas do dia a dia ficam "a medir" (`python engenharia-reversa\cobertura.py Database.cpp Db_files.cpp` é o primeiro passo das levas).

## 1. As funções

### cm3/code/Db_files.cpp (os leitores e gravadores de tabela)

Todos têm a forma `int f(void *stream, int use_zip, void *registros, entrada_do_index *e)` (leitores) ou `(stream, use_zip, registros, quantidade)` (gravadores): `use_zip` escolhe `zread`/`zwrite` (save) ou `fread`/`fwrite` (pasta `Data`). O leitor confere a versão da entrada (+0x3f), que é fixa por tabela (228ª leva, pelo binário): 2 em nação, clubes, competições, staff_comp e jogador/não jogador; 1 em continentes, estádios, staff v1, históricos, nomes, cores e árbitros; 1 ou 2 em cidades. Outra versão abre a caixa "Error" e o jogo sai. Depois o leitor lê `e->quantidade` registros de uma vez. Nenhum tem SEH.

| Endereço | Nome (comunidade) | Bytes | Tabela (id do index) | Registro | Rust | Carreira |
|---|---|---|---|---|---|---|
| 0x54d580 | `db_files_open_for_read` | 43 | monta o caminho em 0xde1f64 e `fopen` "rb" | — | não | sim (24×) |
| 0x54d5b0 | `db_files_open_for_write` | 43 | o mesmo, "wb" | — | não | não |
| 0x54d5e0 | `db_files_read_continents` | 241 | continent (3) | 198 | não | sim |
| 0x54d740 | `db_files_read_nations` | 243 | nation (4) | 290 | não | sim |
| 0x54d8a0 | `db_files_read_cities` | 241 | city (21) | 56 | não | sim |
| 0x54da00 | `db_files_read_stadiums` | 235 | stadium (5) | 78 | não | sim |
| 0x54db50 | `db_files_read_clubs` | 314 | club (0), nat_club (1); depois troca +0x80 = 0 por 1 | 581 | não | sim (2×) |
| 0x54dc90 | `db_files_read_clubs_v1` | 2365 | club versão 1, campo a campo | 581 | não | **não** (a base de fábrica é versão 2) |
| 0x54e630 | `db_files_read_staff` | 237 | staff (6) versão 2 | 110 | não | não (save e bases como a CLZ) |
| 0x54e780 | `db_files_read_staff_v1` | 241 | staff (6) versão 1, para a tabela temporária 0xae2404 | 157 | não | sim |
| 0x54e880 | `db_files_read_players` ¹ | 283 | id 9 → 0xae23c0 | 68 | não | sim |
| 0x54e9a0 | `db_files_read_players_v1` ¹ | 1583 | id 9 versão 1 | 68 | não | **não** |
| 0x54f030 | `db_files_read_non_players` ¹ | 283 | id 10 → 0xae23c4 | 70 | não | sim |
| 0x54f150 | `db_files_read_non_players_v1` ¹ | 2652 | id 10 versão 1 | 70 | não | **não** |
| 0x54fc10 | `db_files_read_staff_extra` | 235 | preferências (22) | 52 | não | não (só bases versão 2 e save) |
| 0x54fd60 | `db_files_read_staff_history` | 235 | staff_history (17) | 17 | não | sim |
| 0x54feb0 | `db_files_read_comps` | 283 | club_comp (12), nation_comp (16) | 107 | não | sim (2×) |
| 0x54ffd0 | `db_files_read_comps_v1` | 510 | comps versão 1 | 107 | não | **não** |
| 0x550230 | `db_files_read_comp_history` | 235 | club_comp_history (19), nation_comp_history (20) | 26 | não | sim (2×) |
| 0x550380 | `db_files_read_staff_comps` | 237 | staff_comp (11) | 101 | não | sim |
| 0x5504d0 | `db_files_read_staff_comp_history` | 235 | staff_comp_history (18) | 58 | não | sim |
| 0x5505c0 | `db_files_read_officials` | 235 | officials (7) | 43 | não | sim |
| 0x550710 | `db_files_read_colours` | 235 | colour (2) | 58 | não | sim |
| 0x550860 | `db_files_read_names` | 235 | first/second/common_names (13, 14, 15) | 60 | não | sim (3×) |
| 0x54d6e0, 0x54d840, 0x54d9a0, 0x54daf0 | `db_files_write_continents/nations/cities/stadiums` | 87, 87, 81, 81 | gravadores | 198, 290, 56, 78 | não | não |
| 0x54e5d0, 0x54e720, 0x54efd0, 0x54fbb0 | `db_files_write_clubs/staff/players/non_players` | 87, 81, 81, 81 | gravadores | 581, 110, 68 ¹, 70 ¹ | não | não |
| 0x54fd00, 0x54fe50, 0x5501d0, 0x550320 | `db_files_write_staff_extra/staff_history/comps/comp_history` | 81 cada | gravadores | 52, 17, 107, 26 | não | não |
| 0x550470, 0x5506b0, 0x550800, 0x550950 | `db_files_write_staff_comps/officials/records_0x3a/or_read_records_0x3c` | 81 cada | gravadores (0x3a: cores e histórico de comps de pessoal; 0x3c: nomes, grava ou lê) | 101, 43, 58, 60 | não | não |

¹ **O mapa da comunidade troca jogador e não jogador.** Pelo `resolve_database_indexes` (pessoa +0x61 → 0xae23c4 de 70 bytes; +0x69 → 0xae23c0 de 68), pelo inventário de criação do Bosman e pelo CMScout (`player.dat` 70, `nonplayer.dat` 68): **id 9 = não jogador (68 bytes), id 10 = jogador (70)**. Bate com `docs/CAMPOS.md` (habilidade atual do jogador em +0x05, curto depois de um byte em +4). Os nomes das rotinas ficam como estão no mapa; as levas usam o significado certo.

### cm3/code/Database.cpp: carga, gravação e liberação

| Endereço | Nome | Bytes | O que faz | Quem chama | Efeitos | Rust | Carreira |
|---|---|---|---|---|---|---|---|
| 0x524d50 | `load_database_from_files(pasta, compartilhado)` | 21 | `load_database(pasta, comp, 0)` | `setup_start_new_game` 0x81cea5, `create_game_from_setup_file` (2×), `create_game` 0x8282a7 | — | não | **sim** |
| 0x524d10 | `load_database_from_stream` | 21 | `load_database(…, 1)` | `load_game_data` 0x82abf2 | — | não | não |
| 0x524d30 | `save_database_for_game` | 26 | `save_database(…, 1, …)` | `save_game_data` 0x82e6d6 | — | não | não |
| 0x528540 | `load_database(pasta, compartilhado, do_save)` | 21903 | lê `index.dat` e as tabelas, converte as pessoas de 157 para 110 bytes, troca índices por ponteiros, corrige pessoas, acha as chaves (§2) | os dois repasses | aloca (bloco único `GlobalAlloc` no modo compartilhado, `malloc` por tabela no outro); arquivo; sorteio (só em dado ruim); caixa; barra de progresso | não | **sim** (1×) |
| 0x52dad0 | `save_database(pasta, no_save, a3, a4)` | 15053 | troca os ponteiros de volta por índices (o id em +0 do alvo, nulo → −1), grava cada tabela e o `index.dat` (com a cauda), empacota os `*.lng` (`zadd`) e volta os ponteiros (`resolve_database_indexes`, 5 chamadas) | `save_database_for_game` | arquivo; caixa ("%s table valid, but no entry in %s") | não | não |
| 0x5315a0 | `resolve_database_indexes(leniente)` | 6226 | troca cada campo de id pelo ponteiro ao registro; negativo ou fora da faixa → nulo (com registro no log ou caixa para pessoa → jogador/preferências/não jogador). Tabela completa dos campos: inventário de criação do Bosman §2d | `load_database`, `save_database` | caixa | não | **sim** |
| 0x528080 | `database_free_all_tables` | 1201 | libera tudo (`free` ou `GlobalFree` pelo modo), zera as contagens, apaga os `.lng` de `CM3_TEMP` | `load_database`, `load_game_data`, `free_game_data` | libera; apaga arquivos | não | **sim** |
| 0x527ee0 | `database_free_staff_history_table` | 57 | libera o histórico de pessoas | `PlayerStats::PlayerStats` 0x7c0ccd (ele passa a viver no `player stats history.tmp`) | libera | não | **sim** |
| 0x527f20 | `database_free_staff_comp_history_table` | 57 | libera o histórico de prêmios | `AWARD_MANAGER::AWARD_MANAGER` 0x413360 | libera | não | **sim** |
| 0x527f60 | `database_grow_comp_history_table(n)` | 287 | cresce o histórico de competições (26 bytes) e move o ponteiro da parte das seleções | `comp_util_add_history_record` 0x4afd2c | aloca | não | sim (fim de competição) |

### cm3/code/Database.cpp: criação do jogo (uma vez)

Ordem em `create_game` 0x828110 (inventário de criação do Bosman §3a, conferido nos `call`):

| Endereço | Nome | Bytes | O que faz | Efeitos | Rust | Carreira |
|---|---|---|---|---|---|---|
| 0x541e60 | `database_trim_senior_and_b_team_squads` | 299 | clube +0x244 = tem par B; elenco conjunto A+B cortado a 50 (qsort 0x5402f0), mantendo 3 goleiros | `qsort` | não | **sim** |
| 0x539750 | `database_build_nation_name_index` | 908 | índice por nação das tabelas de nomes (primeira linha, quantidade, soma dos pesos), em 0xae2338, 0x24 bytes por nação | aloca; caixa | não | **sim** |
| 0x539ae0 | `database_prune_comp_history_table` | 162 | só as linhas de nações escolhidas; `realloc` e `qsort` (0x540780) | aloca | não | **sim** |
| 0x524d70 | `database_trim_staff_tables(tamanho)` | 6730 ² | corta pessoas, jogadores, não jogadores e preferências pela opção de tamanho da base (`[0xae3138]+4 & 7`), mantendo os melhores por nação; `realloc` no lugar com `assert` de mesmo endereço; copia 10 não jogadores sorteados nas 10 últimas linhas | SEH; aloca; sorteio (0x90cfc0 15× e `rand` direto); caixa | não | **sim** |
| 0x527240 | `database_generate_extra_non_playing_staff` | 3232 | cria pessoal extra para clubes sem ele (nomes da nação, quantidade pela reputação) | sorteio; caixa | não | **sim** (chamada pela 0x524d70) |
| 0x5267c0 | `database_trim_club_table(tamanho)` | 2676 | corta os clubes pela mesma opção, mantendo os referidos por pessoas e históricos | aloca; caixa | não | **sim** |
| 0x544970 | `database_merge_b_team_staff` | 756 | elenco do B cortado a 50 conjuntos; comissão técnica do B vai para o A | — | não | **sim** |
| 0x53db40 | `database_sort_clubs_by_reputation(refazer)` | 89 | ordena o vetor de ids de clube 0xae2d1c por reputação (`qsort` 0x540020) | — | não | **sim** (criação, `load_game_data` e `game_main_day_loop`) |
| 0x535a80 | `database_player_initialisation(data)` | 12602 | prepara cada jogador e pessoa do jogo novo: atributos, reputações, contratos, valores; barra "Player Initialisation" | sorteio; caixa; barra | não | **sim** |
| 0x532e00 | `database_apply_fake_players` | 332 | "Use Real Players: No": embaralha jogadores, nomes sorteados da nação, convocações | sorteio | não | não (opção desligada) |
| 0x533080 | `database_shuffle_player_records` | 1378 | troca registros de pares sorteados (mantém id, clube, habilidade) | SEH; sorteio | não | não |
| 0x533710 | `database_shuffle_squad_player_attributes` | 647 | troca habilidade e físicos entre companheiros parecidos | sorteio | não | não |
| 0x53a610, 0x53aa10 | `database_player_to_attribute_vector`, `…vector_to_player` | 661, 378 | registro de jogador ↔ vetor de 63 células (valor, peso) | — | não | **sim** |
| 0x53a8b0, 0x53a980 | `database_non_player_to_attribute_vector`, `…vector_to_non_player` | 208, 134 | o mesmo para os bytes 0x0e..0x22 do não jogador | — | não | **sim** |
| 0x538bc0, 0x538ec0, 0x539490 | `staff_normalise_player_attributes`, `…_physical_attributes`, `…_non_player_attributes` | 758, 1488, 701 | reequilibra o vetor de atributos (também na atualização do treino) | — | não | **sim** |

² 0x524d70 vai até 0x5267c0; o mapa da comunidade corta em 0x524f50 ("pedaço falso", um código de país lido como ponteiro).

### cm3/code/index.cpp (as chaves)

As **chaves** são 1.284 globais (0x9cf178..0x9d0524, mais 26 times amadores alemães em 0xb63c9c..0xb63d00; lista e usos em `CHAVES_DO_EXE.md`) com o índice de registros que o código usa pelo nome: cores ("Black", "Red 1"…), nações, competições de clube e de seleção, continentes, competições de pessoal (prêmios) e clubes. Cada casador é uma cadeia `if (!_stricmp(nome, "Argentina")) chave = linha; else if …` (0x951c50).

| Endereço | Nome | Bytes | O que faz | Efeitos | Rust | Carreira |
|---|---|---|---|---|---|---|
| 0x601a40 | `set_key_ids` | 41022 ³ | põe −1 em todas as chaves; casa cores (em linha), nações, competições de clube (até `quantidade − 127`), continentes, prêmios e clubes; com `[0x9b5984] ≠ 0` lê antes o `CM3_TEMP\eng.lng`; cada chave que ficar −1: "Unable to find the %s index" e caixa | arquivo (só com nomes traduzidos); caixa | não | **sim** (1×) |
| 0x60d810 | `match_nation_key_id(nome, linha)` | 6071 | cadeia das nações | — | não | **sim** (213×) |
| 0x60efd0 | `match_club_comp_key_id` | 9737 | cadeia das competições | — | não | **sim** (452×) |
| 0x6115e0 | `match_staff_comp_key_id` | 4494 | cadeia dos prêmios | — | não | **sim** (543×) |
| 0x6146b0 | `match_club_key_id` | 10341 ³ | cadeia dos clubes | — | não | **sim** (13.640×) |
| 0x60ba80 | `remap_club_key_id(antigo, novo)` | 7416 | grava a linha nova na chave de clube que guardava o id antigo | — | não | **sim** (13.640×) |
| 0x60d780 | `report_missing_key_id` | 129 | registra e abre a caixa de chave faltando (linha 0x16b1) | caixa | não | não (base de fábrica completa) |
| 0x616f20 | `get_region_name(região, forma)` | 2132 ³ | nome ou adjetivo localizado de uma das 14 regiões | — | não | a medir (telas) |
| 0x617780, 0x6177f0 | `get_region_article_form`, `get_continent_adjective` | 104, 629 | artigo italiano; adjetivo do continente | — | não | a medir (telas) |
| 0x6181d0 | `nation_get_nationality_adjective` | 26014 | adjetivo de nacionalidade por idioma, gênero e número | — | não | a medir (telas, notícias) |

³ Medidos até a próxima rotina real: o mapa da comunidade parte a 0x601a40 em 0x606060, a 0x6146b0 em cinco pedaços e a 0x616f20 em quatro.

### cm3/code/Database.cpp: idioma dos nomes (`.lng`)

| Endereço | Nome | Bytes | O que faz | Efeitos | Rust | Carreira |
|---|---|---|---|---|---|---|
| 0x535920 | `database_get_names_language` | 6 | `[0x9b5984]` | — | **sim** | sim |
| 0x535930 | `database_language_file_exists(idioma)` | 329 | `CM3_TEMP\<idioma>.lng` existe? (0, 1, 11, 12 → eng) | arquivo | não | **sim** |
| 0x5339a0 | `database_load_language_names(idioma, forçar)` | 8052 ⁴ | lê os nomes traduzidos de clubes, nações, competições, cidades e estádios do `.lng` para as tabelas | SEH; aloca; arquivo; caixa; barra | não | não (idioma 0) |
| 0x544fc0 | `eng_lng_load_tables` | 1066 ⁴ | lê `CM3_TEMP\eng.lng` e confere as 8 contagens contra as tabelas | aloca; arquivo; caixa | não | não |
| 0x544f30 | `eng_lng_free_tables` | 133 | libera as 8 tabelas em inglês | libera | não | não |
| 0x5453f0, 0x5454c0, 0x5455a0, 0x5456a0, 0x545770, 0x545880, 0x545990 | `nation_get_english_name`, `…three_letter_name`, `comp_…`, `staff_comp_…`, `club_get_english_short_name`, `club_get_english_name`, `stadium_…` | 207, 209, 246, 206, 259, 259, 205 | o nome em inglês (carrega o `eng.lng` no primeiro uso) | SEH; aloca | não | não (idioma 0) |
| 0x53cee0 | `database_get_language_name` | 2136 | nome do idioma no buffer 0xae21d4 | — | não | a medir |

⁴ Com os pedaços 0x535552 (0x5339a0) e 0x545350 (0x544fc0).

### cm3/code/Database.cpp: consultas e textos (as demais)

Não são o "banco" no sentido de carga: são consultas sobre as tabelas, chamadas pela simulação e pelas telas, e seguem pelas filas de sempre (`candidatos.py --simulacao`, telas). Bytes entre parênteses.

- **Seleções e pares A/B** (Rust nas marcadas ✓): `database_get_national_team` 0x53b3d0 (282) ✓, `…_u21_team` 0x53b4f0 (280) ✓, `database_is_national_team` 0x53b900 (159) ✓, `club_is_national_team` 0x5409b0 (156) ✓, `club_get_linked_b_team` 0x540a50 (5132) ✓, `club_is_b_team` 0x5448c0 (162), `club_get_national_team_name` 0x542100 (1588), `database_club_has_reserve_squad` 0x542740 (345), `club_can_be_managed` 0x544820 (151), `club_is_in_season` 0x547770 (23).
- **Elenco e cargos do clube:** `database_club_has_room_for_job` 0x53b9a0 (868), `database_add_staff_to_club` 0x53bd10 (1148), `database_remove_staff_from_club` 0x53c190 (1020; também na carga), `database_count_club_squad` 0x53c590 (820), `…_coaches` 0x53c8d0 (224), `…_physios` 0x53c9b0 (224), `database_get_club_manager` 0x53ca90 (237), `…_assistant` 0x53cb80 (167), `…_coaches` 0x53cc30 (293), `…_scouts` 0x53cd60 (293), `database_collect_club_squad` 0x541f90 (360), `club_collect_non_playing_staff` 0x544c70 (696), `staff_set_squad_flags` 0x540980 (40), `database_is_not_plain_player_job` 0x53b610 (13) ✓, `database_is_player_job` 0x53b620 (29) ✓, `database_get_contract_rating_modifier` 0x53ce90 (79) ✓, `club_count_squad_with_position_mask` 0x548150 (66), `staff_get_natural_position_mask` 0x5481a0 (332).
- **Atributos e reputação:** `staff_get_player_attribute` 0x5431e0 (160), `staff_is_player_attribute_visible` 0x543280 (440), `…_for_viewer` 0x543440 (256), `staff_compute_player_attribute` 0x543540 (1488), `staff_get_non_player_attribute` 0x543f30 (160), `staff_compute_non_player_attribute` 0x543fd0 (788), `player_attribute_rating_with_scaled_ability` 0x5442f0 (85), `player_attribute_rating` 0x544380 (135), `non_player_attribute_rating` 0x544410 (135) ✓, `database_get_staff_reputation_for_viewer` 0x5407e0 (224), `database_get_reputation_scope` 0x5408c0 (184), `club_reputation_for_staff` 0x545e90 (797), `staff_adjust_morale` 0x544710 (265, sorteio), `database_get_staff_age` 0x53ab90 (171), `staff_get_player_strength_flags` 0x546ab0 (1567, sorteio), `…_weakness_flags` 0x5470d0 (1684, sorteio).
- **Preferências e opiniões:** 0x547790, 0x5477d0, 0x547810, 0x547840, 0x547af0, 0x547d30, 0x547d60, 0x547d90 ✓; faltam `club_add_favoured_staff` 0x547870 (208), `club_add_disliked_staff` 0x547940 (208), `staff_add_favoured_staff` 0x547a10 (211), `club_remove_favoured_staff` 0x547bd0 (63), `staff_remove_favoured_staff` 0x547c10 (52), `staff_remove_disliked_staff` 0x547c50 (52), `staff_count_disliked_staff_at_club` 0x547c90 (70), `…favoured…` 0x547ce0 (70), `staff_add_disliked_club` 0x547e70 (213), `staff_remove_favoured_club` 0x547f50 (52), `staff_fade_staff_relationships` 0x547f90 (243, sorteio), `staff_get_favoured_staff` 0x548090 (94), `…disliked…` 0x5480f0 (94), `staff_is_best_favoured_candidate` 0x548810 (1422), `staff_pick_favoured_team_mate` 0x548da0 (856), `staff_clear_opinion_links` 0x549c90 (82), `staff_add_opinion_link` 0x549cf0 (112), `staff_remove_opinion_link` 0x549d60 (101), `staff_get_opinion_link_staff` 0x549dd0 (113).
- **Força do elenco (IA):** 0x549100 (253), 0x549200 (187), 0x5492c0 (253), 0x5493c0 (166), 0x549470 (87), 0x5494d0, 0x5495b0, 0x549690, 0x549770 (216 cada), 0x549850, 0x549940 (238 cada), `club_squad_strength_band` 0x549a30 (506).
- **Contrato e listas:** `staff_is_transfer_listed` 0x5444a0 (128), `…_or_requested` 0x544520 (98), `staff_is_loan_listed` 0x544590 (98), `staff_is_listed_by_request` 0x544600 (98).
- **Regras por nação ou clube (nomes fixos):** `database_is_staff_eu_national` 0x53d740 (144), `database_club_nation_is_eec` 0x53d7d0 (26), `database_is_basque_club` 0x53d7f0 (264), `database_is_melanesian_nation` 0x53d900 (57), `…polynesian…` 0x53d940 (57), `database_is_staff_foreign` 0x53d980 (436), `nation_allows_free_transfers` 0x544670 (85), `nation_is_uk_or_ireland` 0x5446d0 (57), `club_nation_is_not_argentina` 0x5487b0 (32), `club_nation_not_argentina_france_croatia` 0x5487d0 (49), `club_is_in_major_nation` 0x549e50 (73), `nation_is_major_nation` 0x549ea0 (66).
- **Nomes:** `database_pick_first_name_by_weight` 0x539b90 (207), `…_for_nation` 0x539c60 (1152), `database_pick_second_name_by_weight` 0x53a0e0 (208), `…_for_nation` 0x53a1b0 (1016), `database_get_empty_common_name` 0x53a5b0 (93) ✓, `Staff::operator=` 0x5335f0 (286), `database_staff_matches_names` 0x53e990 (282), `database_format_staff_name` 0x53dba0 (1428; também na carga, só para o log), `database_format_name_from_ids` 0x53e140 (408), `…_from_strings` 0x53e2e0 (1708), `database_format_official_name` 0x53eab0 (916).
- **Textos de tela:** `database_get_coaching_job_name` 0x53ac40 (180), `database_get_job_name` 0x53ad00 (1740), `database_format_player_positions` 0x53ee50 (4653), `database_format_years_between_dates` 0x540340 (325), `database_get_player_attribute_name` 0x5428a0 (2356 com 0x542f48), `database_get_non_player_attribute_name` 0x543b10 (1048), `write_position_name` 0x5461b0 (1007), `write_player_description` 0x5465a0 (1291), `position_mask_get_text` 0x5482f0 (1204), `get_adjective_form_index` 0x545a60 (852), `get_regions_sorted_by_name` 0x545dc0 (200), `compare_region_names` 0x545e00 (136).
- **Cores:** `database_get_club_kit_colours` 0x53b640 (373), `database_get_comp_colours` 0x53b7c0 (309), `database_colours_clash` 0x540490 (139), `database_get_colour_family` 0x540520 (604).
- **Comparadores de `qsort`:** 0x53fd40 e 0x53fe50 (265 cada: latitude e longitude da cidade do estádio), 0x53ff60 (124), 0x53ffe0 (61), 0x540020 (93), 0x540080 (489, SEH), 0x540130 (313, SEH), 0x540270 (197), 0x5402f0 (69), 0x540780 (82), 0x549c30 (82).
- **Inicializador estático:** `init_transfer_rule_change_date` 0x524cd0 (33: 31/08/2001) e o destrutor vazio 0x524d00. Rodam antes do jogo; ficam de fora.

## 2. Os fluxos

### Jogo novo (`load_database(pasta Data, 1, 0)`)

Caminho: Start New Game (`setup_start_new_game` 0x81ce00) ou `create_game_from_setup_file` 0x827110 → 0x524d50 → 0x528540. Retorna 1, ou 0 se uma leitura vier curta.

1. Pasta `CM3_TEMP`; `database_free_all_tables`; barra "Loading database" (46 passos: 0, 1, 22 do dimensionamento, 22 da leitura, 1 final).
2. Copia os 10 `*.lng` de `Data` para `CM3_TEMP` (0x4c0aa0/0x4c0b50 e 0x90cc50); sem o `.lng` do idioma das preferências, idioma 0.
3. `index.dat`: dois inteiros (0xae2344, 0xae2348; 0 e 0 na base), 22 entradas de 67 bytes em 0xae1a80 (nome 51, id +0x33, quantidade +0x37, posição +0x3b, versão +0x3f) e **um byte a mais**, que a base não tem (fica 0). Com esse byte ≠ 0 (o `index.dat` de dentro do save), lê 0x1cc bytes, uma lista para 0xae23fc (clube → linha do nome em inglês) e o idioma dos nomes 0x9b5984.
4. Dimensionamento: soma as quantidades por id (`switch`) e reserva linhas: pessoas + 16 técnicos humanos + 23.000 reservas (com o byte do save, sem as 23.000), não jogadores + 16 + 23.000 (idem), mais 1.325 linhas alocadas e não contadas no fim das pessoas e dos jogadores, competições de clube + 127 (os amistosos), cada tabela de nomes + 16. No modo compartilhado (o do jogo novo), **um bloco só de `GlobalAlloc`**; senão um `malloc` por tabela. Tabela de pessoas de 157 bytes temporária em 0xae2404.
5. As 127 competições de reserva, as 23.000 pessoas de reserva (tipo +0x5f = 6, data padrão 10/02/1972) e as 16 dos técnicos humanos (nomes vazios no fim das três tabelas de nomes).
6. Leitura na ordem do índice, reabrindo o arquivo e indo à posição (`fseek`) a cada entrada (o `staff.dat` abre quatro vezes).
7. Conversão das pessoas 157 → 110 e montagem das preferências (52 bytes: id + 12 inteiros do arquivo em +0x61..+0x8d); libera a temporária.
8. `resolve_database_indexes(0)`; apaga `duff.out`.
9. Passe das pessoas (todas menos as 16 últimas e as de tipo 6): tira de clube extinto, corrige cargo 7 → 6, nação ausente (do clube, da seleção ou **sorteada**), dado de jogador/não jogador ausente, nome nulo → primeiro nome comum vazio (para os três nomes: erro do original, a copiar), seleção sem cargo.
10. Seleções sub-21 (a segunda metade de `nat_club`) com metade da reputação da principal (+0x80); divisão chamada "Reserve" → nula.
11. `set_key_ids`.

| # | Arquivo (base de fábrica) | id | Registros | Registro (arquivo / memória) | Ponteiro [quantidade] |
|---|---|---|---|---|---|
| 0 | club.dat | 0 | 13.214 | 581 | 0xae23b4 [0xae235c = 13.640 com nat_club] |
| 1 | nat_club.dat | 1 | 426 | 581 | 0xae23b8 (no bloco dos clubes) [0xae2360] |
| 2 | colour.dat | 2 | 34 | 58 | 0xae23ec [0xae2394] |
| 3 | continent.dat | 3 | 6 | 198 | 0xae23a4 [0xae234c] |
| 4 | nation.dat | 4 | 213 | 290 | 0xae23a8 [0xae2350] |
| 5 | officials.dat | 7 | 4.567 | 43 | 0xae23e8 [0xae2390] |
| 6 | stadium.dat | 5 | 8.917 | 78 | 0xae23b0 [0xae2358] |
| 7 | staff.dat (pos. 0) | 6 | 169.342 | 157 / 110 | 0xae23bc [0xae2364 = 192.358] |
| 8 | staff.dat | 8 | 0 | — | (sem caso no `switch`) |
| 9 | staff.dat (pos. 26.586.694) | 9 | 30.865 | 68 (não jogador) | 0xae23c0 [0xae2368 = 53.881] |
| 10 | staff.dat (pos. 28.685.514) | 10 | 139.769 | 70 (jogador) | 0xae23c4 [0xae236c] |
| 11 | staff_comp.dat | 11 | 543 | 101 | 0xae23e0 [0xae2388] |
| 12 | city.dat | 21 | 6.682 | 56 | 0xae23ac [0xae2354] |
| 13 | club_comp.dat | 12 | 433 | 107 | 0xae23d0 [0xae2378 = 579] |
| 14 | nation_comp.dat | 16 | 19 | 107 | 0xae23d8 (no bloco das competições) [0xae2380] |
| 15–17 | first/second/common_names.dat | 13–15 | 44.281 / 104.395 / 13.256 | 60 | 0xae23f0 / 0xae23f4 / 0xae23f8 [0xae2398..0xae23a0] |
| 18 | staff_history.dat | 17 | 385.846 | 17 | 0xae23cc [0xae2374] |
| 19 | staff_comp_history.dat | 18 | 2.609 | 58 | 0xae23e4 [0xae238c] |
| 20–21 | club/nation_comp_history.dat | 19, 20 | 10.766 / 273 | 26 | 0xae23d4 / 0xae23dc [0xae237c, 0xae2384] |
| — | (montada) preferências | 22 | 169.358 | 52 | 0xae23c8 [0xae2370] |

Na carga são ~70 MB (pessoas 21 MB, jogadores 10, nomes 10, preferências 9, clubes 8, histórico de pessoas 7); depois dos cortes da criação, os ~15 MB que a impressão `banco` da carreira (130ª leva) cobre.

Depois da carga: `Config::apply_and_save` 0x5d8480 (com idioma diferente do dos nomes, `database_load_language_names`), a escolha das ligas e `create_game` 0x828110, com a ordem da tabela de criação da §1 (cortes por `[0xae3138]+4 & 7`, vetor de clubes por reputação, `database_player_initialisation`, os 44 passos e os dias até a data de início). Na criação, o histórico de pessoas e o de prêmios passam para os subsistemas (`PlayerStats`, `AWARD_MANAGER`) e são liberados do banco.

### Ligação com o save

- **Gravar:** `save_game_data` → `save_database_for_game` → `save_database(…, 1, …)`. Cada tabela vira um item do contêiner (`staff.dat` 110 bytes versão 2, `Preferences.dat`, `nonplayer.dat`, `player.dat` em itens próprios, posição 0 no índice). O `index.dat` do save tem cabeçalho `7f 00 00 00 10 00 00 00` (127 competições de reserva, 16 técnicos), as 22 entradas e a **cauda** (o byte ≠ 0, os 0x1cc bytes, a lista 0xae23fc e o idioma: 30.045 bytes de cauda no Valencia.sav do CMExplorer; índice inteiro de 47.235 bytes no save do CM3SaveEditor). `staff_history` e `staff_comp_history` ficam no índice mas não no contêiner (já viraram `.tmp` dos subsistemas). Os `*.lng` vão junto (`zadd`). Ordem conferida: [MAPA_SAVE.md](MAPA_SAVE.md) §2.
- **Carregar:** `load_game_data` → `database_free_all_tables` → `load_database_from_stream` → `load_database(…, 1)`: o mesmo caminho com `zopen`/`zread`/`zseek` no lugar de `fopen`/`fread`/`fseek` e o ramo não compartilhado (0x5289c7..0x52b378). O byte do `index.dat` ligado muda as reservas (sem as 23.000), lê os nomes em inglês e o idioma; `zextract("*.lng")` devolve os `.lng` a `CM3_TEMP`. Uma tabela de histórico ausente é liberada e fica com quantidade 0 (0x52bcea).
- As tabelas do save são as do banco já cortado e vivo (clubes, pessoas e jogadores mudam ao longo da carreira); o save **é** o banco no estado do dia, mais os subsistemas.

## 3. O que os irmãos oferecem (só leitura, regra de dono)

| O quê | Onde | Serve para |
|---|---|---|
| Inventário da base, pseudocódigo da carga, tabela completa do `resolve_database_indexes`, chaves, passos da criação | Bosman Edition: `referencias/cm0102-bosman-edition/docs/research/2026-10-13-database-inventory.md` (atenção: troca jogador e não jogador, corrigido no `2026-10-14-game-creation-inventory.md` §2b), `2026-10-14-start-new-game-inventory.md` §2, `2026-10-14-game-creation-inventory.md` §2–§3 | a referência principal das levas: ordem, contagens, campos que viram ponteiro, condições de nulo e os erros do original a copiar |
| Leitor do `index.dat` e das tabelas, base e save; duas versões de pessoa (157/110); gravação da base e do save | `D:\Sistemas\CMExplorer\docs\ABRIR_GRAVAR.md` §3–§6, `engenharia-reversa\index_dat.py` (`le_index`, `le_sav`) | oráculo de formato para os leitores; mostra que bases da comunidade (CLZ 25-26) já vêm em **versão 2** (110 bytes e preferências no id 22, `staff.dat` com quatro tabelas), caminho que a base de fábrica não exercita |
| Campos dos registros: pessoa, jogador, não jogador, preferências, clube, nação, cidade, estádio, nomes, árbitro, contrato | `D:\Sistemas\CMExplorer\docs\CAMPOS.md` (CMExplorer × SaveGame Editor), `docs\LIGACOES.md` (tela ↔ campo de cada ficha) | nomes dos campos para as levas e para o editor |
| Estruturas e telas de edição em Rust | `D:\Sistemas\CM3SaveEditor\rust\cm3sav\src\`: `pessoa.rs` (`TelaPessoa`, `Registros`), `clube.rs` (`TelaClube`), `nacao.rs` (`TelaNacao`, `limpar_elenco`, `allow_management`), `competicao.rs` (`Tipo`: 107/101 bytes), `arbitro.rs`, `preferencias.rs` (`itens`, `gostar`, `incluir`), `nomes.rs` (`Nomes::completo`); `docs\CAMPOS.md` (TStaff, TClub, TNation, TComp, TPlayer, TStaffComp, TNonPlayer, TStadiums, TCities), `docs\REGISTROS.md`, `docs\FORMATOS.md` §4–§5 (os 20 itens do banco num save real e o tamanho de cada registro), `docs\VALIDADOR.md` | o editor futuro (item 8) e o validador; tamanhos conferidos num save |
| Leitura do banco de dentro do save e troca id → ponteiro | `D:\Sistemas\CMScout\docs\ABRIR.md` §4–§5 (os 8 carregadores e a tabela de ligações), `rust\olheiro\src\banco.rs` (`Banco`: `staff`, `jogador`, `clube`, `nacao`, `registro_nome`, `idade`; constantes de tamanho) | conferência independente das ligações do `resolve_database_indexes` (pessoa +0x61 → jogador, +0x69 → não jogador) e do contrato em +0x6e/+0x72 da memória do CMScout |
| Edição da memória do jogo vivo | `D:\Sistemas\CMRTE\docs\CAMPOS.md`, `rust\cmrte-nucleo` | o "CMRTE dentro do jogo" do item 8: as mesmas tabelas, já com ponteiros |
| Não serve | gravação do CMExplorer (reserializa tabelas inteiras, pula `colour`, `continent`, `staff_comp`, as competições e `nation_comp_history`); leitura dos irmãos com o id fora da faixa → nulo sem os limites `>`/`>=` do jogo | são dos editores; o jogo tem as próprias regras (Bosman §2d) |

## 4. Levas propostas

Tipo novo de item (**banco**), ao lado de telas, simulação e save. 33 itens (≈ 95 rotinas) em 11 levas; as demais consultas e textos da §1 (fora os nomes de N1/N2 e os comparadores de T2) seguem pelas filas de simulação e de telas.

| Leva | Item | Rotinas | Comparador previsto |
|---|---|---|---|
| B1 | L1 leitores | 0x54d5e0 continentes, 0x54d740 nações, 0x54d8a0 cidades | **arquivo**: o mesmo `.dat` (cópia em `jogo_testes`) lido pela original e pela Rust em buffers separados; compara buffer, resposta e posição do arquivo. Autoteste com versões 1 e 2 e leitura curta (nunca versão inválida: caixa). Carreira: passa 1× |
| B1 | L2 leitores | 0x54da00 estádios, 0x54db50 clubes (+0x80), 0x54e780 pessoas 157 | idem |
| B1 | L3 leitores | 0x54e880 não jogadores, 0x54f030 jogadores, 0x54fc10 preferências | idem; o 0x54fc10 só por autoteste (base versão 2 montada em `jogo_testes`) |
| B2 | L4 leitores | 0x54fd60, 0x54feb0, 0x550230 | idem |
| B2 | L5 leitores | 0x550380, 0x5504d0, 0x5505c0 | idem |
| B2 | L6 leitores | 0x550710, 0x550860, 0x54e630 (pessoas 110) | idem; 0x54e630 por autoteste e, depois, pelo `.qst` |
| B3 | V1 conversores da versão 1 | 0x54dc90 clubes, 0x54e9a0, 0x54f150 | `side_by_side` sobre bytes sintéticos de versão 1 (sementes fixas, registros de 1 a N, leitura que falha no meio); a carreira não passa (base de fábrica é versão 2) |
| B3 | V2 | 0x54ffd0 competições v1, 0x54d580 abrir, 0x54d5b0 criar | conversor: idem; abrir/criar: resposta e caminho em 0xde1f64 |
| B3 | G1 gravadores | 0x54d6e0, 0x54d840, 0x54d9a0 | **arquivo**: grava em dois temporários, compara bytes e resposta; também por `zwrite` num contêiner de teste |
| B4 | G2, G3, G4 | 0x54daf0, 0x54e5d0, 0x54e720; 0x54efd0, 0x54fbb0, 0x54fd00; 0x54fe50, 0x5501d0, 0x550320 | idem |
| B5 | G5 | 0x550470, 0x5506b0, 0x550800 | idem |
| B5 | G6 liberar | 0x550950 (60 bytes, lê ou grava), 0x527ee0, 0x527f20 | 0x550950: arquivo; liberações: efeito único, pela carreira (impressão do banco) e autoteste numa tabela de teste |
| B5 | K1 chaves | 0x60d810 nações, 0x60efd0 competições, 0x6115e0 prêmios | `side_by_side`: região = as chaves 0x9cf178..0x9d0528 (~5 KB) + resposta. Cadeias geradas por script do exe (cada par é `push texto; push edi; call 0x951c50; …; mov [chave], esi`). Autoteste: cada nome da cadeia, maiúsculas e minúsculas, nomes ausentes. Carreira: 1.208× |
| B6 | K2 chaves | 0x6146b0 clubes, 0x60ba80 remapear, 0x60d780 relatar | idem (13.640× cada na carreira); 0x60d780 só por autoteste com a caixa desviada |
| B6 | T1 tabelas | 0x527f60 crescer histórico, 0x539ae0 podar, 0x53db40 ordenar | `side_by_side` com as tabelas e o vetor 0xae2d1c; `qsort` 0x945333 e `realloc` 0x945667 chamados do jogo (mesma ordem de iguais, mesmo endereço) |
| B6 | T2 ordens | 0x540020, 0x540780, 0x5402f0 | `side_by_side` puro (dois ponteiros → −1/0/1) com autoteste exaustivo |
| B7 | C1 criação | 0x541e60, 0x544970, 0x539750 | `side_by_side` de uma vez: região = o banco inteiro (ponteiros normalizados, abaixo) + fita de sorteios; carreira |
| B7 | C2 cortes | 0x524d70 (6,7 KB, SEH), 0x5267c0, 0x527240 | idem; risco de `realloc` e `rand` direto (§5) |
| B7 | C3 vetores | 0x53a610, 0x53aa10, 0x53a8b0 | `side_by_side`: registro + vetor de saída (189 bytes); autoteste com registros reais |
| B8 | C4 normalizar | 0x53a980, 0x538bc0, 0x539490 | idem |
| B8 | C5 inicialização | 0x538ec0, 0x535a80 (12,6 KB) | 0x538ec0: lado a lado; 0x535a80: orquestrador de uma vez, banco inteiro + fita (milhares de sorteios); carreira |
| B8 | N1 nomes | 0x539b90, 0x539c60, 0x53a0e0 | `side_by_side` + fita (sorteiam); autoteste por nação |
| B9 | N2 nomes | 0x53a1b0, 0x5335f0, 0x53ab90 | idem |
| B9 | C6 jogadores falsos | 0x532e00, 0x533080 (SEH), 0x533710 | banco inteiro + fita; só com "Use Real Players: No" (a conferir se o `auto_setup.cfg` tem a opção; senão `quick_start.cfg` montado pelo `save_grande.py`) |
| B9 | I1 idioma | 0x535930, 0x544fc0, 0x544f30 | autoteste com cada `.lng` numa cópia de `CM3_TEMP`; compara tabelas em inglês e respostas |
| B10 | I2, I3 nomes em inglês | 0x5453f0, 0x5454c0, 0x5455a0; 0x5456a0, 0x545770, 0x545880 | `side_by_side` com os nomes traduzidos carregados (idioma ≠ 0 numa pasta de preferências isolada, `CM3_PREFS`) |
| B10 | I4 | 0x545990, 0x5339a0 (8 KB, SEH) | idem; 0x5339a0 = tabelas inteiras antes e depois |
| B11 | O1 orquestradores | 0x5315a0 resolver, 0x601a40 chaves, 0x528080 liberar tudo | resolver: banco inteiro de uma vez (ids antes, ponteiros normalizados depois); chaves: as globais; liberar: carreira |
| B11 | O2 carga | 0x528540, 0x524d50, 0x524d10 | **carga** (novo): na criação, a original carrega e tira a foto normalizada (tabelas, contagens 0xae2340..0xae2408, chaves, `index` em 0xae1a80, sorteios); a Rust carrega por cima (a carga começa liberando tudo) e tira a mesma foto; carreira com a impressão do banco no 1º dia |
| depois de S4 | O3 gravação | 0x52dad0, 0x524d30 | pelo `.qst` do banco de provas do save ([MAPA_SAVE.md](MAPA_SAVE.md) §4): itens do banco e `index.dat` byte a byte |

**Ponteiros normalizados:** depois do `resolve`, os campos de id guardam ponteiros. Entre duas cargas o bloco pode cair em outro endereço; a foto troca cada ponteiro por (tabela, deslocamento), como o `save_database` faz (o id em +0 do alvo). No modo Rust sem comparação, a Rust tem de pedir as mesmas alocações, do mesmo tamanho e na mesma ordem (`GlobalAlloc` do jogo), senão a impressão do banco muda sem erro de lógica.

**Banco de provas do bloco:** (1) a carreira automática passa pela carga e pela criação uma vez, e a impressão `banco` do 1º dia cobre as tabelas já cortadas (uma mutação nos leitores, no `resolve` ou nas chaves aparece ali); (2) o comparador **carga** (uma vez por carreira, segundos) localiza a divergência; (3) autotestes de arquivo para leitores, conversores e gravadores, sobre cópias em `jogo_testes`; (4) o `.qst` do bloco do save para a gravação e o caminho do save (`load_database(…, 1)`); (5) idioma e jogadores falsos com preferências isoladas (`CM3_PREFS`, ou cópia e restauração do `game.cfg`). Antes das levas: `cobertura.py Database.cpp Db_files.cpp` para separar as consultas diárias das de criação.

## 5. Riscos

| Risco | Onde | Cuidado |
|---|---|---|
| Efeito único | `load_database` (libera tudo, copia `.lng`, apaga `duff.out`), cortes (`realloc` no lugar), `database_free_*`, `save_database`, `database_player_initialisation` | comparar com foto de uma vez; o resto pela carreira |
| Alocação e endereço | bloco único `GlobalAlloc` no modo compartilhado; `realloc` com `assert` de mesmo endereço (Database.cpp linhas 0x18e..0x191; a conferir como isso convive com o bloco compartilhado) | chamar as rotinas de alocação do jogo, mesma ordem e tamanho |
| Sorteios | passe das pessoas (nação sorteada, nome comum sorteado: não disparam na base de fábrica), cortes (0x90cfc0 e `rand` 0x946a04 direto), criação de pessoal, inicialização, nomes, jogadores falsos | fita de sorteios; `rng.rs` já chama o `rand` da CRT |
| `qsort` da CRT | 0x539ae0, 0x53db40, 0x541e60, `prune` (0x945333) | instável: a Rust chama o mesmo `qsort` com o comparador portado |
| Caixas de erro | versão inválida nos leitores, `resolve` (pessoa → jogador/preferências/não jogador fora da faixa), chave ausente, `assert` de ponteiro nulo em quase todas as consultas | autoteste nunca nos caminhos de erro |
| Lixo de pilha | quadros grandes: `load_database` (0x7d0), `save_database` (0xa10), `resolve` (0x604), `create_game` (0xb58), `database_player_initialisation` | sonda de zeragem (`ZERADAS`) antes de comparar; o byte extra do `index.dat` é zerado na entrada (0x528556) |
| Erros do original a copiar | `resolve`: com +0x5b de clube alto zera +0x60 e deixa +0x5b cru; estádio +0x48 alto zera +0x38; os três nomes nulos vão para a tabela de nomes **comuns**; +0x80 do clube `(char)(sorteio(4000)+1)` | portar igual; anotar em `ROTINAS.md` |
| Caminhos que a base de fábrica não exercita | conversores versão 1, pessoas 110 e preferências no id 22 (bases versão 2 como a CLZ), byte extra do `index.dat` (só o save), chaves ausentes | autoteste com bases sintéticas; a CLZ da pasta do Lucas só com cópia e sem mudar nada (`clz_teste/`) |
| Nomes trocados no mapa da comunidade | `db_files_read_players`/`non_players` e gravadores | usar id 9 = não jogador (68), id 10 = jogador (70) |
| Progresso na tela | barra 0x802f80 dentro da carga, do save e da inicialização | a Rust chama a mesma rotina nos mesmos passos (a tela faz parte do que o Lucas vê) |
| Independência futura (64 bits) | ponteiros em campos de 4 bytes | quando o núcleo sair do exe, guardar ids e resolver no acesso (sugestão do Bosman); não agora |

## 6. Para o editor (item 8 do [PROJETO.md](PROJETO.md))

No molde do editor do Elifoot II Bombado (`D:\Sistemas\Elifoot II\bombado\editor`: núcleo sem tela com base, entidades, validação e patch; janela por cima).

**Dado (editável num editor, está em `Data/`):**

- As 19 tabelas e o `index.dat` (quantidade, posição e versão de cada uma): pessoas, jogadores, não jogadores, preferências (só em bases versão 2), clubes e seleções, nações, continentes, cidades, estádios, cores, competições de clube e de seleção, prêmios, árbitros, nomes, históricos.
- Os nomes traduzidos (`*.lng`) e os textos de `language.ldb`; `player_setup.cfg`, `pundit.cfg`, `weather.cfg`, `euro.cfg`, `events_<idioma>.cfg`.

**Fixo no código (o editor precisa saber, para validar ou para virar dado depois):**

- **As 1.284 chaves por nome** (`index.cpp`; onde o código usa cada uma: `CHAVES_DO_EXE.md` e `chaves_uso.csv`): cores, nações, competições, continentes, prêmios e clubes. Renomear "England", "Premier Division" ou um clube da lista quebra a chave e o jogo para com "Unable to find the %s index". A validação do editor tem de conferir a lista (gerada do exe), e "regras em dados" (PROJETO §6.9) começa por ela.
- Pares A/B (82 pares de chaves de clube da Espanha, Alemanha e Portugal, `club_get_linked_b_team`), os 24 clubes bascos, as 5 nações melanésias e 5 polinésias, Reino Unido e Irlanda, as 6 grandes nações, as exceções de transferência livre, Argentina/França/Croácia, a divisão "Reserve", os clubes "Bright…"/"Wimble…" (+0x6d), as 14 vagas da Grécia (0x839920).
- Estrutura da tabela de seleções: as 213 primeiras de `nat_club` são as principais, as 213 seguintes as sub-21 (metade da reputação na carga); os 16 últimos ids de pessoa são os técnicos humanos; as 127 últimas competições de clube são os amistosos; reservas de 23.000 pessoas e 1.325 jogadores.
- O corte pela opção de tamanho da base (quem fica, por nação e reputação) e a inicialização dos jogadores (tabelas 0x9b5988, 22 curtos de 1 a 175, e 0x9b59c0, 20 `double` de 2,1 a 0,9 pela posição na liga).
- As correções silenciosas da carga (§2, passo 9, e os nulos do `resolve`): o editor pode avisar do que o jogo vai "consertar" (cargo 7, nação ausente, ids fora da faixa) em vez de deixar o jogo decidir.
- Os textos de cargos, atributos e posições estão no código em inglês e passam por `langlib_sprintf` (0x66f4e0), que troca o texto pelo modelo do idioma ativo: o texto em inglês serve de chave da tradução (`database_get_job_name` 0x53ad00, `database_get_player_attribute_name` 0x5428a0, `database_get_non_player_attribute_name` 0x543b10, `position_mask_get_text` 0x5482f0) [observado]. As regiões (`get_region_name` 0x616f20) usam o mesmo caminho e, em alguns idiomas, tabelas próprias no código. Os **adjetivos de nacionalidade** (`nation_get_nationality_adjective` 0x6181d0, os 24 KB) estão no código: quase mil textos distintos para os idiomas 9 (português), 4 (italiano), 5 (espanhol), 10 e 6, por nação-chave e forma (gênero e número). Na forma 0 e nos outros idiomas, o jogo usa o texto do registro da nação [observado; a rotina já está em Rust, 240ª leva]. Para o editor: trocar um adjetivo nesses idiomas exige mudar o exe, não a base.
- Duas versões de pessoa (157 bytes na base de fábrica, 110 + preferências nas bases da comunidade): o editor lê as duas e grava a da base de origem.

## Pedido P7 (22-23/10/2026, aprovado pelo Lucas)

Nos itens B9/B10 (idioma), incorporar o crate `idiomas` da sessão do langtool (D:\Sistemas\CM0102-LangTool\rust, sem dependências; `cargo test --release`) e os formatos de `docs/MAPA_LANGTOOL.md` §2, §4, §5 e §6.1 (language.ldb, .lng, a regra de aplicação dos nomes traduzidos). O §5.4 responde à dúvida dos cargos e adjetivos. Usar o qsort/bsearch do MSVC 6 do crate: 202 chaves repetidas mudam o texto da tela. Prova: a carga em Rust comparada com a original.

## Pedido P8 (23/10/2026, aprovado pelo Lucas): os achados do editor original

Repositório D:\Sistemas\CM0102-Editor (commit 35c219b; MAPA_EDITOR.md §3.1, §3.2, §6 e §10, LIGACOES.md, IDA_E_VOLTA.md):
- staff.dat v1 (157 bytes) decodificado e conferido nos 169.342 registros (fecha a pendência do FORMATOS.md); colour.dat (58) e officials.dat (43) completos;
- 369 campos do editor oficial batem com os deslocamentos e nomes do FORMATOS.md;
- regras da base: reputações de clube e nação ×500; nat_club.dat refeito a partir das nações a cada gravação; vagas fixas do clube 3/50/5/7/3; limiar de reputação 1000 com divisão; bissexto do cm_date em 0 nos nascimentos de anos bissextos;
- a lista de obrigatórios do editor oficial não serve (57 clubes e 6 nações não casam): valem as chaves do index.cpp;
- IDA_E_VOLTA: o que o editor oficial muda numa base (NationalJob −2 → −1, lesões PERMANENT_ somem, sede −1 → 0, arquivos não encurtados);
- rust/nucleo: leitor e gravador da base com ida e volta byte a byte, candidato a oráculo da carga Rust do banco.

Quando as levas do banco começarem: dúvidas de formato vão à sessão "Descompilar o editor original do CM 01/02"; com a carga Rust e a lista de chaves do index.cpp prontas, mandar ao CM0102-Editor um pedido (ORQUESTRACAO.md §4) com as chaves e os nomes reais, para ele retomar o Pré-Editor.

## Cobertura medida (23/10/2026)

Carreira automática até 2002-01-01 com contador (`cobertura.py Database.cpp Db_files.cpp index.cpp`, `simular.py --cobertura`), 160 funções com contador:

| Arquivo | Funções | Não chamadas | 1 vez | Mais de 1 vez, sem Rust |
|---|---|---|---|---|
| Database.cpp | 106 | 25 | 19 | 62 (as consultas do dia a dia: reputação vista por outro técnico, 81 M chamadas; idade, 18 M; estrangeiro, 15 M; transferência livre na nação, 13 M; lista de transferências, 12 M...) |
| Db_files.cpp | 37 | 21 | 12 | 4 (a leitura dos .dat, no começo) |
| index.cpp | 17 | 7 | 1 | 9 (as chaves de clube, ~13 mil vezes) |

Ordem decidida: primeiro as consultas do Database.cpp (levas de simulação comuns, provadas pela carreira); depois a leitura dos .dat e a criação (que a gravação do .qst da prova do save já percorre: o início rápido cria o jogo lendo o banco); por fim o idioma (P7) e a carga.
