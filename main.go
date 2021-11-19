package main

import (
  "os"
  "fmt"
  "bufio"
  "strings"
)

func main() {
  reader := bufio.NewReader(os.Stdin)
  fmt.Println("Oxyl shell")
  fmt.Println("---------------------")

  for {
    fmt.Print("~> ")
    text, _ := reader.ReadString('\n')
    // convert CRLF to LF
    text = strings.Replace(text, "\n", "", -1)

    fmt.Println("=> " + text)

  }
}