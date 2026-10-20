# 184ª leva — a disciplina: a temporada nova, o dia e a vaga da tabela

**Rotinas:**
- `disciplina_temporada_nova()` = 0x556210 (discipline.cpp, thiscall): começa a temporada nova das regras de disciplina da competição (método 0 da tabela virtual);
- `disciplina_do_dia()` = 0x556240 (discipline.cpp, thiscall): o dia de cada uma das 43 regras de disciplina (método 1 da tabela virtual);
- `disciplina_gravar_vaga()` = 0x55cb20 (discipline.cpp, thiscall): grava o tipo e o valor de uma vaga da pessoa na tabela da disciplina;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (4):** 2 acusadas. `d_sem_regra` escapou: a competição com as regras de disciplina de índice 0 não começa temporada na carreira (liga fora das carregadas). `d_43` escapou: a 43ª regra de disciplina está vazia ou não muda nada nas ligas carregadas. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 183ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 184`
