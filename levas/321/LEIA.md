# 321ª leva — as estatísticas das competições no save: a gravação; as estatísticas das competições no save: a carga; o histórico dos clubes: montar, destruir e carregar do save

**Rotinas:**
- `comp_stats_save()` = 0x4ac4e0 (comp_screens.cpp, thiscall): grava no temporário da competição, em ordem: a tabela de clubes (capacidade, contagem, entradas de 0x76), a de jogadores (entradas de 0x2d), o bloco +0x56a, os dois conjuntos de recordes, as chaves do arquivo, a contagem de fins de temporada, as bandeiras de recorde e as sequências guardadas (liberadas antes quando a contagem é 0); gravação curta para o jogo;
- `comp_stats_load()` = 0x4ab890 (comp_screens.cpp, thiscall): lê do temporário da competição na ordem da gravação: tabelas novas de clubes e de jogadores (nos dados antes da versão 0x3e015, entradas de jogador de 0x1d bytes com o resto zerado), o bloco +0x56a (só depois da versão 0x3e007), os recordes, as chaves, as bandeiras e as sequências guardadas; leitura curta para o jogo;
- `club_history_constructor()` = 0x442ab0 (club_history.cpp, thiscall, ret 4): procura o CD (a caixa "Insert CD" até achar; o exe sem CD remenda a busca e a cópia lê os bytes), cria as listas em arquivo do histórico de liga (CM3_TEMP\club league history.tmp, registros de 21 bytes) e de copa (club cup history.tmp, 11 bytes); com save, carrega os registros; senão um registro de 16 bytes por clube com os quatro deslocamentos em -1;
- `club_history_destructor()` = 0x442e20 (club_history.cpp, thiscall): destrói as duas listas em arquivo e libera os registros dos clubes, zerando ponteiros e contagem;
- `club_history_load()` = 0x443cd0 (club_history.cpp, thiscall, ret 4): lê do item "club history.dat" do save a contagem e os registros de 16 bytes de cada clube; leitura curta ou falta de memória param o jogo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos, e o save de férias carregado até 2001-08-28 com e sem elas, mesma impressão; as mutações também foram por ela): IGUAL fora dos campos ignorados; carga IGUAL (parou em 2001-239, 718412161 sorteios, impressao 7146f25fc4cbd6f5).
- **Mutações (5):** 3 acusadas. `tel41_6_club_attendance` escapou: LACUNA EXPLICADA. Na carga das estatisticas da competicao, o total de publico em casa do primeiro clube sobe 1 depois de lido. A prova da carga segue a carreira ate 2001-239 depois de carregar, e o total de publico so e lido pelas medias de publico das telas e pelo recorde de publico do fim da temporada; nada que a simulacao leia antes disso depende dele. A gravacao da mesma tabela foi acusada pela irma tel41_5_player_entry_short (o item da Copa America muda de tamanho). `tel41_6_season_records` escapou: LACUNA EXPLICADA. Na carga, os recordes da temporada vao para a area dos recordes de sempre. Os recordes so sao lidos no fim de cada competicao (update_season_end_records, depois de 2001-239) e pelas telas de estatisticas; a carreira carregada ate 2001-239 nao os le. A gravacao deles foi acusada pela irma tel41_5_season_records (o item da primeira divisao inglesa muda em +0xe41). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (b99a226d).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 6edd502b, original b3431db2 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 320ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 321`

**Campos tocados:**
- Club_History +0 lista em arquivo do histórico de liga (FILE_LLIST, registros de 0x15), +4 a de copa (0xb), +8 registros por clube (0x10: +0/+4 primeiro e último deslocamento das temporadas de liga, +8/+0xc das campanhas de copa, -1 nenhum), +0xc a contagem; ponteiro e valor; grava (club_history_constructor, club_history_load) e libera (club_history_destructor)

**Interessa a:** SGE, CMRTE
