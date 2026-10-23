# 219ª leva — o save: os filtros da caixa de arquivos (o jogo salvo, o início rápido e a versão); o save: as regras FIFA de transferência (gravar e carregar) e o resumo do save; o save: as ligações com o banco (gravar e carregar) e liberar os dados do jogo

**Rotinas:**
- `filtro_do_jogo_salvo()` = 0x838e10 (Setup.cpp, cdecl): filtro da caixa de arquivos dos jogos salvos: 1 se o contêiner tem o id 0x16 (.sav), senão 2 (só ax); caminho nulo dá a caixa de erro;
- `filtro_do_inicio_rapido()` = 0x839290 (Setup.cpp, cdecl): filtro da caixa de arquivos do início rápido: 1 se o contêiner tem o id 0x17 (.qst) e a versão gravada é a do exe, senão 2 (só ax);
- `versao_do_jogo_salvo_valida()` = 0x838eb0 (Setup.cpp, cdecl): abre o general.dat do contêiner, lê a versão e o resumo (0xf70 ou 0xf74 bytes) e devolve 1 se o texto da versão do resumo é o do exe; quando dá certo não fecha o item (como a original);
- `gravar_regras_fifa()` = 0x8395b0 (Setup.cpp, cdecl): grava o último item do save, fifa_transfer_rules.zip: 2 bytes ([0xae233c] e a regra em vigor [0xa83608]); erros dão a caixa "Error";
- `carregar_regras_fifa()` = 0x839760 (Setup.cpp, cdecl): lê os 2 bytes de fifa_transfer_rules.zip; se o primeiro é 1, a regra em vigor vira 5;
- `montar_resumo_do_save()` = 0x591f80 (file_screens.cpp, cdecl): monta o resumo do general.dat: as nações ativas (dois grupos), os técnicos humanos com clube e nação, a data do jogo, a versão, o tempo de jogo e o time();
- `gravar_banco_no_save()` = 0x524d30 (Database.cpp, cdecl): passo 1 do save_game_data: save_database(save, 1, a4, tipo), as tabelas do banco no contêiner;
- `carregar_banco_do_save()` = 0x524d10 (Database.cpp, cdecl): passo 2 do load_game_data: load_database(save, compartilhado, 1), o banco lido do contêiner;
- `liberar_dados_do_jogo()` = 0x82ff40 (Setup.cpp, cdecl): solta as telas, destrói os subsistemas guardados em globais (destrutor + delete, free, destrutor virtual) na ordem da original, apaga os *.tmp, libera as tabelas do banco, volta a data do jogo e zera três marcas; devolve 1;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos; as mutações também foram por ela): IGUAL fora dos campos ignorados.
- **Mutações (4):** 3 acusadas. `sv2_banco_a4` escapou: a cópia da 0x524d30 está desligada (DESLIGADAS: o save_database que ela chama grava 408 bytes de locais sem valor no index.dat e, chamado da cópia na pilha particular, o lixo muda), então a mutação não roda. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 218ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 219`

**Campos tocados:**
- contêiner (arquivo) +4: id do arquivo (0x16 .sav, 0x17 .qst); valor; lê, pelo 0x9331c0 (filtro_do_jogo_salvo, filtro_do_inicio_rapido)
- general.dat +0: versão do save; valor (|v| < 0x3e027 = resumo antigo de 0xf70 bytes); lê (versao_do_jogo_salvo_valida)
- general.dat +4: resumo do save (0xf70 ou 0xf74 bytes; o de build_save_game_summary); bytes; lê (versao_do_jogo_salvo_valida)
- resumo do save +0x7d0: texto da versão do exe que gravou; texto; lê e compara com 0x90d130 (versao_do_jogo_salvo_valida)
- [0xae24d0], [0xae28bc], [0xae26c4] textos da caixa de erro; texto; grava (versao_do_jogo_salvo_valida, nos erros)
- [0xae233c] primeiro byte das regras FIFA (1 = regra 5 ao carregar); valor; lê (gravar_regras_fifa) e grava (carregar_regras_fifa)
- [0xa83608] regra FIFA de transferência em vigor (10 no exe); valor; lê (gravar_regras_fifa) e grava, 5 se [0xae233c] = 1 (carregar_regras_fifa)
- fifa_transfer_rules.zip (item do save): os 2 bytes acima; bytes; grava (gravar_regras_fifa) e lê (carregar_regras_fifa)
- tabela de ligas [0xb63d60] (0x23 itens de 0x48) +0: nação; ponteiro; lê (montar_resumo_do_save)
- nação +0x11c: bits 2 e 1 (os dois grupos de nações do resumo); valor; lê (montar_resumo_do_save)
- nação/clube +0x38: nome; texto; lê (montar_resumo_do_save)
- pessoa (0x6e) +0x5f: técnico humano (1); valor; lê (montar_resumo_do_save)
- pessoa +0x39: clube; ponteiro; lê (montar_resumo_do_save)
- pessoa +0x24: seleção (nação); ponteiro; lê (montar_resumo_do_save)
- [0xae2364] quantidade de pessoas, [0xae23bc] tabela de pessoas; valor, ponteiro; lê (montar_resumo_do_save)
- [0xae2c90] data do jogo; valor; lê (montar_resumo_do_save)
- [0xdd7140] tempo de jogo acumulado, [0xdd7144] último time(); valor; lê e grava (montar_resumo_do_save)
- resumo do save +0 / +0x3e8: nações (bit 2 / bit 1); texto; grava (montar_resumo_do_save)
- resumo do save +0x7d0: versão do exe; texto; grava (montar_resumo_do_save)
- resumo do save +0x7e4 + 0x78 vezes i (i < 16): nome do técnico humano; texto; grava (montar_resumo_do_save)
- resumo do save +0x820 + 0x78 vezes i: "clube / nação" do técnico; texto; grava, só para humano (montar_resumo_do_save)
- resumo do save +0xf64: data do jogo; valor; grava (montar_resumo_do_save)
- resumo do save +0xf6c / +0xf70: tempo de jogo e time(); valor (relógio); grava (montar_resumo_do_save)
- [0xb67a50], [0xb6b7c8], [0xae2a60], [0xb67a44], [0xad9da8], [0xad9dac], [0xde1d08], [0xae28f0], [0xde1f44], [0xad9c60], [0xde1f54], [0xb63d28], [0xae2a3c], [0xad9c6c], [0xae2a30], [0xb7433c], [0xdd7ef4], [0xb74318], [0xde2f70], [0xde1f58], [0xae1000], [0xdd7f00], [0xb6b7c0], [0xb74350], [0xb5e134], [0xae2a58], [0xb63c98], [0xb74448], [0xb7431c], [0xae1050], [0xae2a38], [0xb7434c], [0xb74340], [0xdd7ee4], [0xde2f74], [0xb74430] subsistemas do jogo; ponteiro; lê, destrói e grava 0 (liberar_dados_do_jogo)
- [0xae2c74] (amistosos), [0xb74434] objetos com destrutor virtual; ponteiro; lê, destrói e grava 0 (liberar_dados_do_jogo)
- [0xadadfc] (e [0xadadf8]), [0xae2d0c], [0xae2d14], [0xae2d1c], [0xae2d20], [0xae2d24], [0xae2338], [0xdd7ef0] vetores soltos; ponteiro; lê, libera e grava 0 (liberar_dados_do_jogo)
- [0xae2c90] data do jogo; valor; grava (0x54a130 com 1, 0, 2001, -1) (liberar_dados_do_jogo)
- [0xae2c88] (byte), [0xdd711c], [0xdd7130] marcas do jogo; valor; grava 0 (liberar_dados_do_jogo)

**Interessa a:** SGE, CMRTE
