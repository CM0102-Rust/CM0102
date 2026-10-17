# 148ª leva — o motor nasce, prepara a partida e morre (0x6b7e50, 0x6b7f20, 0x6b97a0)

**Rotinas:**
- `construir_motor()` = 0x6b7e50: o construtor do motor.
- `preparar_partida()` = 0x6b7f20 (6 KB): prepara cada partida. Faz:
  - estado zerado, clima, uniformes, árbitro e gramado;
  - os dois elencos (0x6db500, ainda na original) e o narrador;
  - a força dos lados e a marcação;
  - a tabela de ângulos da bola desviada;
  - a lista de eventos de pré-jogo: insatisfação com o contrato, interesse de outro clube, forma e fim de contrato.
- `destruir_motor()` = 0x6b97a0: o destrutor.

Com elas o `match_eng.cpp` fica 100% em Rust (117 de 117 rotinas).

**Como foi comparada:** sem comparação própria, pela carreira (Rust × original), como o laço do motor da 145ª.
- As três alocam ou liberam (o narrador, a lista de eventos), e a fita teria de reproduzir as alocações.
- As rotinas que elas chamam seguem comparadas.
- A tabela de ângulos, que a original calcula com `fpatan`, sai do `atan` do Rust. A carreira confere: a mutação que muda um ângulo foi acusada.

**Resultado** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutações (18 + 4 refeitas): 16 acusadas e 2 lacunas, ambas em `docs/PENDENCIAS.md`:
  - o céu com chuva;
  - o código do evento de insatisfação.
- Jogo real comparando até 2002-01-01: 0 divergências.
- Carreiras Rust ×2 e original até 2002-06-01: impressão 459815f5e0dca92b.
- Placar: o motor de partidas foi a 73,9%.

**Refazer:** sobre o commit da 147ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 148 -`
4. `python mutacoes.py mutacoes\leva148b.py --carreira 2002-06-01`
