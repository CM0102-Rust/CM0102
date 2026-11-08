# Despejo das tabelas logo depois do -load (regens do CMRTE)

Pedido do CMRTE (26/10/2026): provar que o Save do GPF2 montado **da memória** dá a mesma lista de regens que o montado
**do arquivo** (saída canônica SHA `514961fe…`). Este documento descreve o despejo cru da memória feito para isso.

## O que foi despejado

Em `despejo_regens\` (fora do git): um `.bin` por tabela, a memória como está (ponteiros já resolvidos), na ordem do
vetor do jogo, e `despejo.json` (endereços, contagens, data, cabeçalho, MD5s). Não jogadores não.

| Arquivo | Ponteiro (onde o jogo lê) | Base | Contagem (em) | Registro | Bytes | MD5 |
|---|---|---|---|---|---|---|
| `pessoas.bin` | `[0xae23bc]` (0xae234c+0x70) | 0x1b4183b0 | 62.487 `[0xae2364]` | 0x6e | 6.873.570 | fa9be884cd181e414d94452b22d243a7 |
| `jogadores.bin` | `[0xae23c4]` | 0x1c17c3c0 | 42.553 `[0xae236c]` | 0x46 | 2.978.710 | c360b48aa0ab1ecf61732563c9b85d02 |
| `first_names.bin` | `[0xae23f0]` | 0x1a1c7834 | 51.901 `[0xae2398]` | 0x3c | 3.114.060 | 46cb637ece22661aeb5ab05b93914f43 |
| `second_names.bin` | `[0xae23f4]` | 0x1a4bfc80 | 98.912 `[0xae239c]` | 0x3c | 5.934.720 | b4713a3be656cecd5f87f4cf3eb22034 |
| `common_names.bin` | `[0xae23f8]` | 0x1aa68b00 | 4.786 `[0xae23a0]` | 0x3c | 287.160 | 8cebd1b903efb6f26a09cf045f0957bf |
| `nacoes.bin` | `[0xae23a8]` | 0x1ab27e42 | 231 `[0xae2350]` | 0x122 | 66.990 | 036d37ee71ca58ba06f1e22aacaf6451 |
| `clubes.bin` | `[0xae23b4]` | 0x1abe63a0 | 7.394 `[0xae235c]` (clubes + seleções) | 0x245 | 4.295.914 | b72015e6e5dea2fdf733eab6f084cdf3 |

Endereços conferidos com `docs/MAPA_DATABASE.md` (tabela das 19 tabelas) e `TABELAS_DO_BANCO_DBN2` do `motor.rs`. Os
tamanhos batem com os itens do save: `staff.dat`, `player.dat`, os três de nomes e `nation.dat` têm exatamente os mesmos
bytes; os clubes são `club.dat` (6.932) + `nat_club.dat` (462), contíguos.

Também no JSON: os 8 bytes de `0xae2c90` (`4901ec0701000000` = dia 329 de 2028) e os 0x80 bytes de `0xae234c` (as
contagens e os ponteiros das tabelas).

## Origem

- **Save:** `clz_teste\Valencia.sav` (só lido e copiado), MD5 `67e35978592908917184a0fc7e9f09d3`, 278.992.303 bytes,
  intocado (conferido depois). É **da base do CLZ**: 231 nações (a de fábrica tem 213), igual ao `auto setup game.sav` do
  CLZ. Por isso foi carregado na cópia do CLZ.
- **Exe:** `%TEMP%\clz_compat4\cm0102.exe` (3.9.68 com o patch CLZ 25-26 e a NickDLL), MD5
  `b9630dc08e96accf7683e3b517208424`, rodado como cópia de nome novo (`cm0102_despejo.exe`, apagada no fim).

## O momento exato

Na **entrada da 0x8387e0** (Setup.cpp, restaurar humanos). No tratador do `-load` (0x81bd70) a ordem é: `load_game_data`
0x82a9c0 (0x81bee7) → `Config::apply_and_save` 0x5d8480 → conferência do humano 0x838d10 → `call 0x8387e0` (0x81c03d).
Nenhum dia passou (sem `CM_FERIAS`, nada roda antes do despejo); o processo é encerrado ali mesmo. Ressalva: o 0x5d8480
recarrega os nomes traduzidos (clubes, nações) se o idioma do `game.cfg` for diferente do idioma dos nomes; o que está
nos `.bin` é o que o jogo tem na memória nesse ponto.

## Como reproduzir

1. DLL de diagnóstico: `copias\regens\rust` (o `rust\` do HEAD + `src\despejar.rs` + o desvio em `simulation.rs`; fora
   do git). `cargo build --release` nessa pasta.
2. `python copias\regens\despejar.py`: dentro da trava do laboratório (bancada do `simular.py` apontada para a cópia do
   CLZ; `game.cfg`, `*.tmp`, `ddraw.dll`, `hall_of_fame.bin` e `ddraw.log` da cópia devolvidos), copia o save para
   `%TEMP%\cm0102_regens`, roda `cm0102_despejo.exe -windowed -seed 12345 -load <cópia>` com `CM_PARAR_EM=2099-12-31`
   (modo simulação, necessário para o desvio), `CM_ORIGINAL=1`, `CM_JANELA=960x540`, `CM_OCULTO=1` e
   `CM_DESPEJAR=D:\Sistemas\CM0102\despejo_regens`, sem cliques (~5 s), e acrescenta ao JSON os MD5 do exe, do save e
   dos `.bin`. O `ddraw.log` da rodada fica junto.
