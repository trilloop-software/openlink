import { createRouter, createWebHashHistory } from 'vue-router'

import Configure from '@/pages/Configure.vue'
import Dashboard from '@/pages/Dashboard.vue'
import Start from '@/pages/Start.vue'
import Manage from '@/pages/Manage.vue'
import Telemetry from '@/pages/Telemetry.vue'

export default createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            name: 'Configure',
            path: '/configure',
            component: Configure
        },
        {
            name: 'Dashboard',
            path: '/dashboard',
            component: Dashboard
        },
        {
            name: 'Start',
            path: '/start',
            component: Start
        },
        {
            name: 'Manage',
            path: '/manage',
            component: Manage
        },
        {
            name: 'Telemetry',
            path: '/telemetry',
            component: Telemetry
        },
        {
            path: '/.*',
            redirect: '/start',
        },
        {
            path: '/',
            redirect: '/start',
        },
    ]
})