<script lang="ts" setup>
import routes from '@/router/routes';
</script>

<template>
  <q-scroll-area class="fit">
    <q-list class="rounded-borders text-bold" padding>
      <q-separator />
      <template v-for="(routeItem, _routeIndex) in routes" :key="_routeIndex">
        <q-item class="r-no-sel" v-ripple :active="false" :to="`/${routeItem.path}`" clickable>
          <q-item-section avatar>
            <q-icon :name="routeItem.meta.icon" />
          </q-item-section>
          <q-item-section>
            {{ routeItem.meta.title }}
          </q-item-section>
        </q-item>
        <q-separator />
      </template>
    </q-list>
  </q-scroll-area>
</template>
