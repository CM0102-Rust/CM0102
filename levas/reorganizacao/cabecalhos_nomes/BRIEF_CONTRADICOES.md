# Contradições entre cabeçalho e corpo: pedido aos juízes (11/11/2026)

Os leitores da decisão (b) relataram em `contradicoes.txt` 90 lugares onde o cabeçalho de uma rotina e o corpo Rust
discordam: o cabeçalho chama um deslocamento de uma coisa e o código de outra (campo do `fields.rs`, método do
`records.rs`), ou o corpo lê o registro pelo tipo errado. O código de máquina está certo (cada rotina é comparada com a
original); só as palavras podem estar erradas. O juiz decide qual lado está errado, com evidência (a desmontagem pelo
`desmonta.py`, o FORMATOS, o CAMPOS, os MAPA_*, o mapa da comunidade e os outros usos do campo).

Vereditos: `header` (o cabeçalho novo inteiro, em TSV), `field_name` (o nome novo do campo, com a confiança),
`wrong_type` (arquivo, função, tipo certo e a troca mínima no código), `stale` (texto desatualizado) e `unclear`. Saída
em JSON e TSV; nenhum arquivo do projeto é editado. O pedido completo está em
[BRIEF_CONTRADICOES.en.md](BRIEF_CONTRADICOES.en.md).
