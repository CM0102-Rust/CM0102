# 335ª leva — o menu de nacoes partido em duas partes

**Rotinas:**
- `nation_menu_split_construct()` = 0x66a280 (key_nation.cpp, thiscall, ret 0xc): monta a divisao do menu de nacoes: conta as nacoes da tabela das nacoes-chave escolhidas para jogar (so as marcadas com o bit 2, se pedido) e, na ordem dada, a nacao do meio ((contagem + 1) / 2) da a letra onde comeca a segunda parte: a primeira letra do nome curto ou do nome;
- `nation_menu_split_format_header()` = 0x66a360 (key_nation.cpp, thiscall, ret 0xc): escreve o cabecalho de uma parte do menu: "Nacoes A-{letra}" na primeira, "Nacoes {letra seguinte}-Z" na segunda; texto de menos de 20 bytes: a caixa "Error" e nada;
- `nation_menu_split_in_second_part()` = 0x66a430 (key_nation.cpp, thiscall, ret 4): diz se a nacao vai para a segunda parte: a primeira letra do nome curto (ou do nome) depois da letra da divisao;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 3 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (50d0e931).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 73c66633, original 2520f435 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 334ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 335`
