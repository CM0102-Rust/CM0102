# Os nomes dos tipos de lesão e o texto da notícia da volta aos treinos (injury.cpp; leitor sma, 20/11/2026).
# Convenções conferidas na desmontagem e nos pontos de chamada:
# - injury_get_type_name (0x62ef30): thiscall(this ignorado, tipo (byte), buffer, tamanho, artigo (byte)), ret 0x10;
#   devolve o eax da 0x66f4e0 (o invólucro que termina em 0x62ef1f o devolve adiante); chamada pelas notícias
#   (0x780587..0x781254, 0x7a202d..0x7a2263), pela 0x62ef1f, pela 0x631580 e pela 0x8874aa; duas tabelas de salto
#   (0x63044c com o artigo, 0x630594 sem), 0x52 entradas cada, todas no padrão "push texto; push buffer; call 0x66f4e0"
#   (conferidas por script, nenhuma cai no padrão de erro); o tipo é movsx e comparado sem sinal com 0x51;
# - injury_write_recovery_news_text (0x631580): thiscall(this, notícia, corpo, Text), ret 0xc (o mapa diz "unused"
#   no 3º argumento, mas ele é o Text: mov ecx, [esp+0x1ec] antes da 0x4ae660); o this só vai adiante para a 0x630c50
#   (Rust, injury_get_category_name), a 0x62ef30 e a 0x6306e0, que não o usam; o byte da categoria vai num dword da
#   pilha com lixo em cima (a 0x630c50 Rust só lê o byte);
# - 0x6306e0 (injury_get_type_name_gender): fica original (o prólogo começa com call rel32, que o desvio só copia com
#   uma entrada no RELATIVAS do detours.rs); chamada como thiscall(this, tipo), ret 4, devolve al;
# - 0x785830 (newsitem_get_slot): ecx = notícia -> thiscall; 0x53e140 (nome por ids): cdecl, tamanho 0x99, estilo 5
#   na manchete e 1 no corpo (no corpo da volta completa, também o estilo 5 num segundo buffer); 0x549ef0 (Date):
#   thiscall(data, campo 6, campo 7); 0x53e990: cdecl(pessoa, ids, &data de nascimento); 0x7e5780 (Rust,
#   format_duration_text): cdecl(campo 8 como word, buffer, 0x32, 1); 0x944c9f (sprintf da CRT) só no texto 0x9e9838;
#   0x4ae660, 0x4ae770 e 0x4ae8a0 (o Text): thiscall.
# Comparadores: injury_get_type_name lado a lado (buffer e retorno); a 0x631580 por compare_text_writer.
SAVE = False
ROTINAS = [('injury_get_type_name', 0x62ef30, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('injury_write_recovery_news_text', 0x631580, [0x6a, 0xff, 0x68, 0xeb, 0xb1, 0x95, 0x00])]
DOCS = ('os nomes dos tipos de lesão e o texto da notícia da volta aos treinos',
        'the injury type names and the return to training news text',
        [('injury_get_type_name', 'injury_get_type_name', '0x62ef30', 'injury.cpp', 'thiscall, ret 0x10',
          'o nome traduzido de um tipo de lesão (0 a 0x51), com o artigo indefinido ("um resfriado") ou sem ele; na língua 3 sempre sem; outro valor de artigo fora da língua 3 não escreve nada; buffer abaixo de 30 bytes ou tipo fora da tabela: a caixa de erro',
          'the translated name of an injury type (0 to 0x51), with the indefinite article ("a cold") or without it; in language 3 always without; another article value outside language 3 writes nothing; a buffer under 30 bytes or a type outside the table: the error box'),
         ('injury_write_recovery_news_text', 'injury_write_recovery_news_text', '0x631580', 'injury.cpp', 'thiscall, ret 0xc',
          'o título ou o corpo da notícia da volta aos treinos leves ou completos: a manchete com o nome; o corpo pela categoria do tipo: tratamentos e férias (12) pelo tipo, pelo resultado e pelo campo 10; doenças (0) pelo tipo; as outras com o nome da lesão (a categoria, ou o tipo na categoria 6) e o gênero; nos treinos leves, o tempo previsto até os completos (aproximado); o vínculo da pessoa quando os ids e a data de nascimento ainda batem',
          "the headline or body of the news of the return to light or full training: the headline with the name; the body by the type's category: treatments and holidays (12) by the type, the outcome and slot 10; illnesses (0) by the type; the others with the injury's name (the category, or the type in category 6) and gender; in light training, the expected time to full training (approximate); the person's link when the ids and the birth date still match")])
