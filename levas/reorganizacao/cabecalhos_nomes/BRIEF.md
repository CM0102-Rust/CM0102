# Cabeçalhos das rotinas em nomes: pedido aos leitores (decisão (b), 11/11/2026)

O cabeçalho em inglês de cada rotina portada foi escrito quando o corpo ainda era acesso cru à memória e narra o código
em hexadecimal ("a entrada da pessoa (`0x7e82e0` sem criar) e uma suspensão (`+6`)..."). O corpo hoje usa nomes
(`ban_record::MATCHES`, `exe::find_ban_entry`, `.days()`); o leitor reescreve o cabeçalho com os nomes do corpo.

Entrada: um JSONL, um cabeçalho por linha (`file`, `fn`, `english`, o texto acima da linha `Original:`). Regras:
deslocamento, endereço do exe, global ou constante que o corpo nomeia vira o nome como o código o escreve; parâmetros
pelos nomes da assinatura Rust; o que o corpo não nomeia fica em hexadecimal, e os endereços de quem chama também; o
sentido fica exato (condições, valores, ordem, defeitos do original), sem resumir nem acrescentar. Só o arquivo de
saída é escrito.

Saída: TSV `<arquivo>\t<função>\t<inglês>` (parágrafos separados por ` \n `), só os cabeçalhos mudados; aplicada por
`cabecalhos_completos.py --aplicar`. Resumo final em português. O pedido completo está em
[BRIEF.en.md](BRIEF.en.md).
