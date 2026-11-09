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
