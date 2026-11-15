# Desempenho: marcos medidos

Medições com a DLL de trabalho (desvios no exe original), para comparar quando houver um executável próprio.
Sempre na mesma bancada e com a máquina sem outros testes; cada número foi repetido e deu igual.

## Criar um jogo novo (do início do processo até o primeiro dia da carreira)

`simular.py --ate 2001-07-25 --motor sim --modo {rust|original} --ligas {ENG|MUNDO}`, bancada 5, 15/11/2026,
com 2.478 rotinas em Rust (leva 299 publicada).

| Ligas | Original | Rotinas em Rust (DLL) | Diferença |
|---|---|---|---|
| ENG | 9 s | 10 s | +11% |
| MUNDO (209 nações) | 238 s | 253 s | +6% |

**Leitura:** as cópias em Rust estão hoje *mais lentas* que o original, e a causa é a forma do trabalho, não as
cópias: cada rotina portada é alcançada por um desvio (um salto a mais e a pilha particular da DLL), e a criação
do mundo chama os construtores de competição milhares de vezes. Num executável só, sem DLL nem trampolim, essa
camada desaparece.

**Onde está o tempo da criação do mundo** (hipóteses a medir, não medidas ainda): leitura do banco em muitas
chamadas pequenas (o mesmo padrão que achamos no contêiner do save), alocação intensa (uma competição por vez,
no alocador da biblioteca da época) e os laços de tamanho fixo que a reescrita vem encontrando.

**Sobre 64 bits:** nada nesta medição depende do tamanho do ponteiro. O obstáculo dos 64 bits é outro: o original
faz as contas em x87 com intermediários de 80 bits, que não existem em SSE; um executável de 64 bits mudaria
resultados (reputação, moral, valor) a menos que a precisão seja emulada, o que custa justamente desempenho.
