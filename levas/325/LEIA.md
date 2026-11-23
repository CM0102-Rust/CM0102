# 325ª leva — os torneios e as excursões de pré-temporada da IA; a criação das táticas

**Rotinas:**
- `plan_preseason_tournaments_and_tours()` = 0x5c3040 (friendly.cpp, thiscall, ret 0xc): Na pré-temporada de cada liga escolhida (temporada ainda não começada e a 52 semanas do fim): a partir de 2002, um torneio por nação: entre os clubes da IA sem lista de datas e fora da Intertoto, o maior sorteio da faixa de reputação é a sede (e um sorteio de 10 tem de dar diferente de 0), que recebe três adversários de amistoso (até 50 tentativas cada, sem lista de datas), todos com todos na lista de adversários, numa copa ou minliga (meio a meio); defeito: o contador do laço não é iniciado antes do primeiro torneio e o limite é o byte baixo da reputação da nação. Todo ano, as excursões: cada clube da IA sem adversários, com liga, com até 4 amistosos e que passa num sorteio de 3, com uma nação de excursão, enfrenta clubes dela até ter 3 ou 4 amistosos (faixa = reputação / 500 + sorteio, × 0,8 a cada adversário, 10 abaixo de 1) e, com mais de um adversário, a excursão é marcada;
- `construct_tactics()` = 0x893dc0 (tactics.cpp, thiscall, ret 4): Confere o CD (o primeiro drive de CD com cm0102.exe; sem ele, a caixa "Insert CD" e de novo); zera os 16 espaços de desfazer; com o save, as táticas vêm dele. Num jogo novo: as 28 formações da IA e as 17 pré-definidas vêm dos arquivos de CM3_TACTICS, na ordem (442_wide.pct duas vezes), cada uma no próximo espaço enquanto houver espaço (o arquivo que falha não conta; sem espaço, a caixa de erro da linha dele; faltando formação no fim, a caixa 0x92 ou 0xb4), e as pré-definidas ganham o nome de exibição ("4-4-2", "3-5-2 Defensive"...); uma linha de escalação vazia por clube e mais 32 (0x91 bytes, tudo −1 depois dos 2 primeiros); as estatísticas por tática (45 de 0x28 bytes, zeradas com o índice) e 9 constantes em +0xc..+0x2c;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (5):** 5 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (5828e965).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 ac155f01, original 96da1fba (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 324ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 325`
