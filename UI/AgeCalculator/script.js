
const dobInput = document.getElementById("dob");
const calculateButton = document.getElementById("button");
const currentDateArea = document.getElementById("currentDate");
const ageArea = document.getElementById("age");

// set current date on top
const currDate = new Date();
currentDateArea.innerText = `Current date is : ${currDate.getDate()}/${currDate.getMonth()}/${currDate.getFullYear()}`;


// event listener for click event
calculateButton.addEventListener("click", () => {
    // console.log("DOB input : "+dobInput.value);
    const enteredDob = new Date(dobInput.value);
    const yearsLived = currDate.getFullYear() - enteredDob.getFullYear();
    // console.log(`The years passed is : ${yearsLived}`);
    ageArea.innerText = `You are somewhat ${yearsLived} years old!`;
}) 


// console.log("Runs..");

