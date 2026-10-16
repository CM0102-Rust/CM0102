# 145ª leva — o laço do motor (0x6b98c0)

**Rotina:** `laco_do_motor()` = 0x6b98c0 (match_eng.cpp, 4,8 KB), o laço que roda `n` instantes da partida e chama quase todo o resto do motor: punições, pênaltis, lance marcado, reinício, relógio e períodos, posse, linhas, marcações e a decisão de cada jogador ativo.

**Como foi validada:** é um orquestrador e não faz conta própria, só ordena chamadas. Uma fita dela fotografaria o motor inteiro (63 KB) a cada decisão de jogador, dezenas de vezes por instante. Por isso não tem comparador lado a lado: no modo comparação roda a original, e as rotinas que ela chama seguem comparadas pelos próprios comparadores. A cópia Rust é validada pela carreira, que muda inteira com qualquer erro no laço. As mutações rodam só pela carreira (`cadeia.sh 145 -`), mais uma carreira extra com a semente 777 até 2003-06-01. As chamadas saem pelo endereço (`em0`..`em4`) e cada uma passa pelo próprio desvio.

**O que deu errado no caminho:** três mutações da primeira rodada eram equivalentes ou fracas:
- trocar −5 por −6 numa marca que só é testada como negativa;
- o relógio no pontapé inicial nunca vale 2;
- 19 contra 20 tentativas de sorteio.

Foram refeitas em `leva145b.py` (saída em `refeitas.txt`). A refeita de `l_limpeza` também escapou: o ramo não roda, é lacuna.

**Resultado:**
- **Mutações** (18, saída em `cadeia.txt` e `refeitas.txt`): 16 acusadas e 2 lacunas, em `docs/PENDENCIAS.md`:
  - `l_limpeza`: a ficha em campo sem motor;
  - `l_direcao`: o cobrador da bola parada sem dono.
- **Jogo real comparando até 2002-01-01:** 0 divergências.
- **Carreiras até 2002-06-01** (Rust ×2 e original): impressão 459815f5e0dca92b.
- **Semente 777 até 2003-06-01** (Rust e original, `semente777.txt`): impressão 17dcb32ab70df0bd.

**Refazer:** sobre o commit da 144ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 145 -`
