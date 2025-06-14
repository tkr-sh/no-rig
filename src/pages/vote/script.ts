import "hyperscript.org"

let values = {}

const set = (k, v) => {
    values[k] = v;
    $(`.opt-${k}`).forEach(e => e.rmClass("selected"))
    $(`.opt-${k}`)[v].addClass("selected")
    let scoreEle = $("#score");
    let currScore = Object.values(values).reduce((a,b) => a+b)
    let objScore = $(".option").length * 3
    score.text(currScore + " / " + objScore)

    if (objScore != currScore) {
        score.addClass("invalid")
    } else {
        score.rmClass("invalid")
    }
}


$("#submit").on("click", () => {
    post("/api/vote/" + document.location.href.split("/").at(-1), { with_username: $("#name").value, options: values})
        .then(res => {
            if (res.ok) {
                return res.text()
            } else {
                return res.text().then(errorData => {
                    throw new Error(errorData || "Failed to submit vote");
                });
            }
        })
        .then(() => {
            $("main").forEach(e => e.addClass("hidden"));
            $("body")[0].innerHTML += "<h1 class=\"success\">Vote registered! Thanks</h1>";

        })
        .catch(err => {
            console.log("heyyyyyyyyyyy")
            $("#err").rmClass("hidden")
            $("#err").text("" + err)
        });
});
