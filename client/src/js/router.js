import Vue from 'vue';
import VueRouter from 'vue-router';

import Index from '../views/Index.vue';
import Autotagger from '../views/Autotagger.vue';
import AutotaggerTags from '../views/AutotaggerTags.vue';
import AutotaggerPlatform from '../views/AutotaggerPlatform.vue';
import AutotaggerAdvanced from '../views/AutotaggerAdvanced.vue';
import AutotaggerStatus from '../views/AutotaggerStatus.vue';
import QuickTag from '../views/QuickTag.vue';
import AudioFeatures from '../views/AudioFeatures.vue';

Vue.use(VueRouter);

const routes = [
  {
    path: '/',
    component: Index
  },
  {
    path: '/autotagger',
    component: Autotagger
  },
  {
    path: '/autotagger/2',
    component: AutotaggerTags
  },
  {
    path: '/autotagger/3',
    component: AutotaggerPlatform
  },
  {
    path: '/autotagger/4',
    component: AutotaggerAdvanced
  },
  {
    path: '/autotagger/status',
    component: AutotaggerStatus
  },
  {
    path: '/quicktag',
    component: QuickTag
  },
  {
    path: '/audiofeatures',
    component: AudioFeatures
  }
]

const router = new VueRouter({
  routes
});

export default router;
