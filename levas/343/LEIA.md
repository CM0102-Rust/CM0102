# 343ª leva — os prêmios da Suécia; as equipes da semana suecas e as regras de transferência da Suécia; o jogador do ano pelas estatísticas da temporada e as ordens dos candidatos; os prêmios do mês da Noruega

**Rotinas:**
- `sweden_awards_construct()` = 0x893720 (award/sweden_awards.cpp, thiscall, ret 4): monta os prêmios da Suécia: a base com a tabela virtual da classe, a Suécia como nação, o id 0x13 e o tipo 1; com arquivo, os prêmios salvos; senão os dez prêmios;
- `sweden_awards_init()` = 0x8937b0 (award/sweden_awards.cpp, thiscall, ret 4): cria os dez prêmios suecos: técnico do ano, jogador, jogador da torcida, goleiro, defensor, meio-campista e atacante do ano, a equipe da semana da Premier Division, o artilheiro dela e a equipe da semana da First Division, cada um com as reputações nacional e mundial dele;
- `sweden_awards_daily_update()` = 0x893bf0 (award/sweden_awards.cpp, thiscall, ret 4): em 1º de dezembro, os prêmios do ano suecos: o técnico (pela Premier Division), o jogador e o da torcida pelas notas do ano, goleiro, defensor, meio-campista e atacante pelos três melhores das notas por posição, e o artilheiro da Premier Division; depois, todo dia, a atualização da base;
- `sweden_awards_team_of_the_week()` = 0x893d60 (award/sweden_awards.cpp, thiscall, ret 4): monta a equipe da semana de um prêmio sueco: só para as equipes da semana da Premier Division e da First Division (prêmios 7 e 9), pela competição de clubes do prêmio;
- `construct_sweden_rules()` = 0x893d80 (award/sweden_awards.cpp, thiscall, ret 0x14): monta as regras de transferência da Suécia: a base, a tabela virtual da classe, nenhuma janela de transferências e a marca +0x15 = 1 (o contrato de tipo 5 não tira a elegibilidade);
- `nation_awards_award_player_of_year_by_stats()` = 0x769380 (nation_awards.cpp, thiscall, ret 0x24): regra: cada jogador cujo clube (se houver) joga numa competição em andamento e que passa nos filtros de nação (a do clube, a própria, a excluída, a própria ou a do clube) recebe uma nota pela estatística do modo (categoria 2 no modo 2, senão 6) e os seus jogos; no modo 2 nota 0 fica de fora; no modo 3 a nota é por jogo; fora do modo 2 ela é multiplicada por metade do peso da reputação ((reputação do clube / 500 + reputação da divisão x 4) / 5, ou 1). Ordenados pela nota (menor primeiro no modo 3), os três primeiros que jogam numa das posições (zagueiro que também é lateral não conta), têm jogos bastantes (fração de 25 jogos: 0,66, ou 0,01 nos modos 2 e 5, baixando 0,1) e passam na regra da idade vão para o histórico do prêmio. Defeitos: sem candidato o bloco não é liberado; fora do modo 2 a nota do registro fica com o lixo da pilha;
- `nation_awards_compare_score_ascending()` = 0x76a480 (nation_awards.cpp, cdecl): regra: a ordem dos candidatos pela nota, a menor primeiro (sem ordem conta como menor);
- `nation_awards_compare_score_descending()` = 0x76a4b0 (nation_awards.cpp, cdecl): regra: a ordem dos candidatos pela nota, a maior primeiro (sem ordem conta como menor);
- `norway_awards_on_month_change()` = 0x794430 (award/norway_awards.cpp, thiscall): na virada do mês, o técnico do mês de cada divisão (o melhor técnico do mês, com o clube) e o jogador do mês de cada uma pelas notas do mês;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (10):** 10 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 77f1b62b, original 18b0f378 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 342ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 343`

**Campos tocados:**
- candidato do prêmio pelas estatísticas (10 bytes): +0 pessoa, +4 nota (f32), +8 jogos (word); valores; grava e lê (nation_awards_award_player_of_year_by_stats e as ordens)
