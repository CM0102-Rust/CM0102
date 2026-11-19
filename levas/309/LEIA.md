# 309ª leva — a carga dos temporários do save: as sementes da CONMEBOL e da UEFA e as reservas dos estádios

**Rotinas:**
- `conmebol_seeding_load_from_temp_file()` = 0x4c6970 (conmebol_seeding.cpp, thiscall, ret 0): lê CM3_TEMP\conmebol_seeding.tmp: quantos países (+0x6e), 0xa bytes por país de +0 (o ponteiro da nação +4 refeito pelo id), +0x6f (u16) e, desde a versão de dados 0x3e010, +0x71 (u32; antes, −1); sem o arquivo (jogo novo), 0; erro: a caixa "Error" sem fechar o arquivo;
- `uefa_seeding_load_from_temp_file()` = 0x905230 (uefa_seeding.cpp, thiscall, ret 0): lê CM3_TEMP\uefa_seeding.tmp: quantos países (+0x438), 0x12 bytes por país de +0 (o ponteiro da nação +4 refeito pelo id), +0x439 (u16), quantos u32 (+0x43f) numa lista nova em +0x43b e os 50 registros de 0xc de +0x440; sem o arquivo (jogo novo), 0; erro: a caixa "Error" sem fechar o arquivo;
- `stadium_load_from_temp_file()` = 0x85ba40 (stadium.cpp, thiscall, ret 0): lê CM3_TEMP\Stadium.tmp: as reservas (capacidade +8, quantas +4, 12 bytes cada, num vetor novo em +0), os locais (capacidade +0x14, quantos +0x10, a marca +0x18; 16 bytes cada num vetor novo em +0xc, o estádio +0xc refeito pelo id, −1 nulo) e, desde a versão de dados 0x3e029, o local guardado [0xa835a4]; sem o arquivo (jogo novo), 0; erro: a caixa "Error";

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos, e o save de férias carregado até 2001-08-28 com e sem elas, mesma impressão; as mutações também foram por ela): IGUAL fora dos campos ignorados; carga IGUAL (parou em 2001-239, 732460546 sorteios, impressao 3540600cc39540fa).
- **Mutações (3):** 3 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (36dd02c3).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 e9db46f8, original 60d7b94b (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 308ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 309`

**Campos tocados:**
- ConmebolSeeding +0 (+4 o ponteiro da nação refeito), +0x6e, +0x6f, +0x71; UefaSeeding +0 (+4), +0x438, +0x439, +0x43b, +0x43f, +0x440; Stadium +0, +4, +8, +0xc, +0x10, +0x14, +0x18; lê do .tmp e grava

**Interessa a:** SGE
