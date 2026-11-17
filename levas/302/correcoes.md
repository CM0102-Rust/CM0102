# A 302ª leva: a divergência de um sorteio e a sonda que tornava a comparação irreproduzível (17/11/2026)

A leva passou em tudo na primeira cadeia — **16 de 16 mutações acusadas, 0 escapadas**, carreiras Rust ×2 idênticas
(`be15cf32d7c80775`), detector de lixo igual — menos no jogo real, que trouxe **uma** divergência. Ela não é do código
desta leva, e perseguí-la rendeu um defeito no nosso aparato de teste que afetava todas as comparações.

## 1. A divergência

```
engine: DIVERGENCE contract_manager_offer_contract(person 0x1c6b537e, club 0x1be065ee, slot 589, factor 1, mode 2):
        original 1 rust 1, memory ...
```

Em 2002-145, logo depois dos ramos do campeão da liga. A rotina é `contract_manager_offer_contract` (original
`0x4cda70`), **da 270ª leva**, publicada em 02/11 — não da 302ª.

Comparando os dois retratos de memória byte a byte, as regiões e os bytes que diferem são:

| Região | Bytes diferentes | O que é |
|---|---|---|
| `0xffffe50` (a saída, 0x50 bytes) | +0x38, +0x39, +0x44, +0x4f (nibble baixo) | **exatamente os que o comparador já exclui** como lixo de pilha |
| `0xae1080` | nenhum | — |
| `0xde1f5c` (8 bytes) | offset 4 | **o cursor da tabela de sorteios** (`0xde1f60`): original `0xb8`, Rust `0xb4` |
| `0xad6c30` | nenhum | — |

Ou seja: os termos do contrato batem; o que difere é o cursor, por 4 bytes — **uma entrada, um sorteio**. E o sinal
importa: a corrida da **original** terminou à frente, isto é, ela consumiu um sorteio a mais, não a nossa a menos.

## 2. O que foi descartado (cinco hipóteses)

1. **O alocador da simulação:** `grep "region exhausted"` no `ddraw.log` dá zero — a arena de 1 GB nunca esgotou.
2. **O curto-circuito da checagem de lesão:** o nosso `l != 0 && ...` pula a segunda chamada de
   `Injury::get_player_injury`; a desmontagem mostra que o original pula igual (`test eax,eax; je 0x4ce1bf` em
   `0x4ce1a1`).
3. **Os bytes altos de `status` e `job`:** o original os empurra como dwords com lixo de pilha e nós passamos
   zero-estendido (há um comentário de "simplificação" no código afirmando que só o byte é lido). **O comentário está
   errado** — `status` (`[esp+0x284]`) é lido como dword 7 vezes e `job` (`[esp+0x288]`) 6, e os dwords inteiros são
   empurrados para outras rotinas. Mas não é a causa desta divergência: o único sorteio do caminho não depende deles
   (ver 3). *Fica como pendência à parte: um comentário falso no código.*
4. **Um porte sem o sorteio:** o nosso `staff_contract_set_to_desired_contract` faz o `exe::utils_random(0x1c)`, como
   o original.
5. **Um comparador interno sem restaurar o gerador:** os de `set_to_desired_contract`,
   `add_contract_offer` e `get_squad_status_for_club` foram lidos; os dois primeiros incluem
   `crate::rng::STATE_REGIONS` e repõem; `estimate_squad_status` não sorteia (as chamadas dela, `0x53ca90`,
   `0x5407e0` e `0x7f7490`, não chegam a `utils_random`).

## 3. O sorteio do caminho, e por que ele não explica

Varrendo as 18 rotinas chamadas por `0x4cda70`, **nenhuma sorteia direto**; o único sorteio de todo o caminho é o
`utils_random(0x1c)` em `0x861e43`, dentro de `set_to_desired_contract`. E ele é **incondicional**: o único desvio que
o pularia é `0x861d28` (`mov eax, [esp+0x27c]; test eax, eax; je 0x86237a`), e `[esp+0x27c]` é o **clube** — um
ponteiro, não nulo aqui. Com o frame de `0x274`, `status` fica em `[esp+0x284]` e `job` em `[esp+0x288]`.

## 4. A descoberta: a divergência depende do arranjo de memória

Instrumentei o comparador para medir, por fase, o cursor e quantas vezes a rotina interna rodava. **A divergência não
reproduziu:** a corrida instrumentada chegou a 2002-212 com 0 divergências e nenhuma linha de diagnóstico, e a
impressão da carreira mudou (`3e373fe172e79816` contra `2ce549afd0fa92fa`) — efeito do observador, porque a
instrumentação muda o tamanho da DLL e desloca a memória.

Tirada a instrumentação e **reconstruída a DLL idêntica, na mesma bancada e no mesmo caminho**, também deu 0
divergências. Isto é: a divergência não era determinística nem para o mesmo código no mesmo lugar.

## 5. A causa dessa instabilidade: uma sonda de diagnóstico dentro do caminho comparado

No comparador do `pick` (`motor/compare/match_man.rs`) havia uma sonda que media se zerar o lixo de pilha de
`0x6dbc50` mudava a escala. Sempre que o lixo não era zero, ela chamava a rotina do jogo **uma vez a mais**
(`side_by_side` com o campo zerado), e as regiões dela **não incluíam o gerador**. Como `0x6dbc50` faz **7 sorteios**,
e o lixo não era zero em **13.412 de 26.638** ativações numa carreira, a sonda consumia **cerca de 94 mil sorteios do
fluxo real** por carreira em modo de comparação. A ativação depende do lixo, logo do arranjo de memória: o caminho da
carreira comparada mudava de build para build.

Isso não invalida as comparações anteriores — nas duas fases de cada comparador o fluxo era o mesmo —, mas as tornava
**irreproduzíveis entre builds**, que é exatamente o que faltou para perseguir esta divergência.

A sonda já tinha respondido o que foi medir: **0 de 13.412** casos mudavam ao zerar o lixo, em várias carreiras. Saiu
do código em 17/11/2026, com a medida registrada no lugar dela. Está no git, se precisar voltar.

## 6. O que isso significa para esta leva

A primeira corrida do jogo real da 302 **não é um teste válido do código dela**: ela rodou com a sonda dentro do
caminho comparado, consumindo sorteios do fluxo e tornando o resultado dependente do build. O `cadeia.txt` traz, no
bloco do jogo real, a comparação repetida **sem a sonda**, que é o teste que vale. O resultado da primeira corrida,
com a divergência, está preservado aqui nesta seção.

## 7. O que fica aberto

A causa raiz da divergência de um sorteio **não está fechada**. O que se sabe: é um sorteio, a saída bate, o sorteio do
caminho é incondicional, o porte o faz, os comparadores internos lidos repõem o gerador, e o fenômeno depende do
arranjo de memória. Com a sonda fora, as comparações passam a ser reproduzíveis — então, se ela voltar a aparecer, dá
para persegui-la com a sonda de pilha (`sonda_pilha.py`), que existe no projeto para achar o dword de resíduo que
muda a carreira. Se não voltar em nenhuma leva, era artefato da sonda.

Pendência separada, anotada em 3: o comentário de "simplificação" em `contract_manager_offer_contract` afirma que
`0x85fc50` e `0x597db0` leem só o byte de `status` e `job`. Não é verdade, e o comentário precisa ser corrigido
mesmo não sendo a causa desta divergência.
