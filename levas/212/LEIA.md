# 212ª leva — o registro da partida, a ordem dos eventos e o idioma que o jogador perde

**Rotinas:**
- `preencher_registro_da_partida()` = 0x729ff0 (match_screens.cpp, thiscall): preenche o registro de estatística da partida: clubes, datas, público, renda, clima, eventos ordenados, formação, capitão e os 20 jogadores de cada time;
- `ordem_dos_eventos()` = 0x72b160 (match_screens.cpp, cdecl): ordena dois eventos da partida pelo grupo do tipo, depois pelo minuto e pela ordem de chegada (comparador do qsort);
- `tirar_idioma_do_jogador()` = 0x7c6bf0 (player_stats.cpp, thiscall): tira um idioma do jogador da tabela de idiomas (busca binária) e desce as entradas seguintes;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 2 acusadas. `i_ultima` escapou: na carreira a 0x7c6bf0 só é chamada pelos regens (idiomas 1..64 do jogador recém-criado, que ainda não tem nenhum): a busca nunca acha a entrada, então a última vaga nunca é apagada (o comparador cobre a tabela inteira). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (5afd987e1a1a3ef3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b11bd5cb63f51d72).

**Refazer**, sobre o commit da 211ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 212`
