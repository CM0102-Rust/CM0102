# 218ª leva — o contêiner do save: as consultas (o padrão do nome, o id e o tamanho dos itens); o contêiner do save: empacotar e desempacotar os arquivos da pasta e limpar a pasta temporária; o contêiner do save: ler, gravar e fechar um item (o RLE do tipo 4)

**Rotinas:**
- `casar_nome_com_padrao()` = 0x933a00 (zipdir.cpp, cdecl): casa o nome de um item do contêiner com um padrão com `*` (`*.` vai ao último ponto), sem diferenciar maiúsculas; a vaga "{ Empty }" nunca casa;
- `id_do_conteiner()` = 0x9331c0 (zipdir.cpp, cdecl): lê o cabeçalho do contêiner e devolve o dword +4 (0x16 .sav, 0x17 .qst; 0x14 se falhar);
- `tamanho_dos_itens_do_padrao()` = 0x933860 (zipdir.cpp, cdecl): soma os tamanhos descomprimidos dos itens que casam com o padrão (int com sinal: acima de 2 GB dá negativo, como na original);
- `empacotar_no_conteiner()` = 0x933610 (zipdir.cpp, cdecl): cada arquivo da pasta que casa com o padrão (FindFirstFile) vira um item do contêiner: zopen gravando, fread/zwrite em blocos de 0x30d40, zclose;
- `desempacotar_do_conteiner()` = 0x933320 (zipdir.cpp, cdecl): cada item do contêiner cujo nome casa com o padrão vira um arquivo da pasta (zopen lendo, zread em blocos de 0x30d40, fwrite);
- `limpar_pasta_temporaria()` = 0x90cd50 (utils.cpp, cdecl): monta em [0xde279c] a pasta CM3_TEMP e apaga os arquivos dela que casam com o padrão, em grupos de 100;
- `ler_do_conteiner()` = 0x932c30 (zipdir.cpp, cdecl): lê do item como o fread, cortando no fim do item; no tipo 4 decodifica o RLE (a repetição pode atravessar chamadas);
- `gravar_no_conteiner()` = 0x932e10 (zipdir.cpp, cdecl): grava no item como o fwrite e soma ao tamanho no índice; no tipo 4 codifica o RLE (repetições de até 0x7f guardadas entre as chamadas);
- `fechar_conteiner()` = 0x932b00 (zipdir.cpp, cdecl): gravando: esvazia o buffer, fecha a repetição pendente do RLE e regrava cabeçalho e índice no começo; fecha o arquivo e libera o item;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos; as mutações também foram por ela): IGUAL fora dos campos ignorados.
- **Mutações (9):** 6 acusadas. `sv_padrao_letra` escapou: o casamento do nome com o padrão (0x933a00) só roda na carga (o desempacotar de "*.lng" e "*.tmp"); a prova grava o .qst e a etapa de carregar depende de um save com técnico de férias, que ainda não existe. `sv_padrao_ponto` escapou: o casamento do nome com o padrão (0x933a00) só roda na carga (o desempacotar de "*.lng" e "*.tmp"); a prova grava o .qst e a etapa de carregar depende de um save com técnico de férias, que ainda não existe. `sv_padrao_fim` escapou: o casamento do nome com o padrão (0x933a00) só roda na carga (o desempacotar de "*.lng" e "*.tmp"); a prova grava o .qst e a etapa de carregar depende de um save com técnico de férias, que ainda não existe. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 217ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 218`

**Campos tocados:**
- contêiner (arquivo) +0: tipo (3 sem compressão, 4 RLE; 1/2 antigos viram 3/4); valor; lê (id_do_conteiner, tamanho_dos_itens_do_padrao)
- contêiner (arquivo) +4: id do arquivo (0x16 .sav, 0x17 .qst); valor; lê (id_do_conteiner)
- contêiner (arquivo) +8: quantidade de itens; valor; lê (tamanho_dos_itens_do_padrao)
- índice do contêiner (+0xc, 0x10c por item) +4: tamanho descomprimido do item; valor; lê e soma com sinal (tamanho_dos_itens_do_padrao)
- índice do contêiner +8: nome do item (até 0x104 bytes); texto; lê (tamanho_dos_itens_do_padrao, casar_nome_com_padrao)
- [0xad6ad8] "{ Empty }" (vaga vazia); texto; lê (casar_nome_com_padrao)
- [0xaa2156] tabela de maiúsculas (3 bytes por caractere, via 0x90c2f0); valor; lê (casar_nome_com_padrao)
- [0xb67a34] marca zerada depois da caixa de erro; valor; grava 0 (id_do_conteiner, tamanho_dos_itens_do_padrao, sem memória)
- ZFILE (0xa3191 bytes, do zopen) +0x41705: buffer de leitura (0x30d40), usado como bloco do arquivo lido; bytes; grava (empacotar_no_conteiner)
- ZFILE +0x72445: buffer de escrita (0x30d40), usado como bloco do item lido; bytes; grava (desempacotar_do_conteiner)
- contêiner (arquivo) +0/+8: tipo (1/2 antigos viram 3/4) e quantidade de itens; valor; lê (desempacotar_do_conteiner)
- índice do contêiner (+0xc, 0x10c por item) +8: nome do item; texto; lê e copia para a lista de 1000 × 0x104 (desempacotar_do_conteiner)
- busca de arquivos (0x4c0aa0, 0x114 bytes) +0: alça do FindFirstFile; valor; lê (empacotar_no_conteiner, limpar_pasta_temporaria)
- busca de arquivos +0x10: nome do arquivo achado; texto; lê (empacotar_no_conteiner, limpar_pasta_temporaria)
- [0xde279c] pasta temporária (CM3_TEMP + barra + [0xad9c64]); texto; grava (limpar_pasta_temporaria)
- [0xad9c64] subpasta temporária; texto; lê (limpar_pasta_temporaria)
- [0xad6ad8] "{ Empty }" (vaga vazia da lista); texto; lê (desempacotar_do_conteiner)
- [0xb67a34] marca zerada depois da caixa de erro; valor; grava 0 (empacotar_no_conteiner, desempacotar_do_conteiner)
- ZFILE (0xa3191 bytes, do zopen) +0: FILE da CRT; ponteiro; lê, e grava 0 ao fechar (todas)
- ZFILE +4: modo (1 gravando, 0 lendo); valor; lê (todas)
- ZFILE +8: posição lógica no item (bytes descomprimidos); valor; lê e grava (ler_do_conteiner, gravar_no_conteiner)
- ZFILE +0xc: item aberto; id; lê (todas)
- ZFILE +0x10: bytes da repetição já entregues; valor; lê e grava (ler_do_conteiner)
- ZFILE +0x14: contagem da repetição do RLE (1 a 0x7f); valor; lê e grava (todas)
- ZFILE +0x18: byte da repetição; valor; lê e grava (todas)
- ZFILE +0x19: tipo do contêiner (3 sem compressão, 4 RLE); valor; lê (todas)
- ZFILE +0x19..+0x41705: cabeçalho e índice (0x416ec bytes); bytes; regrava no começo do arquivo (fechar_conteiner)
- índice do item (ZFILE +0x25 + 0x10c × item) +4: tamanho descomprimido; valor; lê (ler_do_conteiner, fechar_conteiner) e soma (gravar_no_conteiner)
- ZFILE +0x41705: buffer de leitura (0x30d40); bytes; grava pelo fread e lê (ler_do_conteiner)
- ZFILE +0x72445: buffer de escrita (0x30d40); bytes; grava e esvazia pelo fwrite (gravar_no_conteiner, fechar_conteiner)
- ZFILE +0xa3185 / +0xa318d: cursor do buffer de leitura e bytes lidos; valor; lê e grava (ler_do_conteiner), zera (fechar_conteiner)
- ZFILE +0xa3189: cursor do buffer de escrita; valor; lê e grava (gravar_no_conteiner, fechar_conteiner)
- item do .sav/.qst tipo 4: RLE (byte <= 0x80 literal; > 0x80 = byte − 0x80 cópias do seguinte); formato; grava (gravar_no_conteiner, fechar_conteiner) e lê (ler_do_conteiner)

**Interessa a:** SGE, CMScout
