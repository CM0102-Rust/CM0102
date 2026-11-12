# Mutacoes (nma_5). O titulo do boato roda em toda historia de boato (18 chamadas ate 2002-06-01, titulo e corpo); a
# historia do time roda 107 vezes. As do texto mudam o texto ou os vinculos e so o comparador as ve (o texto nao
# entra na carreira); as da historia mudam os sorteios (comparador) e a resposta do tecnico, que mexe na torcida e
# na diretoria (carreira):
# - o titulo pelo jogador % 2 em vez de % 3 (outro titulo);
# - o vinculo do jogador no titulo com o tipo de clube (0x7d5);
# - o tom da historia do time com sorteio(1501) (os sorteios mudam);
# - a resposta do tecnico da IA sem a pessoa (outra resposta, outro efeito).
MUTACOES = [
    ('nma5_titulo_dois',
     'match r.player() % 3 {',
     'match r.player() % 2 {',
     'nma_5 rumour headline'),
    ('nma5_vinculo_tipo',
     'exe::text_add_link(out, p as *const u8, 0x7d7, person); // nma5: the headline\'s player link',
     'exe::text_add_link(out, p as *const u8, 0x7d5, person); // mutation: the club link type',
     'nma_5 rumour headline'),
    ('nma5_tom_1501',
     'let tone: u32 = if (club.reputation() as i32) > exe::utils_random(0x5dc) + 0x1194 || national() {',
     'let tone: u32 = if (club.reputation() as i32) > exe::utils_random(0x5dd) + 0x1194 || national() {',
     'nma_5 team story'),
    ('nma5_resposta_sem_pessoa',
     'exe::media_pick_ai_manager_response(this, manager, person, k as i16 as u16 as u32) // nma5: the AI manager\'s answer',
     'exe::media_pick_ai_manager_response(this, manager, Staff(0), k as i16 as u16 as u32) // mutation: without the person',
     'nma_5 team story'),
]
