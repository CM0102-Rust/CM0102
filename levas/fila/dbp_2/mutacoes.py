# Mutacoes de tirar das preferencias: cada uma tira da lista vizinha (favoritos -> rejeitados). A do clube favorito
# roda no diario das pessoas quando o apego ao clube acaba e a da pessoa favorita nas relacoes do elenco (diaria) e na
# volta da lesao: a carreira ate 2002-06-01 deve acusar. A do clube (0x547bd0) so roda nas audiencias das disputas:
# PODE ESCAPAR da carreira (o comparando acusa se houver contest).
MUTACOES = [
    ('dbp_tirar_clube_favorito', '        dbp_tirar_de_tres(pref + 4, clube); // os clubes favoritos', '        dbp_tirar_de_tres(pref + 0x10, clube); // mutacao: dos clubes rejeitados'),
    ('dbp_clube_tirar_favorita', '        dbp_tirar_de_tres(clube + 0x9b, pessoa); // os favoritos do clube', '        dbp_tirar_de_tres(clube + 0xa7, pessoa); // mutacao: dos rejeitados do clube'),
    ('dbp_tirar_pessoa_favorita', '        dbp_tirar_de_tres(pref + 0x1c, alvo); // as pessoas favoritas', '        dbp_tirar_de_tres(pref + 0x28, alvo); // mutacao: das pessoas rejeitadas'),
]
