# Leitor de enumeração: o pedido

O pedido que os agentes leitores de enumeração recebem está em inglês em [BRIEF_ENUMS.en.md](BRIEF_ENUMS.en.md). Em
resumo: para cada campo de registro que o código compara com números soltos (`row.status() == 0xf`,
`p.job_for_club() == 0xb`), o leitor diz se é uma enumeração e dá nome a cada valor, com significado e confiança
(`verified` com evidência concreta, de preferência as strings que o jogo imprime para cada valor; `read` pelo uso;
`inferred` palpite). Os vereditos (`levas/reorganizacao/enums_*.json`) são aplicados por `aplicar_enums.py`, que gera o
`rust/src/motor/enums.rs`.
