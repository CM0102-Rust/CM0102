# 216ª leva — se a liga tem rebaixamento, a data da rodada do modelo da liga e a importância do jogo da liga; as cópias do recordista, do registro de partida e do registro de sequência dos recordes do clube; a partida assistida por algum humano, o apelido dado pelo humano e a cópia da aposentadoria pendente

**Rotinas:**
- `liga_tem_rebaixamento()` = 0x6847b0 (league.cpp, thiscall): se a liga tem vagas de rebaixamento no fim da tabela;
- `data_da_rodada_da_liga()` = 0x68a160 (league.cpp, cdecl): grava num registro de rodada do modelo da liga a data com o ano relativo, o código e um valor;
- `importancia_do_jogo_da_liga()` = 0x68a3e0 (league.cpp, thiscall): a importância do jogo da liga para o clube: a base da liga, +1 no começo do campeonato ou com a margem do último dia;
- `copiar_recordista()` = 0x448490 (club_records.cpp, thiscall): copia o recordista: quatro campos de identificação e a data;
- `copiar_registro_de_partida()` = 0x448110 (club_records.cpp, thiscall): copia o registro de partida: a data, o adversário, o placar e os campos do jogo;
- `copiar_registro_de_sequencia()` = 0x448170 (club_records.cpp, thiscall): copia o registro de sequência: o número de jogos e as datas de início e fim;
- `humano_assiste_a_partida()` = 0x5fe420 (human_manager.cpp, thiscall): se algum dos técnicos humanos ativos escolheu assistir a partida;
- `humano_deu_apelido()` = 0x600740 (human_manager.cpp, thiscall): se o técnico humano (ou o da vez) deu apelido à pessoa, por busca binária na lista dele;
- `copiar_aposentadoria_pendente()` = 0x7a8780 (player_regen.cpp, thiscall): copia a aposentadoria pendente: a pessoa, o clube, as duas datas, o motivo e a marca de anunciada;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 8 acusadas. `ha_motivo` escapou: o motivo (+0x18) da aposentadoria pendente copiada não é lido de forma que decida até 2002-06-01, e no jogo real comparando (até 2002-01-01) a cópia não pega nenhuma aposentadoria com motivo que mude (as aposentadorias se resolvem no fim da temporada). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 215ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 216`

**Campos tocados:**
- liga +0xc1: tem vagas de rebaixamento; valor; lê (liga_tem_rebaixamento)
- rodada do modelo da liga (0x41 bytes) +0: dia do ano; valor; grava (data_da_rodada_da_liga)
- rodada do modelo da liga +2: ano relativo (ano − base); valor; grava (data_da_rodada_da_liga)
- rodada do modelo da liga +4: sem nome (código); valor; grava (data_da_rodada_da_liga)
- rodada do modelo da liga +0x3d: sem nome; valor; grava (data_da_rodada_da_liga)
- liga +0x51: sem nome (dá a importância base: (x − 10) / 4 + 10); valor; lê (importancia_do_jogo_da_liga)
- liga +0x3e: quantidade de clubes; valor; lê (importancia_do_jogo_da_liga)
- liga +0xb1: tabela (linhas de 59 bytes); ponteiro; lê (importancia_do_jogo_da_liga)
- linha da tabela +0: clube; ponteiro; lê (importancia_do_jogo_da_liga)
- linha da tabela +0x37: sem nome (marca; 0xff = sem marca); valor; lê (importancia_do_jogo_da_liga)
- liga +0xa9: sem nome (rodadas; teste > 10); valor; lê (importancia_do_jogo_da_liga)
- partida +0x1c/+0x20: clube da casa e de fora; ponteiro; lê (importancia_do_jogo_da_liga)
- partida +0x34: sem nome (teste < 3); valor; lê (importancia_do_jogo_da_liga)
- recordista (0x18 bytes) +0..+0xc: quatro campos de identificação, sem nome cada; valor; lê e grava (copiar_recordista)
- recordista +0x10: data; valor; lê e grava (copiar_recordista)
- registro de partida do recorde (0x1f bytes) +0: data; valor; lê e grava (copiar_registro_de_partida)
- registro de partida do recorde +8: adversário; valor; lê e grava (copiar_registro_de_partida)
- registro de partida do recorde +0xc, +0xe, +0x10, +0x11, +0x15, +0x19, +0x1a, +0x1b: sem nome (placar e campos do jogo); valor; lê e grava (copiar_registro_de_partida)
- registro de sequência (0x14 bytes) +0: número de jogos; valor; lê e grava (copiar_registro_de_sequencia)
- registro de sequência +4/+0xc: datas de início e fim; valor; lê e grava (copiar_registro_de_sequencia)
- [0xae23bc] tabela de pessoas (110 bytes); ponteiro; lê (humano_assiste_a_partida)
- [0xae2364] quantidade de pessoas (as 16 últimas são os humanos); valor; lê (humano_assiste_a_partida, humano_deu_apelido)
- staff +0x5f: técnico humano ativo; valor; lê (humano_assiste_a_partida)
- staff +0: id da pessoa; id; lê (humano_deu_apelido)
- [0xb77c7a] vez (canal do humano da vez); valor; lê (humano_deu_apelido)
- [0xb74c26 + 0x300 × vez] técnico humano do canal; ponteiro; lê (humano_deu_apelido)
- HUMAN_MANAGER +0: registros dos humanos (0xa6ef bytes); ponteiro; lê (humano_deu_apelido)
- HUMAN_MANAGER +4: quantidade de humanos; valor; lê (humano_deu_apelido)
- registro do humano +0xa6e3: lista de apelidos (itens de 0x1e bytes, +0 id da pessoa); ponteiro; lê (humano_deu_apelido)
- registro do humano +0xa6eb: quantidade de apelidos; valor; lê (humano_deu_apelido)
- aposentadoria pendente (0x1a bytes) +0: pessoa; id; lê e grava (copiar_aposentadoria_pendente)
- aposentadoria pendente +4: clube; id; lê e grava (copiar_aposentadoria_pendente)
- aposentadoria pendente +8/+0x10: duas datas, sem nome cada; valor; lê e grava (copiar_aposentadoria_pendente)
- aposentadoria pendente +0x18: motivo; valor; lê e grava (copiar_aposentadoria_pendente)
- aposentadoria pendente +0x19: marca de anunciada; valor; lê e grava (copiar_aposentadoria_pendente)

**Interessa a:** CMRTE, SGE, GPF2
