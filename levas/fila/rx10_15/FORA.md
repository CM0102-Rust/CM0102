# rx10: o que ficou para a próxima rodada

Leitor rx10, rodada 6, 25/11/2026. A lista `levas/reorganizacao/proximas/rx10.txt` tinha 51 funções; 33 viraram os itens `rx10_1` a `rx10_14`. Estas ficaram de fora. As desmontagens estão no rascunho da sessão, em `scratchpad\rx10\dis\<endereço>.asm`.

## Fora de vez (não são rotinas a desviar)

- **0x412ec0 `AWARD::write_news`:** só um `ret 0xc` (3 bytes, prólogo curto). Já estava assim no `sobras.md` (sx03).
- **0x5dce00 `shared_return_false`:** `xor eax, eax; ret` (3 bytes), corpo vazio compartilhado por vários slots de Comp.
- **0x9292b0 `WorldClubChamp` destrutor escalar:** `push esi; mov esi, ecx; call rel32` (o `call` na posição 3). O cx04 já o deixou fora, junto com os outros 6 destrutores escalares, pelas `RELATIVAS` com k = 3. Se for desviado um dia, o par é `(0x9292b0, 3)`.

## Não escritas por falta de tempo (nenhuma está no `rust/src` nem na fila)

### Mundial de Clubes (8 ligas, `ATE` 2003-08-15, como os `rx10_11` a `rx10_14`)

- **0x92a8b0 `WorldClubChamp::pick_entrants`** (752 bytes, 23 chamadas): a lista de 12 em +0xee, o número em +0xf2, os clubes de cada confederação e da sede pelo `pick_entrant`. Roda no sorteio de 5/fev/2003 (o `update_for_date` em Rust a chama pelo endereço).
- **0x92ae60 `WorldClubChamp::pick_entrant`** (1440 bytes, 20 chamadas): o campeão continental quando a competição roda, senão um clube de reputação alta sorteado do continente ou da nação. O prólogo é `mov eax, 0x2144` (5 bytes) e vem seguido de `call __chkstk`: um quadro de 8,5 KB na pilha.
- **0x92a140 `WorldClubChamp::create_knockout_stage`** (1392 bytes, 46 chamadas): os três primeiros dos grupos e o melhor segundo num Cup_Stage de 4; paga 2.500.000 a cada eliminado. O molde é o `OceaniaClubChamp::create_knockout_stage` (`oceania_club_champ.rs`), mais a escolha do melhor segundo e os prêmios. Chamada pelo `advance_stage` do `rx10_11` ao chegar à fase 2, em 5/ago/2003.

### Gerente de prêmios (award_manager.cpp)

- **0x412ed0 `AWARD` destrutor escalar** (32 bytes; prólogo `mov al, [esp+4]; push esi`, 5 bytes, sem `call` relativo) e **0x412ef0 `AWARD::~AWARD`** (`mov dword [ecx], 0x967864; ret`, prólogo de 6 bytes). Só rodam ao destruir os objetos de prêmio (`AWARD_MANAGER::~AWARD_MANAGER` ou a troca por um jogo carregado). Ver se a prova do save (`CARGA`) os exercita antes de escrever.
- **0x413410 `AWARD_MANAGER::~AWARD_MANAGER`** (176 bytes): está na lista de destruição do `setup.rs` (`G_AWARD_MANAGER`). Pede o mesmo tipo de prova dos destrutores acima.
- **0x413180 `AWARD_MANAGER::AWARD_MANAGER`** (656 bytes, 22 chamadas): começa pela espera do CD (opção (a) do Lucas: `find_game_cd(salto, teste)`, ver `sbI_2` e `sx02b_1`), abre as listas temporárias de histórico e carrega ou cria os prêmios. Roda na criação do jogo.
- **0x4135b0 `AWARD_MANAGER::create_nation_awards`** (2608 bytes, 70 chamadas): a tabela de 30 vagas e o construtor de cada prêmio (Copa do Mundo, internacionais, europeus e de cada nação ativa, com as variantes antigas). Roda na criação do jogo; o comparador só pode rodar a original (aloca tudo).
- **0x413fe0 `AWARD_MANAGER::load`** (448 bytes, item `SAVE` com `CARGA`) e **0x4141a0 `AWARD_MANAGER::save`** (288 bytes, item `SAVE`): o awards.dat do save.
- **0x414420 `AWARD_MANAGER::write_shortlist_news`** (976 bytes, 30 chamadas): o texto da notícia da lista de indicados ou do convite ao voto. Escritor de texto: `compare_text_writer`, os modelos em português no `language.ldb`. Está no despacho do `NewsItem::write_text` (o `nma_7` a cita).

### Regeneração (player_regen.cpp)

- **0x7a30c0 `player_setup_cfg_load`** (128 bytes) e **0x7a3a10 `player_setup_apply_line`** (5760 bytes, 105 chamadas): só rodam com um `player_setup.cfg` em `Data/` (o `sobras.md`, sx16, já o dizia). A prova pede um arquivo de teste nos dados da bancada, com uma linha de cada tipo (lesão, suspensão, aposentadoria, transferência, empréstimo, AWOL e as marcas). A rotina grande vai inteira num item, em pedaços com nome, como a `gra_1`.

### Filtros de busca (search_filters.cpp, telas)

- **0x806630 `SearchFilters::set_option_flag`** (2496 bytes) e **0x807450 `SearchFilters::player_matches_filter`** (3392 bytes): telas da busca de jogadores (`TELAS = True`, a prova é do Lucas buscando com filtros). Já estavam no `sobras.md` (sx20).

## Notas para quem pegar

- **Auxiliares que já existem:** `world_club_champ_group_stage` (`rx10_14`) monta um grupo de 4 num League_Stage; `predicted_by_reputation_*` (Oceania) servem às posições.
- **Ligas divididas:** `construct_divided_league`, `create_section_stage` e os outros do `rx10_4` servem a qualquer liga cuja tabela principal guarda uma seção e cujas outras seções são fases.
