// import { redirect } from 'next/dist/server/api-utils'
import {createRouter, createWebHashHistory} from 'vue-router'

const routes = [
    { path: '/', alias: '/home', name: 'home', component: () => import('../views/HomeView.vue')},
    { path: '/auth/signin', name: 'signin', component: () => import('../views/AuthSignInView.vue')},
    { path: '/auth/signup', name: 'signup', component: () => import('../views/AuthSignUpView.vue')},
    { path: '/board/:userId', name: 'board', component: () => import('../views/BoardView.vue'), props: true},
    { path: '/about', name: 'about', component: () => import('../views/AboutView.vue')},
    { path: '/leaderboard', name:'leaderboard', component: () => import('../views/LeaderboardView.vue')}
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

export default router