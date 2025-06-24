<template>
  <nav class="flex gap-3 ">
    <RouterLink to="/login">Login</RouterLink>
    <RouterLink to="/">Go to Home</RouterLink>
    <RouterLink to="/users"> Users</RouterLink>
    <RouterLink to="/groups">Groups</RouterLink>
  </nav>
  <main class="grow h-1">
    <RouterView />
  </main>
</template>
