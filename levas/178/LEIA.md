# 178ª leva — a média do jogo, o valor do contrato e o nível para a seleção

**Rotinas:**
- `media_do_jogo()` = 0x5d5a60 (game.cpp, thiscall): a média f64 +0x20/+0x28 ou +0x10/+0x18 (divisor zero: 6.8);
- `valor_do_contrato()` = 0x4d0c20 (contract_manager.cpp, thiscall): o dword +0xc da ficha de contrato da pessoa;
- `nivel_para_a_selecao()` = 0x76d810 (national_teams.cpp, stdcall): o nível 1..3 do jogador para a seleção pela reputação e pelas notas de posição;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (5):** 4 acusadas. `m_padrao` escapou: o divisor da média (+0x18/+0x28) nunca é zero quando a carreira consulta a média: o 6,8 padrão não aparece. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 177ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 178`
