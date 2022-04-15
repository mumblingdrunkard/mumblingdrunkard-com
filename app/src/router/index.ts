import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/contact',
    name: 'contact',
    component: () => import('../views/ContactView.vue')
  },
  {
    path: '/blog',
    name: 'blog',
    component: () => import('../views/BlogView.vue')
  },
  {
    path: '/blog/:slug',
    name: 'blog-post',
    component: () => import('../views/PostView.vue')
  }
]

const router = createRouter({
  history: createWebHistory("/"),
  routes
})

export default router
