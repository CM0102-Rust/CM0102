# Mutacoes (tel18_3): provam os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - o status atual do elenco nao fica amarelo (perfil de um jogador do seu clube > Transfer Status);
# - o botao Auto liga sem trocar o preco pedido (mesma tela > Auto).
MUTACOES = [
    ('tel18_3_status_atual',
     "let colour = if squad as i8 as i16 == k as i16 { yellow } else { pale };",
     "let colour = if squad as i8 as i16 == k as i16 + 1 { yellow } else { pale }; // mutation: the next status in yellow",
     'tel18_3 transfer status screen'),
    ('tel18_3_auto_sem_preco',
     "let on = (auto == 0) as usize;",
     "let on = (auto == 0) as usize * 0; // mutation: Auto never turns on",
     'tel18_3 transfer status Auto'),
]
