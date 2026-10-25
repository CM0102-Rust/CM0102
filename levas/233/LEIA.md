# 233ª leva — as regras do contrato: o clube em temporada e o clube e a nação entre as seis grandes; os vínculos de opinião do contrato: limpar, juntar e tirar; a comissão do clube com a do time parceiro: o auxiliar, os treinadores e a contagem dos treinadores; os nomes em inglês da sigla da nação, da competição e da competição de pessoal; o nome em inglês do estádio, o .lng do idioma existe? e soltar as tabelas em inglês; a criação do jogo: gerar a comissão que falta, cortar a tabela de clubes, podar o histórico de competições; a carga do banco: trocar os índices por ponteiros e o vetor de clubes pela reputação

**Rotinas:**
- `clube_em_temporada()` = 0x547770 (Database.cpp, cdecl): clube nulo: 0; senão key_nation_club_in_season 0x669bd0(clube, nulo, nulo): 1 se hoje está dentro da temporada da nação do clube (o eax inteiro);
- `clube_em_nacao_grande()` = 0x549e50 (Database.cpp, cdecl): 1 se a nação do clube (+0x53) é Inglaterra, França, Alemanha, Itália, Espanha ou Holanda (as chaves do index.cpp); clube ou nação nulos: 0;
- `nacao_grande()` = 0x549ea0 (Database.cpp, cdecl): o mesmo teste das seis chaves sobre a própria nação; nula: 0;
- `limpar_vinculos_de_opiniao()` = 0x549c90 (Database.cpp, cdecl): −1 nos dois vínculos (id e valor) do contrato da pessoa; pessoa nula ou id ≥ [0xae2364]: nada;
- `juntar_vinculo_de_opiniao()` = 0x549cf0 (Database.cpp, cdecl): grava o id da outra pessoa e o valor no primeiro vínculo vazio (+0x27 = −1) ou, senão, no segundo, por cima; as duas pessoas têm de ser válidas;
- `tirar_vinculo_de_opiniao()` = 0x549d60 (Database.cpp, cdecl): esvazia o primeiro vínculo com o valor dado ou, se não, o segundo;
- `auxiliar_do_clube()` = 0x53cb80 (Database.cpp, cdecl): o auxiliar (+0xd3) do clube; sem ele, o do time parceiro A/B; nenhum: 0;
- `treinadores_do_clube()` = 0x53cc30 (Database.cpp, cdecl): copia os 5 treinadores do clube para um vetor; com time parceiro (A/B), junta os dois até 5 e zera o resto;
- `contar_treinadores_do_clube()` = 0x53c8d0 (Database.cpp, cdecl): quantos treinadores válidos (não nulos, id < [0xae2364]) o clube e o time parceiro têm juntos;
- `sigla_inglesa_da_nacao()` = 0x5454c0 (Database.cpp, cdecl): a sigla da nação, ou a em inglês do eng.lng (linhas de 109 bytes, pelo id, sigla em +0x4f) quando outro idioma está carregado;
- `nome_ingles_da_competicao()` = 0x5455a0 (Database.cpp, cdecl): o nome da competição, ou o em inglês do eng.lng (linhas de 83 bytes: de clube pelo id, de nação pelo id menos as de clube) quando outro idioma está carregado;
- `nome_ingles_da_competicao_de_pessoal()` = 0x5456a0 (Database.cpp, cdecl): o nome da competição de pessoal, ou o em inglês do eng.lng (linhas de 79 bytes, pelo id) quando outro idioma está carregado;
- `nome_ingles_do_estadio()` = 0x545990 (Database.cpp, cdecl): o nome do estádio, ou o em inglês do eng.lng (linhas de 52 bytes, pelo id) quando outro idioma está carregado;
- `arquivo_do_idioma_existe()` = 0x535930 (Database.cpp, cdecl): monta CM3_TEMP\<idioma>.lng (0, 1, 11 e 12: eng.lng; 2 a 10: fra, ger, ita, esp, dan, hol, nor, por, swe) e diz se existe; outro idioma, 0;
- `liberar_tabelas_em_ingles()` = 0x544f30 (Database.cpp, thiscall): solta (free) as 8 linhas não nulas da tabela do eng.lng, de +0x20 a +0x3c, sem zerar os ponteiros;
- `gerar_comissao_extra()` = 0x527240 (Database.cpp, cdecl): completa técnicos, olheiros e fisioterapeutas dos clubes de nações escolhidas pela reputação (pessoas das reservas, nomes da nação, nascimento sorteado) e cria treinadores sem clube para os clubes sem treinador; mais de 23.000: caixa e 0;
- `cortar_tabela_de_clubes()` = 0x5267c0 (Database.cpp, cdecl): corta a tabela de clubes pelo tamanho da base: os melhores por nação, os referidos por pessoas e históricos, as seleções, as chaves e os de nações escolhidas; troca os ponteiros, compacta, monta [0xae23fc] e realoca no lugar;
- `podar_historico_de_competicoes()` = 0x539ae0 (Database.cpp, cdecl): deixa no histórico de competições de clube só as linhas de competições de nação nula ou escolhida, realoca, acerta a parte das seleções e ordena (qsort com 0x540780);
- `resolver_indices_do_banco()` = 0x5315a0 (Database.cpp, cdecl): troca cada campo de id das 12 tabelas pelo ponteiro ao registro, com os tetos, os registros em 0xde1f64, as caixas de pessoa sem jogador, preferências ou não jogador e os erros do original (+0x5b do clube zera +0x60; +0x48 do estádio zera a cidade);
- `ordenar_clubes_pela_reputacao()` = 0x53db40 (Database.cpp, cdecl): com refazer, enche o vetor [0xae2d1c] com os ids dos clubes; depois o ordena pela reputação (qsort com 0x540020);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos, e o save de férias carregado até 2001-08-28 com e sem elas, mesma impressão; as mutações também foram por ela): IGUAL (16856662 bytes); carga IGUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73).
- **Mutações (21):** 9 acusadas. `dbl_clube_em_temporada` escapou: a cópia da 0x547770 está desligada (DESLIGADAS: o desvio, mesmo de passagem, muda a carreira nos dois modos), então a mutação não roda. `dbl_nacao_grande` escapou: só roda na regeneração dos jovens (0x7abac0), na temporada; a prova do save não passa por ela. `dbl_juntar_vinculo` escapou: roda durante a temporada (distribuição dos treinadores, eventos de contrato, regeneração), não na criação nem na gravação do .qst; o comparador roda no jogo real comparando (0 divergências). `dbl_tirar_vinculo` escapou: roda durante a temporada (distribuição dos treinadores, eventos de contrato, regeneração), não na criação nem na gravação do .qst; o comparador roda no jogo real comparando (0 divergências). `dbl_auxiliar` escapou: roda durante a temporada (distribuição dos treinadores, eventos de contrato, regeneração), não na criação nem na gravação do .qst; o comparador roda no jogo real comparando (0 divergências). `dbl_treinadores` escapou: roda durante a temporada (distribuição dos treinadores, eventos de contrato, regeneração), não na criação nem na gravação do .qst; o comparador roda no jogo real comparando (0 divergências). `dbl_contar_treinadores` escapou: roda durante a temporada (distribuição dos treinadores, eventos de contrato, regeneração), não na criação nem na gravação do .qst; o comparador roda no jogo real comparando (0 divergências). `dbm1_competicao` escapou: o jogo dos testes está em inglês (idioma 0): o caminho do .lng não roda na criação nem na carga; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, abaixo na cadeia.txt). `dbm1_pessoal` escapou: o jogo dos testes está em inglês (idioma 0): o caminho do .lng não roda na criação nem na carga; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, abaixo na cadeia.txt). `dbm2_estadio` escapou: o jogo dos testes está em inglês (idioma 0): o caminho do .lng não roda na criação nem na carga; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, abaixo na cadeia.txt). `dbm2_lng` escapou: o jogo dos testes está em inglês (idioma 0): o caminho do .lng não roda na criação nem na carga; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, abaixo na cadeia.txt). `dbm2_soltar` escapou: o jogo dos testes está em inglês (idioma 0): o caminho do .lng não roda na criação nem na carga; acusada pelo autoteste do banco (CM_AUTOTESTE=banco, abaixo na cadeia.txt). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 232ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 233`

**Campos tocados:**
- TClub +0x53: nação; ponteiro; lê (clube_em_nacao_grande)
- TNation +0: id; inteiro; lê (clube_em_nacao_grande, nacao_grande). Chaves 0x9cf2e4, 0x9cf300, 0x9cf314, 0x9cf360, 0x9cf488, 0x9cf338 (Inglaterra, França, Alemanha, Itália, Espanha, Holanda); lê
- [0xae2d0c] (contratos, 79 bytes por pessoa) +0x27/+0x3b: id da pessoa do vínculo; +0x37/+0x4b: valor do vínculo; inteiros, −1 = vazio; lê e grava (as três)
- TStaff +0: id; inteiro; lê (as três). [0xae2364]: quantidade de pessoas; lê
- TClub +0xd3: auxiliar; ponteiro; lê (auxiliar_do_clube)
- TClub +0x19f (5): treinadores; ponteiros; lê (treinadores_do_clube, contar_treinadores_do_clube)
- TClub +0x244: tem time parceiro; byte; zerado pela 0x540a50 quando recusa o parceiro (as três)
- 0x9b5984 idioma dos nomes; valor; lê (as três)
- 0xae2408 tabela do eng.lng (+0x14 quantas competições de clube, +0x28 competições de nação e +0x38 de clube em linhas de 83, +0x2c nações em linhas de 109, +0x3c competições de pessoal em linhas de 79); ponteiro; lê (as três)
- 0x9b5984 idioma dos nomes; valor; lê (nome_ingles_do_estadio)
- 0xae2408 tabela do eng.lng (+0x34 estádios em linhas de 52; +0x20..+0x3c as 8 linhas); ponteiro; lê (nome_ingles_do_estadio, liberar_tabelas_em_ingles)
- 0xde279c buffer do caminho (0x90bde0); texto; grava (arquivo_do_idioma_existe)
- TStaff +4/+8/+0xc nomes, +0x10 nascimento, +0x1a nação, +0x24 seleção, +0x28 cargo na seleção, +0x39 clube, +0x3d cargo, +0x5f tipo; grava (gerar_comissao_extra)
- TClub +0x19f (5), +0x1b3 (7), +0x1cf (3): comissão; ponteiros; grava (gerar_comissao_extra)
- [0x9b59b4]: primeira linha de nome comum vazia; grava (gerar_comissao_extra)
- TClub +0 id, +0x80 reputação, +0xb3/+0xb7/+0xbb clubes; [0xae23fc] linha nova → id antigo; [0xae235c], [0xae23b8]; grava (cortar_tabela_de_clubes)
- TNation +0x11c: nação escolhida; byte; lê (as três)
- histórico de competições de clube (26 bytes): +4 competição; [0xae23d4], [0xae237c], [0xae23dc]; grava (podar_historico_de_competicoes)
- TClub +0x53..+0xbb, +0xbf..+0xd3, +0xd7 (50), +0x19f, +0x1b3, +0x1cf, +0x1e0 (20); TNation +0x71..+0x118; árbitro +4..+0x1a; estádio +0x38, +0x48; preferências +4..+0x30; não jogador +0x23..+0x3f; TStaff +4..+0x24, +0x61, +0x65, +0x69; competição de pessoa +0x53..+0x5f; histórico de pessoas +4, +0xa; cidade +0x1f; histórico de competições +4..+0x16; competição de clube +0x59..+0x65: id → ponteiro; grava (resolver_indices_do_banco)
- [0xae2d1c]: vetor de ids de clube pela reputação; grava (ordenar_clubes_pela_reputacao)

**Interessa a:** SGE, CMRTE, CMScout
