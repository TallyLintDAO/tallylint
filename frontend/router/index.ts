import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '@/views/home/Home.vue';
import App from '@/views/nns/Home.vue';
import NNS from '@/views/nns/modules/nns.vue';
import errors from './modules/errors';

const routes: Array<RouteRecordRaw> = [
    {
        path: '/home',
        name: 'Home',
        component: Home,
    },
    {
        path: '/',
        name: 'App',
        component: App,
        children: [{path: 'nns', component: NNS,}],
    },
    ...errors,
    {
        path: '/:catchAll(.*)',
        redirect: '/error/404',
    },
];

const router = createRouter({
    history: createWebHistory('/'),
    routes,
});

export default router;
