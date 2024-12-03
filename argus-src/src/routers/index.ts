/**
 * Time:2024/12/3 09:33 23
 * Name:index.ts
 * Path:src/routers
 * ProjectName:argus-src
 * Author:charlatans
 *
 *  Il n'ya qu'un héroïsme au monde :
 *     c'est de voir le monde tel qu'il est et de l'aimer.
 */
import {createMemoryHistory, createRouter, createWebHistory} from 'vue-router'
import HomeView from "../views/HomeView.vue";
import NotFound from "../views/NotFound.vue";
import type {App} from 'vue';
import type {RouteMeta, RouteRecordRaw} from 'vue-router';

const error: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'nofound',
        component: HomeView,
        children: [],
    },
    {
        path: '/home',
        name: 'home',
        component: HomeView,
        children: [
            // {
            //     path: '403',
            //     name: '403',
            //     component: () => import('@/views/error/403.vue'),
            //     meta: { title: t('route.pathName.error403') },
            // },
            // {
            //     path: '404',
            //     name: '404',
            //     component: () => import('@/views/error/404.vue'),
            //     meta: { title: t('route.pathName.error404') },
            // },
            // {
            //     path: '500',
            //     name: '500',
            //     component: () => import('@/views/error/500.vue'),
            //     meta: { title: t('route.pathName.error500') },
            // },
        ],
    },

];


export const router = createRouter({
    history: createWebHistory(''),
    routes: error,
});

export const configMainRouter = async (app: App<Element>) => {
    app.use(router);
    await router.isReady();
};
