# Conversão da fila para o estilo atual: pedido aos conversores (11/11/2026)

Os itens da fila (`levas/fila/<item>/`) foram escritos antes da reorganização de 07/11 e ainda usam o estilo antigo
(nomes, comentários e cabeçalhos em português, prefixos de item, `transmute` de endereços crus, deslocamentos crus,
`crate::comparison::branch`). O conversor reescreve cada item no estilo atual sem mudar o que ele faz: nomes e
cabeçalhos em inglês com a linha `Original:`, registros tipados e campos com nome (os que ainda não têm nome ficam crus
e vão para `campos_novos.json`), `exe::`, `covered!`, as mutações acompanhando o texto novo e os `DOCS` em português e
inglês. Mesmos deslocamentos, larguras, sinais, convenções e ordem dos efeitos. A prova é a cadeia normal de cada leva
quando o item for integrado. O pedido completo está em [BRIEF.en.md](BRIEF.en.md).
