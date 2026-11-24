# sx09c_8 (Fine::is_fine_justified 0x5a69b0, Fine.cpp, 2544 bytes): nao escrito nesta rodada

Parado pelo tamanho (rodada interrompida pelo limite de uso). Notas para quem pegar:

- thiscall(fine, jogador, periodo byte, motivo byte) ret 0xc -> al; so chamada pela fine_player (0x5a5ab9).
- Quadro SEH de 0xe10 bytes: um vetor local de 2 registros de time de 0x625 bytes (eh vector constructor 0x9463af com
  0x453c30 e 0x453c10, destruido por 0x9462bb em toda saida) e uma LLIST local (0x68d020 / 0x68d040).
- Primeiro passeio na lista [fine]: multa anterior do mesmo jogador ainda valendo (data +6 depois de hoje): sem
  aceitacao (+5 = 0) devolve 0; com aceitacao, random(profissionalismo) = 0 devolve 0.
- Por motivo (tabela 0x5a7388, motivo - 1 em 0..4): 1 e 5 olham a midia (0x7357c0), o AWOL (0x418f60, 0x418ff0,
  0x419070) e as partidas das ultimas 8 semanas (0x54cab0 em hoje; a busca de partidas 0x5acbe0 em [0xae2a58] numa lista
  local; passeio para tras 0x68d1d0 ate o jogo com o byte +0x43 >= 0; copia de 0x4f bytes; 0x729600 em [0xb6b7c0] para
  o registro da partida; as 20 linhas de 0x49 bytes do time do jogador: cartoes +0x2e/+0x2f 11 e 12 e os bytes -2/-1);
  2 e 3 a mesma busca com regras proprias; 4 a forma em [0xb74350] + 0x30 + player*0x27, byte +0x26 & 0xf; depois
  random(0x32) contra temperamento + pressao e random(100).
- Devolve 1 (aceita) ou 0 (contesta).

**24/11/2026:** escrito pelo leitor sxA (codigo.rs, rotinas.py, mutacoes.py, PRONTO.txt nesta pasta); estas notas ficam como histórico.
