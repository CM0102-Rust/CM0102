# rxf: o que ficou de fora (leitor rxf, rodada 6, 25/11/2026)

Sem `rotinas.py`: esta pasta só guarda as notas. A lista do rxf juntava as sobras de `rx11b_5`, `rxs1_12` e `rx10_15`
(24 rotinas). Os itens `rxf_1` a `rxf_6` cobrem 10: o gerente de prêmios inteiro (construtor, destrutor, carga,
`create_nation_awards`, gravação, texto da lista de indicados e os dois destrutores da base `AWARD`), o
`Shortlist_Manager::load` e o `WorldClubChamp::pick_entrants`. As 14 abaixo ficaram por tempo; nenhuma é entrada falsa,
thunk ou corpo vazio, e nenhuma está no `rust/src` nem na fila (conferido em 25/11 pelo `Prologue`/`ROTINAS`). As notas de
cada uma continuam valendo nos `FORA.md` de origem.

## Simulação

- **0x92ae60 `WorldClubChamp::pick_entrant`** e **0x92a140 `WorldClubChamp::create_knockout_stage`**: ver
  `rx10_15/FORA.md`. A assinatura do `pick_entrant`, conferida nos `push` do `pick_entrants` (rxf_6): thiscall(continente,
  nação, lista, vaga, competição de clubes) ret 0x14; a vaga 5 passa o id da Copa da UEFA (`[0x9cf6f0]`), as outras -1; a
  vaga 11 passa (-1, sede).
- **0x7a30c0 `player_setup_cfg_load`** e **0x7a3a10 `player_setup_apply_line`**: ver `rx10_15/FORA.md` (pedem um
  `player_setup.cfg` de teste nos dados da bancada).
- **0x8bf060 `TRANSFER_MANAGER::add_config_future_transfer`**: ver `rxs1_12/FORA.md` (só roda com o mesmo arquivo de
  configuração dos jogadores: junto com a 0x7a3a10 seria a mesma prova).

## SAVE

- **0x8985c0 `Tactics::read_from_save`** (4 KB, CARGA): ver `rx11b_5/FORA.md`.
- **0x8bc020 `TRANSFER_MANAGER::load_from_disk`** (5,2 KB, CARGA, depois do rxs1_11): ver `rxs1_12/FORA.md`. Ela chama o
  construtor do `Shortlist_Manager` (0x839a80, sx20_5), que chama o `rxf_5`.
- **0x52dad0 `save_database`** (15 KB): ver `rxs1_12/FORA.md`.

## TELAS

- **0x843800 `Shortlist_Manager::club_wants_player`**: ver `rx11b_5/FORA.md`.
- **0x8c0f40 `submit_loan_contract_offer`**, **0x8ccb80 `get_club_future_transfer_list`** e **0x8ce0e0
  `set_player_squad_status`**: ver `rxs1_12/FORA.md`.
- **0x806630 `SearchFilters::set_option_flag`** e **0x807450 `SearchFilters::player_matches_filter`**: ver
  `rx10_15/FORA.md`.
