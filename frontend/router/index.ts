import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '@/views/home/Home.vue';
import App from '@/views/app/Home.vue';
import NNS from '@/views/app/modules/NNS.vue';
import Transactions from '@/views/app/modules/Transactions.vue';
import Wallet from '@/views/app/modules/Wallet.vue';
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
        children: [
            {path: '', component: Wallet},
            {path: 'transactions/:address', component: Transactions},
            {path: 'nns', component: NNS}],
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
