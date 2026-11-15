# Correções das divergências do jogo real (296ª leva e bancada 2 / 297ª)

Diagnóstico de 14/11/2026. **Nada foi aplicado**: os trechos abaixo são para troca literal.
As três trocas compilam juntas (`copias/bancada5`, `cargo build --release`, 1m05s, só os avisos de sempre).

| # | Rotina | Origem | Arquivo |
|---|---|---|---|
| 1 | `write_player_praise_news_text`, pedaço `praise_body_on_the_rise` | **publicada** (leva 294, item nma_4) | `rust/src/motor.rs` |
| 2 | `eng_charity_select_teams` | **publicada** (EngCharity::select_teams) | `rust/src/motor/comp/cups/eng_cc_cup.rs` (comparador) |
| 3 | `nir_charity_select_teams` | leva 296 (item mwa_5/mwa_6) | `rust/src/motor/comp/cups/spa_super.rs` (comparador) |

As 2 e 3 são **o mesmo defeito**, num comparador comum; a correção é uma por arquivo de comparador.

---

## 1. `praise_body_on_the_rise`: as variantes 1 e "as outras" estão trocadas

### Causa

Uma frase: **o ramo da variante 1 e o ramo das demais variantes estão invertidos** — a cópia escreve o texto do
comentarista de televisão onde a original escreve a reportagem do jornal nacional, e vice-versa.

Prova na desmontagem de `0x7461de..0x746414` (o início do pedaço, o `switch` da variante):

```
007461de  mov  eax, [esp + 0x28]      ; eax = variante
007461e2  sub  eax, 0
007461e5  je   0x7463c0               ; variante == 0  -> 0xa417cc
007461eb  dec  eax
007461ec  mov  eax, [esp + 0x20]      ; eax = home (não mexe nos flags)
007461f0  je   0x7462ea               ; variante == 1  -> 0x7462ea
007461f6  test eax, eax
007461f8  je   0x746256               ; as outras, fora de casa -> 0xa41b28
007461fa                              ; as outras, em casa      -> 0xa41c6c
```

e, em `0x7462ea`, `test eax,eax; je 0x746344` separa a variante 1:
`0x7462ee` (em casa) usa o formato **`0xa41a04`** e `0x746344` (fora, adjetivo forma `0x16`) usa **`0xa418c0`**.

Isto é, o `je 0x7462ea` dispara com `variante − 1 == 0`, e é o ramo do jornal:

| variante | em casa | fora | texto |
|---|---|---|---|
| 0 | `0xa417cc` | — | "A media pundit has indicated that he believes …" |
| **1** | **`0xa41a04`** | **`0xa418c0`** (adjetivo forma **0x16**) | "A report in a national paper has detailed the aspects of …" |
| **as outras** | **`0xa41c6c`** | **`0xa41b28`** (adjetivo forma **0x1c**) | "A television pundit said that he believes …" |

A cópia tem exatamente o contrário. A lista de argumentos de cada `sprintf` (contagem, ordem e os `0x64`
intercalados), a forma do adjetivo e o destino (`HEAD`, `esp+0xcd4` nos quatro rabos `0x749bba`, `0x749ade`,
`0x749643` e o `call 0x66f4e0` embutido em `0x7462dd`) conferem instrução por instrução nos quatro ramos; só os dois
ramos estão no lugar errado. Por isso o comparador acusou `text false` e `memory false` (o `memory` é o buffer
`0xde1f64`, onde o texto é montado) com `links true`: os vínculos não dependem do parágrafo.

### Correção

`rust/src/motor.rs`, cabeçalho de `praise_body_on_the_rise` (por volta da linha 8672).

Trecho antigo:

```rust
/// Kind 0x2d, "{player} on the rise": a media pundit's belief (variant 0), a television pundit's (variant 1), or a
/// national paper's report on his strengths (the others), "{nationality} ..." abroad.
```

Trecho novo:

```rust
/// Kind 0x2d, "{player} on the rise": a media pundit's belief (variant 0), a national paper's report on his strengths
/// (variant 1), or a television pundit's praise (the others), "{nationality} ..." abroad.
```

`rust/src/motor.rs`, corpo de `praise_body_on_the_rise` (por volta da linha 8687).

Trecho antigo:

```rust
        1 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41c6c, cg, cg, g, g, cname, full, short, pos);
            } else {
                p.adjective(0x1c);
                let g = p.describe();
                f(h, 0xa41b28, cg, cg, g, g, 0x64, 0x64, cname, full, short, pos, adj);
            }
        }
        _ => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41a04, g, g, 0x64, lg, lg, pos, full, lname);
            } else {
                p.adjective(0x16);
                let g = p.describe();
                f(h, 0xa418c0, 0x64, g, g, 0x64, lg, lg, adj, pos, full, lname);
            }
        }
```

Trecho novo:

```rust
        1 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41a04, g, g, 0x64, lg, lg, pos, full, lname);
            } else {
                p.adjective(0x16);
                let g = p.describe();
                f(h, 0xa418c0, 0x64, g, g, 0x64, lg, lg, adj, pos, full, lname);
            }
        }
        _ => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41c6c, cg, cg, g, g, cname, full, short, pos);
            } else {
                p.adjective(0x1c);
                let g = p.describe();
                f(h, 0xa41b28, cg, cg, g, g, 0x64, 0x64, cname, full, short, pos, adj);
            }
        }
```

(Só os dois corpos trocam de lugar; nenhum argumento muda.)

---

## 2 e 3. `eng_charity_select_teams` e `nir_charity_select_teams`: o comparador não devolve a tabela de história

### Causa

Uma frase: **a original, ao rodar primeiro, deixa a tabela global de história das competições reordenada (o
`qsort` de dentro de `comp_util_get_latest_placed_club`), e os dois comparadores não guardam nem restauram essa
tabela — então a cópia procura numa tabela com outra ordem e pode escolher outro clube.**

Como se chegou nisso:

1. As duas rotinas foram conferidas instrução por instrução contra a original e **estão certas**:

   - `EngCharity::select_teams` (`0x56d780`): `+0x3e = 2`; `malloc(0xc)` em `+0xa7` (erro linha `0xc5`, −1);
     campeão da Premier `[0x9cf5bc]` lugar 0 na vaga 0 (erro `0xce`); vencedor da FA Cup `[0x9cf74c]` lugar 0 na
     vaga 1, `[lista+6]` (erro `0xd7`); `cmp [eax], ecx` — se forem o mesmo clube, a **vaga 1** recebe
     `[0x9cf5bc]` lugar **1** (erro `0xe3`); classe e situação 0 nas duas vagas (`+4`, `+5`, `+0xa`, `+0xb`);
     devolve 1. É o que a cópia faz.
   - `NirCharity::select_teams` (`0x78a1e0`): mesmo molde (`SpaSuper`), erros `0xc2`, `0xcb`, `0xd4`, `0xe0`;
     vencedor da Irish Cup `[0x9cf8c0]` na vaga 0, campeão da Premier da Irlanda do Norte `[0x9cf8b0]` na vaga 1 e,
     no empate, a **vaga 0** recebe `[0x9cf8c0]` lugar 1 (`mov dword ptr [edx], eax`, bytes `89 02`). É o que a
     cópia faz. As 12 vagas de byte da lista são escritas pelas duas, sem buraco.

2. Logo, para as 12 bytes saírem diferentes, os ponteiros de clube têm de sair diferentes — e eles só vêm de
   `comp_util_get_latest_placed_club` (`0x4afd80`), que é a **única** coisa que as duas chamam e que **mexe em
   estado global**:

   ```
   004afd86  mov eax, [0xae237c]      ; contagem da história
   004afd8b  mov ecx, [0xae23d4]      ; a tabela
   004afd9b  call 0x945333            ; qsort(tabela, contagem, 0x1a, 0x540780)  <-- ordena no lugar
   004afdcb  call 0x946464            ; bsearch da competição com ano 0xffff
   004afde5..004afe00                 ; anda para trás até a primeira linha "igual"
   ```

   O `database_compare_comp_history_rows` (`0x540780`) trata **ano `-1` como "qualquer ano"**: a chave casa com
   todas as linhas da competição, e linhas que casam entre si ficam empatadas. Com empate, a ordem que o `qsort`
   deixa é arbitrária, e é ela que decide em qual linha a volta para trás para — ou seja, **o resultado depende da
   ordem do vetor**.

3. O comparador `comp_util_get_latest_placed_club` faz certo: ele põe a tabela nas regiões do `side_by_side`,
   restaura antes de rodar a cópia e devolve o estado da original no fim. **O problema está um nível acima**: em
   `compare_team_list` (spa_super.rs) e `eng_cc_cup_compare_entrant_list` (eng_cc_cup.rs) as regiões são só
   `cup::TEAM_COUNT` (2 bytes) e `[0xb67a34]` (4). A original roda e **deixa a tabela ordenada**; a cópia roda
   depois com a tabela já ordenada — ordem diferente da que a original viu. Por isso `memory true` (as duas
   regiões conferem) e `list false`.

4. Por que só agora: a escolha dos times da Supercopa só roda na virada de temporada, em julho, logo depois de a
   virada ter acrescentado linhas novas no fim da tabela (é aí que a tabela chega desordenada ao primeiro
   `qsort`). As carreiras até 2002-01-01 / 2002-06-01 nunca chegavam lá. O único irmão que roda em agosto nas
   carreiras antigas, `eur_super_cup_select_teams`, não tem o ramo do "mesmo clube" e lê a tabela já ordenada pela
   chamada anterior do dia.

### Correção

A correção é no comparador, não nas rotinas: fazer o `side_by_side` guardar e devolver também a tabela de
história, para a cópia procurar exatamente na tabela que a original procurou. Isso vale para todas as rotinas que
usam os dois comparadores (Supercopas da Espanha, do Japão, da Ásia, da Itália, da Europa e as duas Supercopas das
ilhas).

#### 2a. `rust/src/motor/comp/cups/eng_cc_cup.rs` — a região nova (ao lado de `latest_placed_club`, linha ~398)

Trecho antigo:

```rust
/// Club that finished at place (0 champion, 1 runner-up) last season in competition id (`comp_util_get_latest_placed_club`).
pub(crate) unsafe fn latest_placed_club(id: usize, place: u32) -> usize {
```

Trecho novo:

```rust
/// The competition history table as a memory region, for the comparers that run a routine calling
/// `latest_placed_club` twice: `comp_util_get_latest_placed_club` sorts the table (`exe::crt_qsort`), so without
/// restoring it the copy searches a table ordered differently from the one the original searched.
pub(crate) unsafe fn comp_history_region() -> (usize, usize) {
    (read_usize(G_CLUB_COMP_HISTORY_TABLE), 0x1a * read_i32(G_CLUB_COMP_HISTORY_COUNT).max(0) as usize)
}

/// Club that finished at place (0 champion, 1 runner-up) last season in competition id (`comp_util_get_latest_placed_club`).
pub(crate) unsafe fn latest_placed_club(id: usize, place: u32) -> usize {
```

#### 2b. `rust/src/motor/comp/cups/eng_cc_cup.rs` — `eng_cc_cup_compare_entrant_list` (linha ~344)

Trecho antigo:

```rust
    let (a, b, igual, _, _) = crate::comparison::side_by_side(reg, || {
        let r = o();
        lo.set(this.team_table());
```

Trecho novo:

```rust
    let mut regs = [(0usize, 0usize); 8];
    let n = reg.len().min(7);
    regs[..n].copy_from_slice(&reg[..n]);
    regs[n] = comp_history_region();
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&regs[..n + 1], || {
        let r = o();
        lo.set(this.team_table());
```

(Nenhum chamador passa mais de duas regiões; o `min(7)` é só para o vetor fixo não estourar.)

#### 3. `rust/src/motor/comp/cups/spa_super.rs` — `compare_team_list` (linha ~77)

Trecho antigo:

```rust
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
```

Trecho novo:

```rust
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), comp_history_region()];
```

### O que muda

Nada no jogo: as três trocas do bloco 2/3 só mexem no que o comparador guarda e devolve entre a rodada da
original e a da cópia. A rodada da cópia passa a ver a tabela de história na mesma ordem que a original viu, e o
estado final continua sendo o que a original deixou (o `side_by_side` escreve `after_orig` no fim).

### Depois de aplicar

- Vale conferir o contador `cqm_10 english double` / `mwa_5 northern irish double` na carreira comparando: é o ramo
  que destapou o defeito.
- Se mesmo assim a lista divergir, o próximo suspeito é o `qsort` do jogo (`0x945333`) com o comparador intransitivo
  de `0x540780`; nesse caso o comparador precisaria registrar a linha escolhida, não só os 12 bytes.

## Próximo passo (sessão principal, 14/11/2026)

Este é o **quarto** comparador com a mesma falha de método: não devolver ao estado anterior algo que a original
mexe antes de a cópia rodar (os outros: os construtores de copa ao carregar um save; o `+0x3a` que a tabela de
rodadas reescreve; a constante que servia de referência à cópia e ao comparador ao mesmo tempo). Vale uma varredura
nos comparadores lado a lado procurando chamadas a rotinas do exe que ordenam, alocam ou gravam em tabela global
(`exe::crt_qsort`, `exe::crt_operator_new`, as `*_history_*`) sem a região correspondente no `side_by_side`.
