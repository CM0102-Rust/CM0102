# 269ª leva — a liga: os jogos de maior nota levados ao dia e à fatia da TV, a notícia do sorteio dos grupos e a notícia dos primeiros jogos

**Rotinas:**
- `liga_jogos_para_a_tv()` = 0x685640 (league.cpp, thiscall, ret 4): nas datas da tabela da TV, cada vaga leva o jogo de maior nota (sorteio pelas reputações, zerado com jogo de um dos clubes 2..5 dias antes ou depois; 100/50 nas vagas de seleção) para o dia da semana e a fatia da vaga, com a notícia, o estádio e o calendário;
- `liga_noticia_dos_grupos()` = 0x684230 (league.cpp, thiscall, ret 0): a notícia 0x15 do grupo (a competição, a primeira data, os adversários) a cada clube de técnico humano;
- `liga_noticia_da_tabela()` = 0x683ee0 (league.cpp, thiscall, ret 4): a notícia 0xd com os três primeiros jogos (data, adversário, mando) a cada clube de técnico humano ainda não avisado, e o mesmo nas fases seguintes;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 3 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (8845ebc9).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 151bdf85, original 385e2e8b (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 268ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 269`

**Campos tocados:**
- liga +0xba (tabela das datas da TV, 0x41 bytes por entrada; conta +0xa9) +0 dia, +2 ano − o da liga, +4 fatia, +5.. vagas de 7 bytes (+0 estádio, +4 dia da semana, +5 fatia da TV, +6 tipo: 0xff = vazia); valor; lê e grava (liga_jogos_para_a_tv: a vaga usada fica 0xff, a ordem pela 0x945333), lê (liga_noticia_dos_grupos, liga_noticia_da_tabela)
- liga +0x40 ano, +0x44 tipo/fase (0xff primeira fase), +0x3e times, +0xb1 times de 0x3b bytes (+0 clube), +0x24, +0x3a, +0xc/+0x30 as fases (+0x43); ponteiro e valor; lê (liga_jogos_para_a_tv, liga_noticia_dos_grupos, liga_noticia_da_tabela)
- partida (0x4f bytes no calendário) +0xc/+0x10 ids, +0x18 estádio, +0x1c/+0x20 clubes, +0x28 ano, +0x2a dia, +0x3b tipo, +0x3f fatia, +0x42 tipo da liga; valor; lê e grava (liga_jogos_para_a_tv), lê (liga_noticia_da_tabela)
- clube +0x80 reputação, +0x53 nação; valor; lê (liga_jogos_para_a_tv)

**Interessa a:** CMRTE
