console.log("hajimeruzoo...");
import { Stack } from "./src/stack/stack";


const stack: Stack<number> = new Stack();

stack.push(4);
stack.push(6);

stack.display();

stack.push(8);
console.log(stack.take());
stack.display();