*[English version](GLOSSARIO.en.md).*

# Glossário: nomes em português no código

Desde 07/11/2026 as rotinas desviadas têm nome em inglês no código (o nome do método original no mapa de maxevans60); os auxiliares e as ferramentas ainda têm nomes em português, trocados em rodadas. Este glossário liga cada nome do código ao nome original e ao nome antigo em português (`docs/NOMES.csv`), que a documentação das levas ainda usa. Endereços são VA no `cm0102.exe` 3.9.68.

## Palavras comuns

| Português | English |
|---|---|
| motor | engine (the match engine object) |
| jogador | player |
| pessoa | person (the player's database record) |
| ficha | record / sheet |
| lado | side (0 or 1) |
| bloco | block (20 players of a side) |
| casa | square (grid cell) |
| grade | grid (pitch, 9 × 12) |
| papel | role (tactical) |
| lance | move / play |
| bola | ball |
| chute | shot |
| passe | pass |
| falta | foul |
| cartão | card |
| impedimento | offside |
| goleiro | goalkeeper |
| dono | owner (of the ball) |
| posse | possession |
| desgaste | fatigue |
| condição | condition |
| formação | formation |
| narração / comentário | commentary |
| sorteio | random draw |
| leva | batch |
| rotina | routine |
| desvio | hook / detour |
| autoteste | self-test |
| mutação | mutation |
| divergência | divergence |
| impressão digital | fingerprint |
| carreira | career |
| ler / gravar | read / write |
| escolher | choose |
| somar | add |
| zerar | clear (set to zero) |
| registrar | record |
| guardado | stored |
| tabela | table |
| faixa | band / range |
| limite / limiar | limit / threshold |

## Rotinas do motor de partidas (`rust/src/motor.rs`)

| Endereço | Nome no código | Original / significado | Nome antigo em português |
|---|---|---|---|

## Sorteio e datas

| Endereço | Português | English |
|---|---|---|
| 0x90cfc0 | `aleatorio(n)` | random(n) |
| 0x54a110 | `Data()` | Date() |
| 0x549ef0 | `Data(dia, ano)` | Date(day, year) |
| 0x549f70 | `Data(dia_do_mes, mes, ano, dia_da_semana)` | Date(day_of_month, month, year, weekday) |
| 0x54a3f0 | `dia_da_semana()` | weekday() |
| 0x54a2d0 | `ajustar_semana(alvo, dest)` | adjust_week(target, dest) |
| 0x54c770 | `mais_semanas(n)` | plus_weeks(n) |

## Auxiliares do `motor.rs`

| Português | English | O que é |
|---|---|---|
| `jogador_do_bloco` | block_player | jogador k do bloco de 20 do lado |
| `casa_da_grade` | grid_square | casa (x, y) da grade do campo |
| `sortear` | draw | chama o sorteio do jogo (0x90cfc0) |
| `ftol` | ftol | truncamento float → inteiro (_ftol, 0x945640) |
| `f32_em / f64_em` | f32_at / f64_at | constante float/double lida do .rdata |
| `byte` | byte | byte com sinal num endereço + deslocamento |
| `lado` | side | lado do jogador (byte 0x27) |
| `degrau` | step | escada de multiplicadores pelo atributo |
| `pow_do_jogo` | game_pow | potência pela CRT do jogo (0x945ff0) |
| `comparar_*` | compare_* | roda original e Rust lado a lado e registra divergências |
| `comentar / porta_6d6f70` | comment / gate_6d6f70 | narração e a porta que grava as chamadas |
| `PLACAR` | SCOREBOARD | contador de comparações e divergências |

## Módulos e ferramentas

| Arquivo | English | O que é |
|---|---|---|
| `video.rs` | video | DirectDraw próprio |
| `apresentacao.rs` | presentation | quadro → janela, mouse |
| `iat_hooks.rs` | hooks (IAT) | ganchos na tabela de importação |
| `detours.rs` | detours | desvios no exe, pilha particular |
| `simulation.rs` | simulation | simulação reproduzível |
| `allocator.rs` | allocator | alocador fixo |
| `rng.rs` | random | sorteio do jogo |
| `date.rs` | date | datas (Date.cpp) |
| `motor.rs` | engine | motor de partidas |
| `spy.rs` | spy | diagnóstico (CM_ESPIAO=1) |
| `comparison.rs` | comparison | comparação lado a lado |
| `registro.rs` | log | registro em ddraw.log |
| `simular.py` | simulate | carreira automática sem tela |
| `laboratorio.py` | laboratory | trava entre sessões e vigia de memória |
| `desmonta.py` | disassemble | desmontagem |
| `desvios.py` | detours | onde dá para desviar |
| `escuta_tela.py` | screen listener | escuta o que a tela desenha |
| `patch_portatil.py` | portable patch | gera o exe sem CD |
| `resolucao.py` | resolution | gera o exe 1920×1080 |
| `cobertura.py` | coverage | contador de chamadas por função |
| `candidatos.py` | candidates | próximas rotinas |
| `mutacoes.py` | mutations | autoteste com mutações |
| `prologos.py` | prologues | confere os bytes desviados |
| `textos.py / refs.py` | strings / refs | textos do exe e quem os usa |
