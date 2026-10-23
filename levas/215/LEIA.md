# 215ª leva — o fim das telas da partida: a partida que o humano deixa e a ficha do jogo de um registro de estatística

**Rotinas:**
- `largar_partida_assistida()` = 0x6b6970 (match_day.cpp, cdecl): o humano da vez deixa a partida que assistia; sem outro humano assistindo, a partida anda até o fim, é registrada e sai da fila;
- `ficha_do_registro()` = 0x729e30 (match_stats.cpp, thiscall): acha na agenda o jogo de um registro de estatística (data, competição e clube) e copia a ficha;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 214ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 215`

**Campos tocados:**
- [0xb77c7a] vez (canal do humano da vez); valor; lê (largar_partida_assistida)
- [0xb74c26 + 0x300 × vez] técnico humano do canal; ponteiro; lê (largar_partida_assistida)
- [0xae23bc] tabela de pessoas (110 bytes); ponteiro; lê (largar_partida_assistida)
- [0xae2364] quantidade de pessoas (as 16 últimas são os humanos); valor; lê (largar_partida_assistida)
- staff +0x5f: técnico humano ativo; valor; lê (largar_partida_assistida)
- registro do humano (0x765480) +0xe2: partida que ele assiste; ponteiro; lê (largar_partida_assistida)
- partida +0x5b: resumo (vetores por vez em +5/+0x45/+0x85/+0xc5); ponteiro; lê, e grava 0 na casa da vez (largar_partida_assistida)
- resumo +0x105..+0x108: contadores dos quatro vetores; valor; lê e grava −1 (largar_partida_assistida)
- partida +0x57: motor da partida (0xf63b bytes); ponteiro; lê e grava (largar_partida_assistida)
- partida +0x67: estado da partida (2 ou 8 = fim); valor; lê e grava (largar_partida_assistida)
- partida +0x5f: registro de estatística (0xd7a bytes); ponteiro; grava (largar_partida_assistida)
- [0xb67a50] (MATCH_DAY) +0x25: sem nome (conta as partidas que chegam ao estado 2 ou 8); valor; grava +1 (largar_partida_assistida)
- [0xb6b7c0] estatísticas da partida (Match_Stats); ponteiro; lê (largar_partida_assistida)
- [0xde2f70] VIRTUAL_STAFF (fila das partidas); ponteiro; lê (largar_partida_assistida)
- Match_Stats +0: índice dos registros (12 bytes cada); ponteiro; lê (ficha_do_registro)
- Match_Stats +0x1c: quantidade de registros; valor; lê (ficha_do_registro)
- índice das estatísticas +0: competição; id; lê (ficha_do_registro)
- índice das estatísticas +4: um dos clubes do jogo; id; lê (ficha_do_registro)
- índice das estatísticas +8/+0xa: data do jogo; valor; lê (ficha_do_registro)
- [0xae2a58] agenda (FIXTURE_MANAGER); ponteiro; lê (ficha_do_registro)
- ficha do jogo (0x4f bytes) +0xc/+0x10: clube da casa e de fora; id; lê, e copia a ficha inteira (ficha_do_registro)

**Interessa a:** SGE
