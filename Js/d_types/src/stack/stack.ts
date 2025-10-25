export class Stack<T> {
    stackArray: T[];

    constructor() {
        this.stackArray = [];
    }

    push(o: T) {
        this.stackArray.push(o);
    }

    take(): T {         // get last inserted item
        if(this.stackArray.length > 0) {
            const val = this.stackArray[this.stackArray.length-1];
            if(val === undefined) throw new Error("no item found at location:"+ this.stackArray.length);
            this.stackArray.splice(this.stackArray.length-1, 1);
            return val;
        }
        throw new Error("stack is empty");
        // const val = this.stackArray.pop();
        // if(!val) throw new Error("no element found!");
        // return val;
    }

    display(): void {
        console.log(this.stackArray);
    }

    size(): number {
        return this.stackArray.length;
    }
}