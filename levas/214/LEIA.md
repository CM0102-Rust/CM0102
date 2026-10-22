# 214ª leva — a ordem das linhas da tabela, a grade de turnos da liga e a margem de gols do último dia; a partida guardada na notícia, o resultado da eliminatória para a notícia e o dia da rodada da copa; o mapa de casas da formação, a nota do clube para o jovem e a partida assistida pelo humano

**Rotinas:**
- `ordem_das_linhas_da_tabela()` = 0x683210 (league.cpp, cdecl): a comparação do qsort da tabela: pontos, os critérios de desempate da liga e o id do clube;
- `montar_grade_de_turnos()` = 0x6844b0 (league.cpp, stdcall): monta a grade em que cada clube enfrenta todos uma vez, com o mando equilibrado;
- `margem_do_ultimo_dia()` = 0x68a4d0 (league.cpp, thiscall): no último dia, a margem de gols que o clube precisa para o título, as vagas de cima ou fugir da queda, e a marca de jogo decisivo;
- `partida_das_vagas_da_noticia()` = 0x4b0010 (comp_util.cpp, cdecl): refaz a partida a partir de 18 vagas seguidas da notícia e liga os clubes, o estádio e a competição;
- `resultado_da_eliminatoria_na_noticia()` = 0x4b0890 (comp_util.cpp, cdecl): a categoria do resultado da eliminatória para o clube, pela margem final, o mando e a diferença de reputação;
- `dia_da_rodada_da_copa()` = 0x521f10 (Cup.cpp, cdecl): grava um dos oito dias da rodada da copa: o mando, o dia da semana, o horário e o código;
- `mapa_de_casas_da_formacao()` = 0x5b7160 (formation.cpp, thiscall): expande as posições de cada casa da formação nas tabelas de células do campo e no espelho delas;
- `nota_do_clube_para_o_jovem()` = 0x7ad220 (player_regen.cpp, stdcall): a nota sorteada de quanto o clube serve ao jovem: reputação contra habilidade, juniores, controle humano e nação;
- `partida_assistida_pelo_humano()` = 0x5fe290 (human_manager.cpp, thiscall): se a partida é a que o técnico humano escolheu assistir;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 8 acusadas. `lt_ultimo_dia` escapou: a margem só é pedida pela 0x68a3e0 (importância do jogo), que grava o byte num campo; no jogo real comparando (até 2002-01-01) ela não é chamada, e na carreira até 2002-06-01 o 0xfe no lugar de 0xff (sem margem) não muda nenhuma decisão. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (5afd987e1a1a3ef3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b11bd5cb63f51d72).

**Refazer**, sobre o commit da 213ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 214`
