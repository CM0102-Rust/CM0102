# A 307ª leva: duas divergências que dependem do arranjo de memória (18/11/2026)

A leva passou nas mutações (**10 de 10 acusadas**) e nas carreiras (Rust ×2 iguais, `31731bcff4c2f351`, detector de
lixo igual). O jogo real (ENG KOR ARG ITA FRA ESP AUS até 2002-08-01) rodou três vezes, com o mesmo código da leva:

| Rodada | Bancada | DLL | Resultado |
|---|---|---|---|
| 1 | 3 | a da leva | 1 divergência, 2002-182: `post_team_media_story(club 0x1bf176c7, kind 0x5b)` |
| 2 | 4 (a da cadeia) | a mesma | 1 divergência: `contract_manager_offer_contract(person 0x1c5c53ce, ...)` |
| 3 | 3 | a da leva + o diagnóstico de bytes no comparador | **0 divergências** (impressão `570739b553297b4b`) |

Nenhuma das duas rotinas é desta leva: `post_team_media_story` é da 294ª, `contract_manager_offer_contract` da 270ª.

## O que se sabe

- **`post_team_media_story`:** os sorteios da cópia são prefixo dos da original e o registro no log bate; difere só o
  conteúdo da notícia entregue ao humano. A resposta automática das férias (`0x784100`) não trata o tipo `0xbdf` e
  fica descartada. O comparador passou a escrever os bytes diferentes (deslocamento, entregue, montado), para que a
  próxima ocorrência diga o campo.
- **`contract_manager_offer_contract`:** é a mesma divergência da 302ª (ver `levas/302/correcoes.md`), lá atribuída a
  uma sonda de diagnóstico que consumia sorteios. A sonda saiu e a divergência voltou: aquela explicação não basta.
- O mesmo código, em pastas diferentes (o arranjo de memória muda com a pasta), dá uma divergência diferente ou
  nenhuma. A hipótese é uma causa comum: um resíduo de pilha ou de memória lido por alguma rotina, original ou porte.

## O que fica aberto

As duas divergências **não estão explicadas**. A leva publica pela terceira rodada, como a 302ª, e a investigação
segue com a sonda de pilha (`sonda_pilha.py`) e com o diagnóstico de bytes do comparador.
