# 346ª leva — os prêmios do País de Gales; o técnico do mês e a equipe da semana de Gales; os prêmios da Escócia

**Rotinas:**
- `wales_awards_construct()` = 0x90ead0 (award/wales_awards.cpp, thiscall, ret 4): monta os prêmios de Gales: a base com a tabela virtual da classe, Gales, o id 0x1c e o tipo 1; com arquivo, os salvos; senão as notas do mês (sem memória, a caixa de erro e nenhum prêmio) e os seis prêmios;
- `wales_awards_init()` = 0x90ec10 (award/wales_awards.cpp, thiscall, ret 4): cria os seis prêmios galeses da Premier Division: a equipe da semana, jogador, jovem, artilheiro e técnico do ano, e o técnico do mês (de agosto do ano corrente a abril do seguinte);
- `wales_awards_daily_update()` = 0x90ef10 (award/wales_awards.cpp, thiscall, ret 4): em 11 de maio, os prêmios galeses: jogador, artilheiro e jovem (até 21 anos) do ano e o técnico do ano; no dia do fim da temporada da nação, o prêmio do mês passa para a temporada seguinte; depois, todo dia, a atualização da base;
- `wales_awards_on_month_change()` = 0x90f020 (award/wales_awards.cpp, thiscall): na virada do mês, o técnico do mês da Premier Division galesa: o melhor técnico do mês dos registros fica com o prêmio do mês, com o clube dele;
- `wales_awards_team_of_the_week()` = 0x90f060 (award/wales_awards.cpp, thiscall, ret 4): monta a equipe da semana de um prêmio galês: só para a equipe da semana (prêmio 0);
- `scotland_awards_construct()` = 0x7f5170 (award/scotland_awards.cpp, thiscall, ret 4): monta os prêmios da Escócia: a base com a tabela virtual da classe, a Escócia, o id 0x11 e o tipo 1; com arquivo, os salvos; senão as notas do mês e os 27 prêmios;
- `scotland_awards_init()` = 0x7f52b0 (award/scotland_awards.cpp, thiscall, ret 4): cria os 27 prêmios: jogador, artilheiro e jovem escocês do ano e, em cada uma das quatro divisões, a equipe da semana, técnico, jogador e jovem do mês (de agosto a maio), técnico do ano e equipe do ano;
- `scotland_awards_daily_update()` = 0x7f5dc0 (award/scotland_awards.cpp, thiscall, ret 4): no sábado de 23 de maio, o técnico do ano de cada divisão, jogador, jovem (até 21) e artilheiro escocês do ano pelos três melhores das notas do ano e a equipe da temporada de cada divisão; no dia do fim da temporada, os doze prêmios do mês passam à temporada seguinte; depois, todo dia, a atualização da base;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (8):** 8 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 abec784e, original eb48bed6 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 345ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 346`
