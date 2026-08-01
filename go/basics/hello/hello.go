package main

import "fmt"
import "rsc.io/quote"
import (
	"example/greetings"
)

func main() {
	fmt.Println("Hello, go...")
	fmt.Println("Go Quote: " + quote.Go())
	fmt.Println(quote.Opt())
	message := greetings.Hello("Baskar")
	fmt.Println(message)
}
