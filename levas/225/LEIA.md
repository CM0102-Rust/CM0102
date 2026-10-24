# 225ª leva — a contagem e a lista do elenco do clube e se o clube pode ser dirigido; os vetores de atributos de volta ao não jogador e ao jogador e o reequilíbrio dos atributos do não jogador; a família da cor, se duas cores se confundem e o atributo calculado do não jogador

**Rotinas:**
- `contar_elenco_do_clube()` = 0x53c590 (Database.cpp, cdecl): quantas pessoas válidas há no elenco do clube (com o parceiro A/B, as marcas de elenco do time reserva e o filtro dos emprestados);
- `reunir_elenco_do_clube()` = 0x541f90 (Database.cpp, cdecl): copia o elenco do clube (e o do parceiro A/B, até 50 ou 100) para um vetor, completando com zeros;
- `clube_pode_ser_dirigido()` = 0x544820 (Database.cpp, cdecl): se o clube pode ser dirigido: seleção principal de nação com +0x88 > 150, ou clube que não é time B, com competição ativa e nação marcada;
- `vetor_para_o_nao_jogador()` = 0x53a980 (Database.cpp, cdecl): devolve o byte de baixo das 21 células do vetor aos atributos de treinador do não jogador (+0x0e..+0x22);
- `vetor_para_o_jogador()` = 0x53aa10 (Database.cpp, cdecl): devolve o byte de baixo das células 0..53 do vetor aos atributos do jogador e o da 54 à pessoa +0x58;
- `normalizar_atributos_do_nao_jogador()` = 0x539490 (Database.cpp, cdecl): reequilibra os atributos do não jogador em duas passadas: média das células perto de zero levada a −5, excesso positivo cortado, teto −80 para o cargo 10;
- `familia_da_cor()` = 0x540520 (Database.cpp, cdecl): a família (1..10) da cor pelas chaves de cor do index.cpp;
- `cores_se_confundem()` = 0x540490 (Database.cpp, cdecl): se duas cores se confundem: mesma família ou os pares 8-9, 8-7, 6-7 e a família 6 com a cor da chave [0x9cf1dc];
- `atributo_calculado_do_nao_jogador()` = 0x543fd0 (Database.cpp, cdecl): o atributo 1..12 de treinador: quatro direto do registro, oito pela fórmula de 1 a 20 com a palavra +4 do não jogador;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 8 acusadas. `dq10_reunir` escapou: a cópia da 0x541f90 está desligada (DESLIGADAS: mesmo a original pelo desvio de passagem muda a carreira, porque a 0x540a50 que ela chama deixa o resíduo na pilha particular), então a mutação não roda. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 224ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 225`

**Campos tocados:**
- clube +0xd7: elenco (50 ponteiros de pessoa); ponteiro; lê (contar_elenco_do_clube, reunir_elenco_do_clube)
- pessoa +0x00: id (válida se < [0xae2364]); id; lê (contar_elenco_do_clube, reunir_elenco_do_clube)
- pessoa +0x5e: marcas de elenco (filtro do time reserva); valor; lê (contar_elenco_do_clube)
- pessoa +0x39: clube; ponteiro; lê (contar_elenco_do_clube)
- contrato (0x4cf5a0) +0x04: id do clube do contrato; id; lê (contar_elenco_do_clube)
- [0xae1080] último contrato consultado; ponteiro; grava (contar_elenco_do_clube)
- clube +0x244: sem nome (marca de parceiro A/B, zerada por 0x540a50 ao recusar); valor; lê e pode zerar (as três, por 0x540a50)
- clube +0x00: id (seleções a partir de [0xae235c] − 2 × [0xae2350]; principais abaixo de [0xae235c] − [0xae2350]); id; lê (clube_pode_ser_dirigido)
- clube +0x53: nação; ponteiro; lê (clube_pode_ser_dirigido)
- clube +0x57: competição; ponteiro; lê (clube_pode_ser_dirigido)
- nação +0x88: sem nome (> 150 deixa dirigir a seleção); valor; lê (clube_pode_ser_dirigido)
- nação +0x11c: marcas (o bit 2 é exigido); valor; lê (clube_pode_ser_dirigido)
- [0xadadfc] objetos das competições (por id); ponteiro; lê (clube_pode_ser_dirigido)
- não jogador +0x0e..+0x22: atributos de treinador (21 bytes com sinal); valor; lê e grava (vetor_para_o_nao_jogador grava, normalizar_atributos_do_nao_jogador lê e grava)
- jogador +0x0f..+0x44: atributos e posições (54 bytes de CELULAS_DO_VETOR); valor; grava (vetor_para_o_jogador)
- staff +0x58: sem nome (célula 54 do vetor do jogador); valor; grava (vetor_para_o_jogador)
- staff +0x69: não jogador; ponteiro; lê (normalizar_atributos_do_nao_jogador)
- staff +0x28 / +0x3d: sem nome (cargos; 10 limita os atributos a −80); valor; lê (normalizar_atributos_do_nao_jogador)
- cor +0x00: id (comparado com as chaves 0x9cf178..0x9cf1fc); id; lê (familia_da_cor, cores_se_confundem)
- [0x9cf178..0x9cf1fc] chaves de cor do index.cpp; id; lê (familia_da_cor, cores_se_confundem)
- staff +0x56, +0x58: atributos 1 e 4 de treinador; valor; lê (atributo_calculado_do_nao_jogador)
- não jogador +0x04: sem nome (palavra com sinal da fórmula); valor; lê (atributo_calculado_do_nao_jogador)
- não jogador +0x10, +0x11, +0x14, +0x17, +0x18, +0x19, +0x1b, +0x1e, +0x21, +0x22: atributos de treinador; valor; lê (atributo_calculado_do_nao_jogador)

**Interessa a:** CMRTE, CMScout, GPF2
