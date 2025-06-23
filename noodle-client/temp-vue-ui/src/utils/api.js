const baseUrl = "/api"

/**
* @return if login was successful `true` else `false`.
*/
export async function login(email, password) {
  const r = await fetch(`${baseUrl}/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify( {email, password} )
  })

  return r.status === 201
}

