# 239ª leva — a criação do jogo: inicializar os jogadores e as pessoas

**Rotinas:**
- `inicializar_jogadores()` = 0x535a80 (Database.cpp, cdecl): prepara cada pessoa do jogo novo: nomes que faltam, datas, atributos de pessoa, nação; do jogador as habilidades, o potencial, a atual, a idade, as posições e os atributos que faltam (pelo jogador mais parecido entre 250 sorteados); do não jogador as reputações, os atributos (o molde entre 250), o cargo e os nomes famosos; depois os vetores relativos à habilidade e as normalizações; barra "Player Initialisation";

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Prova do save** (o `.qst` gravado em modo Rust com e sem as rotinas da leva, fora dos bytes que variam sozinhos; as mutações também foram por ela): IGUAL (16856054 bytes).
- **Mutações (5):** 5 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 238ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 239`

**Campos tocados:**
- TStaff +4/+8/+0xc nomes, +0x10 nascimento, +0x18 idade, +0x1a nação, +0x31/+0x3e/+0x46 datas, +0x39 clube, +0x3d cargo, +0x56..+0x5d atributos; grava (inicializar_jogadores)
- TPlayer +5 atual, +7 potencial, +9/+0xb/+0xd habilidades (×50), +0x45, posições, lados e atributos; grava (inicializar_jogadores)
- TNonPlayer +4/+6 habilidades, +8/+0xa/+0xc reputações (×50), atributos, +0x23..+0x42 zerados, +0x43; grava (inicializar_jogadores)
- tabela de 79 bytes por pessoa [0xae2d0c]: +9 = 0x9c (jogador), +0xb &= 0xf0; grava (inicializar_jogadores)
- vetores dos moldes 0xae2044/0xae2114 (bandeira 0x9b59b8) e 0xae18bc/0xae2238 (0x9b59b9); grava (inicializar_jogadores)

**Interessa a:** SGE, CMRTE
