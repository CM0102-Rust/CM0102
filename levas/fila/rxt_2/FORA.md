# rxt: o que ficou de fora da lista (rodada 6, 25/11/2026)

O leitor rxt escreveu o rxt_1 (0x7b81f0, o texto da notícia da busca do olheiro) e o rxt_3 (0x7be670, o relatório
do próximo adversário), ambos `TELAS` e provados por emulação. As três rotinas abaixo, os escritores de 12 a 13,5 KB,
não foram escritas por tempo: cada uma vai num item próprio (`rxt_2`, `rxt_4`, `rxt_5`), partida em pedaços com nome
e comparada pelo `compare_text_writer`. Nenhuma está em `rust/src` nem em outro item da fila (conferido em 25/11/2026).

Ferramentas no rascunho da sessão (`scratchpad\rxt\`):
- `frame.py`: o `frame.py` do rx08b corrigido para achar o `mov eax, N` do `_chkstk` depois do quadro SEH (a 0x7b95c0
  começa por `mov eax, fs:[0]`; sem a correção os deslocamentos saem errados);
- `check1.py` e `check3.py`: o molde da emulação (unicorn) da original contra a cópia da ddraw.dll compilada, com os
  stubs do langlib, do sprintf, dos nomes, dos vínculos e da caixa de erro;
- `build.sh` e `apply.py`: a cópia sobre o HEAD com o `/MAP` (o mapa dá o endereço da cópia para a emulação);
- as listagens `f2.txt` (0x7b95c0, normalizada) e `f5.txt`/`c5.txt` (0x449810).

## 0x7b95c0 `player_search_write_player_report_news_text` (player_search.cpp, 13.520 bytes) — TELAS

- Quem chama: só o despacho `NewsItem::write_text` (0x782a59), notícia 0x138b, `NEWS_WRITERS` (1, 0x138b,
  Plain(0x7b95c0)): cdecl(notícia, corpo, Text). A notícia sai do relatório de um jogador pedido pelo técnico humano à
  equipe de olheiros (tela do olheiro / ficha do jogador): `TELAS`.
- Fronteiras: 0x7b95c0..0x7bca90 (rets em 0x7bc6f5, 0x7bc83b, 0x7bca07 e 0x7bca82; sem tabela de saltos dentro).
  Prólogo `mov eax, fs:[0]; push -1; push 0x95f651` (6 + 2 bytes sem relativas); o `mov eax, 0x135c` e o `_chkstk`
  vêm depois. Três `Date` locais construídas (0x54a110, F+0x38, F+0x50, F−0x4). Igual no cm0102-1080.exe.
- Lê a vaga 0 e o nome do jogador pelo 0x7e52c0, como o rxt_1.

## 0x487f60 `write_coach_report_news_text` (coach.cpp, 12.240 bytes) — TELAS

Ver as notas de `levas/fila/rx08b_5/FORA.md` (as vagas, as chamadas, os textos dos desenvolvedores) e as listagens
`scratchpad\rx08b\f_487f60.txt` e `c_487f60.txt`.

## 0x449810 `club_records_format_record_text` (club_records.cpp, 11.840 bytes) — TELAS

Ver `levas/fila/rx08b_5/FORA.md`. A listagem com a tabela de saltos resolvida: `python frame.py 449810 44c5d0
44c5cc:32` (`scratchpad\rxt\f5.txt`, compacta em `c5.txt`, 629 linhas, 242 chamadas). Os tipos usam a posição na
competição (0x669630 o ano da temporada), o público (0x4bfa30/0x4bff70 o nome da fase e o gênero, 0x54a650 a data),
o valor (0x43f460/0x441c10) e os vínculos 0x7d0 (competição), 0x7d5 (clube).
