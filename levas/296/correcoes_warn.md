# `warn_squad_of_position_rival`: a divergência do jogo real da 296ª leva

Diagnóstico de 14/11/2026. **Nada foi aplicado na pasta principal**: o trecho abaixo é para troca literal.
A troca compila (`copias/bancada5`, `cargo build --release`, 1m26s, só os avisos de sempre).

## Veredito: **(a) comparador**

A cópia está certa (conferida instrução por instrução contra `0x758010`). O que falha é o comparador: a
rotina original, ao rodar primeiro, **faz a atualização de forma do dia** do jogador recém-chegado — que
sorteia duas vezes e grava a data da atualização — e a cópia, rodando depois, encontra o dia já feito e não
sorteia. O comparador não guarda nem devolve esse estado.

## A prova

A rotina chama, para cada companheiro de elenco, `0x62c400` (`exe::injury_get_player_injury_status`) com o
**jogador recém-chegado** (sempre a mesma pessoa):

```
007580e7  mov      ecx, dword ptr [0xb63d28]
007580ed  push     edi                       ; edi = o jogador do argumento
007580ee  call     0x62c400
007580f3  cmp      al, 2
```

`0x62c400` **não é uma consulta**: antes de calcular o estado ela chama `0x62c680`
(`exe::injury_update_player_fitness`, a atualização de forma do dia — é o que a própria
`injury.rs::get_player_injury_status` faz: `exe::injury_update_player_fitness` + `injury_status`).

Em `0x62c680`:

- **sai na hora se a data gravada na linha física da pessoa for a data do jogo** (a atualização é uma vez por
  dia por pessoa):

  ```
  0062c77c  cmp      dx, word ptr [0xae2c92]     ; ano da linha x ano do jogo
  0062c783  jne      0x62c796
  0062c789  cmp      ax, word ptr [0xae2c90]     ; dia da linha x dia do jogo
  0062c790  je       0x62cde1                    ; já feito hoje: volta sem sortear
  ```

- quando faz o trabalho, **sorteia exatamente duas vezes**, e são estes os dois sorteios da mensagem:

  ```
  0062ca3a  movsx    ecx, byte ptr [ebp + 0x5b]  ; profissionalismo da pessoa
  0062ca3e..0062ca4a                             ; edx = profissionalismo / 3 + 2
  0062ca4e  push     edx
  0062ca4f  call     0x90cfc0                    ; sorteio(prof/3 + 2)  -> (6, 4) com prof 12..14
  ...
  0062cb16  push     2
  0062cb18  call     0x90cfc0                    ; sorteio(2)           -> (2, 0)
  ```

- e grava a forma da pessoa (`mov word ptr [ebp+8], di`, `mov word ptr [ebp+0xa], ax` em `0x62cdd3`) e a data
  da atualização; nada disso está nas regiões do comparador.

Com isso a linha da cadeia fecha inteira:

| | sorteios |
|---|---|
| original | `(6, 4)`, `(2, 0)` = a atualização de forma dentro de `0x62c400`; `(4, 3)`, `(5, 2)` = o par do sorteio da rotina (sorteio(n=4) e sorteio(+0x5a=5)) |
| cópia | só `(4, 3)`, `(5, 2)`: a atualização de forma já estava feita (data = hoje), não sorteou |

O ponteiro da tabela de sorteios ficou 8 bytes (= 2 entradas) adiante na original (`0xaba144` × `0xaba13c`)
com o deslocamento igual (`0xd6e` nas duas), e a divergência é acusada no `0x4cf5f0` da **iteração seguinte** —
o primeiro `state before` depois dos dois sorteios. Depois dele o `depois.write_back()` do próprio
`side_by_side` ressincroniza o estado, e por isso os dois sorteios seguintes saem idênticos nas duas.

**Por que só uma vez na carreira:** normalmente a passagem do dia já atualizou a forma de todos antes da etapa
da mídia, e então nem a original nem a cópia sorteiam. Só pega quando a rotina é a primeira do dia a tocar a
forma daquela pessoa (recém-chegado no dia da transferência). É o mesmo defeito que apareceu na confirmação de
uma mutação da 293ª leva ("fora do item (warn_squad_of_position_rival), nao confirma").

## A correção

Não dá para só acrescentar a região: se a atualização de forma encerrar uma lesão ela publica notícia e aloca
(`0x62e700`), e a cópia, repetindo-a sobre o estado devolvido, publicaria de novo. A correção é **fazer a
atualização de forma uma vez, no comparador, antes do retrato**: a chamada da original e a da cópia passam a
encontrar o dia já feito e nenhuma das duas sorteia. O estado do jogo e a sequência de sorteios ficam os
mesmos que a original produz (é exatamente o trabalho que ela faria no primeiro companheiro de elenco com
contrato — o que a guarda confere), só antecipado dentro da mesma chamada; entre a entrada da rotina e aquele
ponto não há sorteio nenhum.

Arquivo: `rust/src/motor/compare/media.rs` (por volta da linha 50).

Trecho antigo:

```rust
/// Comparer for `warn_squad_of_position_rival`.
pub(crate) unsafe extern "thiscall" fn compare_warn_squad_of_position_rival(this: usize, player: usize, club: usize, a: u32, b: u32) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_WARN_SQUAD_OF_POSITION_RIVAL);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
```

Trecho novo:

```rust
/// Whether the original `warn_squad_of_position_rival` reaches its first `exe::injury_get_player_injury_status` call: the
/// gates of `warn_position_rivals_with` up to that call (the routine's own gates, and one squad player other than the
/// newcomer with a contract at the club and positive happiness).
unsafe fn warn_reaches_fitness_update(player: Staff, club: Club) -> bool {
    if player.is_null() || player.player().is_null() || club.is_null() {
        return false;
    }
    if (exe::club_count_squad_with_position_mask(club, exe::staff_get_natural_position_mask(player)) as i8) <= 2 {
        return false;
    }
    let div = club.division().addr();
    if div == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(div)) == 0 {
        return false;
    }
    if player.player().current_reputation() <= 0x128e {
        return false;
    }
    (0..50).any(|i| {
        let p = Staff(club.squad(i));
        if p.is_null() || p.player().is_null() || p == player {
            return false;
        }
        let c = Contract(media_contract_for_club(p.addr(), club));
        !c.is_null() && c.happiness() > 0
    })
}

/// Comparer for `warn_squad_of_position_rival`.
///
/// `exe::injury_get_player_injury_status` is not a query: it first runs the day's fitness update
/// (`exe::injury_update_player_fitness`, 0x62c680), which draws (0x62ca4f, 0x62cb18) and writes the person's fitness and
/// the date of the update; a second call on the same day returns at once (0x62c790) without drawing. Run from here,
/// before the snapshot, the original's call and the copy's both find the day done and neither draws. Nothing moves that
/// the original would not move: it does exactly this work at its first squad player with a contract, which is what the
/// guard checks.
pub(crate) unsafe extern "thiscall" fn compare_warn_squad_of_position_rival(this: usize, player: usize, club: usize, a: u32, b: u32) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_WARN_SQUAD_OF_POSITION_RIVAL);
    if warn_reaches_fitness_update(Staff(player), club) {
        exe::injury_get_player_injury_status(read_usize(G_INJURY), player);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
```

(Só o comparador muda; a cópia da rotina, em `rust/src/motor/media.rs`, fica como está.)

## O que fica para olhar

- A mesma falha de método pode estar em qualquer comparador cuja cópia chame
  `exe::injury_get_player_injury_status` (há dezenas: `database.rs`, `match_man.rs`, `game.rs`,
  `national_teams.rs`, `contract_manager.rs`...). Só dá divergência quando a rotina comparada é a primeira do
  dia a tocar a forma daquela pessoa — e aí o sintoma é sempre este: a original sorteia 2 vezes a mais, com
  `sorteio(prof/3 + 2)` e `sorteio(2)`.
- A alternativa mais limpa a prazo é tratar `0x62c400` como unidade de fita (como `0x72c1f0`, que também
  sorteia), mas isso é um portão novo em `POS_PROLOGUES` e todas as cópias que hoje chamam `0x62c400` direto
  dentro de uma fita passariam a dar "sobrou a written". Não cabe nesta leva.
