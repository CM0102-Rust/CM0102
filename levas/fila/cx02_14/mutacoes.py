# Mutacoes (cx02_14) pela prova do save com um save que tenha a Grecia, a Irlanda ou a Italia (CARGA). Nenhuma abre
# caixa (a leitura segue alinhada).
# - o id -1 vira o clube 0 e nao a vaga vazia: a lista carregada muda (o comparador acusa).
MUTACOES = [
    ('cx02_14_vaga_vazia_vira_clube',
     '        write_usize(slot, if id == -1 { 0 } else { club_at(id as isize).addr() });\n        k += 1;\n    }\n    1\n}',
     '        write_usize(slot, if id == -1 { club_at(0).addr() } else { club_at(id as isize).addr() }); // mutation\n        k += 1;\n    }\n    1\n}'),
]
