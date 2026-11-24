# Mutacoes (sx01e_1). A rotina so roda no modo Rust (atras da porta do pos-jogo), nos elogios do laco diario e do dia
# de partida:
# - a reputacao atual nao sobe (muda as propostas, as convocacoes e os elogios seguintes);
# - a resposta do tecnico do computador nao e aplicada (muda as confiancas e o humor).
MUTACOES = [
    ('sx01e_elogio_sem_reputacao',
     'sheet.set_current_reputation(sheet.current_reputation().wrapping_add(200).min(10000));',
     'sheet.set_current_reputation(sheet.current_reputation()); // mutation: no reputation rise',
     'sx01e_1 praise story'),
    ('sx01e_elogio_sem_resposta',
     'exe::media_apply_player_story_response(this, player, club, kind as u32, response);',
     '// mutation: the response not applied',
     'sx01e_1 praise story'),
]
