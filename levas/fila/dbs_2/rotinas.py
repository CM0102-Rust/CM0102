# Do Database.cpp: o nome da posição (0x5461b0) e a descrição do jogador (0x5465a0), textos da mídia (0x72d270,
# 0x744f80, 0x755860), dos contratos, da busca, dos olheiros e das transferências (0x8dc370, que a carreira percorre
# 1060 vezes), e o melhor candidato a favorito (0x548810: mídia 0x758200; os filtros e a nota da 0x548da0, já em Rust,
# sem o da máscara da posição). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real
# comparando. Sem SEH, sem sorteio direto; sem variável local lida sem valor (a 0x5461b0 conta numa local zerada antes;
# a 0x5465a0 só guarda o idioma; a 0x548810 grava o cursor e o contador do laço antes de ler e guarda a nota do
# candidato e um float nos próprios argumentos, gravados antes de lidos; a cópia não escreve neles). As locais de 0x200
# bytes da 0x5461b0 e da 0x5465a0 só servem à caixa (tamanho < 30), caminho que fica com a original. A 0x548810 chama
# 0x54cdb0, 0x4cf5f0, 0x8cca10 e 0x7c1e30 pelo endereço (o comparador volta o estado do sorteio por garantia).
SAVE = False
ROTINAS = [('nome_da_posicao', 0x5461b0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('descricao_do_jogador', 0x5465a0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('melhor_candidato_a_favorito', 0x548810, [0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10])]
DOCS = ('o nome da posição, a descrição do jogador e o melhor candidato a favorito',
        'the position name, the player description and the best favoured candidate',
        [('nome_da_posicao', 'write_position_name', '0x5461b0', 'Database.cpp', 'cdecl',
          'o nome traduzido da posição pela máscara (o último escrito vale; mais de 3 posições "utility player", nenhuma "player") e o gênero pelo idioma',
          'the translated position name by mask (the last one written wins; more than 3 positions "utility player", none "player") and the gender by language'),
         ('descricao_do_jogador', 'write_player_description', '0x5465a0', 'Database.cpp', 'cdecl',
          'a descrição do jogador pelas qualidades (0x546ab0) e pela posição: atacante, meia, lateral, defensor, veterano ou revelação pela idade; o gênero pelo idioma',
          'the player description by strengths (0x546ab0) and position: forward, midfielder, full back, defender, veteran or rookie by age; the gender by language'),
         ('melhor_candidato_a_favorito', 'staff_is_best_favoured_candidate', '0x548810', 'Database.cpp', 'cdecl',
          '1 se o candidato passa nos filtros do companheiro favorito e nenhum dos 50 do elenco tem nota maior (a nota da 0x548da0)',
          '1 when the candidate passes the favoured team mate filters and none of the 50 in the squad has a higher score (the 0x548da0 score)')])
CAMPOS = [('[0xae23bc] pessoa (110 bytes) +0x18 idade, +0x22; lê (descricao_do_jogador)', '[0xae23bc] person (110 bytes) +0x18 age, +0x22; reads (descricao_do_jogador)'),
          ('TStaff +0x18, +0x1a nação, +0x3e/+0x42 nascimento, +0x57, +0x59, +0x5b, +0x65 preferências (+0x1c..+0x30); jogador +5, +0xb reputação, +0x21; TClub +0xd7 elenco (50), +0x53 nação, +0x80 reputação; contrato +0x45, +0x4f; [0xae2364], [0xb74350] +0x10/+0x18; lê (melhor_candidato_a_favorito)',
           'TStaff +0x18, +0x1a nation, +0x3e/+0x42 birth date, +0x57, +0x59, +0x5b, +0x65 preferences (+0x1c..+0x30); player +5, +0xb reputation, +0x21; TClub +0xd7 squad (50), +0x53 nation, +0x80 reputation; contract +0x45, +0x4f; [0xae2364], [0xb74350] +0x10/+0x18; reads (melhor_candidato_a_favorito)')]
INTERESSA = ['SGE', 'CMRTE']
