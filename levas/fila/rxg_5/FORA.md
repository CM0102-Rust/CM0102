# rxg: o que ficou de fora (leitor rxg, rodada 6, 25/11/2026)

Sem `rotinas.py`: esta pasta só guarda as notas. A lista do rxg eram as 14 rotinas do `rxf_7/FORA.md`. Os itens `rxg_1` a
`rxg_4` cobrem 5: `TRANSFER_MANAGER::load_from_disk`, `Tactics::read_from_save`, `WorldClubChamp::pick_entrant`,
`WorldClubChamp::create_knockout_stage` e `TRANSFER_MANAGER::submit_loan_contract_offer`. As 9 abaixo ficaram por tempo;
nenhuma é entrada falsa, thunk ou corpo vazio, e as notas dos `FORA.md` de origem (`rx10_15`, `rx11b_5`, `rxs1_12`)
continuam valendo.

## SAVE

- **0x52dad0 `save_database`** (15 KB, Database.cpp): o pseudocódigo do Ghidra tem 2,6 mil linhas. A primeira parte troca
  os ponteiros de cada tabela pelo id (`p ? *p : -1`) em todos os campos-ponteiro de competições (+0x59, +0x5d, +0x61,
  +0x65), clubes (umas 70 vagas, entre elas os vetores de +0xd7 (50), +0x1b3 (7), +0x19f (5), +0x1cf (3) e +0xc3 (3)),
  nações, cidades, estádios, pessoas etc., só quando `[0xae2340]` não é 0. Depois, para cada uma das 24 tabelas: a barra
  de progresso (0x802f80 com o passo), a procura da entrada no índice (`0xae1ab3` até `0xae2074`), a caixa "X table
  valid, but no entry in index.dat" quando falta, e a gravação em arquivo (0x54d5b0) ou no zip (0x9326e0 com
  `param_3`/`param_4`) por uma gravadora de cada tabela (0x54d6e0, 0x5506b0...). Pede uma sessão só para ela, em
  pedaços com nome (o molde é o `gra_1`); a prova é a do save (não sei se o `.qst` da prova passa por ela: confira quem
  chama a 0x524d30).

## Simulação

- **0x7a30c0 `player_setup_cfg_load`** e **0x7a3a10 `player_setup_apply_line`** (5,7 KB) e **0x8bf060
  `TRANSFER_MANAGER::add_config_future_transfer`**: só rodam com um `player_setup.cfg` nos `Data/` (contagem 0 na
  carreira de teste e no mundo). Escrever os três num item só tem sentido junto com o arquivo de teste nos dados da
  bancada (uma linha de cada tipo, a transferência futura incluída); sem isso nenhuma mutação é acusada. Pede a decisão
  da sessão principal sobre o arquivo de teste antes.

## TELAS

- **0x843800 `Shortlist_Manager::club_wants_player`**: ver `rx11b_5/FORA.md` (o corpo é quase o do irmão já em Rust).
- **0x8ccb80 `TRANSFER_MANAGER::get_club_future_transfer_list`** (1,5 KB): lido nesta rodada pelo pseudocódigo; o
  Ghidra esconde o `ecx` das `Date::operator=` (0x417c10) que escrevem a data de cada linha, então falta a desmontagem
  linha a linha. Notas: a lista local de vagas (0x68d020) das ofertas vivas do clube (estados 2, 3, 0x14, 0x10, 4 e 0xf
  com +0x2e ≥ 3 ficam fora), a data de hoje (0x8c6100), a categoria pelo estado (1→1; 5..8→2; 0xa→3; 0xb..0xd→4;
  0xe e 0x13→6; 0x11 e 0x12→5; o resto 0), a data da transferência futura (o vetor de +0x920 pela vaga +0x20) ou a da
  oferta (+0x12), o contrato vai para `G_CONTRACT_LOOKUP_RESULT`, e uma linha a mais por jogador da troca; ordena com
  0x945333 (qsort, 0x1b, 0x8e55f0).
- **0x8ce0e0 `TRANSFER_MANAGER::set_player_squad_status`**: ver `rxs1_12/FORA.md`.
- **0x806630 `SearchFilters::set_option_flag`** e **0x807450 `SearchFilters::player_matches_filter`**: ver
  `rx10_15/FORA.md`.
