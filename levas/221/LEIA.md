# 221ª leva — o save: carregar o jogo inteiro

**Rotinas:**
- `carregar_jogo()` = 0x82a9c0 (Setup.cpp, cdecl): carrega o save: disco, memória, CD, o banco (0x524d10), desempacota os *.tmp, o general.dat (versão, resumo, ids, datas, ligas, nação padrão, os dados de execução de pessoas, clubes e reputação, convertendo saves antigos), os construtores dos subsistemas na ordem da original com disco e memória entre eles, as regras FIFA e a data do último save;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos, e o save de férias carregado até 2001-08-28 com e sem elas, mesma impressão; as mutações também foram por ela): IGUAL fora dos campos ignorados; carga IGUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73).
- **Mutações (3):** 3 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 220ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 221`

**Campos tocados:**
- [0xdd7110] versão do save; valor; grava (lida do general.dat) e lê (carregar_jogo)
- general.dat: versão, resumo (0xf70/0xf74; +0xf6c tempo de jogo → [0xdd7140]), [0xdd7120] id único, [0xdd7114] tamanho do banco, [0xdd6ec8] data, a data do jogo (local → [0xae2c90] pela 0x417c10), [0xae2c88] hora, [0xae2c78] última data, [0xae2cfc] última hora, [0xae2d00] e [0xae2d04] ligas, nação padrão (id → [0xae2d08] = [0xae23a8] + 0x122 × id, ou 0), [0xae2d10]; bytes; lê do save e grava (carregar_jogo)
- [0xae2d0c] dados de execução das pessoas (0x4f × [0xae2d10]; save antigo de 0x26/0x27 bytes convertido: +0..+0xd, +0xe/+0x12 × 5 e +0x27..+0x37 × 2 = −1); ponteiro e bytes; aloca e grava (carregar_jogo)
- [0xae2d14] dados de execução dos clubes (6 × [0xae2d18] = [0xae235c]), [0xae2d1c] reputação (4 × [0xae235c]); ponteiro e bytes; aloca e grava (carregar_jogo)
- os globais dos subsistemas (PASSOS_DA_CARGA, de [0xde1d08] Tactics a [0xb74430] PunditList; o FOG_OF_WAR [0xae2a60] é conferido por [0xb6b7c8], o de Media); ponteiro; grava (carregar_jogo)
- pessoa (0x6e) +0x5f: 5 nos 16 últimos vira 0 (técnico desfeito: nomes +4/+8/+0xc vazios, +0x10 data, +0x18 ano 1966, +0x24 e +0x39 = 0); valor; lê e grava (carregar_jogo)
- [0xdd7134] sem disco, [0xdd7138] sem memória, [0xdd711c] e [0xdd7130] jogo carregado, [0xa801bc] alterações = 0, 0xdd70a0 data do último save; valor; grava (carregar_jogo)
- [0x9b5980] regras FIFA existem; [0xae233c] em vigor, [0xa83608] contrato máximo (10 ou 5), [0xae2330] data da mudança; valor; lê e grava (carregar_jogo)

**Interessa a:** SGE, CMRTE
