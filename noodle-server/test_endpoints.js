import { config as dotenvConfig } from 'dotenv'
dotenvConfig()
import _ from 'lodash'
import { SQL } from "bun"
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

// Kurs- und Template-Testdaten müssen VOR ihrer ersten Verwendung definiert sein
const courses = [
  {
    courseId: 1,
    name: "Course 1"
  },
  {
    courseId: 1,
    name: "Course 1 updated"
  }
]

const templatesData = [
  {
    templateId: 1,
    name: "Template 1"
  },
  {
    templateId: 1,
    name: "Template 1 updated"
  }
]

const sections = [
  {
    sectionId: 1,
    headline: "Section 1",
    orderIndex: 0,
    parentCourseId: courses[0].courseId
  }
]

const nologinTests = [
  Test(method.get, "/user", null, 401, null),
  Test(method.patch, "/user", {}, 405, null),
  Test(method.put, "/user", {}, 401, null),
  Test(method.post, "/user", {}, 405, null),
  Test(method.delete, "/user", {}, 405, null),
  Test(method.get, "/user/groups", null, 401, null),
  Test(method.patch, "/user/groups", {}, 405, null),
  Test(method.put, "/user/groups", {}, 405, null),
  Test(method.post, "/user/groups", {}, 405, null),
  Test(method.delete, "/user/groups", {}, 405, null),
  Test(method.get, "/user/roles", null, 401, null),
  Test(method.patch, "/user/roles", {}, 405, null),
  Test(method.put, "/user/roles", {}, 405, null),
  Test(method.post, "/user/roles", {}, 405, null),
  Test(method.delete, "/user/roles", {}, 405, null),
  Test(method.get, "/users/1", null, 401, null),
  Test(method.patch, "/users/1", {}, 401, null),
  Test(method.put, "/users/1", {}, 405, null),
  Test(method.post, "/users/1", {}, 405, null),
  Test(method.delete, "/users/1", {}, 401, null),
  Test(method.get, "/users/5/groups", null, 401, null),
  Test(method.patch, "/users/5/groups", {}, 405, null),
  Test(method.put, "/users/5/groups", {}, 401, null),
  Test(method.post, "/users/5/groups", {}, 401, null),
  Test(method.delete, "/users/5/groups", {}, 401, null),
  Test(method.get, "/users/3/roles", null, 401, null),
  Test(method.patch, "/users/3/roles", {}, 405, null),
  Test(method.put, "/users/3/roles", {}, 401, null),
  Test(method.post, "/users/3/roles", {}, 401, null),
  Test(method.delete, "/users/3/roles", {}, 401, null),
  Test(method.get, "/roles", null, 401, null),
  Test(method.patch, "/roles", {}, 405, null),
  Test(method.put, "/roles", {}, 405, null),
  Test(method.post, "/roles", {}, 401, null),
  Test(method.delete, "/roles", {}, 405, null),
  Test(method.get, "/roles/4", null, 401, null),
  Test(method.patch, "/roles/4", {}, 401, null),
  Test(method.put, "/roles/4", {}, 405, null),
  Test(method.post, "/roles/4", {}, 405, null),
  Test(method.delete, "/roles/4", {}, 401, null),
  Test(method.get, "/roles/4/users", null, 401, null),
  Test(method.patch, "/roles/4/users", {}, 405, null),
  Test(method.put, "/roles/4/users", {}, 401, null),
  Test(method.post, "/roles/4/users", {}, 401, null),
  Test(method.delete, "/roles/4/users", {}, 401, null),
  Test(method.get, "/groups", null, 401, null),
  Test(method.patch, "/groups", {}, 405, null),
  Test(method.put, "/groups", {}, 405, null),
  Test(method.post, "/groups", {}, 401, null),
  Test(method.delete, "/groups", {}, 405, null),
  Test(method.get, "/groups/4", null, 401, null),
  Test(method.patch, "/groups/4", {}, 401, null),
  Test(method.put, "/groups/4", {}, 405, null),
  Test(method.post, "/groups/4", {}, 405, null),
  Test(method.delete, "/groups/4", {}, 401, null),
  Test(method.get, "/groups/4/users", null, 401, null),
  Test(method.patch, "/groups/4/users", {}, 405, null),
  Test(method.put, "/groups/4/users", {}, 401, null),
  Test(method.post, "/groups/4/users", {}, 401, null),
  Test(method.delete, "/groups/4/users", {}, 401, null),
  Test(method.get, "/login", null, 405, null),
  Test(method.patch, "/login", {}, 405, null),
  Test(method.put, "/login", {}, 405, null),
  Test(method.post, "/login", {}, 400, DONT_CARE),
  Test(method.delete, "/login", {}, 405, null),
  Test(method.get, "/courses", null, 401, null),
  Test(method.post, "/courses", { name: courses[0].name }, 401, null),
  Test(method.get, "/course/1", null, 401, null),
  Test(method.put, "/course/1", { name: courses[1].name }, 401, null),
  Test(method.delete, "/course/1", null, 401, null),
  Test(method.get, "/templates", null, 401, null),
  Test(method.post, "/templates", { name: templatesData[0].name }, 401, null),
  Test(method.get, "/template/1", null, 401, null),
  Test(method.put, "/template/1", { name: templatesData[1].name }, 401, null),
  Test(method.delete, "/template/1", null, 401, null)
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

const groups = [
  {
    groupId: DONT_CARE,
    name: "Programming",
    kind: "lol",
    parent: null
  },
  {
    groupId: 1,
    name: "Programming",
    kind: "learning",
    parent: null
  },
  {
    groupId: DONT_CARE,
    name: "Programming",
    kind: "learning",
    parent: null
  },
  {
    groupId: DONT_CARE,
    name: "Design",
    kind: "learning",
    parent: 3
  },
  {
    groupId: 3,
    name: "Design",
    kind: "learning",
    parent: 1
  }
]

const roles = [
  {
    roleId: 1,
    name: "Dozent",
    permissions: [],
    group: 4,
  },
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
  Test(method.post, "/user", {
    firstname: users[2].firstname,
    lastname: users[2].lastname,
    email: users[2].email,
    password: users[2].password
  }, 201, {
    userId: users[2].userId,
    firstname: users[2].firstname,
    lastname: users[2].lastname,
    email: users[2].email
  }),
  Test(method.get, `/users/${users[2].userId}`, null, 200,
    {
      userId: users[2].userId,
      firstname: users[2].firstname,
      lastname: users[2].lastname,
      email: users[2].email
    }),
  Test(method.post, "/groups", {
    name: groups[0].name,
    kind: groups[0].kind,
    parent: groups[0].parent
  }, 422, DONT_CARE),
  Test(method.post, "/groups", {
    name: groups[1].name,
    kind: groups[1].kind,
    parent: groups[1].parent
  }, 201, {
    groupId: groups[1].groupId,
    name: groups[1].name,
    kind: groups[1].kind,
    parent: groups[1].parent
  }),
  Test(method.post, "/groups", {
    name: groups[2].name,
    kind: groups[2].kind,
    parent: groups[2].parent
  }, 409, DONT_CARE),
  Test(method.post, "/groups", {
    name: groups[3].name,
    kind: groups[3].kind,
    parent: groups[3].parent
  }, 500, DONT_CARE), //TODO: This shoul return 400, fix in another PR eventually.
  Test(method.post, "/groups", {
    name: groups[4].name,
    kind: groups[4].kind,
    parent: groups[4].parent
  }, 201, {
    groupId: groups[4].groupId,
    name: groups[4].name,
    kind: groups[4].kind,
    parent: groups[4].parent
  }),
  //deleting parent group causes child groups to be deleted too.
  Test(method.delete, `/groups/${groups[1].groupId}`, null, 200, null),
  Test(method.get, `/groups/${groups[1].groupId}`, null, 404, null),
  Test(method.get, `/groups/${groups[3].groupId}`, null, 404, null),
  Test(method.post, "/roles", {
    name: roles[0].name,
    permissions: roles[0].permissions
  }, 201, {
    roleId: roles[0].roleId,
    name: roles[0].name,
    permissions: roles[0].permissions,
  }),
  Test(method.get, `/groups/${roles[0].group}`, null, 200, {
    groupId: roles[0].group,
    name: roles[0].name,
    kind: "role",
    parent: null
  }),
  Test(method.post, `/groups/${groups[4].groupId}/users`, [1], 201, DONT_CARE),
  Test(method.get, `/groups/${groups[4].groupId}/users`, null, 200, [{
    userId: users[0].userId,
    firstname: users[0].firstname,
    lastname: users[0].lastname,
    email: users[0].email,
  }]),
  Test(method.post, `/groups/${groups[4].groupId}/users`, [2], 201, DONT_CARE),
  Test(method.get, `/groups/${groups[4].groupId}/users`, null, 200, [
    {
      userId: users[0].userId,
      firstname: users[0].firstname,
      lastname: users[0].lastname,
      email: users[0].email,
    },
    {
      userId: users[2].userId,
      firstname: users[2].firstname,
      lastname: users[2].lastname,
      email: users[2].email,
    }
  ]),
  Test(method.post, "/roles", {
    name: roles[0].name,
    permissions: roles[0].permissions
  }, 409, DONT_CARE),
  Test(method.post, `/roles/${roles[0].roleId}/users`, [users[0].userId], 201, DONT_CARE),
  Test(method.get, `/roles/${roles[0].roleId}/users`, null, 200, [{
    userId: users[0].userId,
    firstname: users[0].firstname,
    lastname: users[0].lastname,
    email: users[0].email,
  }]),
  Test(method.get, `/groups/${roles[0].group}/users`, null, 200, [{
    userId: users[0].userId,
    firstname: users[0].firstname,
    lastname: users[0].lastname,
    email: users[0].email,
  }]),
  Test(method.post, `/roles/${roles[0].roleId}/users`, [users[2].userId], 201, DONT_CARE),
  Test(method.get, `/roles/${roles[0].roleId}/users`, null, 200, [
    {
      userId: users[0].userId,
      firstname: users[0].firstname,
      lastname: users[0].lastname,
      email: users[0].email,
    },
    {
      userId: users[2].userId,
      firstname: users[2].firstname,
      lastname: users[2].lastname,
      email: users[2].email,
    }
  ]),
  Test(method.get, `/groups/${roles[0].group}/users`, null, 200, [
    {
      userId: users[0].userId,
      firstname: users[0].firstname,
      lastname: users[0].lastname,
      email: users[0].email,
    },
    {
      userId: users[2].userId,
      firstname: users[2].firstname,
      lastname: users[2].lastname,
      email: users[2].email,
    }
  ]),
  Test(method.delete, `/roles/${roles[0].roleId}/users`, [users[2].userId], 200, DONT_CARE),
  Test(method.get, `/roles/${roles[0].roleId}/users`, null, 200, [{
    userId: users[0].userId,
    firstname: users[0].firstname,
    lastname: users[0].lastname,
    email: users[0].email,
  }]),
  Test(method.get, `/groups/${roles[0].group}/users`, null, 200, [{
    userId: users[0].userId,
    firstname: users[0].firstname,
    lastname: users[0].lastname,
    email: users[0].email,
  }]),
  Test(method.post, `/users/${users[2].userId}/roles`, [roles[0].roleId], 201, DONT_CARE),
  Test(method.get, `/users/${users[2].userId}/groups`, null, 200, [{
    groupId: roles[0].group,
    name: roles[0].name,
    kind: "role",
    parent: null
  }]),
  Test(method.get, `/users/${users[2].userId}/roles`, null, 200, [{
    roleId: roles[0].roleId,
    name: roles[0].name,
    permissions: [],
    group: roles[0].group
  }]),
  Test(method.delete, `/users/${users[2].userId}/roles`, [roles[0].roleId], 200, DONT_CARE),
  Test(method.get, `/users/${users[2].userId}/groups`, null, 200, []),
  Test(method.get, `/users/${users[2].userId}/roles`, null, 200, []),
  Test(method.post, `/users/${users[2].userId}/groups`, [groups[4].groupId], 201, DONT_CARE),
  Test(method.get, `/users/${users[2].userId}/groups`, null, 200, [{
    groupId: groups[4].groupId,
    name: groups[4].name,
    kind: groups[4].kind,
    parent: null
  }]),
  Test(method.get, "/courses", null, 200, []),
  Test(method.post, "/courses", { name: courses[0].name }, 200, {
    courseId: courses[0].courseId,
    name: courses[0].name
  }),
  Test(method.get, "/courses", null, 200, [{
    courseId: courses[0].courseId,
    name: courses[0].name
  }]),
  Test(method.get, `/course/${courses[0].courseId}`, null, 200, {
    courseId: courses[0].courseId,
    name: courses[0].name
  }),
  Test(method.put, `/course/${courses[1].courseId}`, { name: courses[1].name }, 200, {
    courseId: courses[1].courseId,
    name: courses[1].name
  }),
  Test(method.get, `/course/${courses[1].courseId}`, null, 200, {
    courseId: courses[1].courseId,
    name: courses[1].name
  }),
  Test(method.get, `/course/${courses[1].courseId}/section/${sections[0].sectionId}/content`, null, 200, []),
  Test(method.delete, `/course/${courses[1].courseId}`, null, 200, null),
  Test(method.get, `/course/${courses[1].courseId}`, null, 404, null),
  Test(method.get, "/courses", null, 200, []),
  Test(method.get, "/templates", null, 200, []),
  Test(method.post, "/templates", { name: templatesData[0].name }, 200, {
    templateId: templatesData[0].templateId,
    name: templatesData[0].name
  }),
  Test(method.get, "/templates", null, 200, [{
    templateId: templatesData[0].templateId,
    name: templatesData[0].name
  }]),
  Test(method.get, `/template/${templatesData[0].templateId}`, null, 200, {
    templateId: templatesData[0].templateId,
    name: templatesData[0].name
  }),
  Test(method.put, `/template/${templatesData[1].templateId}`, { name: templatesData[1].name }, 200, {
    templateId: templatesData[1].templateId,
    name: templatesData[1].name
  }),
  Test(method.get, `/template/${templatesData[1].templateId}`, null, 200, {
    templateId: templatesData[1].templateId,
    name: templatesData[1].name
  }),
  Test(method.delete, `/template/${templatesData[1].templateId}`, null, 200, null),
  Test(method.get, `/template/${templatesData[1].templateId}`, null, 404, null),
  Test(method.get, "/templates", null, 200, [])
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
          responseBody: typeof self.expectedResponseBody == 'object' ? JSON.stringify(self.expectedResponseBody, null, 2) : self.expectedResponseBody
        }, actual: {
          responseCode: self.expectedResponseCode,
          responseBody: typeof self.expectedResponseBody == 'object' ? JSON.stringify(self.expectedResponseBody, null, 2) : self.expectedResponseBody
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
        result.actual.responseBody = typeof body == 'object' ? JSON.stringify(body, null, 2) : body
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

const sql = new SQL({
  url: process.env.PG_TEST_URL,
  max: 20,
  idleTimeout: 30,
  maxLifetime: 0,
  connectionTimeout: 30
})

await sql.file("./src/test/reset_tables.sql")
await sql.file("./src/test/example_data.sql")

runTests()
