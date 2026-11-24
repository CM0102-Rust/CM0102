# Mutacoes (cx09_16) pela carreira de 8 ligas ate 2002-06-01 (a Serie B e a Serie C de 2001 acabam em dezembro):
# - o progresso da Serie B com o limite 4 (e nao 5): o comparador do progresso acusa;
# - o campeao da Serie B sem o registro de campeao (o status 5 da linha fica): o historico do tecnico muda; pode escapar
#   da carreira (so os registros do tecnico), dizer na prova.
# A passagem de fase da Serie C so despacha as montagens (7, 8 e 10); o comparador confere a fase + 1. Sem mutacao
# propria: mudar a fase despachada derruba o jogo (as fases seguintes leem as que faltariam).
MUTACOES = [
    ('cx09_16_progresso_limite_4',
     '    league_stage_progress(this, club, stage, current, best, worst, 5, 4)',
     '    league_stage_progress(this, club, stage, current, best, worst, 4, 4) // mutation'),
    ('cx09_16_campeao_sem_registro',
     '            exe::staff_records_add_league_champions_record(records, club, comp);\n            LeagueTableRow(row).set_status(5);',
     '            // mutation: no champions record\n            LeagueTableRow(row).set_status(5);'),
]
