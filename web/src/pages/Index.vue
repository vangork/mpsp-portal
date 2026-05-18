<script lang="ts" setup>
import { useAuthStore } from '../stores/auth'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

const authStore = useAuthStore()

const { t } = useI18n()

const { push } = useRouter()

const login = () => {
  authStore.isLoggedIn ? push({ name: 'dashboard' }) : push({ name: 'login' })
}
</script>

<template>
  <section class="page">
    <div class="container">
      <header>
        <div class="logo">
          <div class="logo-icon">
            <i class="fas fa-dna"></i>
          </div>
          <div class="logo-text">
            <h1>
              <i>Q<sup>4</sup><span>Omics</span></i>
            </h1>
          </div>
        </div>
      </header>

      <main class="main-content">
        <div class="background-elements">
          <div class="floating-shapes">
            <div class="shape shape-1"></div>
            <div class="shape shape-2"></div>
            <div class="shape shape-3"></div>
          </div>
        </div>

        <div class="hero-text">
          <div class="welcome-text">欢迎使用</div>
          <h1 class="main-title">
            <i>Q<sup>4</sup><span>Omics</span></i> Portal
          </h1>
          <p class="intro-text">
            <i>Q<sup>4</sup>Omics</i> Portal 多组学数据质控平台，基于Quartet,
            Plasmix标准品实现多组学数据的标准化、规范化质量控制。平台遵循"科学先进、合规可靠、标准统一、质量可控、过程可溯"的原则，为临床队列研究提供可靠的数据质量保障。
          </p>
          <button class="login-btn" @click="login">
            <i class="fas fa-sign-in-alt btn-icon"></i>
            {{ authStore.isLoggedIn ? t('auth.open_dashboard') : t('auth.login') }}
          </button>
        </div>
      </main>
    </div>
  </section>
</template>
<style lang="scss" scoped>
.page {
  background: linear-gradient(135deg, #0c2d48 0%, #1a6dbb 50%, #4a9fe7 100%);
  color: white;
  min-height: 100vh;
  overflow-x: hidden;
}
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

/* 头部导航 */
header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 0;
  position: relative;
  z-index: 100;
}

.logo {
  display: flex;
  align-items: center;
  gap: 15px;
  animation: fadeInDown 1s ease-out;
}

.logo-icon {
  font-size: 2.5rem;
  color: #4fc3f7;
}

.logo-text h1 {
  font-size: 1.8rem;
  font-weight: 700;
  letter-spacing: 0.5px;
}

.logo-text span {
  color: #4fc3f7;
}

.nav-links {
  display: flex;
  gap: 30px;
}

.nav-links a {
  color: rgba(255, 255, 255, 0.85);
  text-decoration: none;
  font-weight: 500;
  transition: color 0.3s ease;
  position: relative;
  padding-bottom: 5px;
}

.nav-links a:hover {
  color: white;
}

.nav-links a::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 0;
  height: 2px;
  background-color: #4fc3f7;
  transition: width 0.3s ease;
}

.nav-links a:hover::after {
  width: 100%;
}

/* 主要内容 */
.main-content {
  flex: 1;
  display: flex;
  margin-top: 60px;
  //align-items: center;
  padding: 40px 0;
  position: relative;
}

.hero-text {
  max-width: 700px;
  z-index: 10;
  position: relative;
}

.welcome-text {
  font-size: 1.2rem;
  color: #b3e5fc;
  margin-bottom: 15px;
  opacity: 0;
  animation: fadeInLeft 0.8s ease-out 0.3s forwards;
}

.main-title {
  font-size: 3.8rem;
  font-weight: 800;
  line-height: 1.1;
  margin-bottom: 25px;
  opacity: 0;
  animation: fadeInLeft 0.8s ease-out 0.5s forwards;
}

.main-title span {
  color: #4fc3f7;
  position: relative;
}

.intro-text {
  font-size: 1.2rem;
  line-height: 1.7;
  margin-bottom: 40px;
  color: rgba(255, 255, 255, 0.9);
  max-width: 600px;
  opacity: 0;
  animation: fadeInLeft 0.8s ease-out 0.7s forwards;
}

/* 登录按钮 */
.login-btn {
  display: inline-flex;
  align-items: center;
  gap: 12px;
  background: linear-gradient(135deg, #4fc3f7 0%, #1a6dbb 100%);
  color: white;
  border: none;
  padding: 18px 40px;
  font-size: 1.2rem;
  font-weight: 600;
  border-radius: 50px;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
  opacity: 0;
  animation: fadeInUp 0.8s ease-out 1s forwards;
  position: relative;
  overflow: hidden;
}

.login-btn:hover {
  transform: translateY(-5px);
  box-shadow: 0 15px 30px rgba(0, 0, 0, 0.3);
}

.login-btn:active {
  transform: translateY(-2px);
}

.login-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.7s ease;
}

.login-btn:hover::before {
  left: 100%;
}

.btn-icon {
  font-size: 1.3rem;
}

/* 背景动画元素 */
.background-elements {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  z-index: 1;
}

.floating-shapes {
  position: absolute;
  width: 100%;
  height: 100%;
}

.shape {
  position: absolute;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.03);
}

.shape-1 {
  width: 300px;
  height: 300px;
  top: 10%;
  right: 5%;
  animation: float 25s infinite linear;
}

.shape-2 {
  width: 200px;
  height: 200px;
  bottom: 20%;
  right: 15%;
  animation: float 20s infinite linear reverse;
}

.shape-3 {
  width: 150px;
  height: 150px;
  top: 40%;
  right: 30%;
  animation: float 15s infinite linear;
}

/* 动画定义 */
@keyframes fadeInLeft {
  from {
    opacity: 0;
    transform: translateX(-30px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes fadeInDown {
  from {
    opacity: 0;
    transform: translateY(-30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes float {
  0% {
    transform: translate(0, 0) rotate(0deg);
  }
  33% {
    transform: translate(30px, -30px) rotate(120deg);
  }
  66% {
    transform: translate(-20px, 20px) rotate(240deg);
  }
  100% {
    transform: translate(0, 0) rotate(360deg);
  }
}

/* 响应式设计 */
@media (max-width: 992px) {
  .main-title {
    font-size: 3rem;
  }

  .features {
    flex-direction: column;
  }
}

@media (max-width: 768px) {
  header {
    flex-direction: column;
    gap: 20px;
  }

  .nav-links {
    gap: 20px;
  }

  .main-title {
    font-size: 2.5rem;
  }

  .intro-text {
    font-size: 1.1rem;
  }

  .shape-1,
  .shape-2,
  .shape-3 {
    display: none;
  }
}

@media (max-width: 480px) {
  .main-title {
    font-size: 2rem;
  }

  .login-btn {
    padding: 15px 30px;
    font-size: 1rem;
  }
}
</style>
