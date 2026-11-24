# Mutacoes (sx01d_3) pela prova do save (mutacoes.py --prova-save N), sem o 4o campo (a base da prova roda com as
# rotinas desligadas):
# - a carga esvazia o registro de historias (contagem e capacidade 0 depois de ler a contagem; o ultimo bloco do item,
#   entao a leitura segue sem erro): as historias ja registradas voltam a ser postadas depois da carga
#   (has_logged_story), o que muda os sorteios e as noticias; so a carga do FeriasTesteSave (CARGA) acusa.
MUTACOES = [
    ('sx01d_carga_sem_registro',
     'write_u32(this + media_fields::STORY_LOG_CAPACITY, read_u32(logged));',
     'write_u32(logged, 0); write_u32(this + media_fields::STORY_LOG_CAPACITY, 0); // mutation: the log emptied'),
]
