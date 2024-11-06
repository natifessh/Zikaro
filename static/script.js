let id;
let toBeViewed={
    title:"",
    description:"",
    date:""
}

document.addEventListener('DOMContentLoaded', function() {
     //signup page

   
    const signupForm = document.getElementById('signupForm');
    if (signupForm) {
        signupForm.addEventListener('submit', async function(event) {
            event.preventDefault();

            const username = document.getElementById('username').value;
            const password = document.getElementById('password').value;
            let user = {
                user_name: username,
                password: password
            };

            try {
                const response = await fetch("http://127.0.0.1:8080/c/user", {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(user)
                });

                if (response.ok) {
                    console.log("Signup successful", response);
                    window.location.href = "login.html"; 
                } else {
                    const errorText = await response.text();
                    console.error('Signup failed:', errorText);
                    alert("Signup failed. Please try again.");
                }
            } catch (e) {
                console.log("Error during signup:", e);
                alert("An error occurred. Please try again.");
            }
        });
    }

    //login page

    const loginForm = document.getElementById('loginForm');
if (loginForm) {
    loginForm.addEventListener('submit', async function(event) {
        event.preventDefault();

        const username = document.getElementById('username').value;
        const password = document.getElementById('password').value;
        let user = {
            user_name: username,
            password: password
        };

        try {
            console.log("Logging in with user:", JSON.stringify(user));
            const response = await fetch("http://127.0.0.1:8080/login", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(user)
            });

            if (response.ok) {
                const data = await response.json();
                console.log("Login successful, user ID:", data);
                
                id=data;
               
                window.localStorage.setItem("id",id)

                window.location.href = "index.html";
            } else {
                const err = await response.json();
                console.error('Login failed:', err);
                alert(err.error || "Login failed!");
            }
        } catch (error) {
            console.log("Error during login:", error);
            alert("An error occurred. Please try again.");
        }
    });
}

//index page   
let diariesList=document.querySelector(".diaries-list");
//let li=document.createElement("li");
let lid=document.body.querySelector(".id");
let id=window.localStorage.getItem("id");
let uploadForm=document.getElementById("uploadForm")
console.log(uploadForm)



if(diariesList,lid,uploadForm){
uploadForm.addEventListener("submit",async function(event){

let title=document.getElementById("title").value;
console.log(title)
let description=document.getElementById("description").value;
let date=document.getElementById("date").value
    event.preventDefault();
    let entry={
        title:title,
        description:description,
        date:date


    }
    console.log(title)
    try{
        const response = await fetch(`http://127.0.0.1:8080/${id}/upload`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(entry)
        });
        console.log("sucesss")
        console.log(JSON.stringify(entry))
    
    }catch(e){
        console.log(e)
    }
})


window.addEventListener("DOMContentLoaded", async function(){
 let request= await fetch(`http://127.0.0.1:8080/${id}/all`)
 let res=await request.json();
 let hiddenForm=document.querySelector(".hidden-container");
 let titleToBeViewed=document.getElementById("titleToBeViewed");
 let descriptionToBeViewed=document.getElementById("descriptionToBeViewed")
 let  dateToBeViewed=document.getElementById("dateToBeViewed")
 const closeButton = document.getElementById('close-button');

closeButton.addEventListener('click', function(event) {
    event.preventDefault(); 
    event.stopImmediatePropagation()
    hiddenForm.classList.remove('display');

});
 res.reverse().forEach(entry => {
    let li=document.createElement("li")
    let parentDiv=document.createElement("div")
    parentDiv.className="entryDetail"
    let div=document.createElement("div")
    div.className="diary"
    
    let small=document.createElement("small")
    small.textContent=entry.date
    div.appendChild(small)
    let p=document.createElement("p")
    p.className="description"
    p.textContent=entry.description;
    div.appendChild(p)
    let btn=document.createElement("button")
    btn.className="deletebtn"
    btn.textContent="delete"
 
    btn.addEventListener("click",async function(event){
        event.preventDefault();
        event.stopImmediatePropagation()
        let res=await fetch(`http://127.0.0.1:8080/${id}/entry/${entry.id}`,{
         method:"DELETE",
         headers: {
             'Content-Type': 'application/json'
         }
 
        })
 
     }) 
    let h4=document.createElement("h4")

   h4.textContent=entry.title
   parentDiv.appendChild(h4)
   parentDiv.appendChild(div)
   parentDiv.appendChild(btn)
   parentDiv.addEventListener("click",function(event){
    event.preventDefault();

    hiddenForm.classList.toggle("display")
    titleToBeViewed.value=entry.title;
    descriptionToBeViewed.value=entry.description;
    dateToBeViewed.value=entry.date;
   })
    li.appendChild(parentDiv)
    diariesList.appendChild(li)
    
 });

})



}

   
    
});