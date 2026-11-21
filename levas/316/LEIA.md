# 316ª leva — os nomes dos tipos de lesão e o texto da notícia da volta aos treinos; o texto da notícia da saída de um técnico

**Rotinas:**
- `injury_get_type_name()` = 0x62ef30 (injury.cpp, thiscall, ret 0x10): o nome traduzido de um tipo de lesão (0 a 0x51), com o artigo indefinido ("um resfriado") ou sem ele; na língua 3 sempre sem; outro valor de artigo fora da língua 3 não escreve nada; buffer abaixo de 30 bytes ou tipo fora da tabela: a caixa de erro;
- `injury_write_recovery_news_text()` = 0x631580 (injury.cpp, thiscall, ret 0xc): o título ou o corpo da notícia da volta aos treinos leves ou completos: a manchete com o nome; o corpo pela categoria do tipo: tratamentos e férias (12) pelo tipo, pelo resultado e pelo campo 10; doenças (0) pelo tipo; as outras com o nome da lesão (a categoria, ou o tipo na categoria 6) e o gênero; nos treinos leves, o tempo previsto até os completos (aproximado); o vínculo da pessoa quando os ids e a data de nascimento ainda batem;
- `news_manager_departure_text()` = 0x6a1d60 (manager_manager.cpp, stdcall, ret 0xc): o título ou o corpo da notícia de que o técnico de um clube ou de uma seleção foi demitido, pediu demissão ou se aposentou: a manchete pelo modo da saída e pelo cargo (técnico, jogador-técnico, ou com motivo), com o nome curto do clube; o corpo, numa seleção, pela primeira ou segunda seleção, pelo modo e pelo motivo, com o adjetivo da nação; num clube, demitido como técnico ou jogador-técnico com ou sem um segundo nome, ou com um dos 16 motivos de duas tabelas; demissão pedida pelo cargo; os outros modos com um texto só; os vínculos da pessoa (quando os ids batem), do clube e do adjetivo (quando aparecem no texto);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (1):** 1 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (e39bebee).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 e9db46f8, original 60d7b94b (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 315ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 316`
