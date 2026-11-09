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
| `assinaturas_conferidas.json` | a assinatura verdadeira de 53 funções do exe, conferida na desmontagem (com a evidência) | `involucros_exe.py` |
| `objetos/` | os campos de 5 objetos sem módulo antes (a partida marcada, o técnico do dia de jogo, Finance, Stadium, FIXTURE_MANAGER) e quem os guarda | `aplicar_tipos_variaveis.py` |

As ferramentas estão em `engenharia-reversa/` e o passo a passo de legibilidade depois de integrar uma leva, em
[`docs/COMO_CONTINUAR.md`](../../docs/COMO_CONTINUAR.md). O placar está em [`docs/LEGIBILIDADE.md`](../../docs/LEGIBILIDADE.md).
