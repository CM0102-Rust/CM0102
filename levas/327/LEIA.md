# 327ª leva — a carga do temporário de uma copa (Cup::load_from_disk); a carga do temporário de uma liga (League::load_from_temp_file)

**Rotinas:**
- `cup_load_from_disk()` = 0x51fc00 (Cup.cpp, thiscall, ret 4): lê CM3_TEMP\<competição>_<id ou ano>.tmp na ordem da gravação (MAPA_SAVE §6): nome, estatísticas (Comp_Stats novo, ou o antigo destruído com o ponteiro pendurado), dados da classe (virtual +0x20), prêmio, +0x52, +0x51, +0x4b, fechada (+0x4c; antes da versão 0x3e013 um byte e três pulados), estádio, banco, substituições, +0x45, +0xb1, rodadas, times, ano, +0xab, +0xac, +0x42, +0x3a, ligas de cima e de baixo, a tabela de média (+0x18/+0x34), a competição (id → registro), as rodadas (0x68), os times (6 bytes, id → clube), os confrontos de cada rodada (0x35), os estágios (tipo 1 Cup_Stage, tipo 2 League_Stage, cada um construído e lido) e o livro de inscritos (id → clube); sem o arquivo (jogo novo), 0; erro de leitura: a caixa (algumas devolvem 0 com o arquivo aberto, a maioria encerra o jogo);
- `league_load_from_temp_file()` = 0x687b10 (league.cpp, thiscall, ret 4): lê CM3_TEMP\<competição>_<id>.tmp (ou <competição>_<ano>.bak com 0) na ordem da gravação (MAPA_SAVE §6): nome, estatísticas (Comp_Stats novo, sem destruir o antigo; com 0, +8 zerado), dados da classe (virtual +0x20), o cabeçalho (capacidade e lugares exigidos, prêmio, completa, a data da última atualização, jogos por clube, +0x52, importância, +0x4b, fechada, estádio, banco, substituições, o sorteio, ativa, parada, turnos, clubes, ano, tipo, +0x42, +0x3a, a competição (id → registro), rodadas, pontos por vitória e empate, categoria, desempates, as quatro faixas, regras, divisões de cima e de baixo, +0xe6), as datas das rodadas (0x41), a tabela de média (+0x18/+0x34), a tabela (0x3b por clube, id → clube) com o confronto direto (n − 1 registros de 0x0e, id → clube só quando a linha 0 os tem) e o histórico de posições (jogos por clube, refeito de −1 para (n − 1) × turnos), os estágios (tipo 1 Cup_Stage, tipo 2 League_Stage) e o livro de inscritos (o antigo liberado; id → clube); sem o arquivo (jogo novo), 0; erro de leitura: a caixa, devolvendo 0 com o arquivo aberto;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos, e o save de férias carregado até 2001-08-28 com e sem elas, mesma impressão; as mutações também foram por ela): IGUAL fora dos campos ignorados; carga IGUAL (parou em 2001-239, 718412161 sorteios, impressao 7146f25fc4cbd6f5).
- **Mutações (6):** 5 acusadas. `gra29_clube_do_livro` escapou: LACUNA MEDIDA. Na carga do .tmp de uma liga, o clube de cada lugar do livro de inscritos vira o clube seguinte. O laco rodou em duas ligas do save de prova, e a prova segue a carreira carregada ate 2001-239 com a carga IGUAL a base: nada leu esses lugares nesse trecho (o livro de inscritos e lido pelos sorteios das copas europeias e pela montagem da temporada seguinte). A mesma troca na carga de uma copa (gra28_clube_do_livro) foi acusada: a carga caiu. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (f6740b3f).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 ac155f01, original e8ece790 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 326ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 327`

**Campos tocados:**
- Cup +4, +8, +0xc, +0x14, +0x18, +0x1c, +0x20, +0x24, +0x28, +0x2c, +0x30, +0x34, +0x36, +0x38, +0x3a, +0x3c, +0x3e, +0x40, +0x42, +0x44, +0x45, +0x49, +0x4a, +0x4b, +0x4c, +0x51, +0x52, +0x53 (nome do arquivo), +0xa3, +0xa7, +0xab, +0xac, +0xad, +0xb1; lê do .tmp e grava
- League +4, +8, +0xc, +0x14, +0x18, +0x1c, +0x20, +0x24, +0x28, +0x2c, +0x30, +0x34, +0x36, +0x38, +0x3a, +0x3c, +0x3e, +0x40, +0x42, +0x43, +0x44, +0x45, +0x49, +0x4a, +0x4b, +0x4c, +0x51, +0x52, +0x53 (nome do arquivo), +0xa7, +0xa9, +0xab, +0xb1 (linhas +0, +5, +9), +0xb5, +0xb6, +0xb8, +0xba, +0xbe, +0xc2..+0xc5, +0xd1, +0xd9, +0xdb, +0xdd, +0xe1, +0xe2, +0xe4, +0xe6; lê do .tmp e grava

**Interessa a:** SGE
