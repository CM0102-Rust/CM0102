# 166ª leva — o observador do humano, o humano no clube e a categoria do contrato

**Rotinas:**
- `registro_do_observador()` = 0x7f6cd0 (scout_manager.cpp, thiscall): o registro de observação 0..9 do técnico humano (0x2b0 bytes);
- `humano_manda_no_clube()` = 0x600c90 (human_manager.cpp, cdecl): o técnico humano da vez manda no clube (ou na seleção principal da nação)?;
- `categoria_do_contrato()` = 0x4ca470 (contract_manager.cpp, stdcall): a categoria 1..8 do jogador no contrato (situação, satisfação, idade, ambição, reputação);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (7):** 3 acusadas. `h_forcar` escapou: o contador "humano manda no clube" fica em zero: sem técnico humano, a rotina nunca devolve 1 na carreira. `c_k0` escapou: limite exato; nem refeita mais forte (`c_k0_b`, `mutacoes/leva166b.py`) é acusada: a faixa da situação 0 com satisfação de 40 a 75 não aparece. `c_k5` escapou: limite exato; refeita como `c_k5_b` e acusada. `c_tipo5` escapou: contrato do tipo 5 não aparece na carreira; nem refeita mais forte (`c_tipo5_b`) é acusada. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 165ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 166`
