const method = {
  get: "GET",
  patch: "PATCH",
  put: "PUT",
  post: "POST",
  delete: "DELETE"
}

let session_cookie = null

function Test(
  method,
  url,
  requestBody,
  expectedResponseCode,
  expectedResponseBody) {

  return {
    method,
    url,
    requestBody,
    expectedResponseCode,
    expectedResponseBody
  }
}

const admin_user = "a.mester@mosbach.dhbw.de"
const admin_password = "12345678"

function login(email, password) {}

const nologin_tests = [
  Test(method.get,    "/user", {}, 401, {}),
  Test(method.patch,  "/user", {}, 405, {}),
  Test(method.put,    "/user", {}, 401, {}),
  Test(method.post,   "/user", {}, 405, {}),
  Test(method.delete, "/user", {}, 405, {}),
  Test(method.get,    "/user/groups", {}, 401, {}),
  Test(method.patch,  "/user/groups", {}, 405, {}),
  Test(method.put,    "/user/groups", {}, 405, {}),
  Test(method.post,   "/user/groups", {}, 405, {}),
  Test(method.delete, "/user/groups", {}, 405, {}),
  Test(method.get,    "/user/roles", {}, 401, {}),
  Test(method.patch,  "/user/roles", {}, 405, {}),
  Test(method.put,    "/user/roles", {}, 405, {}),
  Test(method.post,   "/user/roles", {}, 405, {}),
  Test(method.delete, "/user/roles", {}, 405, {}),
  Test(method.get,    "/users/1", {}, 401, {}),
  Test(method.patch,  "/users/1", {}, 401, {}),
  Test(method.put,    "/users/1", {}, 405, {}),
  Test(method.post,   "/users/1", {}, 405, {}),
  Test(method.delete, "/users/1", {}, 401, {}),
  Test(method.get,    "/users/5/groups", {}, 401, {}),
  Test(method.patch,  "/users/5/groups", {}, 405, {}),
  Test(method.put,    "/users/5/groups", {}, 401, {}),
  Test(method.post,   "/users/5/groups", {}, 401, {}),
  Test(method.delete, "/users/5/groups", {}, 401, {}),
  Test(method.get,    "/users/3/roles", {}, 401, {}),
  Test(method.patch,  "/users/3/roles", {}, 405, {}),
  Test(method.put,    "/users/3/roles", {}, 401, {}),
  Test(method.post,   "/users/3/roles", {}, 401, {}),
  Test(method.delete, "/users/3/roles", {}, 401, {}),
  Test(method.get,    "/roles", {}, 401, {}),
  Test(method.patch,  "/roles", {}, 405, {}),
  Test(method.put,    "/roles", {}, 405, {}),
  Test(method.post,   "/roles", {}, 401, {}),
  Test(method.delete, "/roles", {}, 405, {}),
  Test(method.get,    "/roles/4", {}, 401, {}),
  Test(method.patch,  "/roles/4", {}, 401, {}),
  Test(method.put,    "/roles/4", {}, 405, {}),
  Test(method.post,   "/roles/4", {}, 405, {}),
  Test(method.delete, "/roles/4", {}, 401, {}),
  Test(method.get,    "/roles/4/users", {}, 401, {}),
  Test(method.patch,  "/roles/4/users", {}, 405, {}),
  Test(method.put,    "/roles/4/users", {}, 401, {}),
  Test(method.post,   "/roles/4/users", {}, 401, {}),
  Test(method.delete, "/roles/4/users", {}, 401, {}),
  Test(method.get,    "/groups", {}, 401, {}),
  Test(method.patch,  "/groups", {}, 405, {}),
  Test(method.put,    "/groups", {}, 405, {}),
  Test(method.post,   "/groups", {}, 401, {}),
  Test(method.delete, "/groups", {}, 405, {}),
  Test(method.get,    "/groups/4", {}, 401, {}),
  Test(method.patch,  "/groups/4", {}, 401, {}),
  Test(method.put,    "/groups/4", {}, 405, {}),
  Test(method.post,   "/groups/4", {}, 405, {}),
  Test(method.delete, "/groups/4", {}, 401, {}),
  Test(method.get,    "/groups/4/users", {}, 401, {}),
  Test(method.patch,  "/groups/4/users", {}, 405, {}),
  Test(method.put,    "/groups/4/users", {}, 401, {}),
  Test(method.post,   "/groups/4/users", {}, 401, {}),
  Test(method.delete, "/groups/4/users", {}, 401, {}),
  Test(method.get,    "/login", {}, 405, {}),
  Test(method.patch,  "/login", {}, 405, {}),
  Test(method.put,    "/login", {}, 405, {}),
  Test(method.post,   "/login", {}, 400, undefined),
  Test(method.delete, "/login", {}, 405, {}),
  Test(method.post,   "/login", {
    email: admin_user,
    password: admin_password
  }, 201, {}),
]

const loggedin_tests = [

]
