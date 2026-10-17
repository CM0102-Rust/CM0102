# 155ª leva — o leitor dos eventos da partida (0x6d9e50)

**Rotina:** `ler_eventos_cfg()` = 0x6d9e50 (match_events.cpp, ~5,2 KB): na abertura, lê o `events_<idioma>.cfg` (ou o `events_eng.cfg`) para a tabela dos 677 códigos de evento em [0xb68788]: opções de cada código, os sons (peso, tipo, nome) e os dois textos de narração. Confere as marcas `<X` dos textos e, com `-verifysounds`, os arquivos de som.

**Junto:** a leitura dos lances da 154ª (0x6d9c40) ganhou comparação de verdade, porque só lê arquivo. Foi o que provou que o erro `Area.cpp 507` no relatório de partida antiga não é do Rust (0 divergências; é o limite de 30 linhas da interface em 1080, anotado no wiki para a etapa das telas).

**Como foi comparada:** sem comparação própria (lê arquivo e aloca), pela carreira (Rust × original).

**Resultado** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutações (6 e 2 refeitas): 2 acusadas e 6 escaparam. Os sons e as opções 0, 1 e 4 só servem ao áudio e à tela: lacuna em `docs/PENDENCIAS.md`. No jogo do Lucas, a narração e os sons funcionaram.
- Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3).
- Carreiras Rust ×2 e original até 2002-06-01: as três iguais (034ff3eb3e55dfd3).

**Refazer:** sobre o commit da 154ª (e a correção 601d37c):
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 155 -`
