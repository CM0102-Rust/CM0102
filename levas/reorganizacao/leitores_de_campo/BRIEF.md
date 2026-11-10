# Leitor de campo: o pedido

O pedido que os agentes leitores de campo recebem está em inglês em [BRIEF.en.md](BRIEF.en.md) (eles trabalham em
inglês). Em resumo: para cada `+ 0x..` cru de um arquivo Rust (o censo, linha a linha), o leitor diz para que registro a
base aponta e dá o nome do campo, com largura, descrição e confiança (`verified` com evidência concreta, `read` pelo uso
claro, `inferred` palpite); ou diz que não é campo (conta, índice, coordenada, posição na tabela de métodos) ou que não
sabe. Também dá nome aos bits dos campos de flags. Os vereditos (`v_*.json`, `mm_vereditos*.json`) são aplicados por
`nomear_por_linha.py`, `nomear_bits.py` e `ajustar_constantes.py`.
