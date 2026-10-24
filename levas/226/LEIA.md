# 226ª leva — a forma do adjetivo por idioma, o atributo de treinador da pessoa e a nota com a habilidade reduzida; o primeiro nome pelo peso, as cores da competição e se o clube tem vaga no cargo; a entrada da pessoa no clube

**Rotinas:**
- `forma_do_adjetivo()` = 0x545a60 (Database.cpp, cdecl): a forma (0..4) do adjetivo para o índice 1..35 nos idiomas 3..10 que flexionam (tabelas de salto); outro idioma ou índice: 0xff;
- `atributo_do_nao_jogador()` = 0x543f30 (Database.cpp, cdecl): o atributo 1..12 de treinador da pessoa: 0x543fd0 com o não jogador dela (+0x69);
- `nota_relativa_com_habilidade_reduzida()` = 0x5442f0 (Database.cpp, cdecl): a nota de 1 a 20 de 2 × valor + habilidade atual × 0,1 (a de 0x544380 com a habilidade reduzida);
- `primeiro_nome_pelo_peso()` = 0x539b90 (Database.cpp, cdecl): um primeiro nome: nação com mais de 15 nomes, o de maior peso em 3 sorteios na faixa dela; senão sorteio na tabela inteira;
- `cores_da_competicao()` = 0x53b7c0 (Database.cpp, cdecl): as cores (frente e fundo) da competição em pixels, do registro +0x5d ou da própria competição;
- `clube_tem_vaga_para_cargo()` = 0x53b9a0 (Database.cpp, cdecl): se o clube tem vaga para a pessoa no cargo 1..15 (casa vazia ou já dela; 12, 13 e 15 também no elenco);
- `incluir_pessoa_no_clube()` = 0x53bd10 (Database.cpp, cdecl): põe a pessoa na casa do cargo no clube (cargos 1..15; jogador acima de 85 sem técnico humano sobe de 6/8 para 13/15), copia a data e grava clube e cargo na pessoa;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 8 acusadas. `dq13_reduzida` escapou: a nota com a habilidade reduzida (0x5442f0) não decide nada na carreira até 2002-06-01 e no jogo real comparando (até 2002-01-01) não houve chamada em que ×0,2 e ×0,1 dessem notas diferentes depois de presas em 1..20. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 225ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 226`

**Campos tocados:**
- [0x545b2c..0x545db4] tabelas de salto das formas do adjetivo; valor; lê (forma_do_adjetivo)
- staff +0x69: não jogador; ponteiro; lê (atributo_do_nao_jogador)
- jogador +0x05: habilidade atual; valor; lê (nota_relativa_com_habilidade_reduzida)
- [0x967880] 0,1, [0x967888] 0,05, [0x9688b0] 10, [0x968a80] 1/3, [0x968968] 0,5, [0x967890] 1, [0x968ea8] 20; valor; lê (nota_relativa_com_habilidade_reduzida)
- [0xae2338] índice de nomes por nação (+0 primeira linha, +0x18 quantos primeiros nomes); valor; lê (primeiro_nome_pelo_peso)
- [0xae23f0] tabela de primeiros nomes (60 bytes; +0x3b peso); ponteiro; lê (primeiro_nome_pelo_peso)
- [0xae2398] total de primeiros nomes; valor; lê (primeiro_nome_pelo_peso)
- [0xad6c30] estado do rand() da CRT; valor; lê e grava (primeiro_nome_pelo_peso)
- competição +0x5d: registro com as cores (+0x90, +0x94); ponteiro; lê (cores_da_competicao)
- competição +0x61, +0x65: cores; ponteiro; lê (cores_da_competicao)
- cor +0x37, +0x38, +0x39: vermelho, verde, azul; valor; lê (cores_da_competicao)
- [0xae3184], [0xaebe04] cores padrão; valor; lê (cores_da_competicao)
- clube +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): casas dos cargos e do elenco; ponteiro; lê (clube_tem_vaga_para_cargo)
- clube +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): casas dos cargos e do elenco; ponteiro; lê e grava (incluir_pessoa_no_clube)
- staff +0x24, +0x28: seleção e cargo nela; ponteiro, valor; grava (incluir_pessoa_no_clube)
- staff +0x29, +0x3e: data de entrada na seleção / no clube (8 bytes); valor; grava (incluir_pessoa_no_clube)
- staff +0x39, +0x3d: clube e cargo nele; ponteiro, valor; grava (incluir_pessoa_no_clube)
- staff +0x61: jogador; ponteiro; lê (incluir_pessoa_no_clube)
- jogador +0x05: habilidade atual (> 85 sobe o cargo); valor; lê (incluir_pessoa_no_clube)
- [0xae2364] total de pessoas (id ≥ total − 16: só no cargo 5); valor; lê (incluir_pessoa_no_clube)
- [0xae235c], [0xae2350] totais de clubes e nações (faixa das seleções); valor; lê (incluir_pessoa_no_clube)

**Interessa a:** CMRTE, CMScout, GPF2, SGE
