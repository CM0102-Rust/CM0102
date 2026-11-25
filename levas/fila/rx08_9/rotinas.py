# Os textos da tela dos recordes do clube: os nomes do periodo e do tipo de recorde, um recorde como texto, e os dois
# ajudantes de Cash (club_records.cpp; leitor rx08, rodada 6, 25/11/2026). TELAS: rodam so na tela dos recordes do clube
# e nas que mostram dinheiro por eles (a cobertura do mundo viu 0 chamadas de cada).
# Caminho no jogo: Clube -> Recordes (Club -> Records): a lista dos 32 tipos, o seletor "Modern Day"/"This Season" e o
# texto de cada recorde; as taxas de transferencia aparecem pelo Cash.
# - 0x448fa0 club_records_get_period_label thiscall(periodo, buf, tamanho) ret 0xc (this nao lido): buf nulo ou tamanho
#   < 0x32: caixa 0x766 e 0; buf[0] = 0; 1 "Modern Day" (0x98b728), 2 "This Season" (0x98b734) por langlib_sprintf
#   0x66f4e0; outro: caixa 0x780 e 0; 1.
# - 0x4490b0 club_records_get_record_type_label thiscall(tipo, buf, tamanho) ret 0xc: o mesmo com a tabela de saltos
#   0x44950c (tipos 1..32, os textos 0x98b9f4..0x98b740 em ordem), caixas 0x793 e 0x87f. A funcao acaba na tabela
#   (0x44958c); 0x449590 e outra rotina.
# - 0x4496d0 club_records_get_record_text thiscall(clube, periodo, tipo, texto) ret 0x10: texto nulo: caixa 0x95f e 0;
#   Text::clear 0x4afa40 e Text::set_string 0x4ae660 ("-"); clube nulo: caixa 0x96b e 0; periodo 1:
#   get_all_time_record 0x452710 (clube, 0); senao get_season_record 0x4528d0 (clube, 0) e, no tipo 0x14 com registro
#   cuja nota da temporada (+0x1f1, f32) e -1.0 ([0x968948]), Club_Records::update_season_rating_record 0x452a90 (ja
#   Rust, chamada direta); depois club_records_format_record_text 0x449810 (clube, registros, tipo, texto) (original).
# - 0x450a90 Cash::Cash thiscall(valor) ret 4: [this] = valor; devolve this.
# - 0x450aa0 Cash::format_display_amount thiscall(buf, flags, passo) ret 0xc: Cash::get_display_amount 0x43f460
#   thiscall(this, passo) ret 4 (edx:eax) e cash_format_money 0x441c10 cdecl(buf, flags, valor de 64 bits, passo) (o
#   push do passo antes da primeira chamada fica para a segunda).
# Prologos (fronteira de instrucao, sem relativo): mov ecx/eax, [esp+8]; sub esp, 0x200 (10); sub esp, 0x200 (6); mov
# eax, ecx; mov ecx, [esp+4] (6); mov eax, [esp+0xc]; push eax (5). Iguais no cm0102-1080.exe (inteiras).
# Comparadores: os rotulos e o dinheiro lado a lado nos 50 primeiros bytes do buffer e no retorno; o recorde em texto
# lado a lado em dois Text (compare_text_writer); o Cash lado a lado no dword.
# Depende do rx08_1 (club_records_consts): integrar depois dele.
TELAS = True
SAVE = False
ROTINAS = [('club_records_get_period_label', 0x448fa0, [0x8b, 0x4c, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('club_records_get_record_type_label', 0x4490b0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('club_records_get_record_text', 0x4496d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('construct_cash', 0x450a90, [0x8b, 0xc1, 0x8b, 0x4c, 0x24, 0x04]),
           ('cash_format_display_amount', 0x450aa0, [0x8b, 0x44, 0x24, 0x0c, 0x50])]
DOCS = ('os textos da tela dos recordes do clube e os ajudantes de dinheiro',
        'the club records screen texts and the money helpers',
        [('club_records_get_period_label', 'club_records_get_period_label', '0x448fa0', 'club_records.cpp', 'thiscall, ret 0xc',
          'o nome do período dos recordes: 1 "Modern Day" (os de sempre), 2 "This Season"',
          'the records period label: 1 "Modern Day" (all time), 2 "This Season"'),
         ('club_records_get_record_type_label', 'club_records_get_record_type_label', '0x4490b0', 'club_records.cpp', 'thiscall, ret 0xc',
          'o nome de cada um dos 32 tipos de recorde do clube: posições na liga, públicos e renda, vitórias, derrotas e jogos de muitos gols, as sequências, artilheiros, assistências, notas, melhor em campo e disciplina, jogos e gols pelo clube, o mais novo, o mais velho e o mais convocado, as transferências e o jogador do ano da torcida',
          "the label of each of the 32 club record types: league positions, attendances and gate, wins, defeats and high scoring games, the runs, scorers, assists, ratings, man of the match and discipline, club appearances and goals, the youngest, oldest and most capped, the transfers and the fans' player of the year"),
         ('club_records_get_record_text', 'club_records_get_record_text', '0x4496d0', 'club_records.cpp', 'thiscall, ret 0x10',
          'um recorde do clube como texto, dos de sempre ou dos da temporada; a melhor nota média da temporada é calculada antes quando ainda não existe; "-" enquanto não há',
          "one club record as text, all time or this season; the season's best average rating is worked out first when still unset; \"-\" until there is one"),
         ('construct_cash', 'Cash::Cash', '0x450a90', 'club_records.cpp', 'thiscall, ret 4',
          'um valor de dinheiro para exibir guarda o próprio valor',
          'a cash amount for display stores the value itself'),
         ('cash_format_display_amount', 'Cash::format_display_amount', '0x450aa0', 'club_records.cpp', 'thiscall, ret 0xc',
          'escreve o valor como dinheiro na moeda de exibição, arredondado ao passo',
          'writes the amount as money in the display currency, rounded to the step')])
CAMPOS = []
INTERESSA = []
