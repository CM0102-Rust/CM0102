# Tres escritores de texto de noticia do manager_manager.cpp (leitor sxB, rodada 5, 24/11/2026): o boato de que alguem
# quer um cargo (0x69fef0, noticia (6, 0x1776)), as expectativas da federacao para a selecao (0x6a4550, (6, 0x177e)) e
# a oferta do cargo de tecnico ao humano (0x6a5120, (6, 0x1774)). Cada um inteiro, de um desvio. Chamados so pelo
# despacho NewsItem::write_text (0x7818f9, 0x78196b, 0x7818ad; NEWS_WRITERS); na carreira automatica pelo modo
# CM_TEXTO_NOTICIAS, ligado na cadeia em todo jogo comparando (levas/reorganizacao/proximas/textos_noticias.md).
# Fronteiras conferidas na desmontagem: 0x69fef0..0x6a0534 (ret 0xc em 0x6a001f e 0x6a0531); 0x6a4550..0x6a494e (ret
# 0xc em 0x6a45aa e 0x6a494b; depois as tabelas 0x6a4950 (dword) e 0x6a4978 (byte) da expectativa + 4, 0..0x9a);
# 0x6a5120..0x6a5605 (ret 0xc em 0x6a5602; o 0x6a536c do mapa e meio desta rotina). As tres referencias absolutas a
# 0x6a5120 (0x40839d, 0x5ed21e, 0x6b515b) sao dados, nao chamadas.
# Prologo de 0x6a4550: mov eax, 0x157c; call _chkstk: o Prologue de 5 bytes (so o mov) dispensa RELATIVAS.
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - 0x69fef0 e 0x6a4550: stdcall(noticia, corpo, Text), ret 0xc (o despacho as chama como thiscall; o ecx nao e lido);
#   0x6a5120: thiscall(manager_manager, noticia, corpo, Text), ret 0xc (o ecx vai para a 0x694440);
# - 0x785830 get_slot thiscall(noticia, k) ret 4; 0x53e140 cdecl(n1, n2, n3, buf, 0x190, estilo, id); 0x53ad00
#   database_get_job_name cdecl(cargo (byte), 0, 0, &genero) (o genero so e escrito no idioma 4); 0x945a60 strstr
#   cdecl(0xde1f64, nome); 0x6181d0 cdecl(nacao, buf, tamanho, 9); 0x5409b0 club_is_national_team cdecl(clube);
#   chamada virtual da vaga 0x78 do objeto da competicao (thiscall, sem argumentos) -> registro de competicao;
#   0x53b900 database_is_national_team cdecl(clube); 0x694440 build_board_expectation_text thiscall(this, clube,
#   bandeiras, buf, 0x1388) ret 0x10; 0x43f460 Cash::get_display_amount thiscall(&valor, 4) -> edx:eax; 0x441c10
#   cdecl(buf, 0x3e8, i64, 4); 0x66f4e0 langlib_sprintf cdecl; 0x4ae660 set_string ret 4; 0x4ae8a0 add_link ret 0xc.
# Defeitos do original: 0x69fef0 fora do idioma 4 passa ao modelo o byte de genero do cargo nunca escrito (lixo da
# pilha; a copia passa 0); 0x6a4550 sem nacao deixa o buffer do adjetivo sem escrever e vincula sempre a competicao,
# inclusive a -1 (le o registro antes da tabela); 0x6a5120 vincula o clube antes de por o texto (inofensivo).
SAVE = False
ROTINAS = [('news_job_rumour_text', 0x69fef0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0xa8, 0x01, 0x00, 0x00]),
           ('news_fa_expectations_text', 0x6a4550, [0xb8, 0x7c, 0x15, 0x00, 0x00]),
           ('news_job_offer_to_human_text', 0x6a5120, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os textos do boato de cargo, das expectativas da federacao e da oferta de emprego ao humano',
        "the texts of a job rumour, the FA's expectations and a job offer to the human",
        [('news_job_rumour_text', 'news_job_rumour_text', '0x69fef0', 'manager_manager.cpp', 'stdcall, ret 0xc',
          'escreve a manchete ou o corpo do boato de que alguem da comissao esta ligado ao cargo de tecnico de um clube: a manchete "ligado ao cargo do <clube>"; o corpo pela forma (vaga 7): uma sondagem (no proprio clube, a promocao a tecnico), a ideia de assumir, a ligacao com o cargo ou o interesse nele, com o cargo atual e o clube dele quando tem um. Vinculos: a pessoa quando os nomes conferem, o clube dele quando aparece no texto e o clube do cargo. Defeito do original: fora do idioma 4 o genero do cargo e um byte nunca escrito',
          "writes the headline or body of the rumour that a staff member is linked with a club's manager's job: the headline \"linked with <club> job\"; the body by the wording (slot 7): an approach (at his own club, promotion to manager), considering taking over, linked with the job or interested in it, with his current job and club when he has one. Links: the person when the names match, his club when it is in the text and the club with the job. Defect of the original: outside language 4 the job's gender is a byte never written"),
         ('news_fa_expectations_text', 'news_fa_expectations_text', '0x6a4550', 'manager_manager.cpp', 'stdcall, ret 0xc',
          'escreve a manchete ou o corpo das expectativas da federacao para a selecao: a manchete "federacao anuncia expectativas"; o corpo pela expectativa (vaga 1): a segunda fase, classificar-se (da competicao ou para a que o objeto dela aponta), vencer, a segunda rodada, as quartas, as semifinais, vencer (esperanca), ou nos amistosos revelar talentos ou honrar o pais, ou ir bem na competicao, com o adjetivo da nacao e o nome curto da competicao. Vinculos: o adjetivo ao clube e a competicao (sempre, mesmo a -1). Defeitos do original: sem nacao o adjetivo nao e escrito; o vinculo da competicao -1 le o registro antes da tabela',
          "writes the headline or body of the football association's expectations for its national team: the headline \"Football Association announce expectations\"; the body by the expectation (slot 1): the second phase, qualifying (from the competition or for the one its object names), winning, the second round, the quarter-finals, the semi-finals, winning (hope), or in the friendlies grooming the stars of the future or upholding the country's honour, or performing well in the competition, with the nation's adjective and the competition's short name. Links: the adjective to the club and the competition (always, even -1). Defects of the original: without a nation the adjective is never written; the link of competition -1 reads the record before the table"),
         ('news_job_offer_to_human_text', 'MANAGER_MANAGER::news_job_offer_to_human_text', '0x6a5120', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da oferta do cargo de tecnico de um clube ou da federacao ao humano: a manchete "<clube> oferece o cargo de tecnico"; o corpo pelo estado (vaga 1: oferecido, aceito, recusado): a federacao (tecnico da selecao ou da sub-21, com o adjetivo da nacao) ou o clube, que ao oferecer junta as expectativas da diretoria e a verba (acima de reputacao ao quadrado x 0,05 + 500000 o valor; acima de 50000 "uma pequena quantia"; senao nada). O nome longo do clube e vinculado antes do texto',
          "writes the headline or body of a club's or football association's offer of its manager's job to the human: the headline \"<club> offer you the position of manager\"; the body by the state (slot 1: offered, accepted, declined): the FA (manager of the national or the Under 21 team, with the nation's adjective) or the club, which when offering adds the board's expectations and the funds (above reputation squared x 0.05 + 500000 the amount; above 50000 \"a small amount\"; else none). The club's long name is linked before the text")])
