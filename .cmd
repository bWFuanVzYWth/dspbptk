for /R %%i in (*.txt) do (
    mklink tmp.txt "%%i"
    dspbptk tmp.txt
    del tmp.txt
)