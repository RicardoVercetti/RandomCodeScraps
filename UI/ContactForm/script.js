const sendBtn = document.getElementById("send");
const nameEl = document.getElementById("name");
const mailEl = document.getElementById("mail");
const messageEl = document.getElementById("message");

sendBtn.addEventListener("click", () => {
    if (nameEl.value === '') {
        // change color to red
        nameEl.setAttribute("")
    }

    if(mailEl.value === '') {
        // change color to red
    }

    if(messageEl.value == '') {
        // change color to red
    }

    console.log("name : " + nameEl.value);
    console.log("mail : " + mailEl.value);
    console.log("message : " + messageEl.value);
});