# 144ª leva — o jogador com a bola (0x7109f0)

**Rotina:** `conduzir()` = 0x7109f0 (match_pl.cpp, 13 KB), a decisão de quem está com a bola. Na bola parada, escanteios, lateral e faltas passam para o companheiro escolhido (0x6c6a30) ou para uma casa sorteada; pênalti e falta perto do gol chutam. Em jogo, ela soma as notas do chute, do passe e da condução: distância ao gol, pressão (0x6bcbd0, que enche uma lista na pilha), placar, goleiro, marcas da posição e relógio. Depois lança (0x6fa490), divide (0x70ec80), chuta (0x702050), passa (0x6fa250) ou conduz (0x6f09f0). No modo 4 é o goleiro nos pênaltis, e ela devolve 1 quando ele fica.

**Como foi lida:** a estrutura veio do pseudocódigo do Ghidra e todas as contas do assembly. São 52 `ftol` e 59 sorteios, e o pseudocódigo esconde as expressões de ponto flutuante. A ordem dos operandos de `fsubp` saiu dos bytes (DE E9: st1 − st0). Duas trocas de nota (+0x10 e +0x14) também foram corrigidas pelo assembly, onde há um `push` antes do acesso à pilha.

**Como foi comparada:** pela fita, com comparação dentro da unidade (ela roda como unidade dos pênaltis e da decisão do jogador). O passe, o lançamento e a dividida rodam por dentro da cópia, com as máscaras deles, e as unidades deles entram na fita da condução. O movimento, a narração e o chute (0x702050, ainda original) entram como unidades. A lista da pressão fica na pilha, com endereço diferente na original e na cópia, então a cópia na lista (0x68d0d0) entra sem o endereço, por `lista_gravada`. A cabeça da lista passada ao chute é ignorada.

**Original determinizado:** no modo 4, a nota do passe (lida em 0x712ec4) e o companheiro (lido em 0x713107) nunca são escritos. Além disso, a parte alta dos argumentos de byte do passe e do chute vem sem valor. Por isso a rotina entrou em `ZERADAS`. A carreira original com a zeragem manteve a impressão de referência.

**O que deu errado no caminho:**
- `na_propria_area` já existia e a função nova virou `na_area_defendida`;
- `lista_gravada` dependia de `gravando()`, que é falso na fase da cópia, e passou a depender só da bandeira;
- o primeiro jogo comparando usou a DLL antiga, porque o `simular.py` não copia a compilada. Virou armadilha em `docs/COMO_CONTINUAR.md`.

**Resultado** (`cadeia.txt`):
- Comparando até 2001-08-20: 0 divergências, passando por 30 dos 32 ramos da condução.
- Mutações (24): 23 acusadas e 1 lacuna (`c_goleiro`: o guarda da posição 1 sem a marca de goleiro; em `docs/PENDENCIAS.md`).
- Jogo real comparando até 2002-01-01: 0 divergências.
- Carreiras Rust ×2 e original até 2002-06-01: impressão 459815f5e0dca92b.

**Refazer:** sobre o commit da 143ª, rodar `python integrar.py` (usa `codigo.rs`) e `cargo build --release`, copiar a DLL para `jogo_testes/` e rodar `bash engenharia-reversa/cadeia.sh 144 2001-10-01`.
