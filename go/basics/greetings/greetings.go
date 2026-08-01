package greetings

import "fmt"

func Hello(name string) string {
	// return a greeting that has the name
	message := fmt.Sprintf("Hi, %v. Welcome!", name)
	return message
}

