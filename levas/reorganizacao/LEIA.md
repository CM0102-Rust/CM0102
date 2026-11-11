# Reorganização (07-08/11/2026): provas e insumos

A reorganização aprovada pelo Lucas em 07/11/2026 parou as levas e arrumou o repositório para ser lido por qualquer pessoa:
(1) remendos nas ferramentas; (2) o crate `cm0102-base`; (3) o `motor.rs` partido em um arquivo por arquivo de origem
do jogo; (4) legibilidade em rodadas, cada uma corrigindo o que um leitor de fora (um agente que não conhece o projeto
nem lê português) apontou. Nenhuma rodada muda o comportamento: muda nome, comentário, lugar de rotina. A exceção é a
rodada 27 (09/11/2026, assinaturas conferidas, abaixo), que corrige leituras erradas do retorno de funções do exe.

## Rodada 27: as assinaturas conferidas na desmontagem

53 funções do exe eram declaradas com assinaturas diferentes em lugares diferentes (convenção, número de argumentos,
float × inteiro, largura do retorno). Três agentes desmontaram cada uma e os seus chamadores no exe original
(`ret N`, uso do ecx, registrador do retorno) e escreveram a verdadeira em `assinaturas_conferidas.json` (com a
evidência); o `involucros_exe.py` usa essa assinatura no invólucro `exe::` e converte os usos para ela.

- **Corrigido (erro de verdade):** usos que liam o eax inteiro de funções que só devolvem `al` ou `ax` (os bits de cima
  são lixo): `transfer_manager_collect_*_news_recipients`, `match_player_decide_ball_action`, `injury_get_condition`,
  `human_manager_get_commentary_speed`, `database_get_staff_age`, `database_get_contract_rating_modifier`,
  `hostcountry_find_hosts`, `comp_util_get_fixture_venue_side` e outras; agora cada uso lê a largura certa e estende.
- **Convertido por clareza, mesmo resultado:** `finance_get_transfer_budget` e `match_player_get_instructions` devolvem
  64 bits (edx:eax; os usos só liam a metade baixa); `formation_set_player_instruction` recebe um `u64` (antes partido em
  dois `u32`).
- **Fica como está, inofensivo:** os usos `stdcall` de rotinas que o original chama como `thiscall` mas que nunca leem o
  ecx (a pilha é a mesma); os argumentos a mais na `utils_get_version_string` (cdecl, o chamador limpa); a `get_stat` lida
  como `f32` no `instructions` do match_man (o original guarda em dword entre as chamadas; passar por f64 arredondaria duas
  vezes).
- **As mutações:** a rodada 25 tinha deixado 48 mutações citando no texto de troca um apelido local que saiu
  (`let _ = apelido;`, que não compilaria); o `mutacoes_sem_apelidos.py` tira esses nomes, e cada mutação mudada foi
  compilada uma a uma.

## Leva 51 da legibilidade: os comentários do corpo que restavam em português

O obstáculo 4 do leitor de fora da rodada 12 (comentários misturando as línguas): um agente traduziu os 54 comentários
curtos do corpo que ainda estavam em português (as marcas de mutação ficaram); os 33 pulados são nomes de clubes e
nações. O gerador do `nation_adjectives.rs` escreve o cabeçalho em inglês. Prova: aa7e9c95 ×2, detector igual,
comparação 0; as 24 mutações mudadas compilam.

## Leva 50 da legibilidade: os passos de vetor com nome

O obstáculo 3 do leitor de fora da rodada 12: `nomear_passos.py` troca o literal de passo numa multiplicação (`k * 0x3b`,
`0x49 * j`, `wrapping_mul(0x4f)`) pelo tamanho do registro com nome (`league_table_row::SIZE`, `match_record_row::SIZE`,
`staff_state::SIZE`, `match_record::SIDE_SIZE`, `formation_fields::SIZE`...) só nas linhas que já citam o registro; 317
linhas. Constantes SIZE novas no `fields.rs` para a linha da tabela, a linha do registro da partida, o estado da pessoa,
o jogo, a rodada de copa, os blocos por lado de estatísticas e de tática e a data de rodada da liga. Prova: aa7e9c95 ×2,
detector igual, comparação 0; as 18 mutações mudadas compilam.

## Leva 49 da legibilidade: os valores de enumeração com nome

O obstáculo 2 do leitor de fora da rodada 12 (valores de enumeração sem constante): dois agentes leitores deram nome aos
valores de 35 campos-enumeração (396 constantes, quase todas verificadas pelas strings que o jogo imprime para cada valor):
tipos de cargo, situação da oferta, situação na tabela, categoria da partida, marcas de resultado, nomes de rodada,
regras disciplinares, tipo de competição, situação profissional, regiões do mundo, idiomas, estado da partida ao vivo,
inscrição europeia... `aplicar_enums.py` gera o `rust/src/motor/enums.rs` e troca 302 literais pelos nomes. Locais
enganosos renomeados (`stadium`, `reputation`, `professional`, `manager_reputation`); `cargo fix` de novo. Prova:
aa7e9c95 ×2, detector igual, comparação 0; as 16 mutações mudadas compilam.

## Leva 48 da legibilidade: registros secundários tipados, renomes da revisão

O obstáculo 1 do leitor de fora da rodada 12 (registros secundários ainda em `usize`): as variáveis de `offer`,
`league_table_row`, `manager_record`, `cup_round`, `shortlist_slot`, `physical_record`, `stadium`, `match_record`,
`cup_tie`, `retirement_entry`, `live_match`, `match_record_row`, `injury_entry` e `comp_stats_club` ganham o tipo do
registro e os campos viram métodos (306 acessos). Três deslocamentos relativos que a ferramenta tinha embrulhado como
registro (`ShortlistSlot(k * 0x25)`) voltaram a número. Renomes conferidos na desmontagem: `manager_record::MONTH_AWARDS`
→ `AWARDS` (0x86c8c0 e 0x86c390 incrementam), `club::EURO_FLAG` saiu (sinônimo de `CONTINENTAL_COMP_ID`). Locais,
rodada 30 (16 nomes). Prova: aa7e9c95 ×2, detector igual, comparação 0; as 32 mutações mudadas compilam (duas trocas
acompanharam o tipo novo).

## Levas 42 a 47 da legibilidade: leitores de campo no resto do motor

Vereditos dos leitores em mais 107 arquivos, dos auxiliares compartilhados aos menores arquivos de competição, com 2 a
4 arquivos por leitor nas últimas: 42 (`shared`, `human_manager`, `fin_prm`, `officials_manager`), 43 (`scout_manager`,
`comp`, `friendly`, `awol`), 44 a 46 (8 arquivos cada) e 47 (os últimos 75 arquivos com deslocamentos crus). Dezenas
de registros novos (o registro do técnico humano, do árbitro, da classificação da FIFA, do contêiner de arquivos, dos
prêmios, as eliminatórias da Copa...). Revisões: `official::RATING` → `REPUTATION`, a ficha do árbitro deixou de ser
tratada como ficha de jogador (`renew_officials`, `init_strictness`), a rotina 0x5fd210 `manager_9a01` →
`is_on_holiday` (nome da comunidade), `comp_stats_club::DANGER_POSITION` → `FINAL_POSITION` (gravado por
`Comp_Stats::set_club_final_position`; quatro leitores acharam o nome antigo errado). `nomear_por_linha.py` não troca mais
dentro de strings. Leitor de fora (`leitor_de_fora_rodada12.txt`): 3,5 → 5,5. Prova conjunta: aa7e9c95 ×2, detector
igual, comparação 0; as 68 mutações mudadas compilam (uma troca ainda citava `official::RATING`, corrigida).

## Leva 41 da legibilidade: leitores de campo em estatísticas, regens, estádios e Copa do Mundo

Vereditos dos leitores no `player_stats.rs`, `player_regen.rs`, `comp/stadium.rs` e `comp/worldcup/world_cup.rs`.
Registros novos: `stadium_venue`, `world_cup_fields`; as estatísticas de partida do `player_category_stats` e da
`lineup_row`, `stadium::SIZE`, os bits de aposentadoria e de regen do estado da pessoa. Uma ferramenta tinha escrito
`career_entry::STATS * HISTORY_ENTRY_SIZE` onde o original quer seis entradas: voltou a `6 * HISTORY_ENTRY_SIZE` (mesmo
valor). Prova: aa7e9c95 ×2, detector igual (cobre também a leva 40), comparação 0; as 3 mutações mudadas compilam.

## Leva 40 da legibilidade: leitores de campo na classificação da UEFA, copas europeias, dia de jogo e treino

Vereditos dos leitores no `comp/uefa_seeding.rs`, `comp/eurocomp/european_cup.rs`, `match_day.rs` e
`training_manager.rs`. Registros novos: `uefa_seeding_nation`, `european_club_entry`, `training_manager_fields`,
`training_schedule`; os vetores por humano dos espectadores da partida e os bits da atualização em rede. Prova:
aa7e9c95 ×2, comparação 0; o detector de lixo não rodou (memória livre abaixo de 4 GB com os leitores rodando) e fica
coberto pela prova da leva 41, que contém este código; as 7 mutações mudadas compilam.

## Leva 39 da legibilidade: leitores de campo em contratos, observação, táticas e áreas da interface

Vereditos dos leitores no `contract_manager.rs`, `player_search.rs`, `tactics.rs` e `area.rs`. Registros novos:
`area_fields` (a área da interface do Area.cpp), `guio`, `undo_slot`, `tactics_row`, `scouted_player_slot`,
`scouted_match_entry`, `search_result`; as estatísticas da partida no `match_record_row` e no bloco de cada lado do motor.
No motor, `FOULS`/`CARDS` passaram a `YELLOW_CARDS`/`RED_CARDS`: o `fill_match_record` os copia para os amarelos e os
vermelhos do registro da partida (a nota do árbitro lê "vermelhos > 2" e "amarelos > 5"). O defeito de cópia da perda de
atributos por lesão (0x62b7b0) foi para o `DEFEITOS_DO_ORIGINAL`. Prova: aa7e9c95 ×2, detector igual, comparação 0; as
mutações mudadas compilam (uma troca ainda citava `engine::CARDS`, corrigida).

## Leva 38 da legibilidade: leitores de campo em jogos, ligas, mídia e listas de observação

Vereditos dos leitores no `comp/fix_man.rs`, `comp/league.rs`, `media.rs` e `shortlist_manager.rs`. Registros novos:
`fixture_leg_check`, `fixed_fixture`, `h2h_entry`. Bits das marcas da liga, das regras de confronto, da situação de
transferência, da contagem de listas da IA no estado da pessoa e duas queixas do contrato (inferência). Placar das levas
34 a 38: deslocamentos crus 4.495 → 3.177; 109 campos novos com nome (verificados 14 → 50) e 55 bits em 23 módulos do
`bits.rs`. Prova: aa7e9c95 ×2, detector igual, comparação 0; as mutações mudadas compilam.

## Leva 37 da legibilidade: leitores de campo em seleções, banco, copas e formações

Vereditos dos leitores no `national_teams.rs`, `database.rs`, `comp/cup.rs` e `formation.rs`. Registros novos:
`call_up_entry`, `human_manager_record`, `staff_history`, `colour`; campos novos no `national_squad_record` (limites do
elenco, reservas por nível), no `formation_fields` (senha, autor, coordenadas das vagas, pelas mensagens de erro da
carga), no `cup_round`/`cup_tie` e no `names` (NATION, WEIGHT). Bits das regras de sorteio e de confronto das copas,
das casas das vagas e das regras de elenco. Prova: aa7e9c95 ×2, detector igual (cobre também a leva 36), comparação 0;
as mutações mudadas compilam.

## Leva 36 da legibilidade: leitores de campo no motor de partidas

Vereditos dos leitores no `match_eng.rs` (175 crus → 64), `match_man.rs` (173 → 124; quase todo o resto é o quadro de
pilha emulado da rotina original) e `match_pl.rs` (160 → 47). Campos novos no `engine` (SHOOTOUT_SHOTS, SHOTS_ON_TARGET,
SIDE_PITCH_HANDICAP e MATCH_ALREADY_OVER como inferência), `match_player::STARTING_POSITION`, o registro `tactic_stats`;
bits das táticas (OFFSIDE_TRAP, PRESSING) e do `staff::PLAYING_SQUAD`. O +0x1a4 do motor ficou sem nome: os dois
leitores discordaram. Prova: aa7e9c95 ×2, comparação 0; o detector de lixo não rodou (memória livre abaixo de 4 GB com
os leitores rodando) e fica coberto pela prova da leva 37, que contém este código; as 13 mutações mudadas compilam.

## Leva 35 da legibilidade: leitores de campo em quatro arquivos, bits das flags com nome

Leitores de campo no `match_screens.rs` (185 → 80 crus; 65 são coordenadas de desenho), `transfer_manager.rs` (122 →
17), `injury.rs` (115 → 19) e a segunda passada do `manager_manager.rs`. Registros novos: `screen_manager`, `display`,
`side_stats`, `recovery_news_entry`. `nomear_bits.py` dá nome aos bits dos campos de flags no `bits.rs` novo
(`expectation_flags::TITLE_CHALLENGE`...); `ajustar_constantes.py` põe o `as T` no nome usado em conta de outro tipo.
Revisão do registro da diretoria: `SPENDING_CEILING` → `SQUAD_INVESTMENT`, `CONFIDENCE_2/3` →
`SUPPORTERS_/STAFF_CONFIDENCE`. O defeito do recurso de suspensão (0x557000) foi para o `DEFEITOS_DO_ORIGINAL`. Prova:
aa7e9c95 ×2, detector igual, comparação 0; as 11 mutações mudadas compilam.

## Leva 34 da legibilidade: leitores de campo no manager_manager.rs (piloto da decisão (e))

Dois agentes leitores deram um veredito para cada um dos 205 `+ 0x..` crus do `manager_manager.rs` (registro, nome,
largura, confiança com evidência): 125 nomes que já existiam, 51 novos (15 campos distintos, um módulo novo `names` para o
TNames), 24 que não são campo e 5 desconhecidos. `nomear_por_linha.py` aplica por linha; o `fields.rs` leva a marca de
confiança. Deslocamentos crus no arquivo: 205 → 33. Os vereditos ficam em `leitores_de_campo/`. Leitor de fora
(`leitor_de_fora_piloto_campos.txt`): 3/10 antes, 6/10 depois; o que falta são os bits sem nome dentro dos campos de
flags, campos ainda crus no mesmo registro e limites mágicos em hexadecimal. Prova: aa7e9c95 ×2, detector igual,
comparação 0; as 6 mutações mudadas compilam.

## Leva 33 da legibilidade: as correções do próprio compilador

`cargo fix` aplica as 975 sugestões do compilador (parênteses desnecessários, importações e `mut` sem uso): os avisos
caem de 1.021 para 46 (o que sobra são valores que o porte fiel atribui e não lê, e os `static mut` emprestados).
`espacos_do_cargo_fix.py` tira os espaços que ele deixa onde tirou parênteses (`x as u8 ;`), só nas linhas mudadas. Os
dois rodam no fim do `padronizar.sh`, então as levas novas entram sem esses avisos. 29 âncoras de mutação seguidas (2 à
mão) e uma troca que deixou de compilar (`(x as i32) < 10`). Prova: aa7e9c95 ×2 (a mesma da leva 32), detector igual,
comparação 0.

## Leva 32 da legibilidade: um contrato de segurança, as flags do motor atômicas (decisão (a))

`estaticos_atomicos.py`: os 71 `static mut` escalares de arquivo do motor (flags de ensaio, trampolins) viram atômicos
com `Relaxed` (o jogo tem uma thread só), e lê-los deixa de pedir `unsafe`. Os 23 que ficam são emprestados (`addr_of!`:
as fitas dos comparadores, os registros que um ensaio monta) ou lidos pela tabela das originais; os 36 do resto do crate
são a infraestrutura que o asm lê. O `motor.rs` ganhou uma seção `# Safety` única para todas as `unsafe fn` do motor, em
vez de uma em cada uma (exe 3.9.68 conferido, thread do jogo, registros vivos como argumento). Prova: aa7e9c95 ×2 (a impressão mudou de 7cc445c2, como já mudou em outras levas: o código gerado mudou e o resíduo de pilha que a original lê também; o critério é Rust ×2 igual e comparação 0),
detector igual, comparação 0; nenhuma mutação cita os estáticos trocados e nenhuma âncora quebrou.

## Leva 31 da legibilidade: o valor tipado fica com o tipo (leitor de fora, rodada 11)

`metodos_em_cadeia.py`: o campo de um registro tipado lido pelo endereço vira o método dele, também numa cadeia
(`read_usize(person.club_job().addr() + club::MANAGER)` → `person.club_job().manager().addr()`), em duas fases: a forma
conferida `Club::manager(EXPR)` só compila se EXPR for Club; depois vira a chamada de método. 429 acessos. Os passos
escritos à mão que sobravam: o ponteiro de clube anda por `club::SIZE`, as contas de índice de clube restantes usam
`club_at(..)`, os blocos de jogadores do motor usam `m.player(lado, k)` e o técnico da partida ganhou `player(k)` sobre o
bloco de 20 jogadores dele. `PlayerStatsEntry::nova` → `new`. O `.addr()` que fica (7.788) é sobretudo fronteira: metade é
argumento das rotinas desviadas e dos comparadores, que recebem `usize` porque o exe as chama assim. Prova: 7cc445c2 ×2,
detector igual, comparação 0; as 14 mutações mudadas compilam.

## Leva 30 da legibilidade: o passo de estilo para as levas novas

`padronizar.sh` passa as ferramentas de legibilidade no código logo depois de integrar uma leva (o `publicar_leva.py`
chama): primeiro o ajuste pelo compilador das chamadas escritas contra a API antiga, depois `read_T`/`write_T`, as tabelas
globais, os fechos leitores e os nomes do `.rdata`, locais e mensagens em inglês, deslocamentos com nome, registros tipados
e a limpeza. Cada ferramenta é idempotente e volta a função que não compila; uma segunda passada sobre a árvore não muda
nada. A primeira passada sobre a árvore atual ainda tipou alguns acessos. Consertos no caminho: o `nomear_rdata.py` mantém
todas as linhas existentes (um endereço pode ter dois nomes); o setter tipado escreve um literal solto como
`0xffffu16 as i16` (um literal acima de `i16::MAX` tipado como `i16` não compila). O `COMO_ESCREVER_UM_ITEM` descreve o
estilo tipado. Leitor de fora, rodada 11: `leitor_de_fora_rodada11.txt`. Prova: 7cc445c2 ×2, detector igual, comparação
0; as 5 mutações mudadas compilam.

## Leva 29 da legibilidade: assinaturas tipadas e as últimas constantes de ponto flutuante cruas

`tipar_registros.py --assinaturas` para os 25 tipos de registro em uso e `--propagar` sobre eles (`.addr()` 8.079 → 8.017:
o resto são funções com chamadas mistas). `expandir_leitoras.py` expande 596 chamadas de fechos locais que só leem um
endereço fixo do exe (`let k = |a: usize| read_f64(a); k(0x968dc0)`), e o `nomear_rdata.py` dá nome pelo valor: 63
constantes novas, nenhuma leitura de ponto flutuante crua no código. O `nomear_rdata.py` mantinha só as constantes das
leituras cruas da vez e apagava as que já tinham nome: agora mantém e só acrescenta. Prova: 7cc445c2 ×2, detector igual,
comparação 0; as 28 mutações mudadas compilam.

## Leva 28 da legibilidade: nomes de função em português e mais locais

`nomes_em_ingles.py`: as funções e os comparadores que tinham ficado com nome em português ganharam nome em inglês
(`compare_condicao` → `compare_player_condition`, `compare_papel_ataque` → `compare_attacking_role`,
`posicao_no_conteiner` → `position_in_container`, `instalar_passagem` → `install_passthrough`, os construtores `novo()` →
`new()`...), com as constantes `IDX_`. O `cm0102-base` fica com os nomes dele por enquanto (o Tweaks leva cópias dele): um
apelido no `use`. `traduzir_locais.py` deixou de barrar um nome que só colide com um método de registro ou com um item
chamado pelo caminho, e `partida` → `fixture` (`match` é palavra reservada): mais 178 locais. A contagem deixou de tomar
`total`, `time` e `media` como português. Locais em português 474 → 243. Prova: 7cc445c2 ×2, detector igual, comparação
0; as 12 mutações mudadas compilam.

## Leva 27 da legibilidade: as mensagens de depuração em inglês

`textos_em_ingles.py --todos`: as mensagens de depuração fora dos comparadores (as auxiliares das competições:
"returned", "stage", "without stage", "differences in") passam pela mesma tradução; só textos com um marcador `{}` e
sem caminho ou nome de arquivo; palavras acentuadas são procuradas sem o acento. Mais 217 mensagens; os textos em
português no código caíram de 777 para menos de 80 (quase todos nomes de variáveis de ambiente). Prova: 7cc445c2 ×2,
detector igual, comparação 0.

## Leva 26 da legibilidade: o log da DLL em inglês

As linhas do log que as ferramentas leem mudaram junto com os leitores, que aceitam as duas formas (logs e diários antigos
continuam legíveis): `DIVERGENCIA` → `DIVERGENCE`, `EXCECAO` → `EXCEPTION`, `N comparisons, N divergences`,
`SIMULATION stopped at .. draws= pointer= offset=`, `matches: N engines, fingerprint`, `memory: bank`, o diário `day ..
draws= people= memory= clocks=`, `branch <nome>: N`, `coverage`, `allocator:/coverage: WARNING`, `<nome>: SELFTEST N
cases`; os placares são `engine`, `date` e `random`. As outras ~70 mensagens (alocador, desvios, resolução, msgbox,
simulação, sonda de pilha, diag) também. `CM_AUTOTESTE` aceita também `engine:N` e `database`. Leitores atualizados:
`simular.py`, `mutacoes.py`, `cadeia.sh`, `fila_continua.sh`, `gen_docs.py`, `pendencias.py` e os leitores do diário
(`deixar_infeliz.py`, `leitoras.py`, `sonda_pilha.py`). Prova: 7cc445c2 ×2, detector igual, comparação 0 (o `simular.py`
leu as linhas novas: `engine: 1000000000 comparisons, 0 divergences`); o teste de mutação com o log novo acusa as
mutações.

## Leva 25 da legibilidade: as mensagens dos comparadores em inglês

`textos_em_ingles.py`: 720 mensagens que os comparadores escrevem numa divergência (`"semana({:?}) original={} rust={}"`)
passam a nomear a rotina pela função Rust (dentro de `compare_X`, um nome português no começo vira X; fora, o mapa do
`docs/NOMES_ROTULOS.csv`), e as palavras portuguesas passam pelo dicionário (expected, got, club, person, cell, side,
count...). Nenhuma ferramenta lê esses textos; as palavras-chave do log que as ferramentas leem mudam numa leva própria.
Prova: 7cc445c2 ×2, detector igual, comparação 0.

## Leva 24 da legibilidade: as rotinas desviadas com um nome só, em inglês

Cada linha da tabela `routines!` levava um rótulo em português para o log (`"motor_tabela_a"`) que repetia o que o nome
da função Rust já diz; os rótulos saíram e o macro tira o nome da própria função (`stringify!`), então o log, o
`SEM_ROTINAS` e as ferramentas usam `table_a` (2.092 rótulos; o mapa antigo → novo em `docs/NOMES_ROTULOS.csv`, para os
logs e as notas antigas continuarem legíveis). `tabela_motor.py` lê as duas formas, `fila.py` escreve a nova e
`prova_save.py` passa os rótulos de uma leva antiga pelo CSV. Os nomes de desvio escritos à mão (as portas, o relógio,
a RAM e o disco da simulação, a sonda de pilha) também passaram ao inglês (decisão do Lucas, 10/11: tudo o que der vai
para o inglês). Prova: 7cc445c2 ×2, detector igual, comparação 0.

## Leva 23 da legibilidade: as tabelas globais de clubes e de pessoas com nome

`club_at(i)` e `staff_at(id)` (no `motor/mem.rs`) substituem a conta à mão nas tabelas do jogo
(`read_usize(G_CLUB_TABLE) + i * 0x245` e variantes, 27 lugares, `tabelas_globais.py`); o ponteiro de clube que anda
pela tabela usa `club::SIZE`. O parâmetro `percentual` tinha sido renomeado para `write_percent_pair` (nome de outra
coisa) em seis arquivos: agora é `percentage`. A compilação de todas as 3.250 mutações (em lotes) achou 20 textos de troca
envelhecidos ao longo das levas (tipos, locais traduzidos, locais de transmute que saíram): consertados, os 20 compilam.
Prova: 7cc445c2 ×2, detector igual, comparação 0.

## Leva 22 da legibilidade: um dialeto de memória só

`unificar_acessos.py`: 5.697 acessos crus por ponteiro em 1.558 funções viraram os acessos sem alinhamento do
`motor/mem.rs` (`*((E) as *const T)` → `read_T(E)`, `*((E) as *mut T) = V` → `write_T(E, V)`, `read_unaligned` e
`write_unaligned` idem). Ficam: a atribuição composta (`+=` avaliaria E duas vezes), o deref tomado por referência e o
cast de algo que não é endereço; 159 funções que não compilaram voltaram. Saem os parênteses em volta de um `read_T(..)`
sozinho. As âncoras de mutação quebradas foram refeitas reaplicando a conversão na função antiga com a mutação; dez
âncoras antigas ambíguas ganharam contexto único (81 → 71). Prova: 7cc445c2 ×2, detector igual, comparação 0; as 318
mutações mudadas compilam (em lotes).

## Migração para os registros tipados, leva 21: locais em português e transmutes repetidos

`traduzir_locais.py` ganhou 160 palavras (rodada 20): 364 nomes locais em 285 funções passaram ao inglês (locais em
português 856 → 474; a contagem deixou de tomar `data` como português, porque se lê como inglês e o tradutor a mantém).
`usar_involucros.py`: 186 `transmute` inline que repetiam um invólucro `exe::` com a mesma convenção de chamada, o mesmo
número de argumentos e o mesmo retorno viraram o invólucro (25 funções voltaram: tipos de argumento que o invólucro
recebe de outro jeito). `compilar_mutacoes.py` confere as mutações mudadas em lotes de âncoras que não se sobrepõem, um
`cargo check` por lote, e volta ao um-a-um nas funções onde cai um erro (provado com uma mutação estragada de propósito).
Prova: 7cc445c2 ×2, detector igual, comparação 0; as 45 mutações mudadas compilam.

## Migração para os registros tipados, leva 20: as funções devolvem o tipo do registro

`tipar_retornos.py`: 29 funções nossas que devolviam um registro em `usize` (`contract_of_staff`, `friendly_club_by_id`,
`staff_records_staff_by_id`, `transfer_manager_main_contract_stored`...) passam a devolver o tipo, votado pelo uso que as
chamadas fazem do valor (`Club(f(..))` ou `f(..) + club::CAMPO`); o compilador ajusta o resto. O `--ajustar` passou a
tratar o `if`/`else` cujos ramos ficaram de tipos diferentes. `limpar_tipos.py` tira o reembrulho do mesmo tipo
(`Club(x.club().addr())` → `x.club()`) quando a função continua compilando (305 funções). Prova: 7cc445c2 ×2,
detector igual, comparação 0; as mutações mudadas compilam.

## Migração para os registros tipados, leva 19: mais dez campos-ponteiro com tipo

`ponteiros_registros.py` achou, pelo uso, mais dez campos-ponteiro: `staff::PLAYER` e `match_player::PLAYER` →
`PlayerSheet`, `staff::NON_PLAYER` e `match_manager::MANAGER_SHEET` → `NonPlayer`, `offer::EXCHANGE_PLAYERS` e as listas
do amistoso → `Llist`, `match_manager::ENGINE` → `Engine`, `CLUB` → `Club`, `FIXTURE` → `Fixture`. O gerador
(`registros.py`) passou a manter como ponteiro um campo tipado numa rodada anterior (gerava `read_Club` quando o
`ponteiros.json` já não o listava). `limpar_tipos.py` lê todos os tipos do `records.rs` e tira parênteses duplos em volta
de uma cadeia (`((sheet())).addr()` → `sheet().addr()`). Prova: 7cc445c2 ×2, detector igual, comparação 0; as 25
mutações mudadas compilam.

## Migração para os registros tipados, leva 18: deslocamentos com nome e mais oito registros

`nomear_deslocamentos.py`: numa função, o deslocamento cru sobre uma variável tipada (`x.addr() + 0x27`, ou uma cadeia
`x.club_job().addr() + 0xcf`) vira o nome do campo quando o módulo do registro tem exatamente um campo naquele
deslocamento; `--acessos` transforma o acesso em método. 108 nomeados (club 43, staff 30, engine 12, nation 9,
competition 9...). Os registros `TransferManagerFields`, `Llist`, `MatchRecord`, `ManagerRecord`, `Friendly`,
`NonPlayer`, `MatchManager` e `LeagueTableRow` passaram pelo `tipar_leva.sh`. Deslocamentos crus 5.090 → 4.982. Âncoras
de mutação 82 → 81 (a `l_conta`, ambígua, ganhou contexto). Prova: 7cc445c2 ×2, detector igual, comparação 0; as 65
mutações mudadas compilam.

## Migração para os registros tipados, leva 17: ofertas, copas e fichas de jogador

`tipar_leva.sh MÓDULO=Tipo ...` roda o padrão completo de uma leva para cada par (`tipar_registros.py MÓDULO`, `--exe`,
`--assinaturas`, `--ajustar`, `--acessos`, depois `--propagar` e `limpar_tipos.py`). As variáveis de `Offer`, `Cup` e
`PlayerSheet` ganharam o tipo: acessos crus a `offer` 923 → 178, a `cup` 805 → 88, a `player_sheet` 681 → 438.
`seguir_ancoras.py` ganhou o recurso para a âncora cuja única mudança é um `.addr()` que saiu (a função passou a receber o
tipo); 5 textos de troca de mutação foram ajustados à mão. Prova: 7cc445c2 ×2, detector igual, comparação 0; as
mutações mudadas compilam.

## Migração para os registros tipados, leva 16: as partes internas do motor (leitor de fora, rodada 10, item 5)

O jogador em campo e a casa da grade ganham acesso com nome. `Engine::player(lado, k)` devolve o `MatchPlayer` k (0..20)
do lado (motor + `engine::PLAYERS` + 8920 × lado + 0x1be × k) e substitui os ~200 cálculos à mão
(`(m as isize + 0x4796 + 0x1be * (k + 20 * lado)) as usize`, `side_player_slot`, `player_in_block` por dentro);
`engine.cell(x, y)` devolve a `GridCell` (motor + `engine::GRID` + 90 × (12x + y)) e substitui `occupancy_grid_cell` e as
fórmulas da grade (as vizinhas viram `home(±1)`). `jogador_do_motor.py` faz a troca: as formas cruas viram a chamada, o
receptor tipado perde o `.addr()`, o `usize` cru ganha `Engine(..)`, e os casts dos argumentos vêm das mensagens do
compilador; ~20 casos à mão. Depois, `tipar_registros.py match_player` tipou as variáveis que recebem o jogador (137 →
51 `.addr()` sobre `player(..)`). Também viraram nomes os códigos de narração que tinham ficado (as zonas do campo,
`event::IN_LEFT_MIDFIELD`.., e o gol na estreia). Nenhum `0x4796`/`8920`/`0x22d8`/`0x215e`/`0x21b6` fica fora de
`fields.rs`. Prova: 7cc445c2 ×2, detector igual, comparação 0; a mutação mudada compila.

## Migração para os registros tipados, leva 15: os códigos de narração (leitor de fora, rodada 10, item 4)

`aplicar_eventos.py` lê os identificadores do `Data/events_eng.cfg` do próprio jogo (`# EVENT_HAND_BALL` sobre
`= 72, ...`; o código que o exe passa ao `Match_Events::add_event` é 8000 + o número do evento) e gera
`rust/src/motor/events.rs`: o módulo `event`, 677 constantes (`event::HAND_BALL = 8072`), nível do nome "jogo" (o arquivo
do próprio jogo dá o nome). Só os identificadores entram, nenhum texto da narração. Os literais trocam SÓ onde o valor
é com certeza um código: o argumento na posição de um parâmetro `code`/`codigo` (ou `abertura`/`common`/`quieta`) das
closures e funções de narração, e o primeiro elemento do vetor passado com `FN_MATCH_EVENTS_ADD_EVENT`; os outros
números do mesmo intervalo (a grade 0x21b6, reputações 8000, flags 0x2000) ficam. 516 literais viraram nomes
(`narrate(call, event::SILENT_OFF_OFF_OFF, ..)`). Prova: 7cc445c2 ×2, detector igual, comparação 0; as 17 mutações
mudadas compilam.

## Migração para os registros tipados, leva 14b: a limpeza

`limpar_tipos.py`: `(x.f()).addr()` → `x.f().addr()` (387 → 2), `x == Club(0)` → `x.is_null()` (238 → 10) e
`a.addr() == b.addr()` → `a == b` quando os dois lados têm o mesmo tipo (48 → 7; o compilador decide, e a função que não
compila volta ao texto de antes); as duas primeiras trocas valem também nas listas de mutação. A propagação reconhece
agora as cadeias (`f(x.club_job().addr())`), mais 7 funções. `.addr()` 6.215 → 6.086; o que sobra é sobretudo argumento
de funções que recebem `usize` com chamadas mistas (~3.200), acessos de campo que ficam crus (~760: largura, vetor,
escrita composta), deslocamentos sem nome (~300) e conversões/comparações (~500). Prova: 7cc445c2 ×2, detector igual,
comparação 0; as 17 mutações mudadas compilam.

## Migração para os registros tipados, leva 14: os campos-ponteiro tipados (leitor de fora, rodada 10, item 1)

`ponteiros_registros.py` descobre, pelo uso no código, para que registro aponta cada campo que devolvia `usize`
(base de outro registro, embrulhado num tipo): 14 campos (`staff::CLUB_JOB` → `Club`, `match_player::ENGINE` →
`Engine`, `fixture::HOME_CLUB` → `Club`, `staff::NATION` → `Nation`, `engine::PENDING_PLAY_OFFENDER` → `MatchPlayer`...),
em `levas/reorganizacao/ponteiros.json`, que o `registros.py` lê. O compilador guiou ~1.570 ajustes (`--ajustar`, que
agora trata também `Club + 0x..` e `club as u32`, só aplica edições que não se sobrepõem num passo e põe `else { T(if ..) }`
no `else if`); o passo novo `--cadeias` converte os acessos pela cadeia tipada (`person.club_job().manager()`). Os
registros ganham `LowerHex` (o `{:x}` dos logs continua). Cadeias tipadas 376 → 471; leituras cruas 19.118 → 18.924.
Efeito colateral: o `.addr()` subiu (5.259 → 6.215), onde o valor tipado vai para algo que ainda pede `usize` (funções
e comparações); a propagação atual só reconhece `x.addr()` de variável, não de cadeia. Mutações: 13 textos de troca
acertados; 12 âncoras seguidas à mão. Prova: 7cc445c2 ×2, detector igual, comparação 0.

## Migração para os registros tipados, leva 13: os números mágicos do motor

Dois agentes leram no original os valores dos campos com cara de enumeração do motor e do jogador na partida e os
códigos de lance (o índice do `MOVE_SKILL`), confirmando pelos textos da narração (`events_eng.cfg`: o número do evento
é o código de narração menos 8000): 98 constantes em `rust/src/motor/match_codes.rs`, geradas pelo
`aplicar_codigos.py` a partir de `levas/reorganizacao/codigos_motor.json` e `codigos_lance.json`, cada uma com o
significado e a confiança (`restart::THROW_IN`, `ball_height::HELD_BY_GOALKEEPER`, `shootout::PENALTIES_KICKS`,
`period::FIRST_HALF_END = 495`, `moves::SLIDE_TACKLE`...); o `action` do jogador guarda o próprio código de lance e usa o
mesmo módulo. 182 literais viraram nomes (`m.restart_type() == restart::OPEN_PLAY`). Também: a leitura e a escrita de
elemento de vetor por ponteiro (`*((j + match_player::MOVE_SKILL + 2 * 12) as *mut i16)` → `j.move_skill(12)`).

Nomes errados que a leitura achou na cópia, corrigidos: o código 37 é a mão na bola, não a cabeçada (a variável
`cabeca` → `handball`; rótulos "shot header"/"play header" → "shot handball"/"play handball"); no cobrador da bola
parada, os rótulos dos casos 2 (lateral) e 4 (tiro de meta) estavam trocados ("dribble throw_in without teammate",
"dribble goal_kick drawn"). Prova: 7cc445c2 ×2, detector igual, comparação 0; as 14 mutações mudadas compilam.

## Migração para os registros tipados, leva 12: o motor de partidas (`Engine`)

A leva 12 combinada (subclasses da liga e restos) foi trocada, a pedido do Lucas depois do balanço, pelo motor de
partidas, o maior objeto ainda sem tipo. O `--fechos` agora expande também as closures de um parâmetro sobre uma base
capturada (`let b = |o: usize| (m + o) as *mut u8;` → `((m + o) as *mut u8)`; 29 funções), e então o pacote completo:
209 variáveis de motor em 198 funções (73 delas o `this`), 143 `.addr()` tirados dos invólucros, 42 funções recebem
`Engine` (109 chamadas ajustadas), a propagação mais 5 e 2 closures. Acessos crus ao motor 1.290 → 692. Cinco âncoras
de mutação montadas com `chr(10)` foram acertadas à mão. Prova: 7cc445c2 ×2, detector igual, comparação 0; as 115
mutações mudadas e as 7 em risco compilam.

## Migração para os registros tipados, leva 11: incrementos, operações compostas, leitoras e nomes lidos

- O conversor reconhece os auxiliares de incremento de uma linha pelo formato (sete: `add_to_i32`, `fin_prm_add_i32`,
  `add_u8_at`...): `add_to_i32(x + F, v)` → `x.set_f(x.f().wrapping_add(v))`; as operações compostas `*((x + F) as *mut T)
  += v` (e `-=`, `|=`, `&=`, `^=`) → `x.set_f(x.f().wrapping_add(v))`; e o `--fechos` expande as closures genéricas de
  leitura (`let e = |a| read_unaligned(a as *const T)`) quando o argumento é um campo.
- Os deslocamentos sem nome dos nove registros (53, sem os 6 de subclasse já vistos) lidos por três agentes: 11 nomes
  novos (no jogador na partida `SPACE_ON_BALL`, `CONSISTENCY_ROLL`, `INCONSISTENCY_SWING`, `WEAK_FOOT_DEFICIT`,
  `OWN_CLUB_FEELING`, `MATCH_TEMPER`, `KEY_PASSES`, `DEBUT`, `POSITION_ADJUSTMENT`; no contrato `DATE_STARTED_YEAR`,
  `SQUAD_NUMBER`), cada um com a confiança: onde os dois leitores concordaram, conferido; onde divergiram (+0x31), inferido.
  Os elementos do vetor `MOVE_SKILL` viram `p.move_skill(k)`. `aplicar_nomes_lidos.py` aplica os vereditos.
- Hexadecimais crus sobre variáveis tipadas 262 → 107; campos crus via `.addr()` 741 → 625; `.addr()` fora dos
  invólucros 5.041 → 4.735. Prova: 7cc445c2 ×2, detector igual, comparação 0; as 22 mutações mudadas compilam.

## Migração para os registros tipados, leva 10: as closures

`tipar_registros.py --closures Tipo,..`: uma closure local cujo parâmetro `usize` é usado como um registro (`let zero =
|p: usize| (read_i8(p + match_player::SIDE) == 0) as u32`) passa a receber o tipo (`|p: MatchPlayer| (p.side() == 0) as
u32`); o corpo é convertido como uma variável tipada e as chamadas, pelo compilador (278 ajustes). 81 closures.
`.addr()` fora dos invólucros 5.160 → 5.041 (e 122 `Tipo(..)` onde a chamada passava um `usize` sem tipo). Ficam como
estão: o `call` genérico da gravação da comparação e o `o` (a rotina original) dos comparadores, que recebem `usize` de
propósito. Mutações: 9 textos de troca acertados (chamavam a closure ou a função tipada com `usize`). Prova: 7cc445c2 ×2,
detector igual, comparação 0.

## Migração para os registros tipados, leva 9: contrato, nação e finanças

O pacote completo para três registros médios: 202 + 126 + 29 variáveis; 22 funções recebem `Contract`, 6 `Nation`, 3
`Finance`, e a propagação mais 10; 81 `.addr()` tirados dos invólucros. Acessos crus: contrato 634 → 228, nação 311 →
92, finanças 158 → 44. `.addr()` fora dos invólucros 4.760 → 5.160. A revisar nos nomes: `nation::THREE_LETTER_NAME`
é comparado com um ponteiro de nação em `tactics.rs` (o nome do campo deve estar errado). Prova: 7cc445c2 ×2,
detector igual, comparação 0; as 62 mutações mudadas e as 4 em risco compilam (uma corrigida).

## Migração para os registros tipados, leva 8: o jogador na partida (`MatchPlayer`)

O pacote completo para `match_player`, com o `this` dos métodos: 294 variáveis em 239 funções, 222 `.addr()` tirados
dos invólucros, 58 funções recebem `MatchPlayer` (240 chamadas ajustadas), a propagação mais 13 (76). Passo novo,
`tipar_registros.py --fechos`: as closures locais que devolvem um ponteiro (`let b = |q: usize, o: usize| (q + o) as
*mut u8;`, usadas como `*b(j.addr(), match_player::X)`) são expandidas na chamada para a forma que o conversor entende
(27 funções), e a closure sem uso sai. Acessos crus ao jogador na partida 2.552 → ~1.070 (o resto: variáveis de closure,
reatribuídas ou com vários `let`). `.addr()` fora dos invólucros 3.815 → 4.760. Mutações: 3 textos de troca sem o
`MatchPlayer(..)` exigido foram corrigidos. Prova: 7cc445c2 ×2, detector igual, comparação 0.

## Migração para os registros tipados, leva 7: a partida (`Fixture`)

O pacote completo para `fixture`: 184 variáveis em 182 funções, 75 `.addr()` tirados dos invólucros, 31 funções recebem
`Fixture` (67 chamadas ajustadas) e a propagação, mais 7. Acessos crus à partida 1.307 → 365 (o resto usa como base uma
expressão, não uma variável). `.addr()` fora dos invólucros 3.488 → 3.815. Antes: o achado da leva 6 investigado e
registrado em `docs/DEFEITOS_DO_ORIGINAL.md` (o atrito diário usa o contrato da vaga 10 do elenco, não o do jogador).
Prova: 7cc445c2 ×2, detector igual, comparação 0; as 43 mutações mudadas e as 4 em risco pela `Fixture` compilam.

## Migração para os registros tipados, leva 6: confiança dos nomes e resíduos dos quatro registros

- **Confiança dos nomes** (pedido do Lucas): `confianca_campos.py` classifica cada campo pela origem que o comentário do
  `fields.rs` já diz: comunidade (leiautes dos editores, 247), conferido (`[verified: como]`, segunda fonte
  independente, 9), lido (leitura do código com evidência, 1.334), inferido (`[inference]`, 142). Gera
  `docs/CONFIANCA_DOS_CAMPOS.md`; cada método do `records.rs` traz o nível (`Name: community.`).
- **Deslocamentos sem nome** usados sobre variáveis tipadas (30, 403 usos), conferidos por dois agentes: 17 já tinham
  nome, 7 novos (`league::COMPETITION`, `COMP_STATS`, `ENTRANTS_ARE_QUALIFIERS`, mais dois da base `Comp` copiados do
  `cup::`; as metades de ano das datas da pessoa), 6 são de subclasses (o mesmo deslocamento é outra coisa em cada tipo
  de competição: ficam para quando as subclasses tiverem tipo). 306 hexadecimais crus viraram nomes; hexadecimais crus
  sobre variáveis tipadas 209 → 107. Achado a conferir: o clube +0xff que o original lê é a vaga 10 do elenco, não o
  técnico (+0xcf), talvez um defeito do original.
- **Vetores:** `club.squad(i)`, `coaches(i)`, `scouts(i)`, `physios(i)` (o `TIEBREAKS` da liga é lido inteiro em outros
  lugares e ficou escalar).

`.addr()` fora dos invólucros 3.683 → 3.488. Prova: 7cc445c2 ×2, detector igual, comparação 0; as mutações mudadas
compilam.

## Migração para os registros tipados, leva 5: a liga e a competição

O pacote completo (variáveis, acessos, invólucros, assinaturas, propagação) para `league` e `competition`, agora também
no `this` dos métodos (`let this = League(this);`): 424 + 50 variáveis, 142 + 5 funções recebem o tipo, a propagação
mais 17. Formas novas no conversor: `core::ptr::read_unaligned/write_unaligned((x + mod::F) as *T, v)`; a escrita por
ponteiro cujo valor tem uma leitura já convertida substitui a edição de dentro (antes as duas se sobrepunham e
estragavam o texto: a primeira gravação foi desfeita). Acessos crus à liga 1.914 → 278, à competição 119 → 47. O
`.addr()` fora dos invólucros subiu (2.503 → 3.683): o `this` tipado é repassado como `usize` às rotinas originais e lido
em deslocamentos sem nome (`this.addr() + 0xee`). Mutações: 57 âncoras seguidas, 27 pelo conserto dos `.addr()` que as
assinaturas tiraram do código, 4 à mão; compiladas as mudadas e as 93 cujo texto de troca chama uma função tipada (5
corrigidas). Prova: 7cc445c2 ×2, detector igual, comparação 0.

## Migração para os registros tipados, leva 4: a propagação (clube e pessoa)

`tipar_registros.py --propagar Club,Staff`: uma função nossa com um parâmetro `usize` que a maioria das chamadas
preenche com um registro tipado (`f(club.addr())`) passa a receber o tipo, mesmo quando só o repassa; o corpo é
convertido como uma variável tipada, e as chamadas, pelo compilador (`ajustar_pelo_compilador`: o trecho do erro é o
do rótulo "expected .., found .." e não o principal, que pode ser o nome da função chamada). Repete até não sobrar
função: 120 + 5 funções em duas rodadas, 515 ajustes. `.addr()` fora dos invólucros 2.875 → 2.503; 321 parâmetros
tipados. O que sobra: acessos a vetores e de outra largura (`club::SQUAD + v * 4`, ~450), leituras em deslocamentos
sem nome (~450) e closures locais que recebem `usize` (~230). `compilar_mutacoes.py --lista` compila uma lista dada:
as 20 mutações cujo texto de troca chama uma função que passou a receber o tipo (uma corrigida). Prova: 7cc445c2 ×2,
detector igual, comparação 0.

## Migração para os registros tipados, leva 3: a pessoa (`Staff`)

O padrão completo numa leva: `tipar_registros.py staff` (532 variáveis em 479 funções), `--exe` (633 `.addr()` tirados
dos argumentos `exe::`), `--assinaturas Staff` (74 funções recebem `Staff`, 191 chamadas ajustadas pelo compilador) e
o modo novo `--acessos MOD` sobre as variáveis já tipadas: uma leitura mais estreita que o campo é o byte de baixo dele
(little-endian, sem efeito colateral), então `read_i8(p + staff::AGE)` (campo de 2 bytes que o original lê como byte)
vira `(p.age() as i8)`; `read_i8_as_i32` vira `(p.f() as i8 as i32)`; a escrita continua na largura exata. Acessos crus
à pessoa 2.262 → 195. O `.addr()` fora dos invólucros subiu (1.700 → 2.875): a pessoa é passada a muitas funções que a
recebem como `usize` sem ler campo dela (a leva dos resíduos propaga o tipo) e há leituras em deslocamentos sem nome
(`read_usize(p.addr() + 0x61)`). Duas mutações da rodada 25 com apelido no meio do texto de troca (`remove`,
`reputacao`) foram consertadas. Prova: 7cc445c2 ×2, detector igual, comparação 0; as mutações mudadas compilam.

## Migração para os registros tipados, leva 2: o clube nas assinaturas

- O traço `Addr` (records.rs): um endereço na memória do jogo, `usize` ou um ponteiro de registro tipado. Os invólucros
  `exe::` recebem `impl Addr` onde o original passa um ponteiro (`involucros_exe.py`), então `exe::x(club)` dispensa o
  `.addr()` (442 tirados no código, 27 nas mutações: `tipar_registros.py --exe`). `i32` implementa `Addr` só para o `0`
  sem tipo.
- As nossas funções que recebiam um clube em `usize` e o tipavam na entrada recebem `Club` (112;
  `tipar_registros.py --assinaturas Club`); as chamadas foram ajustadas pelo compilador (294: `y.addr()` → `y`, o resto
  `Club(..)`).
- O `registros.py` fixa o tipo de um campo depois de gerado (a contagem dos usos muda à medida que o código vira método).
- `seguir_ancoras.py COMMIT`: segue as âncoras de mutação quebradas por uma troca mecânica (12 seguidas; 3 textos de
  troca acertados à mão).

`.addr()` fora dos invólucros: 2.246 → 1.700. O que sobra: funções que recebem o clube como `usize` sem ler campo dele
(só repassam) e vetores/larguras diferentes (`club.addr() + club::SQUAD + v * 4`). Prova: 7cc445c2 ×2, detector igual,
comparação 0; as 27 mutações mudadas compilam.

## Migração para os registros tipados, leva 1: o clube

`engenharia-reversa/tipar_registros.py club`: numa função, a variável usada como `x + club::CAMPO` passa a ser um
`Club` (parâmetro: `let x = Club(x);` no começo do corpo; `let x = e;` vira `let x = Club(e);`), os acessos viram
métodos (`x.reputation()`, `x.set_own_stadium(1)`; ponteiro para outro registro: `x.stadium().addr()` por enquanto),
`x == 0` vira `x.is_null()` e os outros usos, `x.addr()`. Ficam como estão as variáveis reatribuídas ou com vários
`let` (107), os acessos com outra largura e os vetores (`club::SQUAD + v * 4`). 544 variáveis em 475 funções; 4
funções voltaram ao texto de antes (não compilavam). Acessos crus ao clube 1.676 → 657; 881 métodos; o atrito: os
`.addr()` onde o clube vai para uma função que ainda recebe `usize` (some quando ela receber o tipo). Prova: 7cc445c2
×2, detector igual, comparação 0; as 83 mutações mudadas compilam (uma, `cpo_abrir_a_vaga`, tinha um apelido da rodada
25 que escapou e foi consertada).

## Rodada 29: piloto dos registros tipados

`engenharia-reversa/registros.py` gera `rust/src/motor/records.rs`: para cada objeto do `fields.rs` (162), um tipo
ponteiro `#[repr(transparent)]` (`Club(usize)`, a mesma ABI do endereço cru) com um método de leitura e um de escrita
por campo (1.576 com tipo), no tipo que o código já usa naquele campo (o que a rodada 28 conferiu contra o original).
Campos que apontam para outro registro devolvem o tipo dele (`club.stadium()` é um `Stadium`; a tabela `POINTERS`).
Piloto numa função, `season_end_club_update` (fin_prm.rs, as obras do estádio): leituras e escritas cruas 62 → 20,
deslocamentos `+ 0x` 11 → 1, conversões de ponteiro 10 → 2, nomes certos (`cash` em vez de `goal_difference`;
`'works`); o atrito: 39 `.addr()` para passar o clube aos auxiliares que ainda recebem `usize` (somem quando eles
recebem `Club`). Prova: 7cc445c2 ×2, detector igual, comparação 0.

## Rodada 28: o conferidor automático das assinaturas e das larguras

A rodada 27 só pegou as funções declaradas de jeitos diferentes; uma declarada errada do mesmo jeito em todo lugar
passava. Agora duas ferramentas conferem tudo contra o código de máquina original:

- `engenharia-reversa/conferir_assinaturas.py`: cada função do exe chamada pelo Rust (1.435, contando os invólucros, as
  declarações locais e os endereços fixos passados a auxiliares) contra a rotina e os seus chamadores no original: a
  pilha (`ret N` ou o `add esp` do chamador), o ecx lido antes de escrito (`thiscall`) e a largura do retorno (o que a
  rotina escreve por último antes de cada `ret` e o que os chamadores leem: `al`, `ax`, `eax`, `edx`, `st0`). Validada
  contra as 53 da rodada 27: não acusa nenhuma verdadeira e pega todas as erradas que eram erro de verdade. Achou 31
  funções que só devolvem `al`/`ax` declaradas `u32`: estreitadas pelo `estreitar_retorno.py` (com sinal onde os
  chamadores do original usam `movsx`), cada chamada existente com o `as` do tipo antigo. 24 suspeitas ambíguas foram
  conferidas por agentes: todas inofensivas (retorno ignorado ou truncado), registradas como `accepted_sites` em
  `assinaturas_conferidas.json`. Resultado: 0 erros. Roda a cada leva no `publicar_leva.py` e para a publicação se achar
  erro (saída 1); a lista completa vai para `conferencia_assinaturas.tsv`.
- `engenharia-reversa/larguras_campos.py --original`: cada acesso a um campo com nome dentro de uma rotina portada
  contra as larguras que a rotina original usa naquele deslocamento. Dos 8.964 acessos conferíveis, 8.886 batem com a
  instrução original; os 78 restantes foram conferidos por agentes (55 colisões com outro objeto no mesmo deslocamento,
  14 campos fundidos numa leitura maior ou menor, 9 só o endereço), nenhum errado (`larguras_conferidas.json`). Sem
  `--original`, a lista dos campos lidos com larguras diferentes entre os usos (`larguras_campos.tsv`), para revisão.
  Limite conhecido: o tamanho de algumas rotinas no mapa da comunidade é curto (0x6c6a30, 0x6e2f20, 0x6ec010,
  0x736720) e o acesso equivalente fica depois do fim indicado.

As chamadas por ponteiro que só existe em tempo de execução (tabelas e vtables do jogo) ficam fora: não dá para
conferir estaticamente.

## A prova de cada passo

Cada passo foi provado antes do commit, numa bancada (`engenharia-reversa/bancada.py`), com a DLL compilada do passo:

- `prova_*.txt`: duas carreiras Rust até 2002-01-01 (`simular.py --repetir 2 --detector`): as duas iguais entre si e o
  detector de lixo igual. A impressão alterna entre duas formas conhecidas (7cc445c2 e b38ccaa6) conforme o compilador
  dispõe o código (sobra de pilha); pelo critério de 31/10/2026 isso vale.
- `comparar_*.txt`: o modo comparação até 2002-01-01 (cada rotina reescrita roda ao lado da original); a última linha
  é o número de divergências e exceções no registro, que tem de ser 0.

## Os insumos (o que os agentes leram e as ferramentas aplicaram)

| Arquivos | O que são | Ferramenta |
|---|---|---|
| `leitor_de_fora_rodada1..7.txt` | os relatórios do leitor de fora, rodada a rodada | — |
| `tipos_*.tsv`, `tipos2_*.tsv`, `tipos3_*.tsv` | o registro ou objeto que cada variável guarda em cada função (`arquivo\tfunção\tvariável\tobjeto`) | `aplicar_tipos_variaveis.py` |
| `campos_motor/` | os campos do motor de partidas e do jogador na partida, e as funções onde `m`/`j` são outra coisa | `aplicar_campos_motor.py`, `aplicar_tipos_variaveis.py` |
| `cabecalhos_en/` | o inglês completo dos cabeçalhos que só tinham o detalhe no `[pt]` | `cabecalhos_completos.py` |
| `docs_en/` | o inglês dos outros docs (`///`) que estavam só em português ou curtos (chave: o hash do bloco) | `docs_em_ingles.py` |
| `rodada5/` | a 5ª rodada de tipos: variáveis (`tipos5_*`) e 835 campos propostos (`campos5_*`, 113 objetos novos) | `juntar_campos.py`, `aplicar_tipos_variaveis.py` |
| `rodada6/` | a 6ª rodada (motor.rs, gerentes, partida, comparadores): `tipos6_*`, `campos6_*` | `juntar_campos.py`, `aplicar_tipos_variaveis.py` |
| `comentarios5/` | o inglês de 873 comentários de corpo | `aplicar_comentarios.py` |
| `conferencia_assinaturas.tsv`, `larguras_no_original.tsv`, `larguras_conferidas.json`, `larguras_campos.tsv` | a saída do conferidor das assinaturas e do das larguras, e os vereditos das 78 larguras | `conferir_assinaturas.py`, `larguras_campos.py` |
| `assinaturas_conferidas.json` | a assinatura verdadeira de 53 funções do exe, conferida na desmontagem (com a evidência) | `involucros_exe.py` |
| `objetos/` | os campos de 5 objetos sem módulo antes (a partida marcada, o técnico do dia de jogo, Finance, Stadium, FIXTURE_MANAGER) e quem os guarda | `aplicar_tipos_variaveis.py` |

As ferramentas estão em `engenharia-reversa/` e o passo a passo de legibilidade depois de integrar uma leva, em
[`docs/COMO_CONTINUAR.md`](../../docs/COMO_CONTINUAR.md). O placar está em [`docs/LEGIBILIDADE.md`](../../docs/LEGIBILIDADE.md).
