# 331ª leva — os registros dos clubes: os tipos de recorde de cada clube e o fim da temporada de uma nacao

**Rotinas:**
- `club_records_record_type_applies()` = 0x449590 (club_records.cpp, thiscall (ecx nao lido), ret 0xc): diz se o tipo de recorde (1 a 32) vale para o clube no periodo (1 de todos os tempos, 2 a temporada): os tipos so de clube (posicoes na liga, publico, resultados, jogos e gols na liga, transferencias...) valem para quem nao e selecao; o 27 (convocacoes) so para selecao; o 32 (o jogador da torcida no ano) so para clube e nunca no periodo de todos os tempos; os demais, e um tipo fora de 1..32, valem sempre. Clube nulo: a caixa de erro e 0;
- `club_records_end_season_for_nation()` = 0x447880 (club_records.cpp, thiscall, ret 4): fecha a temporada de cada clube da nacao-chave: o publico medio da temporada acima de -1 que supera o de todos os tempos (-1 = nenhum) vai para o de todos os tempos com o ano; a nota da temporada ainda em -1,0 e calculada; a nota media da temporada acima de -1,0 que supera a de todos os tempos e copiada com o ano; em cada recorde batido, a noticia entra na fila (clube humano ou selecao com 5 temporadas ou mais, sem repetir); com registro da temporada, o jogador da torcida no ano (tipo 32): quando ha um, a noticia 0x3e8 para o clube de tecnico humano e o registro dos torcedores da pessoa (com o ano e a marca de temporada dividida da nacao); a temporada vai para o arquivo do historico (o ano e o conjunto de registros de 0x38b bytes) e o registro da temporada e zerado (-1); sempre, uma temporada a mais na entrada do clube e os contadores da temporada zerados. Devolve 1; nacao nula: a caixa de erro e 0. Defeito do original: o publico e a nota leem o registro de todos os tempos antes de testar se e nulo (so depois o criam); como a partida cria os dois juntos, a criacao nunca roda;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (5):** 5 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (40d35960).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 e8d3d34b, original 531044b3 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 330ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 331`
