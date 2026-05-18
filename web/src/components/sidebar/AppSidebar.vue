<template>
  <VaSidebar v-model="writableVisible" :width="sidebarWidth" :color="color" minimized-width="0">
    <VaAccordion v-if="$props.isAdmin" v-model="value" multiple>
      <VaCollapse v-for="(item, index) in adminNavigationRoutes.routes" :key="index">
        <template #header="{ value: isCollapsed }">
          <VaSidebarItem
            :to="item.children ? undefined : { name: item.name }"
            :active="routeHasActiveChild(item)"
            :active-color="activeColor"
            :text-color="textColor(item)"
            role="button"
            hover-opacity="0.10"
          >
            <VaSidebarItemContent class="py-3 pr-2 pl-4">
              <VaIcon
                v-if="item.meta.icon"
                aria-hidden="true"
                :name="item.meta.icon"
                size="20px"
                :color="iconColor(item)"
              />
              <VaSidebarItemTitle class="flex justify-between items-center leading-5 font-semibold">
                {{ t(item.displayName) }}
                <VaIcon v-if="item.children" :name="arrowDirection(isCollapsed)" size="20px" />
              </VaSidebarItemTitle>
            </VaSidebarItemContent>
          </VaSidebarItem>
        </template>
        <template #body>
          <div v-for="(childRoute, index2) in item.children" :key="index2">
            <VaSidebarItem
              :to="{ name: childRoute.name }"
              :active="isActiveChildRoute(childRoute)"
              :active-color="activeColor"
              :text-color="textColor(childRoute)"
              hover-opacity="0.10"
            >
              <VaSidebarItemContent class="py-3 pr-2 pl-11">
                <VaSidebarItemTitle class="leading-5 font-semibold">
                  {{ t(childRoute.displayName) }}
                </VaSidebarItemTitle>
              </VaSidebarItemContent>
            </VaSidebarItem>
          </div>
        </template>
      </VaCollapse>
      <VaCollapse>
        <template #header>
          <VaSidebarItem
            :to="{ name: 'dashboard' }"
            :active="false"
            :active-color="activeColor"
            text-color="textPrimary"
            role="button"
            hover-opacity="0.10"
          >
            <VaSidebarItemContent class="py-3 pr-2 pl-4">
              <VaIcon
                aria-hidden="true"
                :name="$props.isAdmin ? 'exit_to_app' : 'manage_accounts'"
                size="20px"
                color="secondary"
              />
              <VaSidebarItemTitle class="flex justify-between items-center leading-5 font-semibold">
                {{ t('menu.user_panel') }}
              </VaSidebarItemTitle>
            </VaSidebarItemContent>
          </VaSidebarItem>
        </template>
      </VaCollapse>
    </VaAccordion>
    <VaAccordion v-else v-model="value" multiple>
      <VaCollapse v-for="(item, index) in userNavigationRoutes.routes" :key="index">
        <template #header="{ value: isCollapsed }">
          <VaSidebarItem
            :to="item.children ? undefined : { name: item.name }"
            :active="routeHasActiveChild(item)"
            :active-color="activeColor"
            :text-color="textColor(item)"
            role="button"
            hover-opacity="0.10"
          >
            <VaSidebarItemContent class="py-3 pr-2 pl-4">
              <VaIcon
                v-if="item.meta.icon"
                aria-hidden="true"
                :name="item.meta.icon"
                size="20px"
                :color="iconColor(item)"
              />
              <VaSidebarItemTitle class="flex justify-between items-center leading-5 font-semibold">
                {{ t(item.displayName) }}
                <VaIcon v-if="item.children" :name="arrowDirection(isCollapsed)" size="20px" />
              </VaSidebarItemTitle>
            </VaSidebarItemContent>
          </VaSidebarItem>
        </template>
        <template #body>
          <div v-for="(childRoute, index2) in item.children" :key="index2">
            <VaSidebarItem
              :to="{ name: childRoute.name }"
              :active="isActiveChildRoute(childRoute)"
              :active-color="activeColor"
              :text-color="textColor(childRoute)"
              hover-opacity="0.10"
            >
              <VaSidebarItemContent class="py-3 pr-2 pl-11">
                <VaSidebarItemTitle class="leading-5 font-semibold">
                  {{ t(childRoute.displayName) }}
                </VaSidebarItemTitle>
              </VaSidebarItemContent>
            </VaSidebarItem>
          </div>
        </template>
      </VaCollapse>
      <VaCollapse v-if="authStore.isAdmin">
        <template #header>
          <VaSidebarItem
            :to="{ name: 'admin_dashboard' }"
            :active="false"
            :active-color="activeColor"
            text-color="textPrimary"
            role="button"
            hover-opacity="0.10"
          >
            <VaSidebarItemContent class="py-3 pr-2 pl-4">
              <VaIcon
                aria-hidden="true"
                :name="$props.isAdmin ? 'exit_to_app' : 'manage_accounts'"
                size="20px"
                color="secondary"
              />
              <VaSidebarItemTitle class="flex justify-between items-center leading-5 font-semibold">
                {{ t('menu.admin_panel') }}
              </VaSidebarItemTitle>
            </VaSidebarItemContent>
          </VaSidebarItem>
        </template>
      </VaCollapse>
    </VaAccordion>
  </VaSidebar>
</template>

<script setup lang="ts">
import { watch, ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import { useColors } from 'vuestic-ui'
import { useAuthStore } from '../../stores/auth'
import { userNavigationRoutes, adminNavigationRoutes, type INavigationRoute } from './NavigationRoutes'

const props = defineProps({
  isAdmin: { type: Boolean, default: false },
  visible: { type: Boolean, default: true },
  mobile: { type: Boolean, default: false },
})

const emit = defineEmits<{
  (event: 'update:visible', visible: boolean): void
}>()

const { getColor, colorToRgba } = useColors()
const route = useRoute()
const { t } = useI18n()
const authStore = useAuthStore()

const value = ref<boolean[]>([])

const writableVisible = computed({
  get: () => props.visible,
  set: (v: boolean) => emit('update:visible', v),
})

const isActiveChildRoute = (child: INavigationRoute) => route.name === child.name

const routeHasActiveChild = (section: INavigationRoute) => {
  if (!section.children) {
    return route.name === section.name
  }

  return section.children.some(({ name }) => route.name === name)
}

const sidebarWidth = computed(() => (props.mobile ? '100vw' : '280px'))
const color = computed(() => getColor('background-secondary'))
const activeColor = computed(() => colorToRgba(getColor('focus'), 0.1))

const iconColor = (route: INavigationRoute) => (routeHasActiveChild(route) ? 'primary' : 'secondary')
const textColor = (route: INavigationRoute) => (routeHasActiveChild(route) ? 'primary' : 'textPrimary')
const arrowDirection = (state: boolean) => (state ? 'va-arrow-up' : 'va-arrow-down')

watch(
  () => route.fullPath,
  () => {
    const navigationRoutes = props.isAdmin ? adminNavigationRoutes : userNavigationRoutes
    const values = navigationRoutes.routes.map((route: INavigationRoute) => routeHasActiveChild(route))
    value.value = [...values, false]
  },
  { immediate: true },
)
</script>
