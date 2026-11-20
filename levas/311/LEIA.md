# 311ª leva — a virada da temporada polonesa; os inscritos das copas da Polônia e da Grécia; a virada e as fases da categoria B turca e as fases da Primeira Divisão polonesa

**Rotinas:**
- `pol_first_start_new_season()` = 0x7c9740 (pol_first.cpp, thiscall, ret 0): regra: a virada polonesa, puxada pela Primeira Divisão: o acesso e o rebaixamento pela cadeia; os quatro melhores da divisão mais baixa sobem à Segunda e os últimos dela descem; a Primeira refeita para o ano seguinte (regras, tabela, jogos, ordem) e a sua segunda fase de grupos; a virada da Segunda; as posições previstas, as expectativas da diretoria e os árbitros;
- `pol_first_promote_lower_division_clubs()` = 0x7ca030 (pol_first.cpp, thiscall (ecx = a Primeira Divisão), ret 0): regra: na virada polonesa, os quatro melhores clubes poloneses da divisão mais baixa (ordenados por força, os doze melhores embaralhados) sobem à Segunda Divisão, e os últimos da Segunda (tantos quantos os rebaixados diretos dela) descem; cada candidato guarda a divisão como a anterior; com menos de 4 candidatos ninguém sobe nem desce;
- `pol_second_start_new_season()` = 0x7cc000 (pol_second.cpp, thiscall, ret 0): regra: a virada da Segunda Divisão polonesa (chamada pela da Primeira): a temporada reaberta, as fases da temporada passada destruídas, o ano seguinte, as regras, a tabela, os jogos e a ordem refeitos; as posições previstas, as expectativas da diretoria e os árbitros;
- `pol_cup_collect_entrants()` = 0x7c8550 (pol_cup.cpp, thiscall, ret 0): regra: a Copa da Polônia tem 52 clubes: os da Segunda Divisão que ficaram entre o 13º e o 20º na temporada passada, depois os até o 12º (classe 2); depois os da divisão mais baixa (os que subiram primeiro, depois os outros, ordenados por força e os melhores embaralhados) até serem 36 (classe 3); depois os da Primeira Divisão da temporada passada (classe 1);
- `pol_lge_cup_collect_entrants()` = 0x7cad30 (pol_lge_cup.cpp, thiscall, ret 0): regra: a Copa da Liga polonesa tem 36 clubes em 2001 e 34 depois, todos da classe 0: os da Segunda Divisão que subiram de baixo, os outros da Segunda, os da Primeira que ficaram abaixo do 7º e os até o 7º, na ordem do banco até a lista encher;
- `gre_cup_collect_entrants()` = 0x5e9a80 (gre_cup.cpp, thiscall, ret 0): regra: a Copa da Grécia tem 44 clubes: 16 da Terceira Divisão (ordenados por força, os 48 melhores embaralhados), depois os da Segunda e os da Primeira Divisão na ordem do banco até a lista encher; faltando clubes, a caixa de erro;
- `tur_second_b_start_new_season()` = 0x900500 (tur_second_b.cpp, thiscall, ret 0): regra: a virada da segunda divisão turca categoria B (chamada pela da Primeira): a temporada reaberta, as fases destruídas, o ano seguinte; as regras, a tabela, a ordem e os jogos (nesta ordem); as quatro fases de grupos refeitas; as posições previstas e os árbitros (sem as expectativas da diretoria);
- `tur_second_b_advance_stage()` = 0x9006c0 (tur_second_b.cpp, thiscall, ret 0): regra: a fase seguinte da categoria B turca: depois da última, a caixa de erro; senão a fase atual sobe uma; ao chegar à fase 4 são montados o grupo de acesso e os grupos de classificação, ao chegar à 10, o mata-mata;
- `pol_first_advance_stage()` = 0x7c9ad0 (pol_first.cpp, thiscall, ret 0): regra: a fase seguinte da Primeira Divisão polonesa: depois da última, a caixa de erro; senão a fase atual sobe uma; ao sair da fase 0 (chegando à 1) é montada a fase final;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 3 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (db2706cd).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 be15cf32, original 8dd48626 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 310ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 311`

**Campos tocados:**
- liga +0x4c, +0xba, +8, +0xc, +0x30, +0x40, +0x3e, +0xc1; clube +0x53 nação, +0x57 divisão, +0x5b divisão anterior
- copa +0x3e número de times, +0x40 ano, +0xa7 lista de 6 bytes; copa grega +0xee lista de clubes, +0xf2 tamanho; clube +0x57 divisão, +0x5b divisão anterior, +0x5f posição anterior
- liga +0x2c número de fases, +0x30 fase atual, +0x4c, +0xba, +8, +0xc, +0x40
