import { config as dotenvConfig } from 'dotenv'
dotenvConfig()
import _ from 'lodash'
import { SQL, sql } from "bun"
const DONT_CARE = -1
const method = {
  get: "GET",
  patch: "PATCH",
  put: "PUT",
  post: "POST",
  delete: "DELETE"
}
// Test data (see src/test/example_data.sql)
const adminFirstname = "Admin"
const adminLastname = "Istrator"
const adminMail = "admin@noodle.de"
const adminPassword = "12345678"


const nologinTests = [
  Test(method.get, "/user", null, 401, {}),
  Test(method.patch, "/user", {}, 405, {}),
  Test(method.put, "/user", {}, 401, {}),
  Test(method.post, "/user", {}, 405, {}),
  Test(method.delete, "/user", {}, 405, {}),
  Test(method.get, "/user/groups", null, 401, {}),
  Test(method.patch, "/user/groups", {}, 405, {}),
  Test(method.put, "/user/groups", {}, 405, {}),
  Test(method.post, "/user/groups", {}, 405, {}),
  Test(method.delete, "/user/groups", {}, 405, {}),
  Test(method.get, "/user/roles", null, 401, {}),
  Test(method.patch, "/user/roles", {}, 405, {}),
  Test(method.put, "/user/roles", {}, 405, {}),
  Test(method.post, "/user/roles", {}, 405, {}),
  Test(method.delete, "/user/roles", {}, 405, {}),
  Test(method.get, "/users/1", null, 401, {}),
  Test(method.patch, "/users/1", {}, 401, {}),
  Test(method.put, "/users/1", {}, 405, {}),
  Test(method.post, "/users/1", {}, 405, {}),
  Test(method.delete, "/users/1", {}, 401, {}),
  Test(method.get, "/users/5/groups", null, 401, {}),
  Test(method.patch, "/users/5/groups", {}, 405, {}),
  Test(method.put, "/users/5/groups", {}, 401, {}),
  Test(method.post, "/users/5/groups", {}, 401, {}),
  Test(method.delete, "/users/5/groups", {}, 401, {}),
  Test(method.get, "/users/3/roles", null, 401, {}),
  Test(method.patch, "/users/3/roles", {}, 405, {}),
  Test(method.put, "/users/3/roles", {}, 401, {}),
  Test(method.post, "/users/3/roles", {}, 401, {}),
  Test(method.delete, "/users/3/roles", {}, 401, {}),
  Test(method.get, "/roles", null, 401, {}),
  Test(method.patch, "/roles", {}, 405, {}),
  Test(method.put, "/roles", {}, 405, {}),
  Test(method.post, "/roles", {}, 401, {}),
  Test(method.delete, "/roles", {}, 405, {}),
  Test(method.get, "/roles/4", null, 401, {}),
  Test(method.patch, "/roles/4", {}, 401, {}),
  Test(method.put, "/roles/4", {}, 405, {}),
  Test(method.post, "/roles/4", {}, 405, {}),
  Test(method.delete, "/roles/4", {}, 401, {}),
  Test(method.get, "/roles/4/users", null, 401, {}),
  Test(method.patch, "/roles/4/users", {}, 405, {}),
  Test(method.put, "/roles/4/users", {}, 401, {}),
  Test(method.post, "/roles/4/users", {}, 401, {}),
  Test(method.delete, "/roles/4/users", {}, 401, {}),
  Test(method.get, "/groups", null, 401, {}),
  Test(method.patch, "/groups", {}, 405, {}),
  Test(method.put, "/groups", {}, 405, {}),
  Test(method.post, "/groups", {}, 401, {}),
  Test(method.delete, "/groups", {}, 405, {}),
  Test(method.get, "/groups/4", null, 401, {}),
  Test(method.patch, "/groups/4", {}, 401, {}),
  Test(method.put, "/groups/4", {}, 405, {}),
  Test(method.post, "/groups/4", {}, 405, {}),
  Test(method.delete, "/groups/4", {}, 401, {}),
  Test(method.get, "/groups/4/users", null, 401, {}),
  Test(method.patch, "/groups/4/users", {}, 405, {}),
  Test(method.put, "/groups/4/users", {}, 401, {}),
  Test(method.post, "/groups/4/users", {}, 401, {}),
  Test(method.delete, "/groups/4/users", {}, 401, {}),
  Test(method.get, "/login", null, 405, {}),
  Test(method.patch, "/login", {}, 405, {}),
  Test(method.put, "/login", {}, 405, {}),
  Test(method.post, "/login", {}, 400, DONT_CARE),
  Test(method.delete, "/login", {}, 405, {}),
]

const users = [
  {
    userId: 1,
    firstname: adminFirstname,
    lastname: adminLastname,
    email: adminMail,
    password: adminPassword
  },
  {
    userId: DONT_CARE,
    firstname: "Firstname1",
    lastname: "Lastname1",
    email: "Invalid",
    password: "1"
  },
  {
    userId: 2,
    firstname: "Firstname2",
    lastname: "Lastname2",
    email: "mail@mail.com",
    password: "SecurePassword!1234"
  }
]

const loggedinTests = [
  Test(method.get, "/user", null, 200,
    {
      userId: DONT_CARE,
      firstname: adminFirstname,
      lastname: adminLastname,
      email: adminMail
    }),
  Test(method.post, "/user", {
    firstname: users[1].firstname,
    lastname: users[1].lastname,
    email: users[1].email,
    password: users[1].password
  }, 400,
    {
      email: {
        tooShort: false,
        tooLong: false,
        illegalChar: false,
        invalidFormat: true
      },
      password: {
        tooShort: true,
        uppercaseMissing: true,
        lowercaseMissing: true,
        digitMissing: false,
        specialMissing: true
      }
    }),
  Test(method.get, "/user/groups", null, 200, []),
  Test(method.get, "/user/roles", null, 200, []),
  Test(method.get, `/users/${users[0].userId}`, null, 200,
    {
      userId: users[0].userId,
      firstname: users[0].firstname,
      lastname: users[0].lastname,
      email: users[0].email
    }),
]

async function runTests() {
  let failedTests = []

  for (const test of nologinTests) {
    let result = test.run()
    if (result.failed) {
      failedTests.push(
        {
          expected: result.expected,
          actual: result.actual,
          request: test
        }
      )
    }
  }

  await login(adminMail, adminPassword)

  for (const test of loggedinTests) {
    let result = await test.run()
    if (result.failed) {
      failedTests.push(
        {
          expected: result.expected,
          actual: result.actual,
          request: test
        }
      )
    }
  }

  console.log(`Ran ${nologinTests.length + loggedinTests.length} Tests. ${failedTests.length} failed.`)
  for (const test of failedTests) {
    console.log(`Test ${test.request.method} ${test.request.url}
\texpected:
\tResponse Code: ${test.expected.responseCode}
\tResponse Body: ${test.expected.responseBody}
\tactual:
\tResponse Code: ${test.actual.responseCode}
\tResponse Body: ${test.actual.responseBody}`)
  }
}

const BASE_URL = "http://localhost:3000"

let sessionCookie = null

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
    expectedResponseBody,
    run: async function() {
      const self = this
      let requestOpts = {
        method: self.method,
        body: requestBody,
        headers: {
          "Cookie": sessionCookie,
          "Content-Type": "application/json"
        }
      }

      if (self.requestBody === null) {
        requestOpts.headers['Content-Type'] = undefined
      } else {
        requestOpts.body = JSON.stringify(requestBody)
      }

      let response = await fetch(BASE_URL + self.url, requestOpts)
      let result = {
        failed: false, expected: {
          responseCode: self.expectedResponseCode,
          responseBody: typeof self.expectedResponseBody == 'object' ? JSON.stringify(self.expectedResponseBody) : self.expectedResponseBody
        }, actual: {
          responseCode: self.expectedResponseCode,
          responseBody: typeof self.expectedResponseBody == 'object' ? JSON.stringify(self.expectedResponseBody) : self.expectedResponseBody
        }
      }
      const status = response.status
      let body = null

      try {
        body = await response.json()
      } catch { }

      let bodyMatches = true
      let statusMatches = self.expectedResponseCode == DONT_CARE || self.expectedResponseCode == status

      if (self.expectedResponseBody != DONT_CARE) {
        if (self.expectedResponseBody != null && body != null) {
          const expectedKeys = Object.keys(self.expectedResponseBody)
          const actualKeys = Object.keys(body)

          bodyMatches = _.isEqual(expectedKeys, actualKeys)
          if (bodyMatches) {
            for (const k of expectedKeys) {
              if (self.expectedResponseBody[k] != DONT_CARE && !_.isEqual(self.expectedResponseBody[k], body[k])) {
                console.log("bodies dont match")
                bodyMatches = false
                break
              }
            }
          }
        } else {
          bodyMatches = self.expectedResponseBody == body
        }
      }

      if (!bodyMatches || !statusMatches) {
        result.failed = true
        result.actual.responseCode = status
        result.actual.responseBody = typeof body == 'object' ? JSON.stringify(body) : body
      }

      return result
    }
  }
}


async function login(email, password) {
  const response = await fetch(`${BASE_URL}/login`, {
    method: "POST",
    body: JSON.stringify({ email, password }),
    headers: {
      "Content-Type": "application/json"
    }
  })
  if (response.status != 201) {
    console.log("login failed!")
    console.log(await response.text())
    process.exit(1)
  }
  sessionCookie = response.headers.get("set-cookie")
}

const db = new SQL({
  url: process.env.PG_TEST_URL,
  max: 20,
  idleTimeout: 30,
  maxLifetime: 0,
  connectionTimeout: 30
})

await db.file("./src/web/db/setup.sql")
await db.file("./src/test/example_data.sql")

runTests()
