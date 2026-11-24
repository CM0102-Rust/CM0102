# Mutacoes (sx01e_2). A rotina so roda no modo Rust (atras da porta do pos-jogo), nas historias de jogador depois das
# partidas e no laco diario:
# - a moral do contrato nao muda (muda o humor, os pedidos de transferencia e as historias seguintes);
# - a resposta do tecnico do computador nao e aplicada (muda as confiancas e o humor).
MUTACOES = [
    ('sx01e_midia_sem_moral',
     'contract.set_morale(if morale > 100 { 100 } else if morale < -100 { -100 } else { change.wrapping_add(contract.morale()) });',
     '// mutation: the morale unchanged',
     'sx01e_2 media story'),
    ('sx01e_midia_sem_resposta',
     'exe::media_apply_player_story_response(this, player, club, kind, response);',
     '// mutation: the response not applied',
     'sx01e_2 media story'),
]
