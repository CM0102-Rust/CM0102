# Batch 218 — the saved game container: the queries (name pattern, id and item size); the saved game container: packing and unpacking the folder files and clearing the temporary folder; the saved game container: reading, writing and closing an item (the type 4 RLE)

**Routines:**
- `casar_nome_com_padrao()` = 0x933a00 (zipdir.cpp, cdecl): matches a container item name against a pattern with `*` (`*.` goes to the last dot), case-insensitively; the "{ Empty }" slot never matches;
- `id_do_conteiner()` = 0x9331c0 (zipdir.cpp, cdecl): reads the container header and returns dword +4 (0x16 .sav, 0x17 .qst; 0x14 on failure);
- `tamanho_dos_itens_do_padrao()` = 0x933860 (zipdir.cpp, cdecl): adds up the uncompressed sizes of the items matching the pattern (signed int: negative above 2 GB, as in the original);
- `empacotar_no_conteiner()` = 0x933610 (zipdir.cpp, cdecl): each folder file matching the pattern (FindFirstFile) becomes a container item: zopen for writing, fread/zwrite in 0x30d40 blocks, zclose;
- `desempacotar_do_conteiner()` = 0x933320 (zipdir.cpp, cdecl): each container item whose name matches the pattern becomes a folder file (zopen for reading, zread in 0x30d40 blocks, fwrite);
- `limpar_pasta_temporaria()` = 0x90cd50 (utils.cpp, cdecl): builds the CM3_TEMP folder path in [0xde279c] and deletes its files matching the pattern, in groups of 100;
- `ler_do_conteiner()` = 0x932c30 (zipdir.cpp, cdecl): reads from the item like fread, clipped at the item end; on type 4 decodes the RLE (a run may span calls);
- `gravar_no_conteiner()` = 0x932e10 (zipdir.cpp, cdecl): writes to the item like fwrite and adds to the size in the index; on type 4 encodes the RLE (runs of up to 0x7f kept across calls);
- `fechar_conteiner()` = 0x932b00 (zipdir.cpp, cdecl): when writing: flushes the buffer, closes the pending RLE run and rewrites header and index at the start; closes the file and frees the item;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own; the mutations also went through it): EQUAL fora dos campos ignorados.
- **Mutations (9):** 6 caught. `sv_padrao_letra` escaped: the name pattern match (0x933a00) only runs when loading (unpacking "*.lng" and "*.tmp"); the proof writes the .qst and the load step needs a save with a manager on holiday, which does not exist yet. `sv_padrao_ponto` escaped: the name pattern match (0x933a00) only runs when loading (unpacking "*.lng" and "*.tmp"); the proof writes the .qst and the load step needs a save with a manager on holiday, which does not exist yet. `sv_padrao_fim` escaped: the name pattern match (0x933a00) only runs when loading (unpacking "*.lng" and "*.tmp"); the proof writes the .qst and the load step needs a save with a manager on holiday, which does not exist yet. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 217's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 218`

**Fields touched:**
- container (file) +0: type (3 uncompressed, 4 RLE; old 1/2 become 3/4); value; reads (id_do_conteiner, tamanho_dos_itens_do_padrao)
- container (file) +4: archive id (0x16 .sav, 0x17 .qst); value; reads (id_do_conteiner)
- container (file) +8: item count; value; reads (tamanho_dos_itens_do_padrao)
- container index (+0xc, 0x10c per item) +4: uncompressed item size; value; reads and adds as signed (tamanho_dos_itens_do_padrao)
- container index +8: item name (up to 0x104 bytes); text; reads (tamanho_dos_itens_do_padrao, casar_nome_com_padrao)
- [0xad6ad8] "{ Empty }" (empty slot); text; reads (casar_nome_com_padrao)
- [0xaa2156] upper-case table (3 bytes per character, via 0x90c2f0); value; reads (casar_nome_com_padrao)
- [0xb67a34] flag cleared after the error box; value; writes 0 (id_do_conteiner, tamanho_dos_itens_do_padrao, out of memory)
- ZFILE (0xa3191 bytes, from zopen) +0x41705: read buffer (0x30d40), used as the block read from the file; bytes; writes (empacotar_no_conteiner)
- ZFILE +0x72445: write buffer (0x30d40), used as the block read from the item; bytes; writes (desempacotar_do_conteiner)
- container (file) +0/+8: type (old 1/2 become 3/4) and item count; value; reads (desempacotar_do_conteiner)
- container index (+0xc, 0x10c per item) +8: item name; text; reads and copies into the 1000 × 0x104 list (desempacotar_do_conteiner)
- file search (0x4c0aa0, 0x114 bytes) +0: FindFirstFile handle; value; reads (empacotar_no_conteiner, limpar_pasta_temporaria)
- file search +0x10: found file name; text; reads (empacotar_no_conteiner, limpar_pasta_temporaria)
- [0xde279c] temporary folder (CM3_TEMP + backslash + [0xad9c64]); text; writes (limpar_pasta_temporaria)
- [0xad9c64] temporary subfolder; text; reads (limpar_pasta_temporaria)
- [0xad6ad8] "{ Empty }" (empty list slot); text; reads (desempacotar_do_conteiner)
- [0xb67a34] flag cleared after the error box; value; writes 0 (empacotar_no_conteiner, desempacotar_do_conteiner)
- ZFILE (0xa3191 bytes, from zopen) +0: CRT FILE; pointer; reads, and writes 0 on close (all)
- ZFILE +4: mode (1 writing, 0 reading); value; reads (all)
- ZFILE +8: logical position in the item (uncompressed bytes); value; reads and writes (ler_do_conteiner, gravar_no_conteiner)
- ZFILE +0xc: open item; id; reads (all)
- ZFILE +0x10: run bytes already delivered; value; reads and writes (ler_do_conteiner)
- ZFILE +0x14: RLE run count (1 to 0x7f); value; reads and writes (all)
- ZFILE +0x18: run byte; value; reads and writes (all)
- ZFILE +0x19: container type (3 uncompressed, 4 RLE); value; reads (all)
- ZFILE +0x19..+0x41705: header and index (0x416ec bytes); bytes; rewritten at the file start (fechar_conteiner)
- item index entry (ZFILE +0x25 + 0x10c × item) +4: uncompressed size; value; reads (ler_do_conteiner, fechar_conteiner) and adds (gravar_no_conteiner)
- ZFILE +0x41705: read buffer (0x30d40); bytes; filled by fread and read (ler_do_conteiner)
- ZFILE +0x72445: write buffer (0x30d40); bytes; writes and flushes via fwrite (gravar_no_conteiner, fechar_conteiner)
- ZFILE +0xa3185 / +0xa318d: read buffer cursor and bytes read; value; reads and writes (ler_do_conteiner), clears (fechar_conteiner)
- ZFILE +0xa3189: write buffer cursor; value; reads and writes (gravar_no_conteiner, fechar_conteiner)
- type 4 .sav/.qst item: RLE (byte <= 0x80 literal; > 0x80 = byte − 0x80 copies of the next); format; writes (gravar_no_conteiner, fechar_conteiner) and reads (ler_do_conteiner)

**Of interest to:** SGE, CMScout
