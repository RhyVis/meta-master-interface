<script lang="ts" setup>
import routes from '@/router/routes';
</script>

<template>
  <q-scroll-area class="fit">
    <q-list>
      <template v-for="(routeItem, routeIndex) in routes" :key="routeIndex">
        <q-item class="r-no-sel" v-ripple :to="`/${routeItem.path}`" clickable>
          <q-item-section avatar>
            <q-icon :name="routeItem.meta.icon" />
          </q-item-section>
          <q-item-section>
            {{ routeItem.meta.title }}
          </q-item-section>
          <q-separator v-if="routeIndex < routes.length - 1" :key="`sep${routeIndex}`" />
        </q-item>
      </template>
    </q-list>
  </q-scroll-area>
</template>
