# A frase do status no elenco, o batedor de penalti da disputa, as listas de historico de pessoa e de clube e a linha do
# historico do clube (shortlist_manager.cpp, tactics.cpp, staff_records.cpp; leitor sx20, rodada 5, 24/11/2026):
# shortlist_manager_squad_status_text (0x84b330), Tactics::set_shootout_taker_slot (0x898150),
# Staff_Records::get_staff_history (0x86de70), get_club_history (0x86df90) e format_club_history_entry (0x86f300).
# Rotinas inteiras (0x84b330..0x84b4a4 ret 0xc mais a tabela de 0x84b4a8; 0x898150..0x8982a6 ret 0x10;
# 0x86de70..0x86df8a ret 8; 0x86df90..0x86e0ab ret 8; 0x86f300..0x86f5a3 ret 8).
# DEPENDE DO sx20_11 (integrar junto ou depois): usa langlib_text e contract_compare.
# TELAS: a prova e o Lucas jogando comparando: o status no elenco (a frase no perfil de um jogador, na tela de
# contrato e na de transferencias); o batedor de penalti (Taticas, os batedores da disputa por penaltis: escolher o
# batedor de cada cobranca); o historico de uma pessoa (perfil, historico) e o de um clube (clube, historico: as
# trocas de tecnico, as tacas, os titulos, os acessos e as quedas).
# Convencoes conferidas na desmontagem:
# - 0x84b330 thiscall(?, status, saida, tamanho) ret 0xc (nao le ecx; tamanho < 30 a caixa shortlist_manager.cpp
#   0x2dae; o eax de sobra);
# - 0x898150 thiscall(taticas, clube, cobranca, vaga, humano) ret 0x10 -> 1/0: 0x8982b0 get_club_tactics_index
#   thiscall(taticas, clube, humano) ret 8 (a copia Rust dela e stdcall: nao le ecx); a caixa tactics.cpp 0x6d4,
#   0x6dc, 0x6e4;
# - 0x86de70 / 0x86df90 thiscall(registros, pessoa ou clube, mascara) ret 8 -> LLIST: 0x58f120 FILE_LLIST::get_nodes
#   thiscall(arquivo, inicio, fim) ret 8, o cursor volta a cabeca, 0x68d1b0 get_next, 0x68d1f0 LLIST::remove
#   thiscall(lista, dado) ret 4; o tipo em +0xc (pessoa) e +8 (clube); a caixa staff_records.cpp 0xa74 / 0xad7;
# - 0x86f300 thiscall(registros, no, texto) ret 8: 0x53e140 database_format_name_from_ids cdecl(nome1, nome2, comum,
#   buf, 0xc8, 1, id), 0x66f4e0 langlib no texto comum 0xde1f64, 0x4ae660 Text::set_string thiscall(texto, 0xde1f64)
#   ret 4, 0x53e990 database_staff_matches_names cdecl(pessoa, nome1, nome2, comum, 0), 0x4ae8a0 Text::add_link
#   thiscall(texto, nome, 0x7d7 / 0x7d0, alvo) ret 0xc; a competicao em [0xae23d0] + id x 0x6b.
# Defeitos do original: a linha de acesso ou de queda sem competicao (id -1) le o byte em 0x37 e derruba o jogo.
# Comparadores: o texto e a resposta lado a lado (a frase); a linha da tatica lado a lado; as listas pelo conteudo dos
# nos (club_history_compare_list, a lista da copia destruida); a linha do historico pelo compare_text_writer; so a
# original quando o argumento abre a caixa.
TELAS = True
ROTINAS = [('shortlist_manager_squad_status_text', 0x84b330, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tactics_set_shootout_taker_slot', 0x898150, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('staff_records_get_staff_history', 0x86de70, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('staff_records_get_club_history', 0x86df90, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('staff_records_format_club_history_entry', 0x86f300, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0xcc, 0x00, 0x00, 0x00])]
DOCS = ('a frase do status no elenco, o batedor da disputa por penaltis e o historico de pessoa e de clube',
        'the squad status sentence, the shoot-out taker and the person and club history',
        [('shortlist_manager_squad_status_text', 'shortlist_manager_squad_status_text', '0x84b330', 'shortlist_manager.cpp', 'thiscall, ret 0xc',
          'escreve a frase do status no elenco (1 indispensavel, 2 titular importante, 3 rodizio, 4 reserva, 5 promessa, 6 jovem razoavel, 7 nao necessario, 8 em teste; outro "Uncertain"); buffer menor que 30, a caixa e nada',
          'writes the squad status sentence (1 indispensable, 2 important first team player, 3 rotation, 4 backup, 5 hot prospect, 6 decent young player, 7 not needed, 8 on trial; any other "Uncertain"); a buffer under 30, the box and nothing'),
         ('tactics_set_shootout_taker_slot', 'Tactics::set_shootout_taker_slot', '0x898150', 'tactics.cpp', 'thiscall, ret 0x10',
          'guarda a vaga (-1 a 19) do jogador que bate a cobranca (0 a 10) da disputa por penaltis na linha de taticas do clube e devolve 1; sem clube, cobranca ou vaga fora da faixa, a caixa e 0; clube sem linha, 0',
          "stores the slot (-1 to 19) of the player who takes the kick (0 to 10) of the shoot-out in the club's tactics row and returns 1; no club, a kick or slot out of range, the box and 0; a club without a row, 0"),
         ('staff_records_get_staff_history', 'Staff_Records::get_staff_history', '0x86de70', 'staff_records.cpp', 'thiscall, ret 8',
          'devolve a lista do historico de uma pessoa, sem as entradas cujo tipo cai fora da mascara (1 ate 0x31, 2 de 0x32 a 0x63, 4 de 0x64 a 0x95, 8 de 0x96 a 0xc7; mascara 0 nao tira nada); sem pessoa, a caixa e nenhuma lista',
          "returns a person's history list, without the entries whose kind falls outside the mask (1 up to 0x31, 2 from 0x32 to 0x63, 4 from 0x64 to 0x95, 8 from 0x96 to 0xc7; mask 0 removes nothing); without a person, the box and no list"),
         ('staff_records_get_club_history', 'Staff_Records::get_club_history', '0x86df90', 'staff_records.cpp', 'thiscall, ret 8',
          'devolve a lista do historico de um clube, com o mesmo filtro pela mascara; sem clube, a caixa e nenhuma lista',
          "returns a club's history list, with the same mask filter; without a club, the box and no list"),
         ('staff_records_format_club_history_entry', 'Staff_Records::format_club_history_entry', '0x86f300', 'staff_records.cpp', 'thiscall, ret 8',
          'monta o texto e o link de uma linha do historico do clube pelo tipo: 0x96 troca de tecnico ("Hired", "resigned" ou "Sacked" com o nome das chaves guardadas, link para a pessoa enquanto ela ainda tem esses nomes); 0 campeao de taca, 1 vice, 2 terceiro, 5 campeao da liga (link para a competicao); 8 acesso e 9 queda de uma liga (link quando ha); outro tipo nao escreve nada',
          'builds the text and link of one club history line by kind: 0x96 a manager change ("Hired", "resigned" or "Sacked" with the name from the stored keys, linked to the person while he still has those names); 0 cup winners, 1 runners up, 2 third placed, 5 league champions (linked to the competition); 8 promotion and 9 relegation from a league (linked when there is one); any other kind writes nothing')])
