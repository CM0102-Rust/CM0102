# 176ª leva — a recuperação da lesão, a pessoa na lista de transferências e a punição da disciplina

**Rotinas:**
- `recuperar_lesao()` = 0x62bd10 (injury.cpp, thiscall): o dia de recuperação (abaixo de −1000 em x87 com −0.0035 e 2.5; senão o ritmo ou sorteio);
- `pessoa_na_lista_de_transferencias()` = 0x8c1870 (transfer_manager.cpp, thiscall): a pessoa está na lista +0x92c?;
- `punicao_da_tabela()` = 0x55cae0 (discipline.cpp, thiscall): a punição da linha e da coluna da tabela, se o tipo bater;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (6):** 5 acusadas. `t_item` escapou: a lista +0x92c do gerente de transferências nunca tem a pessoa procurada na carreira automática (a rotina devolve sempre 0); vai para a carreira ampla. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 175ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 176`
