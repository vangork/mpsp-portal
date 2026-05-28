<script lang="ts" setup>
import { useBreakpoint } from 'vuestic-ui'
import { useAuthStore } from '../../../stores/auth'
import { formatDate } from '../../omics/composables/useOmics'

const authStore = useAuthStore()
const breakpoints = useBreakpoint()

import ProjectTable from './cards/ProjectTable.vue'
import Timeline from './cards/Timeline.vue'
</script>

<template>
  <section class="content">
    <header class="header-class">
      <div class="header-content">
        <div class="logo">
          <div class="logo-icon">
            <i class="fa-solid fa-chart-bar"></i>
          </div>
          <div class="logo-text">
            <h1>欢迎使用分子表型核酸组学平台</h1>
            <p>复旦大学人类表型组研究院</p>
          </div>
        </div>
        <div v-if="!breakpoints.smDown" class="user-info">
          <div class="user-avatar">
            <i class="fas fa-user"></i>
          </div>
          <div>
            <div style="font-weight: 600">{{ authStore.profile.username }}</div>
            <div style="font-size: 0.9rem; opacity: 0.9">
              上次登录: {{ authStore.profile.last_seen ? formatDate(authStore.profile.last_seen) : '' }}
            </div>
          </div>
        </div>
      </div>
    </header>
  </section>

  <section class="flex flex-col gap-4">
    <div class="flex flex-col md:flex-row gap-4">
      <ProjectTable class="w-full md:w-1/2" />
      <Timeline class="w-full md:w-1/2" />
    </div>
  </section>
</template>

<style lang="scss" scoped>
.header-class {
  background: linear-gradient(135deg, #0c2d48 0%, var(--va-primary) 50%, #4a9fe7 100%);
  color: white;
  padding: 25px 30px;
  border-radius: 12px;
  line-height: 1.6;
  margin-top: 15px;
  margin-bottom: 25px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}
.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.logo {
  display: flex;
  align-items: center;
  gap: 15px;
}
.logo-icon {
  font-size: 2.5rem;
}
.logo-text h1 {
  font-size: 1.8rem;
  margin-bottom: 5px;
}
.logo-text p {
  opacity: 0.9;
  font-size: 0.95rem;
}
.user-info {
  display: flex;
  align-items: center;
  gap: 15px;
}
.user-avatar {
  width: 45px;
  height: 45px;
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.3rem;
}
</style>
