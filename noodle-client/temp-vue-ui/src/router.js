import { createWebHistory, createRouter } from 'vue-router'


import Dashbord from './pages/Dashbord.vue'
import Login from './pages/Login.vue'

const routes = [
  { path: '/', component: Dashbord },
  { path: '/login', component: Login },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
