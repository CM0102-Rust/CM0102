# 237ª leva — o nome pelos textos, o nome do cargo e o nome do idioma; o nome da seleção, o nome do atributo do jogador e o texto das posições; o nome da região e o adjetivo do continente

**Rotinas:**
- `nome_pelos_textos()` = 0x53e2e0 (Database.cpp, cdecl): o nome da pessoa pela forma (1..7) a partir do nome, sobrenome e nome comum: "%s %s", "%c.%s", "%s, %s", "%s, %c", só um deles ou as iniciais;
- `nome_do_cargo()` = 0x53ad00 (Database.cpp, cdecl): o nome traduzido do cargo (0..0x11), curto ou inteiro, minúsculo ou maiúsculo, em [0xae19b4]; o gênero com o idioma 4;
- `nome_do_idioma()` = 0x53cee0 (Database.cpp, cdecl): o nome traduzido do idioma (1..64, senão "Unknown") em [0xae21d4];
- `nome_da_selecao()` = 0x542100 (Database.cpp, cdecl): o nome do clube de seleção pela forma (1..4): o nome, "Reserves", "B"/"'B'", "U21"/"Under 21"/"Under 21's"/"Under 21 Squad" (ou 23) e "Squad"/"Main Squad";
- `nome_do_atributo_do_jogador()` = 0x5428a0 (Database.cpp, cdecl): o nome traduzido dos 31 atributos de jogador, abreviado ou inteiro; devolve 1;
- `texto_das_posicoes()` = 0x53ee50 (Database.cpp, cdecl): o texto das posições pelas notas (15 a 10) e os lados (15), abreviado ("D/DM RC"), inteiro ou em letras;
- `nome_da_regiao()` = 0x616f20 (index.cpp, cdecl): o nome da região (1..14): pelas tabelas dos idiomas 4, 5 e 9 com a forma do adjetivo, senão o adjetivo ou o substantivo traduzido pela forma;
- `adjetivo_do_continente()` = 0x6177f0 (index.cpp, cdecl): o adjetivo do continente: o do registro (+0x23) ou, nos idiomas 9, 5 e 4, o texto pelo continente e pela forma do adjetivo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (8):** 1 acusadas. `dbu_cargo` escapou: texto de tela/notícia que não chega à impressão (mídia, transferências, telas da pessoa, do clube e de contratos); escapou também no jogo real comparando (0 divergências). `dbu_idioma` escapou: texto de tela/notícia que não chega à impressão (busca e tela da pessoa); escapou também no jogo real comparando (0 divergências). `dbu_selecao` escapou: texto de tela/notícia que não chega à impressão (telas da seleção e de base, tela do clube, escalação, hall da fama); escapou também no jogo real comparando (0 divergências). `dbu_atributo` escapou: texto de tela/notícia que não chega à impressão (telas do jogador, do clube, de busca e de treino); escapou também no jogo real comparando (0 divergências). `dbu_posicoes` escapou: texto de tela/notícia que não chega à impressão (tela da pessoa com jogador de lado, busca, tática, treino); escapou também no jogo real comparando (0 divergências). `dbu_regiao` escapou: texto de tela/notícia que não chega à impressão (filtro de região da busca de jogadores); escapou também no jogo real comparando (0 divergências). `dbu_continente` escapou: texto de tela/notícia que não chega à impressão (telas de busca, barra de menu, jogos do dia); escapou também no jogo real comparando (0 divergências). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 236ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 237`

**Campos tocados:**
- [0xae19b4]: texto do nome do cargo; escreve (nome_do_cargo); [0xae21d4]: texto do nome do idioma (100 bytes); escreve (nome_do_idioma)
- TClub +0 id, +4 nome longo, +0x37 e +0x52 gênero, +0x38 nome, +0x53 nação; [0xae2350] nações, [0xae235c] clubes, [0xb74318] +0x36 sub-23 por nação; lê (nome_da_selecao)
- jogador +0xf..+0x1a notas das posições; lê; +0x13 meia: escreve 15 (texto_das_posicoes); [0xde1f64] texto global: escreve
- [0x9d0518], [0x9d06d4], [0x9d05f8]: tabelas dos nomes das regiões por idioma; [0x9eb2dc] idioma; lê (nome_da_regiao)
- continente +0 id, +0x23 adjetivo; [0x9cfa08..0x9cfa18] chaves dos continentes; lê (adjetivo_do_continente)

**Interessa a:** SGE, CMRTE
