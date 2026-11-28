# 344ª leva — os prêmios da Turquia; as equipes da semana turcas; os prêmios da Polônia; os técnicos do mês da Polônia; os prêmios da Rússia

**Rotinas:**
- `turkey_awards_construct()` = 0x901c70 (award/turkey_awards.cpp, thiscall, ret 4): monta os prêmios da Turquia: a base com a tabela virtual da classe, a Turquia, o id 0x1a e o tipo 1; com arquivo, os salvos; senão os seis prêmios;
- `turkey_awards_init()` = 0x901d00 (award/turkey_awards.cpp, thiscall, ret 4): cria os seis prêmios turcos: jogador do ano, técnico do ano e as equipes da semana e do ano da primeira e da segunda divisão (sem reputações);
- `turkey_awards_daily_update()` = 0x901fd0 (award/turkey_awards.cpp, thiscall, ret 4): em 30 de maio, os prêmios turcos: o jogador do ano pelos três melhores das notas do ano, o técnico do ano e as equipes do ano das duas divisões; depois, todo dia, a atualização da base;
- `turkey_awards_team_of_the_week()` = 0x902080 (award/turkey_awards.cpp, thiscall, ret 4): monta a equipe da semana de um prêmio turco: só para as equipes da semana da primeira e da segunda divisão (prêmios 2 e 4), pela competição de clubes do prêmio;
- `poland_awards_construct()` = 0x7cd130 (award/poland_awards.cpp, thiscall, ret 4): monta os prêmios da Polônia: a base com a tabela virtual da classe, a Polônia, o id 0x15 e o tipo 1, sem notas do mês; com arquivo, os salvos; senão os dez prêmios;
- `poland_awards_init()` = 0x7cd1c0 (award/poland_awards.cpp, thiscall, ret 4): cria os dez prêmios: em cada divisão a equipe da semana, jogador, jovem e técnico do ano e o técnico do mês (de julho a maio);
- `poland_awards_daily_update()` = 0x7cd630 (award/poland_awards.cpp, thiscall, ret 4): em 30 de maio, em cada divisão o jogador e o jovem (até 21) do ano pelas estatísticas e o técnico do ano; no dia do fim da temporada, os técnicos do mês passam à temporada seguinte; depois, todo dia, a atualização da base;
- `poland_awards_on_month_change()` = 0x7cd790 (award/poland_awards.cpp, thiscall): na virada do mês, o técnico do mês de cada divisão; nunca roda, porque a Polônia não tem notas do mês (defeito do original);
- `russia_awards_construct()` = 0x7ec210 (award/russia_awards.cpp, thiscall, ret 4): monta os prêmios da Rússia: a base com a tabela virtual da classe, a Rússia, o id 0x1b e o tipo 1, sem notas do mês; com arquivo, os salvos; senão os sete prêmios;
- `russia_awards_init()` = 0x7ec2a0 (award/russia_awards.cpp, thiscall, ret 4): cria os sete prêmios: o jogador russo do ano e, em cada divisão, a equipe da semana, a do ano e o jogador do mês (de abril a novembro);
- `russia_awards_daily_update()` = 0x7ec600 (award/russia_awards.cpp, thiscall, ret 4): em 30 de novembro, o jogador russo do ano pelos três melhores das notas do ano e as equipes da temporada das duas divisões; depois, todo dia, a atualização da base;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (10):** 10 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 f632fb18, original ced0d1c2 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 343ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 344`
