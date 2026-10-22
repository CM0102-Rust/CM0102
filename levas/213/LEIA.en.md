# Batch 213 — the signing by the nation rules and the exchanges offering the player; the unread news, the slot written into the inbox news item and the human-managed club; the draw and match dates of the cup round and the club removed from the planned friendlies

**Routines:**
- `pode_contratar_jogador()` = 0x8ced40 (transfer_manager.cpp, thiscall): whether the club may sign the player under the nation rules, with the refusal reason and the human club's second try on the foreign player limit;
- `trocas_oferecendo_o_jogador()` = 0x8c5e40 (transfer_manager.cpp, thiscall): how many of the club's open offers include the player in the exchange, listing their indices when asked;
- `noticias_nao_lidas()` = 0x786fe0 (news.cpp, thiscall): the unread news in a human manager's inbox, in total or for one category;
- `gravar_marca_da_noticia()` = 0x786d30 (news.cpp, thiscall): writes a value into a slot of a news item in the human manager's inbox and into its local copy;
- `clube_de_tecnico_humano()` = 0x7875e0 (news.cpp, stdcall): whether a human manager runs the club, ignoring holidays;
- `data_do_sorteio_da_rodada()` = 0x521e60 (Cup.cpp, cdecl): stores the cup round draw date: the day number and the year offset from the base year;
- `data_do_jogo_da_rodada()` = 0x521eb0 (Cup.cpp, cdecl): stores the cup round match date, its match time and the venue flags;
- `tirar_clube_dos_amistosos()` = 0x5c64e0 (friendly.cpp, thiscall): removes the club from the planned AI friendlies and from the friendly lists;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (8):** 7 caught. `tr_recusa` escaped: in the career the transfer rules never refuse a signing asked without the second chance (a5 = 0), so the "refused, no second question" path does not run (not even in comparison mode). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (5afd987e1a1a3ef3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b11bd5cb63f51d72).

**Redo**, on top of batch 212's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 213`
