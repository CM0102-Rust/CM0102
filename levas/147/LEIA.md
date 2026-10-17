# 147ª leva — o chute (0x702050)

**Rotinas:**
- `chutar()` = 0x702050 (match_pl.cpp, 35 KB e 10.356 instruções, a maior portada até aqui). Executa todo chute: o desafiante com falta ou desarme, a narração, o impedimento, a interceptação no caminho do gol, a classe do desfecho e o gol (que pode ser anulado), a defesa, a trave ou o chute para fora, com centenas de narrações.
- `atras_no_agregado()` = 0x6d5950 e `companheiro_livre()` = 0x6d5a70, duas consultas pequenas que só o chute usa.

**Como foi lida:** pelo pacote (`pacote.py`), com o pseudocódigo do Ghidra para a estrutura e o assembly para cada detalhe (deslocamentos na pilha, ordem dos sorteios, tabelas de salto conferidas byte a byte).
- Pontos que o pseudocódigo escondia:
  - variáveis da pilha reaproveitadas: o flag do tipo vira a razão do gol em ponto flutuante, e o minuto vira o número de marcadores;
  - a divisão por 50 (e não por 100) do custo do erro;
  - o `and dl` que dá 220 (e não 476) no ganho do chute no alvo;
  - o argumento da procura de impedimento em outra ordem.
- A rotina é recursiva: a falta na cobrança vira tiro livre (tipo 0x33) e os modos 5 e 6 repetem o lance.

**Como foi comparada:** pela fita, com comparação dentro da unidade, porque ela roda como unidade da condução 0x7109f0, dos pênaltis 0x6be710 e dela mesma.
- A porta de gravação antiga sai: o comparador grava a unidade sempre que há fita.
- A interceptação 0x6cc540 roda por dentro da cópia, porque o comparador dela não grava unidade. As jogadas 0x70dfd0 são as unidades, com a máscara só no byte baixo do estilo (o original passa lixo nos bytes altos).

**Resultado** (`cadeia.txt`, `comparar_0820.txt`):
- Comparando até 2001-08-20: 0 divergências, com 23 dos 31 ramos exercitados.
- Mutações (26): 25 acusadas pela carreira e 1 lacuna. É `k_bloqueio`: o chute bloqueado no caminho não ocorreu e está em `docs/PENDENCIAS.md`.
- Jogo real comparando até 2002-01-01: 0 divergências.
- Carreiras Rust ×2 e original até 2002-06-01: impressão 459815f5e0dca92b.
- Placar: o motor de partidas foi de 63,6% a 72,3%, e `match_pl.cpp` está em 99,3%.

**Refazer:** sobre o commit da 146ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 147 2001-10-01`
