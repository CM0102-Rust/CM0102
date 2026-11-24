# sx09c_7 (Fine::fine_player 0x5a5980, Fine.cpp, 4144 bytes): nao escrito nesta rodada

O leitor sx09c fechou os outros itens do grupo e parou aqui pelo tamanho (a rodada foi interrompida pelo limite de uso).
Notas para quem pegar:

- thiscall(fine [0xae2a3c], jogador, periodo, motivo) ret 0xc; chamado pelo AWOL (0x41a60a, 0x41b003: o tecnico da IA
  multa quem falta ao treino, entao RODA na carreira de ferias) e pela tela da Disciplina (0x5a8ecf).
- Quadro SEH com uma NewsItem local de 0xe0 bytes (estados 3, 5, 6) e duas datas; motivo 0 ou 1 com o jogador ausente
  sem licenca (0x418f60) vira 5.
- Ramo "nao justa" (is_fine_justified 0x5a69b0 devolve 0): contagem por clube em [fine+8] (teto 100), moral do tecnico
  (staff +0xcf, +0x59), confianca da diretoria em [0xb67a44] + clube*0x49 (+6, +0xa, +0xc; limites 1..10000; -500,
  -100 x byte +0x14 do presidente, -250, -100), noticia 0x1784 ao clube (0x7861e0), reacoes do elenco (50 vagas,
  staff_add_disliked_staff), post_fine_reaction_news (0x5a74b0, ja Rust), AWOL 0x419840 e, no fim,
  DISPUTE::file_complaint (0x56a100, sx09c_6) com (4, 0) ou (3, 1).
- Ramo "justa": efeitos por motivo (bits 1/2/4/8/0x10, profissionalismo, temperamento, player +0x44, +0x1c, o
  contrato de 79 bytes em [0xae2d0c] + id*0x4f + 10 com _ftol) e, com efeito, o retorno do auxiliar (registro de 17
  bytes na lista [fine+4]: +0 auxiliar club+0xd3, +4 jogador, +8 resultado, +9 hoje + random(5) + 2 dias).
- Sempre no fim: o registro de 14 bytes na lista [fine]: +0 jogador, +4 motivo, +5 resposta da is_fine_justified, +6 a
  data de 0x54c770(hoje, periodo).
- Comparador: so a original, com o plano (contagem, listas) conferido depois; mutacoes pela carreira ENG.

**24/11/2026:** escrito pelo leitor sxA (codigo.rs, rotinas.py, mutacoes.py, PRONTO.txt nesta pasta); estas notas ficam como histórico.
