# A noticia do pedido da lista de numeros, o texto dela e a noticia da inscricao europeia (squad_manager.cpp; leitor
# cx15, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS): o pedido sai na numeracao da liga para o clube humano
# (assign_league_squad_numbers 0x859f29, ja em Rust, na criacao e na virada); a inscricao europeia nos prazos
# (process_euro_registration 0x85aa7f) para clube humano numa copa europeia; o texto quando a noticia e aberta e, com
# CM_TEXTO_NOTICIAS=1 (CM_TEXTOS=1 na comparacao da cadeia), quando e enviada (0x77ffcb no despachante dos textos).
# - 0x85aec0 SQUAD_MANAGER::send_squad_numbers_request_news thiscall(clube) ret 4 (o ecx nao e lido): noticia 0x18
#   (NewsItem::NewsItem 0x77bbe0 local; push 0; push 0x18 -> thiscall, ret 8) com 0 = id do clube, 1 = 0; News::
#   send_to_club_manager 0x7861e0 (ecx = [0xb7431c]; 2 pushes -> thiscall, ret 8). O quadro SEH so cobre o NewsItem
#   local (0xe0 bytes). Prologo push -1; push imm32 (7 bytes).
# - 0x85af40 SQUAD_MANAGER::write_squad_numbers_news_text thiscall(noticia, corpo, texto) ret 0xc (o ecx nao e lido):
#   o clube pelo indice do slot 0 (registro de 0x245 bytes em [0xae23b4]), o slot 1 (lista entregue); corpo 0xa82c34 e,
#   com o slot 1, 0xa82c14 acrescentado (Text::append_string 0x4ae770); manchete 0xa82bd4. langlib_sprintf 0x66f4e0
#   (cdecl) no buffer 0xde1f64 com o genero (+0x52) duas vezes e o nome curto (+0x38); Text::set_string 0x4ae660 e
#   Text::add_link 0x4ae8a0 (o clube, 0x7d5). Prologo push ebx; push ebp; push esi; push edi; mov edi, [esp+0x14] (8).
# - 0x85b020 SQUAD_MANAGER::send_euro_registration_news thiscall(clube, competicao, passo) ret 0xc (o ecx nao e lido):
#   o elenco (database_collect_club_squad 0x541f90 cdecl(clube, &lista, 0), 50 pessoas) contado onde a pessoa existe
#   (id < [0xae2364], com sinal), tem marca de inscricao (+0x6d) e a marca nao tem 0x18; 25 inscritos fora do passo 3:
#   nada. Vagas: 25, no passo 3 os inscritos + 2 ate 25. Intertoto (chave 0x9cf6f8) vira Copa da UEFA (0x9cf6f0).
#   Noticia 0x19: 0 id do clube, 1 competicao, 2 passo, 3 = 0, 4 vagas, 5 inscritos, 6 = 0 (bytes com sinal). O
#   quadro SEH so cobre o NewsItem local. Prologo push -1; push imm32 (7 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os envios so a original (mandam noticias); o texto lado a lado
# (compare_text_writer).
SAVE = False
ROTINAS = [('send_squad_numbers_request_news', 0x85aec0, [0x6a, 0xff, 0x68, 0x7b, 0x29, 0x96, 0x00]),
           ('write_squad_numbers_news_text', 0x85af40, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('send_euro_registration_news', 0x85b020, [0x6a, 0xff, 0x68, 0x9b, 0x29, 0x96, 0x00])]
DOCS = ('o pedido da lista de números e o texto dele; a notícia da inscrição europeia',
        'the squad numbers request and its text; the European registration news',
        [('send_squad_numbers_request_news', 'SQUAD_MANAGER::send_squad_numbers_request_news', '0x85aec0', 'squad_manager.cpp', 'thiscall, ret 4',
          'manda ao técnico do clube a notícia que pede a lista dos números da temporada, ainda não entregue',
          'sends the club\'s manager the news asking for the season\'s squad numbers list, not yet submitted'),
         ('write_squad_numbers_news_text', 'SQUAD_MANAGER::write_squad_numbers_news_text', '0x85af40', 'squad_manager.cpp', 'thiscall, ret 0xc',
          'escreve o texto do pedido da lista dos números: a manchete com o clube, o corpo com o pedido e, se a lista já foi entregue, a linha que o diz',
          'writes the text of the squad numbers request: the headline with the club, the body with the request and, when the list was submitted, the line saying so'),
         ('send_euro_registration_news', 'SQUAD_MANAGER::send_euro_registration_news', '0x85b020', 'squad_manager.cpp', 'thiscall, ret 0xc',
          'conta os inscritos do clube na Europa e manda ao técnico a notícia da inscrição da fase, com as vagas (25, ou na terceira fase os inscritos mais dois); com 25 inscritos só a terceira fase manda; a Intertoto é dada como Copa da UEFA',
          'counts the club\'s players registered for Europe and sends the manager the registration news of the phase, with the places (25, or at the third phase the registered plus two); with 25 registered only the third phase sends it; the Intertoto is given as the UEFA Cup')])
CAMPOS = []
INTERESSA = []
