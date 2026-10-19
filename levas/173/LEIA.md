# 173ª leva — o fechamento do motor (7): a regra do elenco e as notas do jogador

**Rotinas:**
- `cumprir_regra_do_elenco()` = 0x6e8910 (match_man.cpp, cdecl): cumpre a regra do elenco (estrangeiros, extracomunitários ou idade): troca os restritos pelos candidatos de menor perda na nota;
- `nota_fora_da_partida()` = 0x6e6340 (match_man.cpp, cdecl): a nota do jogador para uma posição fora da partida (transferências e observação): posição, forma, habilidade vista, contrato, idade, conhecimento do observador;
- `nota_para_a_vaga()` = 0x6e2f20 (match_man.cpp, cdecl): a nota do jogador para uma vaga da escalação: como a nota fora da partida, mais a segunda posição, o fôlego na partida, a tática, os veteranos e as férias;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (12):** 8 acusadas. `r_banco` escapou: a fase do banco da regra de elenco (0x6e8910) não teve restritos no banco na liga inglesa (vai para a carreira ampla). `r_idade` escapou: a regra por idade (tipo 3) nunca teve um jogador exatamente no limite. `n_veterano` escapou: o ajuste do contrato (flag 0x2000000) nunca pegou um veterano (mais de 34 anos) sem jogos. `v_ferias` escapou: o bônus das férias da 0x6e2f20 só roda com a lista de férias de um técnico humano. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 172ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 173`
