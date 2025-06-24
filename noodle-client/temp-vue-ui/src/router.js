import { createWebHistory, createRouter } from 'vue-router'


import Dashbord from './pages/Dashbord.vue'
import Login from './pages/Login.vue'
import Users from './pages/Users.vue'
import Groups from './pages/Groups.vue'

const routes = [
  { path: '/', component: Dashbord },
  { path: '/login', component: Login },
  { path: '/users', component: Users},
  { path: '/groups', component: Groups},
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
