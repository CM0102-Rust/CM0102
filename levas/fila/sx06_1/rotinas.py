# Os registros dos clubes: a que tipos de recorde um clube concorre e o fim da temporada de uma nacao (club_records.cpp;
# leitor sx06, rodada 5, 23/11/2026). Convencoes conferidas na desmontagem:
# - club_records_record_type_applies (0x449590): ret 0xc, os 3 argumentos (tipo em byte, clube, periodo em byte); os
#   chamadores do exe poem ecx (mov ecx, ebx) mas a rotina nao o le, e o Rust a chama como stdcall
#   (exe::club_records_record_type_applies): o desvio e thiscall com o this ignorado. Tabela de saltos em 0x449694
#   (4 casos) e de bytes em 0x4496a8 (32 tipos). Clube nulo: a caixa (linha 0x892) e 0, so a original no comparador.
# - Club_Records::end_season_for_nation (0x447880): thiscall(this = [0xad9dac], linha da nacao-chave em 0xb63d60),
#   ret 4, chamada so pelo laco da temporada (0x5d5e46, game.cpp). Chamadas: 0x4528d0/0x452710 thiscall(clube, criar)
#   ret 8; 0x669950 e 0x669360 cdecl(clube); 0x449590 acima; 0x600a50 cdecl(clube, 0, 1, 0, 0); 0x53b900 cdecl;
#   0x452cb0 thiscall(id, tipo) ret 8; 0x452a90 thiscall(clube) ret 4; 0x417c10 Date::operator= thiscall(&local, origem);
#   0x452da0 thiscall(clube, registro) ret 8; 0x7875e0 thiscall([0xb7431c], clube); 0x77bbe0 thiscall(noticia, 0x3e8,
#   0); 0x453120 cdecl(id, registro, 0x20, noticia); 0x7861e0 thiscall([0xb7431c], noticia, clube); 0x86dc30
#   thiscall([0xdd7f00], pessoa, clube, ano (o dword da volta de 0x669950), linha +0x17); 0x58e340
#   thiscall([this], entrada+8, entrada+0xc, &registro de 0x38d); 0x452fe0 cdecl(registro). A foto local (0x448260) e
#   construida uma vez e sobrescrita inteira pelas copias campo a campo (0x448110, 0x448170, 0x448490, 0x448230,
#   0x4484d0, 0x448500, 0x448530, 0x4a1fb0, 0x448550, 0x4485b0 e os dwords/words em linha), que cobrem os 0x38b bytes.
# Comparadores: 0x449590 lado a lado pelo retorno; 0x447880 lado a lado pela memoria (entradas, registros, gerador e
# preferencias) com plano para o que nao volta (fila de noticias, arquivo do historico).
SAVE = False
ATE = '2002-06-01'  # a comparacao precisa chegar ao fim de temporada (a 0x447880 roda 40 vezes ate 2002-06-01)
ROTINAS = [('club_records_record_type_applies', 0x449590, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('club_records_end_season_for_nation', 0x447880, [0x6a, 0xff, 0x68, 0x03, 0x52, 0x95, 0x00])]
DOCS = ('os registros dos clubes: os tipos de recorde de cada clube e o fim da temporada de uma nacao',
        'club records: the record types of each club and the end of a nation\'s season',
        [('club_records_record_type_applies', 'club_records_record_type_applies', '0x449590', 'club_records.cpp', 'thiscall (ecx nao lido), ret 0xc',
          'diz se o tipo de recorde (1 a 32) vale para o clube no periodo (1 de todos os tempos, 2 a temporada): os tipos so de clube (posicoes na liga, publico, resultados, jogos e gols na liga, transferencias...) valem para quem nao e selecao; o 27 (convocacoes) so para selecao; o 32 (o jogador da torcida no ano) so para clube e nunca no periodo de todos os tempos; os demais, e um tipo fora de 1..32, valem sempre. Clube nulo: a caixa de erro e 0',
          'tells whether record type (1 to 32) applies to the club in the period (1 all-time, 2 the season): the club-only types (league positions, attendance, results, league appearances and goals, transfers...) apply to anyone who is not a national team; 27 (caps) only to national teams; 32 (the fans\' player of the year) only to clubs and never in the all-time period; the others, and a type outside 1..32, always. A null club: the error box and 0'),
         ('club_records_end_season_for_nation', 'Club_Records::end_season_for_nation', '0x447880', 'club_records.cpp', 'thiscall, ret 4',
          'fecha a temporada de cada clube da nacao-chave: o publico medio da temporada acima de -1 que supera o de todos os tempos (-1 = nenhum) vai para o de todos os tempos com o ano; a nota da temporada ainda em -1,0 e calculada; a nota media da temporada acima de -1,0 que supera a de todos os tempos e copiada com o ano; em cada recorde batido, a noticia entra na fila (clube humano ou selecao com 5 temporadas ou mais, sem repetir); com registro da temporada, o jogador da torcida no ano (tipo 32): quando ha um, a noticia 0x3e8 para o clube de tecnico humano e o registro dos torcedores da pessoa (com o ano e a marca de temporada dividida da nacao); a temporada vai para o arquivo do historico (o ano e o conjunto de registros de 0x38b bytes) e o registro da temporada e zerado (-1); sempre, uma temporada a mais na entrada do clube e os contadores da temporada zerados. Devolve 1; nacao nula: a caixa de erro e 0. Defeito do original: o publico e a nota leem o registro de todos os tempos antes de testar se e nulo (so depois o criam); como a partida cria os dois juntos, a criacao nunca roda',
          'closes the season of every club of the key nation: a season average attendance above -1 that beats the all-time one (-1 = none) goes to the all-time record with the year; a season rating still at -1.0 is worked out; a season average rating above -1.0 that beats the all-time one is copied with the year; for each broken record the news is queued (a human club or national team with 5 seasons or more, without repeats); with a season record, the fans\' player of the year (type 32): when there is one, news item 0x3e8 for a club with a human manager and the person\'s supporters\' record (with the year and the nation\'s split-season flag); the season goes to the history file (the year and the 0x38b-byte record set) and the season record is reset (-1); always, one more season in the club\'s entry and the season counters cleared. Returns 1; a null nation: the error box and 0. Defect of the original: the attendance and rating checks read the all-time record before testing it for null (and only then create it); since the match creates both together, the creation never runs')])
