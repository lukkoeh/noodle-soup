let host = "http://localhost:3000"

let cookie = ((http post -fe $"($host)/login" --content-type application/json { email: "a.mester@mosbach.dhbw.de", password: "12345678" }).headers.response | where name == "set-cookie").value.0

# http post -fe $"($host)/templates" --content-type "application/json" -H [Cookie $cookie] {name: "TestTemplate"}
# http get -fe $"($host)/template/1/section/1/content" -H [Cookie $cookie]
# http post -fe $"($host)/template/1/section/1/content" --content-type "application/json" -H [Cookie $cookie] {type: "text", content: "Hello There!", orderIndex: 0}
# print (http put -fe $"($host)/template/1/section/1/content" --content-type "application/json" -H [Cookie $cookie] {contentId: 1, parentSectionId: 1, orderIndex: 4, type: "video", content: "{fileUid: fca3de}"})
# http get -fe $"($host)/template/1/section/1/content" -H [Cookie $cookie]
# http delete -fe $"($host)/template/1/section/1/content" -H [Cookie $cookie]
# print (http delete -fe $"($host)/template/1/section/1" -H [Cookie $cookie])
# http get -fe $"($host)/template/1/section/1/content" -H [Cookie $cookie]
# http delete -fe $"($host)/template/1" -H [Cookie $cookie]
http get -fe $"($host)/templates" -H [Cookie $cookie]
