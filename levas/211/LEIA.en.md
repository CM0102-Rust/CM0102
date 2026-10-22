# Batch 211 — closing the contract offers of a transfer offer, the free transfer queue and the player exchange negotiations; the player leaving the national side, the fixture time slice and the shortlist fee ceiling; the offers for a player, the club target reputation and the fixture in the news item

**Routines:**
- `fechar_propostas_da_oferta()` = 0x8c6db0 (transfer_manager.cpp, stdcall): closes (state 4) the contract offers linked to the transfer offer: the target player and the exchange players;
- `enfileirar_transferencia_livre()` = 0x8c16b0 (transfer_manager.cpp, thiscall): queues the player for a free transfer (id, reason and compensation), refusing players out on loan;
- `negociacoes_de_troca_do_jogador()` = 0x8c5fc0 (transfer_manager.cpp, thiscall): counts the open negotiations of the player's club that include him in a part exchange and, if asked, lists the offer indices;
- `tirar_jogador_da_selecao()` = 0x775ba0 (national_teams.cpp, thiscall): takes the player out of his level pool in the nation and out of the squad, clearing his national team fields;
- `faixa_do_horario()` = 0x5af2f0 (fix_man.cpp, thiscall): the fixture time slice (1 weekend, 2 otherwise) by competition nation and weekday;
- `teto_da_observacao()` = 0x84b230 (shortlist_manager.cpp, stdcall): the shortlist slot fee ceiling: club reputation times a factor (or -1);
- `ofertas_pelo_jogador()` = 0x8c5cf0 (transfer_manager.cpp, thiscall): counts the transfer offers for a player (skipping closed ones unless asked) and may list their indices;
- `reputacao_alvo_do_clube()` = 0x83bf40 (shortlist_manager.cpp, thiscall): the reputation a club aims for in signings, adjusted by manager and chairman, between the club floor and 10000;
- `gravar_jogo_na_noticia()` = 0x4afeb0 (comp_util.cpp, cdecl): writes the fixture clubs, venue, competition, score and round into 18 consecutive news slots and advances the cursor;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 8 caught. `s_faixa` escaped: in the career the player leaving the national team is never in his level band with the level mark below the record one, so the band size never shrinks (the comparator covers the whole nation record, where the size lives). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 210's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 211`
