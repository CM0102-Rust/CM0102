# Os textos de suspensao das telas da pessoa (leitor sx09b, rodada 5, 23/11/2026). As tres so rodam nas telas:
# - Discipline::get_ban_history (0x55cbc0, 80 bytes ate o ret 4 de 0x55cc0f): thiscall(Discipline, id) ret 4; chamada so
#   pela aba de historico da pessoa (staff_screens_draw_history_tab 0x885820, ja Rust, 0x887718). 0x58f120
#   FILE_LLIST::get_nodes thiscall([this+0x18], inicio +0x14, fim +0x18 do registro de 29 bytes) ret 8 (devolve uma
#   lista nova a cada chamada);
# - Discipline::describe_ban (0x55cc20, 534 bytes ate o ret 0xc de 0x55ce35): thiscall(Discipline, suspensao, saida,
#   tamanho) ret 0xc; chamada so pela aba de historico (0x887827). Chama a get_scope_text (0x55ce40, o item sx09b_3)
#   pelo endereco, com o ecx = this, &regras, um buffer de 0x64, 0x64, bit 1 das marcas e 4; 0x456740 Date::operator>
#   thiscall(inicio, dword [0xae2c90], dword [0xae2c94]) ret 8; 0x54a650 Date::get_short_date_string thiscall(inicio,
#   buf, 0x32, 0xff) ret 0xc; 0x7e5780 format_duration_text cdecl(dias com sinal, buf, 0x1e, 0); os textos por
#   0x66f4e0 langlib_sprintf cdecl, com os argumentos na ordem dos push (inclusive o 0x64 que a rotina empurra entre
#   eles); tamanho < 0x32: caixa "Error" linha 0x1403;
# - Discipline::get_reason_text (0x55d0a0, 457 bytes ate o ret 0xc de 0x55d268): thiscall(Discipline, suspensao,
#   saida, tamanho) ret 0xc (o ecx nao e lido); chamada pela aba de historico (0x887894) e pelo texto da suspensao de
#   uma regra (RulingBody::get_ban_text 0x7e88d0, original), que a tela da pessoa pede pela get_slot_text (0x55d3c0,
#   ja Rust, de 0x87618a); tamanho < 0xf: caixa "Error" linha 0x150e.
# Prologos sem salto: 0x55cbc0 mov eax,[esp+4]; push esi (5); 0x55cc20 e 0x55d0a0 mov eax,[esp+0xc]; sub esp,N (10).
# Comparadores: describe_ban e get_reason_text lado a lado sobre a saida; get_ban_history so a original (cria uma lista
# nova a cada chamada), com o plano conferido (0 para pessoa fora da tabela).
# Como chegar no jogo (para o Lucas provar): o perfil de um jogador que ja foi suspenso (a suspensao cumprida aparece na
# aba Historico: get_ban_history, describe_ban e get_reason_text) e o de um jogador suspenso agora (a linha da suspensao
# no perfil: get_reason_text pela get_ban_text).
TELAS = True
SAVE = False
ROTINAS = [('get_ban_history', 0x55cbc0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('describe_ban', 0x55cc20, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0xb8, 0x02, 0x00, 0x00]),
           ('get_reason_text', 0x55d0a0, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os textos de suspensao das telas da pessoa',
        "the ban texts of the person's screens",
        [('get_ban_history', 'Discipline::get_ban_history', '0x55cbc0', 'discipline.cpp', 'thiscall, ret 4',
          'devolve as suspensoes cumpridas de uma pessoa, lidas do arquivo do historico entre o primeiro e o ultimo no do registro dela; pessoa fora da tabela: 0',
          "returns a person's served bans, read from the history file between the first and last node of her record; a person outside the table: 0"),
         ('describe_ban', 'Discipline::describe_ban', '0x55cc20', 'discipline.cpp', 'thiscall, ret 0xc',
          'descreve uma suspensao numa linha: "1 match <ambito> ban", "<n> match <ambito> ban" ou "<duracao> <ambito> ban" (suspensao por dias), com "( starts <data> )" quando o inicio ainda vai chegar; o ambito e a forma pela get_scope_text; saida com menos de 50 bytes: a caixa de erro',
          'describes a ban in one line: "1 match <scope> ban", "<n> match <scope> ban" or "<length> <scope> ban" (a ban by days), with "( starts <date> )" when the start is still to come; the scope and the form by get_scope_text; an output under 50 bytes: the error box'),
         ('get_reason_text', 'Discipline::get_reason_text', '0x55d0a0', 'discipline.cpp', 'thiscall, ret 0xc',
          'escreve o motivo da suspensao: uso de substancia ilegal (5), passaporte invalido (6), "<n> points" (0), cartao amarelo ou "<n> yellow cards" (1), "<n> successive yellow cards" (4), cartao vermelho ou "<n> red cards" (2), "Misconduct" (3), comportamento antiprofissional (outro); saida com menos de 15 bytes: a caixa de erro',
          'writes the reason of the ban: illegal substance (5), invalid passport (6), "<n> points" (0), yellow card or "<n> yellow cards" (1), "<n> successive yellow cards" (4), red card or "<n> red cards" (2), "Misconduct" (3), unprofessional behaviour (other); an output under 15 bytes: the error box')])
