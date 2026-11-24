# Mutacoes (cx09_18) pela carreira com a Croacia ate 2002-08-01 (a Segunda Divisao croata de 2001-02 acaba em maio de
# 2002 e a virada de julho usa os status das linhas):
# - o fim da temporada sem a atualizacao final da tabela (virtual 0x94): os status de acesso e rebaixamento do grupo
#   norte ficam como estavam e a virada muda; o comparador do dia tambem confere o fim.
# A passagem e a repescagem sao o molde do grupo sul (cro_a2b, ja publicado e provado) com o arquivo do grupo norte:
# sem mutacao propria.
MUTACOES = [
    ('cx09_18_fim_sem_atualizar',
     '    l.set_tiebreak_matches(0);\n    let status: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + league_vtable::UPDATE_POSITIONS_AND_STATUS));\n    status(this);\n    r\n}',
     '    l.set_tiebreak_matches(0);\n    // mutation: no final table update\n    r\n}'),
]
