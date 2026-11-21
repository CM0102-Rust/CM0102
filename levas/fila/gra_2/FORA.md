*Versão em inglês: [FORA.en.md](FORA.en.md).*

# gra_2: game_daily_staff_update (0x5ce2d0, game.cpp): fora, por enquanto

Leitor gra, rodada 4, 21/11/2026. A rotina tem 30.544 bytes, 9.230 instruções, 630 de x87 e 395 sorteios. Não deu
para portar inteira com segurança numa sessão; ela **não** foi desviada pela metade. Ficaram portados (em
[pedacos.rs](pedacos.rs), lidos da desmontagem byte a byte, compilados sobre o HEAD 1b33a4990 sem aviso novo, nunca
rodados) os pedaços abaixo; faltam os dois grandes. Não há `codigo.rs`: o item não entra na fila (nem em `ordem.txt`).

## O mapa da rotina

| Faixa | O que é | Situação |
|---|---|---|
| 0x5ce2d0..0x5ce33c | prólogo; LLIST local (0x68d020, nunca usada; destruída no fim por 0x68d040); laço na tabela de pessoas | falta (trivial) |
| 0x5ce33c..0x5ce400 | por pessoa: pula sem nação (+0x1a) ou com o estado +0xb & 0x30 == 0x20; clube = o do contrato (`get_contract`) ou o emprego se o cargo é 5 ou 12; `club_get_linked_b_team(emprego, &local, 1)` (escondido no pseudo-C; o local é lido em 0x5d2392); contrato do clube (0x4cf5f0) | falta (pequeno) |
| 0x5ce400 | sem ficha de jogador: 0x5d3367 | — |
| 0x5ce406..0x5ce4b6 | jogador: `daily_wear` (0x62c2f0); o portão (3 dias do ano pelo id, ou contrato/divisão, ou reputação ≥ 0x128e, ou id % 7 + dia == 0); fora dele, a próxima pessoa | falta (pequeno) |
| 0x5ce4b6..0x5d2392 | o dia do jogador: moral, queixas (+0x45 bit a bit), pedidos de transferência, imprensa, contrato (~16 KB, dezenas de `_ftol` e comparações de float) | **falta** |
| 0x5d2392..0x5d352d | as queixas com clube, o status no elenco (sobe por sorteio com `_ftol`), a vontade de voltar à nação [inferência] (dias de contrato × moral em double) | **falta** |
| 0x5d3367..0x5d352d | o não jogador | `staff_day_non_player` |
| 0x5d3539..0x5d377a | o desempregado perde reputação | `staff_day_unattached_decline` |
| 0x5d3786..0x5d38f9 | cansaço no treino (pow do profissionalismo) | `staff_day_training_condition` |
| 0x5d38f9..0x5d393e | o portão dos 3 dias do ano (id % 365, +120, +240); fora: 0x5d578d | falta (trivial) |
| 0x5d393e..0x5d3a0c | as duas tabelas de habilidade do dia (malloc, qsort) | `staff_day_build_ability_tables` |
| 0x5d3a13..0x5d3a23 | `game_average(G_PLAYER_STATS, 0)` guardado em [esp+0x4c] (escondido no pseudo-C; lido no desenvolvimento) | falta |
| 0x5d3a29..0x5d3a8d | caixa de erro da linha 0x1901 (sem ficha): morta, a ficha já foi testada | — |
| 0x5d3a92..0x5d3af0 | a idade em que o desenvolvimento vira, pela posição | `staff_day_development_age` |
| 0x5d3af0..0x5d3d09 | a deriva da personalidade (1 vez em 10) | `staff_day_personality_drift` |
| 0x5d3d09..0x5d5771 | o desenvolvimento da habilidade e dos atributos (~6,8 KB, quase todo o x87: idade em float, `clamp_char_sum` 0x5d61d0, `pow_double` 0x5d5a20, a habilidade nova, a marca "mudou" em [esp+0x28]) | **falta** |
| 0x5d5771..0x5d578d | normaliza os atributos (0x538bc0), apaga relações (0x547f90), a marca do dia [esp+0x67] | falta (trivial) |
| 0x5d578d..0x5d5864 | a lista de observação dos clubes | `staff_day_shortlist_check` |
| 0x5d587a..0x5d58e0 | fim do dia: as habilidades devolvidas na ordem | `staff_day_hand_back_abilities` |

Também: `staff_day_add_morale`, a trava -100..100 da moral do contrato que a rotina repete umas 60 vezes.

## Armadilhas do pseudo-C achadas

- argumentos escondidos: `club_get_linked_b_team` (0x5ce3cd), o `_ftol(random(dias - 360) × 0,25)` (0x5d36a4), o
  `game_average` (0x5d3a1b), o `get_shortlist_manager` (0x5d5ab0) cujos argumentos ficam na pilha para o 0x842c00;
- no desenvolvimento (0x5d3d09..0x5d5771) as chamadas de `clamp_char_sum` (0x5d61d0) e de `random` saem com os
  argumentos embaralhados no pseudo-C; só a desmontagem serve.

## Defeitos do original

1. O laço para um registro antes: o fim é base + (n − 1) × 0x6e com `<`, então a última pessoa da tabela nunca tem
   o dia.
2. A idade do desenvolvimento testa "defensor ≠ 0" (e não "≥ 15"): quase todo jogador de linha volta a 31 anos,
   desfazendo as idades do meio-campo e do ataque.

## O que impede o desvio, além do tamanho

O comparador de passagem (só a original roda), como os irmãos do game.cpp (`compare_game_update_squad_relationships`),
não confere nada: com ele uma cópia errada passa na cadeia (Rust ×2 só prova o determinismo). A rotina chama dezenas de
rotinas com efeito fora das pessoas (notícias, contratos, listas de observação, `0x588cd0`, `0x744240`, `0x72c1f0`),
então o lado a lado também engana. Antes de desviar: um comparador de verdade (p. ex. os pedaços puros lado a lado com
o estado da pessoa e o gerador, e o resto conferido por um plano), ou aceitar a passagem com mutações fortes.

## Para continuar

Os dois blocos que faltam, cada um com a desmontagem na mão (`desmonta.py jogo/cm0102.exe 0x5ce4b6 0x5d352d` e
`0x5d3d09 0x5d5771`), um pedaço por vez; depois o laço (0x5ce33c..0x5ce4b6, 0x5d38f9, 0x5d5771) e o desvio. A rotina
roda todo dia na carreira ENG (o desenvolvimento só nos 3 dias do ano de cada jogador).
