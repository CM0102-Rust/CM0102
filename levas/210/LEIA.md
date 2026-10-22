# 210ª leva — a história punível da imprensa, o tipo de lesão sorteado e a categoria da notícia; as convocações repetidas das seleções, a vaga do escalado e o comentarista ex-internacional; as regras da rodada no jogo da copa, a chave do jogo e o país da excursão

**Rotinas:**
- `historia_punivel_do_jogador()` = 0x7357c0 (media.cpp, thiscall): o tipo da história da imprensa sobre o jogador quando é de indisciplina, senão 0;
- `escolher_tipo_de_lesao()` = 0x62d140 (injury.cpp, thiscall): sorteia o tipo de lesão da parte do corpo e da gravidade, mantendo a lesão atual ou preferindo a recorrente;
- `categoria_da_noticia()` = 0x785670 (news.cpp, thiscall): a categoria da notícia pelo milhar do tipo;
- `apagar_repetidos_das_convocacoes()` = 0x76c060 (national_teams.cpp, stdcall): apaga as convocações repetidas da seleção principal e da seleção B da nação;
- `vaga_do_escalado()` = 0x8972c0 (tactics.cpp, thiscall): a vaga (0..19) da escalação do clube em que o jogador está, ou 0xff;
- `escolher_comentarista()` = 0x72b810 (media.cpp, thiscall): sorteia um ex-internacional (37 anos ou mais, 50 jogos pela seleção) para comentar, pesando a reputação da nação e os jogos;
- `regras_da_rodada_no_jogo()` = 0x521d60 (Cup.cpp, thiscall): passa as regras da rodada para o jogo e, com decisão no empate, marca quem passa pelo nível dos dois lados;
- `chave_do_jogo_na_copa()` = 0x519c90 (Cup.cpp, thiscall): a chave da rodada em que está o mandante do jogo (-1 se nenhuma);
- `pais_da_excursao()` = 0x5c4680 (friendly.cpp, thiscall): sorteia o país da excursão do clube, mais perto quanto menor a reputação;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 8 acusadas. `m_dobro` escapou: dobrar o peso (ou somar 1) só decide quando concorrem candidatos da nação do assunto e de outra nação; na carreira os de outra nação quase nunca passam do corte (reputação acima de 6500 + sorteio e ficha ≥ 6000), e entre os da mesma nação a ordem de 2a e a + 1 é a mesma. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 209ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 210`
