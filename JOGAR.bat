@echo off
rem Abre o CM 01/02 em janela, sem CD, com a camada de video em Rust (jogo\ddraw.dll).
rem A janela pode ser redimensionada: o quadro do jogo acompanha, sem distorcer.
start "" /D "%~dp0jogo" "%~dp0jogo\cm0102-portatil.exe" -windowed
