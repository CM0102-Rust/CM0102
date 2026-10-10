@echo off
rem Abre o CM 01/02 como o JOGAR.bat, com a comparacao ligada: cada rotina reescrita em Rust roda junto
rem com a original e as diferencas vao para jogo\ddraw.log (o jogo segue com o resultado da original).
rem Um pouco mais lento nas partidas. Use para a carreira de teste do Lucas.
set CM_COMPARAR=1
start "" /D "%~dp0jogo" "%~dp0jogo\cm0102-1080.exe" -windowed
