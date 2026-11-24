# Mutacoes (sx01d_5). A rotina so roda no modo Rust (atras da porta do pos-jogo), em todo desabafo depois de uma
# partida; nao aparece na cobertura ENG ate 2002-06-01 do modo original:
# - a historia nao e registrada (log_story): o mesmo desabafo volta a ser noticiado e respondido;
# - o tecnico do computador responde 1 em vez de 0 (muda as confiancas e o humor).
MUTACOES = [
    ('sx01d_desabafo_sem_registro',
     'log_story(this, player.addr(), club.addr(), 0xbd9, read_u8(story + 0x13) as u32, 0);',
     '// mutation: the story not logged',
     'sx01d_5 outburst story'),
    ('sx01d_desabafo_resposta_1',
     'exe::media_apply_player_story_response(this, player, club, kind, 0);',
     'exe::media_apply_player_story_response(this, player, club, kind, 1); // mutation',
     'sx01d_5 outburst story'),
]
