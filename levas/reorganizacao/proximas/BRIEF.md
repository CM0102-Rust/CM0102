# As próximas levas: os assuntos que a carreira roda (12/11/2026)

O port do CM 01/02 em Rust (`D:\Sistemas\CM0102`; leia antes `CLAUDE.md`, o cabeçalho de `rust/src/motor.rs` e
`docs/COMO_ESCREVER_UM_ITEM.md`) quer fechar, assunto por assunto, tudo o que a carreira automática roda e ainda está no
original. Cada leitor recebe uma lista em `levas/reorganizacao/proximas/<prefixo>.txt` (endereço, tamanho em bytes,
arquivo de origem, nome e propósito pelo mapa de maxevans60, `referencias/cm0102-function-map`). As listas vêm da
cobertura das carreiras (`docs/cobertura_simulacao.txt`, ENG até 2002-06-01; `docs/cobertura_competicoes.txt`, 8 ligas
até 2002-06-01): toda função delas roda na carreira de teste.

Ordem da fila: `mke` (o resto do mercado), `cqx`/`cqy`/`cqz` (competições), `sma` (técnicos, disciplina, lesões,
registros), `nma` (notícias, mídia, o laço do dia).

Para cada função da sua lista:
1. **Prove a regra pela desmontagem** (`python engenharia-reversa/desmonta.py jogo/cm0102.exe INÍCIO FIM`; o
   pseudo-C de `descompilar.py` esconde argumentos de `_ftol` e a ordem do x87). Confira primeiro se a função já está em
   `rust/src/motor.rs`/`rust/src/motor/` ou em `levas/fila/*`: nunca porte duas vezes.
2. **Porte** como itens da fila (`levas/fila/<prefixo>_<n>/`, até 3 rotinas cada, no estilo atual do guia: nomes e
   cabeçalhos em inglês com a linha `Original:`, registros tipados, `exe::`, `covered!`, o comparador, `DOCS` em
   português e inglês, mutações que mudam o resultado, não só o momento), fiel ao byte. Rotina grande (mais de ~4 KB)
   pode ser partida em pedaços com nome, cada pedaço com o seu comparador, desde que a rotina inteira continue desviada
   num item só. Itens de competições levam `LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'` no `rotinas.py` (os outros, sem
   `LIGAS`). Agrupe por arquivo de origem: o objetivo é fechar arquivos e assuntos inteiros.
3. **Só compile**: aplique os itens numa cópia própria de `rust/` e `cm0102-base/` no seu scratchpad, como o `fila.py`
   faz (um `cargo build --release` por vez; apague a cópia e o `target` no fim). **Não rode o jogo**: a sessão principal
   prova cada item nas bancadas (comparação, Rust ×2, mutações). Escreva em `PRONTO.txt` o que foi compilado e o que a
   prova precisa (ligas, data, ramos que só a comparação vê).

Não faça commit; não mexa em `rust/src` da cópia principal, em `levas/<N>`, no `ordem.txt` nem nas bancadas. Outros
leitores trabalham ao mesmo tempo com os outros prefixos: se precisar de uma rotina de outra lista, leia, não porte.
Relatório curto em português: os itens, o que ficou de fora e por quê, defeitos do original achados, dúvidas.
