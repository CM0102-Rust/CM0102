# Auditoria das 8 ligas (madrugada de 15 para 16/11/2026)

Ligas: `ENG ESP ITA FRA BRA JPN AUS USA`; DLL da principal com a 300ª publicada (2.490 rotinas).

## Carreiras até 2003-01-31
- Rust ×2: iguais (impressão `23ede44babb08db7`, partidas 99.398, banco `518b3070cb17e55c`, 15.523.980.521 sorteios),
  determinismo **OK**, detector de lixo **igual**.
- Original (só registrada): parou na mesma data.

## Comparando até 2002-06-01
Data 0 e aleatório 0 divergências; motor: **1 divergência**, em 100 bilhões de comparações.

| Rotina | Linhas | Região | Leva |
|---|---|---|---|
| `add_cup_winners_record` | 1 | `0xae2470` (`G_DAYS_SINCE_SCRATCH_DATE`) | 294ª |

Nenhuma caixa do jogo.

## Comparando `ENG SCO IRL ITA ESP` até 2002-06-01
Data, aleatório e motor com **0 divergências**.

## A divergência e a correção

**Veredito: defeito do comparador; a cópia Rust estava certa.** Única região diferente: `0xae2470`, a data estática de
`exe::date_days_since` (0x54cdb0), com o mesmo dia (306) e o ano diferente — original 1983 (a conta de idade dentro da
renovação), cópia 2000 (a última contagem de dias que ela fez).

`compare_champion_update` roda a cópia como ensaio, sem a renovação, e marca `DRY_RUN_STRAY_SCRATCH_DATE` para o
comparador excluir aquele endereço. Mas `offer_renewal_if_due` (`rust/src/motor/staff_records.rs`) apagava a marca sem
condição, enquanto `exe::date_days_since` só reescreve `0xae2470` **quando os dois anos diferem**: num jogador cujo
contrato começou no ano corrente a marca era apagada e o comparador conferia um endereço que ainda guardava o resíduo
da renovação da original.

A guarda do ano entrou agora, condicionando o apagamento:

```rust
if read_u16(G_CURRENT_DATE_YEAR) != k.date_started_year() {
    DRY_RUN_STRAY_SCRATCH_DATE.store(false, Relaxed);
}
```

**Nota de processo:** esta é a mesma correção que o commit `f90bc84e1` (11/11/2026) anunciou na mensagem sem a ter no
diff — as 13 linhas daquele commit em `staff_records.rs` eram só limpeza mecânica do `padronizar.sh`, e
`git log -S DRY_RUN_STRAY_SCRATCH_DATE` não devolve nenhuma correção. O diagnóstico de 11/11 estava certo; o remendo é
que se perdeu, e a divergência voltou aqui. Detalhes em [correcao.md](correcao.md).

## Prova da correção
Comparação das 8 ligas até 2002-06-01 repetida com a DLL corrigida (bancada 2, 24.032 s): chegou a 2002-151, partidas
63.373, impressão `a60f8fb6966b0913`; **data, aleatório e motor com 0 divergências** e nenhuma linha `DIVERGENCE` no
log da DLL. Auditoria fechada.

## Varredura da mesma classe de defeito
Os cinco comparadores que chamam `exe::crt_qsort` (`goldcup`, `bra_nat_third`, `usa_mls`, `world_cup`,
`scout_manager`) ordenam `Vec` locais, não as tabelas do jogo: nada de estado compartilhado movido. A classe fica
fechada por ora.
