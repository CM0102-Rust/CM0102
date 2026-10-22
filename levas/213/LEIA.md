# 213ª leva — a contratação pelas regras da nação e as trocas que oferecem o jogador; as notícias não lidas, a marca gravada na notícia da caixa e o clube de técnico humano; as datas do sorteio e do jogo da rodada da copa e o clube que sai dos amistosos planejados

**Rotinas:**
- `pode_contratar_jogador()` = 0x8ced40 (transfer_manager.cpp, thiscall): se o clube pode contratar o jogador pelas regras da nação, com o motivo da recusa e a segunda chance do clube humano no limite de estrangeiros;
- `trocas_oferecendo_o_jogador()` = 0x8c5e40 (transfer_manager.cpp, thiscall): quantas ofertas abertas do clube oferecem o jogador na troca, com a lista dos índices se pedida;
- `noticias_nao_lidas()` = 0x786fe0 (news.cpp, thiscall): as notícias não lidas da caixa do técnico humano, no total ou de uma categoria;
- `gravar_marca_da_noticia()` = 0x786d30 (news.cpp, thiscall): grava um valor numa vaga da notícia da caixa do técnico humano e na cópia local dela;
- `clube_de_tecnico_humano()` = 0x7875e0 (news.cpp, stdcall): se um humano dirige o clube, sem contar as férias;
- `data_do_sorteio_da_rodada()` = 0x521e60 (Cup.cpp, cdecl): grava a data do sorteio da rodada da copa: o dia do ano e o ano contado do ano-base;
- `data_do_jogo_da_rodada()` = 0x521eb0 (Cup.cpp, cdecl): grava a data do jogo da rodada da copa, o horário e as marcas do mando;
- `tirar_clube_dos_amistosos()` = 0x5c64e0 (friendly.cpp, thiscall): tira o clube dos amistosos planejados pelo computador e das listas de amistosos;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (8):** 7 acusadas. `tr_recusa` escapou: na carreira as regras de transferência não recusam nenhuma contratação perguntada sem a segunda chance (a5 = 0), então o caminho "recusado, sem segunda pergunta" não roda (nem no modo comparação). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (5afd987e1a1a3ef3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b11bd5cb63f51d72).

**Refazer**, sobre o commit da 212ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 213`
