*Versão em inglês: [FORA.en.md](FORA.en.md).*

# r7_01_1: o que do modificador de gramática (0x6701f0) continua na original

Leitor r7_01, rodada 7, 28/11/2026. O item porta a entrada, o inglês (0), o português (9) e as línguas sem regra. As outras
línguas continuam na original: a cópia chama a 0x6701f0 pelo trampolim (`original_fn`), como a `area_layout` faz.

## Por que não virou vários itens

A rotina não tem auxiliares próprios: tudo é código em linha (cerca de 540 `strcmp` e 100 `strcpy` em linha e ~170 tabelas
de saltos). Os únicos chamados são o CRT (`toupper` 0x9471cd, `tolower` 0x9465fc, `sprintf` 0x944c9f, `strstr`
0x945a60, `_chkstk` 0x945610) e as duas formas do tipo de jogador do rx04_5 (0x6820a0, 0x682150). Um desvio só cabe na
entrada, e um item da fila não muda o código de outro. Então a rotina é um item só, e cada língua que faltar entra
depois como um braço novo do `match` de `grammar_modifier_into`, numa leva comum que edita o código já integrado.

## Os ramos que faltam

Pela escada de `cmp eax, N` em 0x670271 (`[0x9eb2dc]`); linhas da listagem colapsada (o `colapsa.py` do rascunho r7_01:
os `strcmp` em linha viram uma linha, as tabelas de saltos resolvidas):

| Língua | Início | Linhas colapsadas | Notas |
|---|---|---|---|
| holandês (7) | 0x6704bd | ~45 | |
| sueco (10) | 0x6705a8 | ~540 | |
| norueguês (8) | 0x670e2f | ~780 | chama a 0x6820a0 (rx04_5) |
| dinamarquês (6) | 0x6718c2 | ~270 | |
| francês (2) | 0x671de8 | ~900 | e o acerto final "de le " → "du " (0x67099b), que só roda em francês |
| italiano (4) | 0x673777 | ~730 | |
| alemão (3) | 0x674ba9 | ~4.000 | o maior; chama a 0x682150 (rx04_5) |
| espanhol (5) | 0x67c2e5 | ~2.000 | os verbos das globais 0x9ed9cc.. |

Todos terminam na cauda comum 0x67fd61: se a saída já tem texto, ela vira `prefixo + saída + sufixo`; senão (0x670974),
`prefixo + texto + sufixo`. Depois, só em francês, o acerto de 0x67099b.

## Como continuar

1. Uma língua por vez, começando pelas pequenas (holandês, dinamarquês). O modelo é o `grammar_portuguese`: as marcas e as
   formas lidas do exe pelo endereço, numa tabela conferida pelas tabelas de saltos.
2. Provar pelo emulador: o `check_gram.py` do rascunho r7_01 roda a original no unicorn contra um modelo que lê as tabelas
   do `codigo.rs`; basta acrescentar a língua ao modelo e à lista `for lang in ...`.
3. Na cadeia, uma língua que não é a do jogo de teste (português) só se prova pelo emulador ou por uma sessão comparando
   com o `game.cfg` naquela língua.
