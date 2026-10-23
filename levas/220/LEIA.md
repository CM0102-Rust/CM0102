# 220ª leva — o save: criar o jogo pelo arquivo de configuração (o início rápido e a carreira automática); o save: gravar o jogo inteiro

**Rotinas:**
- `criar_jogo_pela_configuracao()` = 0x827110 (Setup.cpp, cdecl): lê o auto_setup.cfg/quick_start.cfg (banco, estatísticas, ligas de frente e de fundo, idioma, motor, propostas, férias), cria o jogo (0x828110) e grava o .qst (0x82e430 com 0x17) e libera tudo, ou põe o técnico humano de férias num clube sorteado;
- `gravar_jogo()` = 0x82e430 (Setup.cpp, cdecl): grava o save: apaga o antigo, o banco (0x524d30), o general.dat (16 campos e o resumo), o save dos subsistemas na ordem da original com disco e memória entre eles, empacota os *.tmp, as regras FIFA; em rede, grava em temp_save_file e renomeia;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos; as mutações também foram por ela): IGUAL (16856662 bytes).
- **Mutações (5):** 3 acusadas. `sv3_criar_id` escapou: a cópia da 0x827110 está desligada (DESLIGADAS: rodando na pilha particular, ela muda o resíduo que a create_game 0x828110 lê e o club.dat sai diferente), então a mutação não roda. `sv3_criar_frente` escapou: a cópia da 0x827110 está desligada (DESLIGADAS: rodando na pilha particular, ela muda o resíduo que a create_game 0x828110 lê e o club.dat sai diferente), então a mutação não roda. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 219ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 220`

**Campos tocados:**
- [0xae3138] opções do jogo (game.cfg) +4: bit 2 (o tipo das ligas); valor; lê; +0x113: nome do jogo ("auto_setup_game"); texto; grava (criar_jogo_pela_configuracao)
- nação +0x11c: marca da liga (de frente 2 ou 6, de fundo 1 ou 5, pelo bit 2 das opções); valor; lê e grava (criar_jogo_pela_configuracao)
- [0xae2d00] ligas marcadas, [0xae2d04] ligas de frente; valor; grava; [0xae2d08] nação padrão; ponteiro; grava (criar_jogo_pela_configuracao)
- [0xa5a390] motor da partida ligado (byte), [0x9cd1f8] propostas do computador; valor; grava (criar_jogo_pela_configuracao)
- [0xdd7124], [0xdd713c], [0xdd7130] marcas do jogo; valor; grava 1 (criar_jogo_pela_configuracao)
- humano da vez 0xb74c26 + 0x300 × (i16)[0xb77c7a]: pessoa do técnico; ponteiro; lê e grava (criar_jogo_pela_configuracao)
- pessoa (0x6e) +0x5f: humano (1); valor; lê e grava; +0x18 ano 1966, +0x10 data, +0x24 e +0x39 = 0, +0xc texto zerado; +0x1a nação; ponteiro; +4 e +8 nomes; id; grava (criar_jogo_pela_configuracao)
- clube (0x245) +0x53: nação; ponteiro; lê (criar_jogo_pela_configuracao)
- [0xdd7110] versão do save (0xfffc1fb9); valor; grava (gravar_jogo)
- general.dat: [0xdd7110] versão, resumo (0xf74), [0xdd7120] id único, [0xdd7114] tamanho do banco (1 byte), [0xdd6ec8] e [0xae2c90] datas, [0xae2c88] hora, [0xae2c78] última data, [0xae2cfc] última hora, [0xae2d00] e [0xae2d04] ligas, nação padrão (id em +0 de [0xae2d08] ou −1), [0xae2d10] × 0x4f de [0xae2d0c], [0xae2d18] × 6 de [0xae2d14], [0xae235c] × 4 de [0xae2d1c]; bytes; lê e grava no save (gravar_jogo)
- [0xae3138] opções do jogo +4: bits 0x8000 (sem compressão), 0x20000000 (rede), 0x40000000 (conferir disco antes); valor; lê (gravar_jogo)
- [0xdd7134] sem disco, [0xdd7138] sem memória; valor; grava 1 (gravar_jogo)
- os globais dos subsistemas (PASSOS_DO_SAVE; [0xb6b7c8], o de Media, também é a condição do FOG_OF_WAR [0xae2a60]) e [0xb5ded8] (hall da fama); ponteiro; lê (gravar_jogo)
- [0xdd7130] jogo gravado, [0xa801bc]; valor; grava 1 e 0; 0xdd70a0 data do último save; valor; grava (gravar_jogo)

**Interessa a:** SGE, CMRTE
