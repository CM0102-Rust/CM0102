# Reorganização (07-08/11/2026): provas e insumos

A reorganização aprovada pelo Lucas em 07/11/2026 parou as levas e arrumou o repositório para ser lido por qualquer pessoa:
(1) remendos nas ferramentas; (2) o crate `cm0102-base`; (3) o `motor.rs` partido em um arquivo por arquivo de origem
do jogo; (4) legibilidade em rodadas, cada uma corrigindo o que um leitor de fora (um agente que não conhece o projeto
nem lê português) apontou. Nenhuma rodada muda o comportamento: muda nome, comentário, lugar de rotina.

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
| `leitor_de_fora_rodada1..6.txt` | os relatórios do leitor de fora, rodada a rodada | — |
| `tipos_*.tsv`, `tipos2_*.tsv`, `tipos3_*.tsv` | o registro ou objeto que cada variável guarda em cada função (`arquivo\tfunção\tvariável\tobjeto`) | `aplicar_tipos_variaveis.py` |
| `campos_motor/` | os campos do motor de partidas e do jogador na partida, e as funções onde `m`/`j` são outra coisa | `aplicar_campos_motor.py`, `aplicar_tipos_variaveis.py` |
| `cabecalhos_en/` | o inglês completo dos cabeçalhos que só tinham o detalhe no `[pt]` | `cabecalhos_completos.py` |
| `docs_en/` | o inglês dos outros docs (`///`) que estavam só em português ou curtos (chave: o hash do bloco) | `docs_em_ingles.py` |
| `objetos/` | os campos de 5 objetos sem módulo antes (a partida marcada, o técnico do dia de jogo, Finance, Stadium, FIXTURE_MANAGER) e quem os guarda | `aplicar_tipos_variaveis.py` |

As ferramentas estão em `engenharia-reversa/` e o passo a passo de legibilidade depois de integrar uma leva, em
[`docs/COMO_CONTINUAR.md`](../../docs/COMO_CONTINUAR.md). O placar está em [`docs/LEGIBILIDADE.md`](../../docs/LEGIBILIDADE.md).
