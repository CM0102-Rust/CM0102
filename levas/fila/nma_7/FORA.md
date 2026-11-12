# nma_7 fora (12/11/2026): NewsItem::write_text (0x77bc80, news.cpp) grande demais para uma sessão

A rotina não foi portada. Não há cópia parcial (nem desviada, nem guardada): o desvio tem de ser da rotina inteira, num `Prologue` só, e uma porta fiel de tudo não cabe numa sessão com a conferência que o item pede. Fica aqui o mapa medido na desmontagem (`desmonta.py jogo/cm0102.exe 0x77bc80 0x782b70`, 8.440 instruções, 599 chamadas), para a próxima rodada partir em sessões.

## Assinatura e moldura

- thiscall, `ret 0x10` (4 argumentos): ecx = a notícia; arg1 (`[esp+0x13c8]` depois dos 4 push) não pode ser 0; arg2 = o objeto `Text` (ebp), limpo por 0x4afa40 (comp_text.cpp) logo no início; arg3 = manchete (≠0) ou corpo; arg4 passa adiante. Devolve 1 (escrito) ou 0 (erro).
- Moldura de 0x13a8 bytes (`__chkstk` 0x945610) com SEH (0x95e1cb): vários `Text` locais destruídos no caminho de erro e na saída.
- arg1 ou arg2 nulos: a caixa "Error" com `news.cpp` linhas 0x9f / 0xa7 (0x944cff, 0x90d130, 0x944c9f, 0x5e8290), zera 0xb67a34 e devolve 0.
- A categoria vem de 0x785670 (thiscall, a notícia; devolve o grupo em al); o tipo é o dword `[noticia+0]`.

## Os pedaços (por categoria)

| Categoria (0x785670) | Tipos que só despacham para o escritor do exe | Corpo próprio (o grosso da rotina) |
|---|---|---|
| 2 | 0x1b–0x25, 0x29, 0x2a, 0x2b, 0x2c, 0x2e, 0x2f, 2, 0x18, 0x19, 0x1a (21 escritores: 0x414420, 0x76e3b0, ..., 0x6aadb0) | **tipos 1, 3–0xd, 0x10–0x17, 0x26, 0x27, 0x28: 0x77bfe2–0x7801e9, ~16,9 KB**; o 0x15 (0x77f3e4, 0x9a4 bytes) tem um sub-switch (0x41c, 0xa, 0x14, 0x82, 0x449, 0x44a) |
| 7 | 0x3e8 (0x44c650) | 0x3e9: 0x780229–0x7802cd |
| 3 | 0x7d1, 0x7d2, 0x7d3 (0x631580, 0x5576e0, 0x55b8b0) | **0x7d0: 0x7802e5–0x78164b, ~4,9 KB**, com tabelas de salto (0x782ae8, 0x782afc, 0x782b3c, 0x782b50) |
| 6 | 0x1770–0x1790, 31 escritores (0x59f3e0 ... 0x6a3060) | — |
| 4 | 0xbb8–0xbea, 41 escritores (0x7aa4c0 ... 0x738d50; inclui 0x72d270 e 0x744f80, em porta por outros agentes) | — |
| 5 | 0xfa0–0xfc3, 36 escritores (0x8d3520 ... 0x8e51d0; inclui 0x755860, em porta) | — |
| 1 | 0x1389–0x1394 (12 escritores) | 0x1388: 0x782848–0x782a02, monta um `Text` local e **chama a própria 0x77bc80** sobre outra notícia (recursão; 0x786b70, 0x785a20, 0x53dba0) |

Tipo não reconhecido: cai em 0x782a68 (devolve 0, sem escrever).

O despacho (≈150 tipos) é mecânico. O que não coube são os ~22 KB de corpo próprio: textos montados com `langlib_sprintf` (0x66f4e0, 101 chamadas), `Text::add_link` (0x4ae8a0, 86), `Text::set_string` (0x4ae660, 52), datas (0x549ef0, 0x54bef0, 0x54a3f0, 0x54a110), nomes de competição (0x4bfa30, 0x4bff70, com as trocas de artigo "f"/"r" por comparação de cadeia), lesões (0x6306e0 ×22, 0x62ec00, 0x62ef30), tabelas de clube/pessoa/competição lidas por campo cru, e os `Text` locais com destrutor.

## Como fazer (proposta)

1. Uma sessão por grupo de corpo próprio, numa pasta de preparo **sem** `Prologue` (por ex. `levas/fila/nma_7/pedacos/`): (a) categoria 2, tipos 1–0xd; (b) categoria 2, tipos 0x10–0x17, 0x26–0x28 (o 0x15 com o sub-switch); (c) categoria 3, 0x7d0 com as tabelas de salto; (d) categoria 7 0x3e9 e categoria 1 0x1388 (a recursão chama a 0x77bc80 pelo exe, que é o desvio).
2. Depois, uma sessão junta o despacho, as asserções e os pedaços, desviando a rotina inteira, com o comparador de texto (original em `out`, a cópia num `Text` local dentro de `side_by_side` com `STATE_REGIONS` e `without_counting`, comparando cadeia e links) e o `covered!` de cada pedaço (`nma_7 cat2 type 6`, ...), nomeando o pedaço da divergência.
3. Saber antes quais tipos a carreira ENG roda (só a contagem da função está em `docs/cobertura_simulacao.txt`: 4.568 chamadas até 2002-06-01), para pôr as mutações nos pedaços que rodam.

Defeitos da original: nenhum achado nesta leitura (só o mapa foi feito).
