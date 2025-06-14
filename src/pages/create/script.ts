// import "htmx.org"
import "hyperscript.org"

$("#submit").on("click", () => {
    let users = [];
    let options = [];
    $("#users").$("li").forEach(e => users.push(e.textContent));
    $("#options").$("li").forEach(e => options.push(e.textContent));
    let uuid = post("/api/create", { users, options, title: $("#title").value }).then(res => {
        res.text().then(uuid => {
            console.log(uuid);
            $("#res").text(document.location.href.replace("create", "vote") + "/" + uuid);
        });
    });
});
// post("/api/create", { $("#users li") }))
