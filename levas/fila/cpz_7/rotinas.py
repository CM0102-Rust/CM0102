# Os registros de partida e de nome (record_utils.cpp; player_stats.cpp, a família pedida, ficou só com rotinas de
# tela, de gravação/carga e do histórico em arquivo temporário, grandes ou sem efeito na carreira, e record_utils.cpp
# não está com nenhum outro agente desta rodada): fill_match_record_from_fixture (0x7e4750: a partida vista por um
# clube, com a data, o lado e o placar pró e contra; da estatística da tática 0x896290, que a escolha da formação do
# computador lê, e dos recordes), fill_staff_name_ref (0x7e4920: os ids dos nomes, o id e a data de nascimento da
# pessoa; dos recordes do clube e da competição, da suspensão, dos prêmios e da busca de jogadores) e
# match_record_result_code (0x7e56d0: 3, 2, 1 ou −3 pelo placar; da estatística da competição 0x49f570, a cada
# partida). Iguais no cm0102-1080.exe; sem x87; a 0x7e4750 tem quadro SEH só pela data temporária (destrutor vazio).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x549ef0 (Date::Date(dia, ano)): push ano (+0x28); push dia (+0x2a); lea ecx, [data]; call; sem add esp ->
#   thiscall(data, dia, ano), ret 8 (só as palavras baixas contam).
# - 0x417c10 (Date::operator=): push origem; push ret; mov ecx, destino; call; sem add esp -> thiscall(destino, ret,
#   origem), ret 8 (nas duas rotinas).
# - 0x4afa80 (o lado do clube no jogo): push jogo; push clube; call; add esp, 8 -> cdecl(clube, jogo).
# - a caixa de erro (aviso_em do motor.rs: 0x944cff cdecl 5, 0x90d130 cdecl 3, 0x944c9f cdecl, 0x5e8290 cdecl 3), com
#   o fonte 0xa7a088 (record_utils.cpp).
# - quem chama: 0x89642e push registro; push clube; push jogo; call 0x7e4750; add esp, 0xc -> cdecl 3; 0x44485a push
#   saida; push pessoa; call 0x7e4920; add esp, 8 -> cdecl 2; 0x49f6bd push registro; call 0x7e56d0; ... add esp, 4 ->
#   cdecl 1 (lê só al).
# Comparadores: a 0x7e4750 e a 0x7e4920 lado a lado (leve) no registro de saída e na devolução (ponteiro nulo: a caixa,
# só a original); a 0x7e56d0 pelo byte devolvido.
SAVE = False
ROTINAS = [('registro_da_partida_pelo_clube', 0x7e4750, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('referencia_ao_nome', 0x7e4920, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('codigo_do_resultado', 0x7e56d0, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os registros de partida e de nome: a partida vista por um clube, a referência ao nome da pessoa e o código do resultado',
        "the match and name records: the match as seen by one club, the person's name reference and the result code",
        [('registro_da_partida_pelo_clube', 'fill_match_record_from_fixture', '0x7e4750', 'record_utils.cpp', 'cdecl',
          'preenche o registro da partida pelo clube: data, id do jogo, competição, lado, clube, adversário e o placar pró e contra (o da prorrogação, se houver)',
          'fills the match record as seen by the club: date, fixture id, competition, venue, club, opponent and goals for and against (the extra-time score, if any)'),
         ('referencia_ao_nome', 'fill_staff_name_ref', '0x7e4920', 'record_utils.cpp', 'cdecl',
          'guarda os ids dos três nomes (−1 sem nome), o id e a data de nascimento da pessoa numa referência compacta',
          "stores the three name ids (-1 when missing), the person's id and birth date in a compact reference"),
         ('codigo_do_resultado', 'match_record_result_code', '0x7e56d0', 'record_utils.cpp', 'cdecl',
          'o código do resultado do registro: 3 vitória, 2 empate com gols, 1 empate sem gols, −3 derrota',
          'the result code of the record: 3 win, 2 score draw, 1 goalless draw, -3 defeat')])
CAMPOS = [('fixture +0 / +8 / +0xc / +0x10 / +0x28 / +0x2a / +0x30 / +0x32: id, competição, os dois adversários, ano, dia e duas palavras; id, ponteiro e valor; lê (registro_da_partida_pelo_clube)',
           'fixture +0 / +8 / +0xc / +0x10 / +0x28 / +0x2a / +0x30 / +0x32: id, competition, the two opponents, year, day and two words; id, pointer and value; reads (registro_da_partida_pelo_clube)'),
          ('fixture +0x43 / +0x44 / +0x47 / +0x48: o placar normal e o da prorrogação (−1 sem); valor; lê (registro_da_partida_pelo_clube)',
           'fixture +0x43 / +0x44 / +0x47 / +0x48: the normal and the extra-time score (-1 when none); value; reads (registro_da_partida_pelo_clube)'),
          ('registro da partida (0x1f bytes): +0 data, +8 jogo, +0xc/+0xe, +0x10 lado, +0x11 clube, +0x15 adversário, +0x19 pró, +0x1a contra, +0x1b competição; valor e id; grava (registro_da_partida_pelo_clube), lê o placar (codigo_do_resultado)',
           'match record (0x1f bytes): +0 date, +8 fixture, +0xc/+0xe, +0x10 venue, +0x11 club, +0x15 opponent, +0x19 for, +0x1a against, +0x1b competition; value and id; writes (registro_da_partida_pelo_clube), reads the score (codigo_do_resultado)'),
          ('staff +4 / +8 / +0xc: os três nomes (registro de nome +0x33: o id); ponteiro; lê (referencia_ao_nome). staff +0x10: data de nascimento; valor; lê',
           'staff +4 / +8 / +0xc: the three names (name record +0x33: the id); pointer; reads (referencia_ao_nome). staff +0x10: birth date; value; reads')]
INTERESSA = ['CMRTE']
