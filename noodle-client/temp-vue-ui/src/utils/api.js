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
    body: JSON.stringify({ email, password })
  })

  return r.status
}

export async function fetchSelf() {
  const r = await fetch(`${baseUrl}/user`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function createUser(firstname, lastname, email, password) {
  const r = await fetch(`${baseUrl}/user`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({ firstname, lastname, email, password })
  })
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function fetchSelfGroups() {
  const r = await fetch(`${baseUrl}/user/groups`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function fetchSelfRoles() {
  const r = await fetch(`${baseUrl}/user/roles`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function fetchAllUsers() {
  const r = await fetch(`${baseUrl}/users`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function fetchUser(id) {
  const r = await fetch(`${baseUrl}/users/${id}`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function updateUser(userId, firstname, lastname, email) {
  const r = await fetch(`${baseUrl}/users/${userId}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({ userId, firstname, lastname, email })
  })
  if (r.status === 201 || r.status === 400)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function deleteUser(id) {
  const r = await fetch(`${baseUrl}/users/${id}`, {
    method: "DELETE",
  })

  return r.status
}

export async function fetchUserGroups(userId) {
  const r = await fetch(`${baseUrl}/users/${userId}/groups`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function replaceUserGroups(userId, groupIds) {
  const r = await fetch(`${baseUrl}/users/${userId}/groups`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(groupIds)
  })

  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function addUserToGroups(userId, groupIds) {
  const r = await fetch(`${baseUrl}/users/${userId}/groups`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(groupIds)
  })

  return r.status
}

export async function removeUserFromGroups(userId, groupIds) {
  const r = await fetch(`${baseUrl}/users/${userId}/groups`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(groupIds)
  })

  return r.status
}

export async function fetchUserRoles(userId) {
  const r = await fetch(`${baseUrl}/users/${userId}/roles`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function replaceUserRoles(userId, roleIds) {
  const r = await fetch(`${baseUrl}/users/${userId}/roles`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(roleIds)
  })

  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function addUserToRoles(userId, roleIds) {
  const r = await fetch(`${baseUrl}/users/${userId}/roles`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(roleIds)
  })

  return r.status
}

export async function removeUserFromRoles(userId, roleIds) {
  const r = await fetch(`${baseUrl}/users/${userId}/roles`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(roleIds)
  })

  return r.status
}

export async function fetchRoles() {
  const r = await fetch(`${baseUrl}/roles`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function createRole(name, permissions) {
  const r = await fetch(`${baseUrl}/roles`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      name, permissions
    })
  })

  if (r.status === 201)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function fetchRole(id) {
  const r = await fetch(`${baseUrl}/roles/${id}`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function updateRole(roleId, name, permissions) {
  const r = await fetch(`${baseUrl}/roles/${roleId}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      name, permissions
    })
  })

  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function deleteRole(id) {
  const r = await fetch(`${baseUrl}/roles/${id}`, {
    method: "DELETE"
  })

  return r.status
}

export async function fetchUsersOfRole(id) {
  const r = await fetch(`${baseUrl}/roles/${id}/users`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function replaceUsersOfRole(id, userIds) {
  const r = await fetch(`${baseUrl}/roles/${id}/users`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(userIds)
  })
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function addUsersToRole(id, userIds) {
  const r = await fetch(`${baseUrl}/roles/${id}/users`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(userIds)
  })

  return r.status
}

export async function removeUsersFromRole(id, userIds) {
  const r = await fetch(`${baseUrl}/roles/${id}/users`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(userIds)
  })

  return r.status
}

export async function fetchGroups() {
  const r = await fetch(`${baseUrl}/groups`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function createGroup(name, kind, parent) {
  const r = await fetch(`${baseUrl}/roles`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      name, kind, parent
    })
  })

  if (r.status === 201)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function fetchGroup(id) {
  const r = await fetch(`${baseUrl}/groups/${id}`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function updateGroup(id, name, kind, parent) {
  const r = await fetch(`${baseUrl}/groups/${id}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      name, kind, parent
    })
  })

  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function deleteGroup(id) {
  const r = await fetch(`${baseUrl}/groups/${id}`, {
    method: "DELETE"
  })

  return r.status
}

export async function fetchUsersOfGroup(id) {
  const r = await fetch(`${baseUrl}/groups/${id}/users`)
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function replaceUsersOfGroup(id, userIds) {
  const r = await fetch(`${baseUrl}/groups/${id}/users`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(userIds)
  })
  if (r.status === 200)
    return Response(r.status, await r.json())

  return Response(r.status, null)
}

export async function addUsersToGroup(id, userIds) {
  const r = await fetch(`${baseUrl}/groups/${id}/users`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(userIds)
  })

  return r.status
}

export async function removeUsersFromGroup(id, userIds) {
  const r = await fetch(`${baseUrl}/groups/${id}/users`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(userIds)
  })

  return r.status
}


function Response(status, body) {
  return {
    status,
    body,
    success: function() {
      return this.status > 199 && this.status < 400
    }
  }
}
