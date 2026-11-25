*Versão em inglês: [FORA.en.md](FORA.en.md).*

# gra_2b: game_daily_staff_update (0x5ce2d0, game.cpp): fora, por enquanto

> **25/11/2026 (leitor gra, rodada 6):** continuado e entregue no [gra_2c](../gra_2c/rotinas.py): a rotina inteira desviada, o desenvolvimento e o laço portados, provados pela emulação da rotina inteira (`check_routine.py`). Este `pedacos.rs` fica só como histórico.

Leitor gra, rodada 6, 25/11/2026, continuando o [gra_2](../gra_2/FORA.md). A rotina **não** foi desviada nem entra na
fila: falta o desenvolvimento (0x5d3d09..0x5d5771), e o desvio é da rotina inteira. Não há `rotinas.py`,
`codigo.rs`, `mutacoes.py` nem `PRONTO.txt`.

## O que avançou

O dia inteiro do jogador com contrato, **0x5ce4b6..0x5d352d** (os dois terços da rotina), está portado em
[pedacos.rs](pedacos.rs), lido da desmontagem e provado por emulação. Junto ficam os pedaços do gra_2 (o não jogador,
a queda do desempregado, o cansaço no treino, as tabelas de habilidade, a idade da virada, a deriva da personalidade,
a lista de observação e a devolução das habilidades), que seguem sem emulação.

| Faixa | Pedaço (Rust) |
|---|---|
| 0x5ce4b6..0x5ce9f9 | `staff_day_unrest_odds`: o companheiro favorito, o cansaço e a forma, o status do jovem, as chances do sorteio da inquietação |
| 0x5ce9f9..0x5cf106 | `staff_day_grievance_unrest`: cada queixa sorteia contra as suas chances |
| 0x5cf106..0x5cf391 | `staff_day_unrest_roll`: o presidente sem paciência, o relatório de treino do assistente |
| 0x5cf391..0x5cf57c | `staff_day_disputes_and_morale`: as disputas, a inquietação que cai, o status de transferência, a moral rumo a 10 |
| 0x5cf57c..0x5cfa3a | `staff_day_contract_moods`, `staff_day_european_flag`: as marcas 0x10, 0x1000, 0x200, 4 e as queixas que vêm e vão |
| 0x5cfa97..0x5d0041 | `staff_day_grievance_fades`: as queixas que passam ou começam, a ida à mídia (0x23) |
| 0x5d0041..0x5d0e27 | `staff_day_youngster_praise`, `staff_day_international_praise`, `staff_day_club_stories` (+ `_criticism`, `_praise`): as histórias da mídia |
| 0x5d0e27..0x5d10e5 | `staff_day_value_demand`: o valor e a marca 8 |
| 0x5d10e5..0x5d1448 | `staff_day_squad_status`: Bosman, aposentadoria, cargos, as garantias, o jogador 0x70 |
| 0x5d1448..0x5d16e8 | `staff_day_wants_move`: a queixa 1 (quer sair) e a volta |
| 0x5d16e8..0x5d2392, 0x5d24e7..0x5d28f8 | `staff_day_contract_talks` (+ `_new_contract`, `_contract_reminder`, `_contract_running_out`, `_going_home`, `_relations`) |
| 0x5d28f8 | o fim dos cargos de fora (queixas 0x10 e 2) em `staff_day_player` |
| 0x5d2392..0x5d352d | `staff_day_settling` (+ `staff_day_homesick`, `sd_learn_language`) |
| (ordem) | `staff_day_player`, `staff_day_part_b`: a sequência dos pedaços com os saltos do original |

Todas as chamadas a rotinas do exe vão pelo endereço do exe (`exe::...` ou `FN_` com `transmute`), inclusive as já
desviadas, para que a emulação as pegue e o jogo passe pelos desvios.

## A prova: emulação (unicorn)

[check_player.py](check_player.py) roda a original a partir de 0x5ce4b6 (com o quadro de pilha montado: clube em
[esp+0x1c], contrato, ficha, id, a sobra [esp+0x70], o time B em [esp+0x7c]) até 0x5d352d/0x5d3539, e a cópia
(`staff_day_player` por uma exportação de teste, [apply_test_build.py](apply_test_build.py)) no **mesmo mundo falso**:
pessoa, ficha, contrato, 10 clubes, nações, continentes, divisões, tabelas de estado, gerentes. As ~80 rotinas chamadas
são simuladas pelo endereço, com valores sorteados na ordem da chamada (iguais dos dois lados se a cópia chamar igual),
as de ponto flutuante devolvidas em st0; as marcas e queixas do contrato mudam de verdade
(`clear_flags`, `set_flags`, `clear_grievances`, `apply_event_flags`). Compara a lista de chamadas com os argumentos
(os de byte ou short comparados só na largura lida pelo chamado) e cada byte escrito no mundo, e a sobra [esp+0x70].

- **8.000 casos, 0 diferenças** (na compilação final do `pedacos.rs`); 5.502 das 6.262 instruções da faixa executadas (a faixa inclui o não jogador,
  0x5d3367..0x5d352d, que este teste não percorre: no dia do jogador, cerca de 94%).
- Não percorridos pela emulação (conferidos só na leitura): 0x5cf367..0x5cf38e, 0x5cf953..0x5cf962, 0x5cfbfe..0x5cfc3d, 0x5cff18..0x5cff58, 0x5d080a..0x5d0823, 0x5d09d4..0x5d0a07, 0x5d0a1f..0x5d0a69, 0x5d0b12..0x5d0b25, 0x5d0bb2..0x5d0bc7, 0x5d0d2f..0x5d0d91, 0x5d0db3..0x5d0e0c, 0x5d1025..0x5d1033, 0x5d16c3..0x5d16d0, 0x5d192c..0x5d19de, 0x5d19ef..0x5d1ae0, 0x5d1b1e..0x5d1b42, 0x5d1cda..0x5d1d07, 0x5d1d25..0x5d1d37, 0x5d1d91..0x5d1e2d, 0x5d1e45..0x5d1ebd, 0x5d1ece..0x5d1ee0, 0x5d236c..0x5d237b, 0x5d2499..0x5d24ae, 0x5d2591..0x5d25a0, 0x5d263a..0x5d2649, 0x5d269d..0x5d26ac, 0x5d2738..0x5d2745, 0x5d2840..0x5d287e, 0x5d29b7..0x5d29c9, 0x5d2d1c..0x5d2d7e, 0x5d2e7c..0x5d2e8b, 0x5d3183..0x5d31b1, 0x5d3235..0x5d3244, 0x5d32cd..0x5d3325.
- Os dias usados abrem de propósito os portões de dia da rotina ((dia + id + k) % m), e 15% dos casos têm id 0.
- **Discrimina:** a mutação `random(5)` → `random(6)` no sorteio de uma queixa acusa diferença já no caso 0.
- Uso: `python check_player.py DLL MAPA [N] [primeiro] [c.txt]`, com a DLL de teste compilada com `/MAP`;
  `c.txt` é a desmontagem compacta da rotina ([compact.py](compact.py)), só para a cobertura. `emu.py` é o motor
  (do sbB).
- Achado da própria ferramenta: o unicorn guarda o código traduzido; o retorno em st0 usa um trecho fixo por tamanho
  de `ret`, nunca reescrito (reescrever deu pilha errada).

## O comparador proposto

O lado a lado no jogo não serve: a rotina mexe em tudo, chama dezenas de rotinas com efeito (notícias, contratos,
listas, mídia) e sorteia centenas de vezes. Proposta: **comparador de passagem** (só a original roda, como os irmãos do
game.cpp) na cadeia, mais a **emulação da rotina inteira** como prova do item (estender o `check_player.py` ao laço
e ao desenvolvimento) e **mutações fortes** que a carreira ENG acuse. Os pedaços puros lado a lado não valem a pena: quase
todos chamam rotinas do exe com efeito.

## O que falta

1. **O desenvolvimento, 0x5d3d09..0x5d5771** (6,8 KB, quase todo x87). Lido até 0x5d50a0; o mapa:
   - 0x5d3d09..0x5d3eba: o fator de idade x = (idade da virada - idade), × 0,25 quando positivo (float em [esp+0x20]);
     abaixo de 0, a queda das reputações: acima de 45 anos as três viram 1; acima de 41, × 0,875/0,875/0,75; com
     x < -3 (`0x96afc0`), × 0,925/0,925/0,825; senão × 0,975^-x e 0,95^-x (`pow` do CRT); com random(fôlego) > 14, x vira
     random(ftol(x × 10)) × 0,1.
   - 0x5d3eba..0x5d3fd7: as estatísticas 6/1, 6/0x11, 1/0x11 e 1/1, uma parte t = (x × 25 + 50) × 0,01 × (1/1) presa a
     0..100, e uma nota ponderada (a média do jogo - 0,25 sem jogos; `game_average` em [esp+0x4c]).
   - 0x5d3fd7..0x5d4384: o crescimento: as marcas [esp+0x48] (byte, contador) e [esp+0x58] (byte), a reputação
     sobe 1,025/1,01 antes dos 22, o crescimento em [esp+0x28] (short) pela tabela de 13 shorts em `0x9cd21c`
     (-15..14) indexada por duas notas presas a 0..12, a fadiga (`injury_get_fatigue`) e a lesão.
   - 0x5d4384..0x5d46b1: com crescimento > 0, o treino: o fator do clube (treino +0x7f, ou reputação), o técnico
     humano ou o treinador de jovens, **o laço do assistente e dos 5 treinadores** (cl de -1 a 4), a divisão, preso a
     0,1..3, e o crescimento final preso a crescimento ± 5 e a 2..25.
   - 0x5d46b1..0x5d4996: o profissionalismo encolhe ou estica o crescimento; com fadiga acima de 1050 a queda é limitada
     a 5% da habilidade; a habilidade sobe random(crescimento); num clube de divisão fraca, a subida acima de um teto
     (reputação da divisão² × 25 + reputação do clube, × 0,01) é cortada (`0x96af50` = -0,9), nunca abaixo da de antes.
   - 0x5d4996..0x5d4ad2: com a habilidade acima de 80 e uma subida maior que random(20), o técnico humano pode receber
     o relatório de treino com elogio (`news_post_training_report`, 1), pelo assistente, conforme a escalação.
   - 0x5d4ad2..0x5d50a0: a habilidade presa ao potencial; os atributos físicos por `clamp_char_sum` (26 chamadas,
     argumentos embaralhados no pseudo-C), por random(crescimento) / fator.
   - 0x5d50a0..0x5d5771: **não lido** (o resto dos atributos; os sorteios random(2) em bytes [esp+0x74] e [esp+0x78],
     cujo dword sai com 3 bytes de sobra de pilha, inofensivos porque `clamp_char_sum` só lê o byte baixo).
   - Armadilhas: quase todo `fstp`/`fld` com `[esp+X]` está deslocado por `push` pendentes (ex. 0x5d4a94 grava em
     [esp+0x4c], não em [esp+0x70]); [esp+0x30] guarda o cargo de clube (0x5d3952); [esp+0x27] vira o contador do laço
     dos treinadores; [esp+0x28] é o crescimento (lido em 0x5d5780 para a marca "mudou", [esp+0x67]).
2. **O laço e as junções**: o prólogo e o portão por pessoa (0x5ce2d0..0x5ce4b6, descritos no gra_2), 0x5d352d (o
   desempregado só sem clube: `staff_day_unattached_decline(p, ficha)`), 0x5d377a (sem ficha, a próxima pessoa), o
   cansaço no treino, o portão dos 3 dias (0x5d38f9), as tabelas, `game_average`, a idade da virada, a deriva, o
   desenvolvimento, 0x5d5771 (`staff_normalise_player_attributes`, `staff_fade_staff_relationships`, a marca), a lista
   de observação e o fim (0x5d5864).
3. Depois: `rotinas.py` (o prólogo de 0x5ce2d0 começa com `push ebp; mov ebp, esp; and esp, -8`, sem relativo),
   `codigo.rs` a partir do `pedacos.rs`, o comparador de passagem, `mutacoes.py` e a prova por emulação da rotina
   inteira.

## Defeitos do original achados nesta leitura

1. **[esp+0x70]**, o bônus do companheiro favorito (0x5ce830): um short nunca iniciado, somado de uma pessoa à
   seguinte; a primeira lê sobra de pilha. A cópia começa com 0 (o `StaffDayRun::slot70`).
2. **Portões que quase nunca abrem**: `(id % 7) + dia == 0` (0x5ce48d e 0x5cfbe8), `(dia % 14) + id == 0` (0x5cf23c),
   `(dia % 10) + id == 0` (0x5cf890), `(dia % 7) + id == 0` (0x5d2485): só para o id 0 (ou o dia 0); o certo seria
   `(id + dia) % k`. Efeito: o reforço do dia do jogador comum, o relatório de treino do jogador inquieto, a perda da
   queixa 0x80000000, a cobrança de time titular e o pedido de aumento da estrela quase nunca acontecem.
3. **0x5d2e6f**: `(queixas & 1) == 0x10000` nunca vale: o sorteio pelo temperamento que tiraria a queixa 0x10000 é
   código morto (só a lista de transferência a tira).
4. **0x5d23a6 e 0x5d23e6**: os testes de ambição e temperamento antes de tirar as queixas 0x10 e 4 são inúteis: em
   todos os ramos decide "deixado de fora mais de uma vez".
5. **O laço dos treinadores (0x5d4454)**: a vaga de treinador vazia repete o treinador anterior (edi mantido), que
   conta duas vezes no fator de treino.
6. O nome `staff_state::INJURY_COUNTER` (+0xa) não combina com o uso aqui: é o contador da inquietação do dia
   (comparado com 50, 15 e habilidade × 0,15, sobe por sorteios das queixas, cai com a moral). Pedido aos leitores de
   campo.

Os defeitos do gra_2 continuam valendo (a última pessoa da tabela sem o dia; a idade da virada com "defensor ≠ 0").

## Para continuar

Ler 0x5d50a0..0x5d5771 (`compact.py` e `desmonta.py jogo/cm0102.exe 5d50a0 5d5771`), portar o desenvolvimento a
partir do mapa acima, juntar o laço, e estender o `check_player.py` à rotina inteira a partir de 0x5ce2d0 (o mundo já
tem quase tudo; faltam a tabela de pessoas, a de fichas para as tabelas de habilidade e os simuladores de
`daily_wear`, `game_average` e `get_shortlist_manager`).
