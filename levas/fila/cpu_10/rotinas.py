# As suspensoes dos orgaos disciplinares (ruling_body.cpp; a familia de treino, regen, lesao, midia e noticias nao
# tinha mais rotina boa que rode na carreira, e ruling_body.cpp nao e de nenhum outro agente): a suspensao do cartao
# vermelho (RulingBody::make_points_ban 0x7e8e10, pelos orgaos de cada pais, rb_*.cpp, ao processar os cartoes de cada
# partida: os jogos ou os dias pelo codigo da falta e pela escala do orgao), a virada da temporada do orgao
# (RulingBody::start_new_season 0x7e7ed0, pela tabela virtual: solta as suspensoes velhas, libera as entradas sem
# suspensao e zera os cartoes das outras) e a compactacao das entradas (RulingBody::compact_entries 0x7e9050, pela
# 0x7e7ed0). Chamadas originais pelo endereco: 0x417c10 (copia da data), realloc 0x945667 e a caixa de erro
# (aviso_em); 0x55cb60, 0x55cb20 e 0x556d00 ja em Rust. Iguais no cm0102-1080.exe. Trocas: a RulingBody::daily_update
# (0x7e7d50) chama quatro originais nao portadas e fica de fora. Simulacao: as mutacoes na 0x7e8e10 pela carreira ate
# 2002-06-01 (cartoes vermelhos em toda rodada); a 0x7e7ed0 e a 0x7e9050 vao de passageiras (so rodam na virada da
# temporada de cada orgao). Comparadores: a 0x7e8e10 lado a lado no registro; a 0x7e7ed0 so a original (solta
# suspensoes e realoca; a copia leva o contador no modo Rust); a 0x7e9050 pelo plano (realoca: as entradas ocupadas
# copiadas antes, conferidas depois).
SAVE = False
ROTINAS = [('suspensao_do_vermelho', 0x7e8e10, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('nova_temporada_do_orgao', 0x7e7ed0, [0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1]),
           ('compactar_disciplina_do_orgao', 0x7e9050, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as suspensões dos órgãos disciplinares: a suspensão do cartão vermelho, a virada da temporada do órgão e a compactação das suas entradas',
        "the ruling bodies' bans: the red card ban, the body's new season and the compaction of its entries",
        [('suspensao_do_vermelho', 'RulingBody::make_points_ban', '0x7e8e10', 'ruling_body.cpp', 'thiscall, ret 0xc',
          'monta a suspensão do cartão vermelho: os jogos ou os dias pelo código da falta (9 a 12) e pela escala do órgão, a data de hoje e o jogador; 0 sem suspensão',
          'builds the red card ban: the matches or days from the offence code (9 to 12) and the body scale, today\'s date and the player; 0 when there is no ban'),
         ('nova_temporada_do_orgao', 'RulingBody::start_new_season', '0x7e7ed0', 'ruling_body.cpp', 'thiscall',
          'na virada da temporada, solta as suspensões de 2 (ou 4) anos, libera as entradas sem suspensão, zera os cartões das outras e compacta a tabela',
          'at the new season, releases bans 2 (or 4) years old, frees the entries without a ban, clears the bookings of the others and compacts the table'),
         ('compactar_disciplina_do_orgao', 'RulingBody::compact_entries', '0x7e9050', 'ruling_body.cpp', 'thiscall',
          'tira as entradas livres da tabela disciplinar do órgão, acerta o índice guardado de cada jogador que muda de lugar e realoca com folga de 100',
          'removes the free entries from the body\'s discipline table, fixes the stored index of each moved player and reallocates with 100 spare rows')])
CAMPOS = [('jogador da partida +0x2c: tipo do cartão (0 ou 1); +0x2e e +0x2f: código da falta (9 a 12); +0: id; +0x48; valor e id; lê (suspensao_do_vermelho)',
           'match player +0x2c: card type (0 or 1); +0x2e and +0x2f: offence code (9 to 12); +0: id; +0x48; value and id; reads (suspensao_do_vermelho)'),
          ('suspensão nova (registro do chamador) +0 jogos, +1, +2 dias, +4 data, +0x10 = 3, +0x1c id do jogador, +0x6f; valor; grava (suspensao_do_vermelho)',
           'new ban (caller record) +0 matches, +1, +2 days, +4 date, +0x10 = 3, +0x1c player id, +0x6f; value; writes (suspensao_do_vermelho)'),
          ('RulingBody +4 vaga e +5 tipo no índice da disciplina; +6 escala; +0xb entradas (14 bytes), +0xf capacidade, +0x13 conta; +0x17 e +0x1b; valor e ponteiro; lê (as três), grava (nova_temporada_do_orgao, compactar_disciplina_do_orgao)',
           'RulingBody +4 slot and +5 type in the discipline index; +6 scale; +0xb entries (14 bytes), +0xf capacity, +0x13 count; +0x17 and +0x1b; value and pointer; reads (all three), writes (nova_temporada_do_orgao, compactar_disciplina_do_orgao)'),
          ('entrada disciplinar +0 id (−1 livre), +4 e +5 cartões, +6 suspensão (índice com sinal, −1 sem), +8 palavra, +0xd bits; id e valor; lê e grava (nova_temporada_do_orgao, compactar_disciplina_do_orgao)',
           'discipline entry +0 id (−1 free), +4 and +5 bookings, +6 ban (signed index, −1 none), +8 word, +0xd bits; id and value; reads and writes (nova_temporada_do_orgao, compactar_disciplina_do_orgao)'),
          ('[0xae28f0] disciplina: +4 suspensões (20 bytes, +6 ano); vagas guardadas (0x55cb60/0x55cb20, +6 índice); valor; lê e grava (nova_temporada_do_orgao, compactar_disciplina_do_orgao)',
           '[0xae28f0] discipline: +4 bans (20 bytes, +6 year); stored slots (0x55cb60/0x55cb20, +6 index); value; reads and writes (nova_temporada_do_orgao, compactar_disciplina_do_orgao)'),
          ('[0xae2c90] data de hoje (+2 ano); valor; lê (suspensao_do_vermelho, nova_temporada_do_orgao)',
           "[0xae2c90] today's date (+2 year); value; reads (suspensao_do_vermelho, nova_temporada_do_orgao)")]
INTERESSA = ['CMRTE', 'SGE']
