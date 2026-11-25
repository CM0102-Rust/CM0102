*Versão em inglês: [FORA.en.md](FORA.en.md).*

# rx04_6: langlib_apply_grammar_modifier (0x6701f0, Langlib.cpp): fora, por enquanto

Leitor rx04, rodada 6, 25/11/2026. A rotina **não** foi portada nem entra na fila: é grande demais para uma rodada.
O resto do Langlib.cpp entrou ([rx04_1](../rx04_1/rotinas.py) a [rx04_5](../rx04_5/rotinas.py)); a cópia do
`langlib_sprintf` (rx04_2) chama esta original por `exe::langlib_apply_grammar_modifier`.

## O tamanho

- A entrada do mapa vai de 0x6701f0 a 0x682060 (73.328 bytes). Pela desmontagem, o código vai até 0x67fdc4 (~64 KB, 14
  `ret`; o corpo segue depois do último `ret` em 0x678466, por saltos) e de 0x67fdc4 a 0x682060 ficam ~170 tabelas de
  saltos e de índices (8,8 KB) dos `switch` pela primeira letra (`movsx eax, byte [esp+0x1018]; add eax, -0x41; cmp eax,
  0x34`) e pelo gênero.
- Quadro de 0x2808 bytes (`call 0x945610`, o `_chkstk`), 4 buffers de 0x800 (o texto em 0x1018, as saídas em 0x18 e
  0x818) e um de 0x800 zerado em 0x1818.

## O que já se sabe (para quem continuar)

- **Convenção:** cdecl, 5 dwords: o texto, a marca antes ("upper", "lower"...), a marca depois ("s", "S", artigos), e os
  dois bytes de gênero (lidos só como byte: `movsx eax, byte [esp+0x2828]` 82 vezes, `[esp+0x282c]` 59 vezes). O
  `langlib_sprintf` passa os gêneros do chamador (0xff 0xff = nenhum, e aí não chama) ou 0x64 0x64 na 2a passada inglesa.
- **Saída:** devolve 0xb64804 (o único global que escreve, zerado na entrada), 0xad9c64 (texto vazio, quando o texto é
  nulo) ou 0 (7 saídas `xor eax, eax`: o `langlib_sprintf` lê o byte do ponteiro e cai; conferir quando acontece).
- **Ramos por língua** (`[0x9eb2dc]` em 0x670271): 0 inglês (0x670276: "upper" põe a 1a letra em maiúscula pelo
  `toupper` 0x9471cd; "s"/"S" e os artigos "a "/"an "/"A "/"An " pela 1a letra, tabelas 0x67fdc4/0x67fe04), 7 holandês
  (0x6704bd), 10 sueco (0x6705a8), 8 norueguês (0x670e2f; chama a 0x6820a0 do rx04_5), 6 dinamarquês (0x6718c2), 2
  francês (0x671de8), e depois espanhol (5; os verbos "seguía", "ganará", "comenzó" das globais 0x9ed9cc..), italiano,
  alemão (chama a 0x682150 do rx04_5) e português. 23 chamadas ao `sprintf` 0x944c9f, ~30 textos fixos.
- **Os dados:** o language.ldb tem 9.807 textos e 10 línguas (0, 2..10); o `scan_ldb.py` do rascunho `rx04` lê as
  traduções e as marcas usadas, que são a lista de casos a provar por língua.

## Como continuar

1. Partir pelos ramos de língua (um pedaço com nome por língua, como o gra_1), começando pelo português (9), que é o do
   jogo de teste.
2. Provar cada pedaço pela emulação (o `gra_2b/emu.py` ou o `sbB\emu.py`): rodar a original no unicorn com cada
   (marca antes, marca depois, gênero) que o language.ldb usa em português, e cada texto de pedaço comum, e comparar
   com a cópia.
3. Item de simulação, a rotina inteira desviada num item só; o comparador lado a lado no 0xb64804 (a cópia num
   buffer próprio, a original no global).
