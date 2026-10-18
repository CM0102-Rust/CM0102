# 163ª leva — a formação (marcas e casas) e o reinício dos recordes do clube

**Rotinas:**
- `casa_guardada()` = 0x5b3c70 (formation.cpp, thiscall): a casa guardada da vaga pelo ataque e pela faixa;
- `ligar_marca_da_formacao()` = 0x5b50d0 (formation.cpp, thiscall): liga a marca em +0x584 apagando as do mesmo grupo e zera +0x187f;
- `reiniciar_recordes()` = 0x452fe0 (club_records.cpp, cdecl): −1.0f e −1 nos campos dos recordes do clube;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (5):** 5 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 162ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 163`
