# A linha da suspensao de uma pessoa num orgao disciplinar (ruling_body.cpp; leitor sx19b, rodada 5, 24/11/2026):
# RulingBody::get_ban_text (0x7e88d0..0x7e8d52, ret 0xc; o resto ate 0x7e8d60 e enchimento).
# Quem chama: Discipline::get_slot_text (0x55d40c), a tela da pessoa (a disciplina: suspensoes e "a um amarelo da
# suspensao"). A carreira automatica nao passa (cobertura 0).
# TELAS: o caminho no jogo e a ficha de um jogador suspenso ou pendurado (Pessoa > Disciplina / o resumo da ficha),
# por exemplo um jogador com vermelho na rodada anterior, ou um com amarelos perto do limite.
# Convencoes conferidas na desmontagem:
# - 0x7e88d0 thiscall(orgao, pessoa, texto, tamanho) ret 0xc -> 1/0; quadro SEH (as duas datas locais nao tem
#   destrutor que faca algo);
# - 0x55cae0 table_penalty thiscall([0xae28f0], id, vaga +4, regra +5) ret 0xc (ja Rust); 0x556e90
#   update_ban_progress thiscall([0xae28f0], id, regra, 0) ret 0xc; vtable +0x14 (um amarelo da suspensao)
#   thiscall(orgao, entrada, &jogos, &dias, [clube da pessoa +0x57]) ret 0x10; vtable +0x10 get_name thiscall(orgao,
#   texto, 100, 4) ret 0xc; 0x55d0a0 Discipline::get_reason_text thiscall([0xae28f0], suspensao, texto, 0x1e) ret 0xc;
#   0x54a650 Date::get_short_date_string thiscall(inicio, texto, 0x32, 0xff) ret 0xc; 0x54cc30 Date::get_difference
#   thiscall(0xae2c90, &copia de hoje, inicio d0, d1) ret 0xc (grava a diferenca por cima da copia de hoje; os dias que
#   faltam = dias - 365 x anos - dias da diferenca); 0x7e5780 format_duration_text cdecl(dias, texto, 0x1e, 0);
#   0x66f4e0 langlib_sprintf cdecl.
# Comparador: lado a lado, a original no texto da tela e a copia num local (o update_ban_progress com 0 jogos ja rodou
# na original e nao muda nada na copia); o texto e a resposta. Tamanho abaixo de 100: so a original (a caixa).
TELAS = True
ROTINAS = [('ruling_body_get_ban_text', 0x7e88d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a linha da suspensão de uma pessoa num órgão disciplinar',
        'the ban line of a person under a ruling body',
        [('ruling_body_get_ban_text', 'RulingBody::get_ban_text', '0x7e88d0', 'ruling_body/ruling_body.cpp', 'thiscall, ret 0xc',
          'escreve a linha da suspensão da pessoa neste órgão: tamanho abaixo de 100, a caixa e 0; sem entrada no órgão, 0; atualiza as suspensões dela (0 jogos); sem suspensão, pergunta ao órgão se está a um amarelo da suspensão e escreve "a um amarelo de uma suspensão <âmbito> de 1 jogo / n jogos / n dias"; com suspensão (que precisa ser deste órgão nas quatro regras dela), o âmbito ("global" para a suspensão global da regra 0, senão o nome do órgão) e o motivo: se começa depois de hoje, "1 jogo / n jogos / o tempo que falta ... a partir de <data> (<motivo>)", senão o mesmo sem a data; devolve 1 quando escreveu',
          'writes the person\'s ban line under this body: a size below 100, the box and 0; no entry in the body, 0; moves the person\'s bans on (0 matches); without a ban, asks the body whether the person is one booking away and writes "one yellow card away from a <scope> 1 match / n match / n day ban"; with a ban (which must be this body\'s in its four rule bytes), the scope ("global" for the rule set 0 global ban, otherwise the body\'s name) and the reason: when it starts after today, "1 match / n matches / the time left ... from <date> (<reason>)", otherwise the same without the date; returns 1 when it wrote')])
CAMPOS = []
INTERESSA = []
