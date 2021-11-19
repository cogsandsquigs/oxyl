echo "Welcome to the Oxyl REPL!"
echo "This language is not complete, so sorry for any bugs!"

while true:
    stdout.write "~> "
    var line: string = readLine(stdin)
    echo "=> ", line
