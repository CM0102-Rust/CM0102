# 208ª leva — as áreas da interface (1): organizar a grade, iniciar e desenhar uma área (com a trava das 30 linhas: o "Area.cpp 507" do relatório em 1080)

**Rotinas:**
- `organizar_area()` = 0x403350 (Area.cpp, thiscall): reparte a largura e a altura da área pelos pesos das colunas e linhas (até 30 de cada; mais dá a caixa Area.cpp 497/507) e liga a barra de rolagem;
- `iniciar_area()` = 0x402ac0 (Area.cpp, thiscall): inicia a área: retângulo limitado à tela, campos, barra de rolagem zerada e os pesos das colunas e linhas; as linhas à mostra travadas em 30 (o tamanho da grade), o resto vai para a barra de rolagem;
- `desenhar_area()` = 0x402e70 (Area.cpp, thiscall): desenha a área: o painel, as linhas da grade ou cada item à mostra na sua célula, e a barra de rolagem;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 207ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 208`
