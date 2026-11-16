# Auditoria das 8 ligas de 15/11/2026: a divergência em `add_cup_winners_record`

**Veredito: (a) comparador.** A cópia Rust está certa; o comparador é que deixou de excluir a data
de rascunho de `exe::date_days_since` num caso que ele já deveria cobrir.

**A correção anterior (f90bc84e1) estava incompleta — na verdade nunca entrou no código.** A mensagem
daquele commit descreve exatamente esta correção ("keep the scratch-date exclusion when the contract
started this year"), mas o diff dele só tem trocas mecânicas de tipagem
(`add_contract_morale`, parênteses de `staff::SIZE`, `club.stadium()`); `offer_renewal_if_due` saiu
igual antes e depois (`git show f90bc84e1^:rust/src/motor/staff_records.rs` × `git show f90bc84e1:…`).
Provavelmente foi a armadilha de restaurar o `.rs` por cima antes de commitar. O diagnóstico de 11/11
estava certo; o remendo é que se perdeu.

## A divergência

```
engine: DIVERGENCE add_cup_winners_record(club 0x1bda9094) renovados 12
  (o first in the sheet 0 de 34; away of the band: false)
  original=(... (ae2470, [32, 1, bf, 7, 0,0,0,0]) ...)
  rust=    (... (ae2470, [32, 1, d0, 7, 0,0,0,0]) ...)
```

Única região diferente: `0xae2470` = `G_DAYS_SINCE_SCRATCH_DATE`, a data estática de
`exe::date_days_since` (0x54cdb0). Dia igual nos dois (0x132 = 306); só o ano difere:
original 0x7bf = **1983** (ano de nascimento — a conta de idade dentro da renovação), cópia 0x7d0 =
**2000** (a última contagem de dias que a cópia fez). Todo o resto (reputação, registro do treinador
0xd2, notas do lado, moral dos 34 do elenco, moral dos contratos) bateu.

## Por que o comparador errou

`compare_champion_update` (rust/src/motor/staff_records.rs) roda a cópia como ensaio
(`STAFF_HISTORY_DRY_RUN`): sem históricos, sem notícias, sem encerrar convocação e **sem a renovação**.
Como a renovação do original chama `contract_manager_offer_contract` (0x4cda70), que conta dias por
idade e deixa a data estática em outro ano, o ensaio marca `DRY_RUN_STRAY_SCRATCH_DATE` e o comparador
tira `G_DAYS_SINCE_SCRATCH_DATE` da conferência.

A marca, porém, é zerada **sem condição** no começo de cada `offer_renewal_if_due`
(`rust/src/motor/staff_records.rs:46`), na ideia de que a própria contagem de dias daquele jogador
(`days_between(G_CURRENT_DATE, k + contract::DATE_STARTED)`) reescreve a estática e reancora os dois
lados. Só que ela **não reescreve sempre**. Pela desmontagem de 0x54cdb0:

```
0054cdfa  mov ax, word ptr [ebp + 2]      ; ano de this  (a data atual)
0054cdfe  mov bx, word ptr [esp + 0x22]   ; ano do argumento (a data do contrato)
0054ce05  cmp bx, ax
0054ce08  ja  0x54ce2f                    ; anos diferentes -> grava em 0xae2470
0054ce0a  jb  0x54ceb4                    ; (o outro ramo também grava, em 0x54cebd)
0054ce26  cmp ax, bx
0054ce29  je  0x54cf41                    ; anos IGUAIS -> salta a gravação; só devolve a diferença
...
0054ceb4  cmp ax, bx
0054ceb7  je  0x54cf41                    ; idem no ramo de baixo
```

Com os dois anos iguais, `date_days_since` só devolve a diferença de dias e **não toca** em
`0xae2470`. Foi o que aconteceu aqui: depois do 12º contrato renovado no ensaio, um jogador seguinte
do elenco entrou em `offer_renewal_if_due` com contrato **começado no ano corrente**; a marca foi
zerada, a contagem de dias não reescreveu a estática, e o comparador passou a conferir um endereço que
no original guarda o ano de nascimento (1983) deixado pela renovação que a cópia não fez. Divergência
de ensaio, não de lógica.

(O resto da ordem confere: depois da última renovação do original, as contagens de dias dos jogadores
seguintes são idênticas nos dois lados; se alguma delas reescrevesse a estática, os dois ficariam
iguais — zerar a marca aí está certo.)

## A correção (troca literal)

Arquivo: `rust/src/motor/staff_records.rs`, em `offer_renewal_if_due`.

Trecho antigo:

```rust
    DRY_RUN_STRAY_SCRATCH_DATE.store(false, Relaxed);
    if days_between(G_CURRENT_DATE, k.addr() + contract::DATE_STARTED) <= 300 {
```

Trecho novo:

```rust
    // exe::date_days_since (0x54cdb0) rewrites its scratch date (G_DAYS_SINCE_SCRATCH_DATE) only when the two years
    // differ: with a contract started in the current year the day count below leaves the date of an earlier stray call
    // there, so the rehearsal's mark stays.
    if read_i16(G_CURRENT_DATE + cm_date::YEAR) != read_i16(k.addr() + contract::DATE_STARTED_YEAR) {
        DRY_RUN_STRAY_SCRATCH_DATE.store(false, Relaxed);
    }
    if days_between(G_CURRENT_DATE, k.addr() + contract::DATE_STARTED) <= 300 {
```

É o único ponto que zera a marca (fora da inicialização em `compare_champion_update`), então a
correção vale para os dois comparadores que usam o ensaio (`add_league_champions_record` e
`add_cup_winners_record`).

Compila: aplicado e `cargo build --release` na bancada 5 (1m01s, só os avisos de sempre). **A bancada 5
está com a correção aplicada** (`copias/bancada5/rust/src/motor/staff_records.rs`); a árvore principal
não foi tocada.

## Prova sugerida

A comparação longa das 8 ligas até 2002-06-01 (a mesma da auditoria) com a DLL corrigida; a
divergência era única, e a mutação que a derruba é a própria renovação (`offer_renewal_if_due`), já
coberta pelas mutações do item.

## Conferido e aplicado (16/11/2026)

`git show f90bc84e1 -- rust/src/motor/staff_records.rs` confirma: as 13 linhas daquele commit são só
limpeza mecânica; a guarda do ano não está em nenhum trecho. `git log -S DRY_RUN_STRAY_SCRATCH_DATE`
só devolve as levas de legibilidade, nenhuma correção. A mensagem do commit afirmava o que o diff não
tinha — anotado na memória da sessão para não repetir.

A guarda entrou agora em `rust/src/motor/staff_records.rs`, em `offer_renewal_if_due`:

```rust
// exe::date_days_since (0x54cdb0) only rewrites its scratch date (0xae2470) when the two years differ, so a
// contract started this year leaves the dry run's mark standing: the address may still hold the original's residue.
if read_u16(G_CURRENT_DATE_YEAR) != k.date_started_year() {
    DRY_RUN_STRAY_SCRATCH_DATE.store(false, Relaxed);
}
```

Varredura da mesma classe de defeito nos comparadores: os cinco que chamam `exe::crt_qsort`
(`goldcup`, `bra_nat_third`, `usa_mls`, `world_cup`, `scout_manager`) ordenam `Vec` locais, não as
tabelas do jogo — nada de estado compartilhado movido. A classe fica fechada por ora.

Prova em curso: a comparação longa das 8 ligas até 2002-06-01, na bancada 2, com a DLL corrigida.
