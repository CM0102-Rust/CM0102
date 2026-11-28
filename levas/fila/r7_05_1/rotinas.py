# Um recorde do clube como texto da tela de recordes, do club_records.cpp (leitor r7_05, rodada 7, 28/11/2026): 0x449810,
# 11,8 KB, inteira num desvio (estava adiada no levas/fila/rx08_11/FORA.md e no rx08b_5/FORA.md so por tempo). Chamada
# pelo club_records_get_record_text 0x4496d0 (rx08_9, pelo exe) e pelas telas do clube (0x46c3b8, 0x46c89b, 0x46cd6b,
# 0x46d225, 0x46d717, 0x46dbd5; o historico do tel03_3). A carreira automatica nao passa (a cobertura do mundo viu 0
# chamadas): TELAS.
# Caminho no jogo: Clube -> Recordes (Club -> Records): a lista dos 32 tipos, "Modern Day" e "This Season", e o
# historico por temporada.
# Fronteiras: 0x449810..0x44c5cb (ret 0x10 em 0x44c5c8) e a tabela de saltos 0x44c5cc (32 tipos) ate 0x44c64c.
# Prologo mov eax, fs:[0] (6 bytes, sem relativas). Igual no cm0102-1080.exe (a rotina inteira e a tabela).
# Convencao: thiscall(Club_Records, clube, registros, tipo, Text) ret 0x10 -> 1 (0 nas caixas 0x99a, 0x9a6 e 0xecf e
# sem registros). Chamadas conferidas: 0x4afa40 thiscall(Text); 0x4ae660 thiscall(Text, s) ret 4; 0x4ae8a0
# thiscall(Text, s, codigo, alvo) ret 0xc; 0x66f4e0 e 0x944c9f cdecl variadicas; 0x90c310 cdecl(posicao + 1, -1) ->
# texto; 0x669630 cdecl(ano, clube, buf, 0x14); 0x4bfa30 cdecl(comp, rodada, fase, 3, buf, 0xc8); 0x4bff70 cdecl(comp,
# rodada, fase, 3) -> al; 0x54a650 thiscall(data, buf, 0x14 (0x64 nas transferencias), 0xff) ret 0xc; 0x7e4ca0
# cdecl(lado, buf, 5); 0x53e140 cdecl(n1, n2, n3, buf, 0xc8, forma, id); 0x53e990 cdecl(pessoa, n1, n2, n3, data);
# 0x729500 thiscall([0xb6b7c0], jogo, clube, adversario, dia, ano) ret 0x14; 0x540340 cdecl(data do jogo, nascimento,
# buf, 5) (ja Rust, chamada pelo endereco); 0x43f460 thiscall(&valor, 6) ret 4 e 0x441c10 cdecl(buf, 0xc8, valor de
# 64 bits, 6) (o Cash::format, record_money_text).
# O original passa como dword a rodada, a fase, o ano, o lado e o dia e ano do jogo: os dois bytes de cima sao lixo
# de registro e as chamadas so leem a palavra (o byte do lado). A copia passa os valores limpos.
TELAS = True
SAVE = False
ROTINAS = [('club_records_format_record_text', 0x449810, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('um recorde do clube como texto da tela de recordes', 'one club record as the club records screen text',
        [('club_records_format_record_text', 'club_records_format_record_text', '0x449810', 'club_records.cpp', 'thiscall, ret 0x10',
          'escreve um dos 32 recordes do clube (os de sempre ou os da temporada) como texto com vinculos; "-" enquanto nao ha. 1 e 2: a melhor e a pior posicao na liga ("3o em <competicao>", com a temporada quando o ano existe; a competicao vinculada). 3 e 5: o maior e o menor publico, 4: a maior renda: "<valor> v <adversario> <rodada> <data>". 6: o publico medio, com a temporada ou so o numero. 7 a 11: a maior vitoria (e na liga), a maior derrota, o jogo de mais gols (e na liga): "<gols pro>-<gols contra> v <adversario> (<mando>) <rodada> <data>", o placar vinculado ao jogo enquanto ainda confere. 12 a 15: as sequencias de vitorias, derrotas, invencibilidade e sem vencer: "<jogos> - <inicio> a <fim>"; um jogo so (ou nenhum) e so o numero; a invencibilidade e o jejum terminam em "?" enquanto a sequencia atual do clube e a mesma (mesma contagem e mesmo inicio). 16, 17, 19, 21, 22: gols, gols na liga, assistencias, melhor em campo e cartoes (amarelos e vermelhos), com a temporada quando o ano existe; 23, 24, 27: jogos e gols na liga pelo clube e convocacoes; 20: a melhor nota media ("<nota> - <jogador> (<jogos> jogos)"). 18: os gols num jogo, 25 e 26: o jogador mais novo e o mais velho (a idade no dia do jogo), com o jogo como em 7 a 11 e o nome na forma 2. 28 e 29: a maior compra e a maior venda ("<valor> [p/x] - <jogador> de/para <clube> - <data>", "p/x" com a marca da transferencia). 30 e 31: o gasto e a receita de transferencias da temporada, com o ano quando existe. 32: o jogador do ano da torcida, so o nome. O jogador vinculado enquanto os nomes ainda conferem; os clubes e a competicao sempre',
          'writes one of the 32 club records (all time or this season) as text with links; "-" while there is none. 1 and 2: the best and worst league position ("3rd in <competition>", with the season when the year is set; the competition linked). 3 and 5: the highest and lowest attendance, 4: the highest gate receipts: "<value> v <opponent> <round> <date>". 6: the average attendance, with the season or the number alone. 7 to 11: the record win (and league win), defeat, highest scoring game (and league game): "<goals for>-<goals against> v <opponent> (<venue>) <round> <date>", the score linked to the match while it still matches. 12 to 15: the runs won, lost, unbeaten and without winning: "<games> - <start> to <end>"; one game (or none) is the number alone; the unbeaten and winless runs end in "?" while the club\'s current run is the same one (same count and start). 16, 17, 19, 21, 22: goals, league goals, assists, man of the match awards and cards (yellow and red), with the season when the year is set; 23, 24, 27: league appearances and goals for the club and caps; 20: the best average rating ("<rating> - <player> (<apps> apps)"). 18: the goals in one match, 25 and 26: the youngest and oldest player (his age on the day), with the match as in 7 to 11 and the name in form 2. 28 and 29: the record purchase and sale ("<money> [p/x] - <player> from/to <club> - <date>", "p/x" with the transfer\'s flag). 30 and 31: the season\'s transfer spending and income, with the year when set. 32: the fans\' player of the year, the name alone. The player linked while his names still match; the clubs and the competition always')])
CAMPOS = []
INTERESSA = []
