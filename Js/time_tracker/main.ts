// req short listed
// - This is a calender in the first place
// - Starting the app without any param should show the calender looking thing to the screen 
// then try command liners

const COMMANDS: string[] = ["all", "might"];

// custom functions
function print(param: any) {
  console.log(param);
}

const currentDateTime = () => {
  return new Date();
}

const printBootTime = () => {
  const now = new Date();
  const t_str = now.toDateString();
  const str_len = t_str.length;
  print("===" + "=".repeat(str_len) + "===");
  print("== " + t_str + " ==");
  print("===" + "=".repeat(str_len) + "===");
}

const printOnlyUsableArgs = (param: string[]) => {
  for(const str of param) {
    if(str.length < 10) {
      print("meaningful param : " + str);
    }
  }
}

const entryDecision = (clArgs: string[]) => {
  if(clArgs.length == 2) {
    printBootTime();
    return;
  }

  for (const par of clArgs) {
    if(COMMANDS.includes(par)) {
      print("Command found : " + par);
    }
  }

}

print("-- Welcome to the time tracker--");
// printBootTime();

// ts expects type to be mentioned
const args: string[] = process.argv;
// print(args);
// printOnlyUsableArgs(args);
entryDecision(args);
