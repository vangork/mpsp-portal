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

const goToOrder = () => {
  push({ name: 'order' })
}

const capabilities = [
  {
    icon: 'fas fa-vial',
    title: '全流程标准化',
    desc: '从样本制备到生信分析的完整工作流标准化，确保每个环节规范可控',
  },
  {
    icon: 'fas fa-shield-alt',
    title: '多节点质量控制',
    desc: '多节点质量控制体系，全程监控数据质量，保障分析结果的可靠性',
  },
  {
    icon: 'fas fa-sync-alt',
    title: '批间数据可重复性',
    desc: '相对定量方法显著提升批间数据重复性，消除系统误差对结果的影响',
  },
  {
    icon: 'fas fa-layer-group',
    title: '多组学整合分析',
    desc: '支持转录组、基因组及多组学研究的一站式解决方案',
  },
]

const achievements = [
  { icon: 'fas fa-globe', number: '1', label: 'ISO国际标准', sub: 'ISO/TS 22690:2021' },
  { icon: 'fas fa-flask', number: '8', label: '国家级标准物质', sub: '自主研发' },
  { icon: 'fas fa-newspaper', number: '10+', label: '顶刊论文', sub: 'Nature Biotechnology · Genome Biology' },
  { icon: 'fas fa-users', number: '∞', label: '开放合作', sub: '诚邀科研合作' },
]
</script>

<template>
  <div class="home">
    <!-- ── NAVBAR ── -->
    <nav class="navbar">
      <div class="nav-inner">
        <div class="brand">
          <i class="fas fa-dna brand-icon"></i>
          <span class="brand-name">MPSP <span class="brand-highlight">Portal</span></span>
        </div>
        <div class="nav-links">
          <router-link :to="{ hash: '#about' }">平台简介</router-link>
          <router-link :to="{ hash: '#capabilities' }">核心能力</router-link>
          <router-link :to="{ hash: '#achievements' }">科研成果</router-link>
          <router-link :to="{ hash: '#contact' }">联系我们</router-link>
        </div>
      </div>
    </nav>

    <!-- ── HERO ── -->
    <section class="hero">
      <div class="hero-bg">
        <div class="blob blob-1"></div>
        <div class="blob blob-2"></div>
        <div class="blob blob-3"></div>
        <div class="grid-overlay"></div>
      </div>
      <div class="hero-content">
        <div class="badge-row">
          <span class="badge"><i class="fas fa-university"></i> 复旦大学人类表型组研究院</span>
        </div>
        <h1 class="hero-title">
          分子表型<br />
          <span class="highlight">核酸组学</span><br />
          平台
        </h1>
        <p class="hero-sub">
          (Molecular Phenotyping Sequencing Platform)以<strong>标准化</strong>为核心竞争力，制定 ISO 国际标准，研发国家级标准物质，<br class="d-none-sm" />
          为临床队列研究提供可靠的多组学数据质量保障。
        </p>
        <div class="hero-actions">
          <button class="btn-primary" @click="login">
            <i class="fas fa-sign-in-alt"></i>
            {{ authStore.isLoggedIn ? t('auth.open_dashboard') : t('auth.login') }}
          </button>
          <router-link :to="{ hash: '#about' }" class="btn-ghost">
            <i class="fas fa-chevron-down"></i>
            了解更多
          </router-link>
        </div>
      </div>
      <div class="hero-visual">
        <div class="dna-ring">
          <div class="ring r1"></div>
          <div class="ring r2"></div>
          <div class="ring r3"></div>
          <div class="center-icon"><i class="fas fa-dna"></i></div>
        </div>
      </div>
    </section>

    <!-- ── ABOUT ── -->
    <section id="about" class="about">
      <div class="section-inner">
        <div class="about-text">
          <div class="section-tag">平台简介</div>
          <h2 class="section-title">国际标准 · 质量领先</h2>
          <p>
            分子表型核酸组学平台依托复旦大学人类表型组研究院，基于上海市级重大专项建设而成，
            是具备全流程标准化基因组与转录组数据生产与分析能力的技术平台，覆盖从样本制备到生物信息解读的全环节。
          </p>
          <p>
            平台以“标准化”为核心竞争力，主导制定 <strong>ISO/TS 22690:2021</strong> 国际标准，自主研发 <strong>8 项</strong>
            国家级标准物质，发表 10 余篇顶级期刊论文，包括
            <em>Nature Biotechnology</em>、<em>Genome Biology</em>、<em>Nature Cancer</em> 等。
          </p>
          <p>
            平台遵循"科学先进、合规可靠、标准统一、质量可控、过程可溯"的原则，为科研及临床队列研究提供全流程数据质量解决方案。
          </p>
        </div>
        <div class="about-card">
          <div class="card-header">
            <i class="fas fa-star card-icon"></i>
            <span>代表性成果</span>
          </div>
          <div class="featured-project">
            <div class="project-badge">Nature Cancer</div>
            <h3>中国乳腺癌基因组图谱（CBCGA）</h3>
            <p>
              支持邵志敏教授团队完成中国最大规模乳腺癌多组学自然队列研究，
              为乳腺癌精准诊疗提供重要基因组数据支撑。
            </p>
            <div class="project-meta">
              <span><i class="fas fa-user"></i> 邵志敏教授团队</span>
              <span><i class="fas fa-database"></i> 最大中国乳腺癌多组学队列</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ── CAPABILITIES ── -->
    <section id="capabilities" class="capabilities">
      <div class="section-inner center">
        <div class="section-tag">核心能力</div>
        <h2 class="section-title">全方位质控解决方案</h2>
        <p class="section-desc">从样本到结果，每一个节点都有精准的质量保障</p>
        <div class="cap-grid">
          <div v-for="cap in capabilities" :key="cap.title" class="cap-card">
            <div class="cap-icon-wrap">
              <i :class="cap.icon" class="cap-icon"></i>
            </div>
            <h3>{{ cap.title }}</h3>
            <p>{{ cap.desc }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- ── ACHIEVEMENTS ── -->
    <section id="achievements" class="achievements">
      <div class="section-inner center">
        <div class="section-tag">科研成果</div>
        <h2 class="section-title">数字见证实力</h2>
        <div class="stats-grid">
          <div v-for="a in achievements" :key="a.label" class="stat-card">
            <i :class="a.icon" class="stat-icon"></i>
            <div class="stat-number">{{ a.number }}</div>
            <div class="stat-label">{{ a.label }}</div>
            <div class="stat-sub">{{ a.sub }}</div>
          </div>
        </div>
      </div>
    </section>

    <!-- ── CONTACT ── -->
    <section id="contact" class="contact">
      <div class="section-inner center">
        <div class="section-tag">联系我们</div>
        <h2 class="section-title">期待与您合作</h2>
        <p class="section-desc">如有科研合作意向，欢迎随时联系平台负责人</p>
        <div class="contact-card">
          <div class="contact-row">
            <div class="contact-item">
              <i class="fas fa-user-tie contact-icon"></i>
              <div>
                <div class="contact-label">联系人</div>
                <div class="contact-value">侯老师</div>
              </div>
            </div>
            <div class="contact-item">
              <i class="fas fa-phone contact-icon"></i>
              <div>
                <div class="contact-label">电话 / 微信</div>
                <div class="contact-value">18721040304</div>
              </div>
            </div>
            <div class="contact-item">
              <i class="fas fa-envelope contact-icon"></i>
              <div>
                <div class="contact-label">邮箱</div>
                <div class="contact-value">houwanwan@fudan.edu.cn</div>
              </div>
            </div>
            <div class="contact-item">
              <i class="fas fa-map-marker-alt contact-icon"></i>
              <div>
                <div class="contact-label">地址</div>
                <div class="contact-value">上海市浦东新区张衡路825号科研2号楼</div>
              </div>
            </div>
          </div>
        </div>
        <div class="contact-actions">
          <button class="btn-sample" @click="goToOrder">
            <i class="fas fa-file-medical-alt"></i>
            发送样本信息单
          </button>
        </div>
      </div>
    </section>

    <!-- ── FOOTER ── -->
    <footer class="footer">
      <div class="footer-inner">
        <div class="footer-brand">
          <i class="fas fa-dna"></i>
          <span>MPSP Portal</span>
        </div>
        <div class="footer-copy">© 2026 复旦大学人类表型组研究院 · 分子表型核酸组学平台</div>
      </div>
    </footer>
  </div>
</template>

<style lang="scss" scoped>
// ── tokens ──
$primary: #1a6dbb;
$primary-light: #4fc3f7;
$dark: #0c2048;
$darker: #07132b;
$text: #e2eaf4;
$muted: rgba(255, 255, 255, 0.65);
$card-bg: rgba(255, 255, 255, 0.06);
$card-border: rgba(255, 255, 255, 0.12);
$radius: 16px;

// ── base ──
.home {
  background: $darker;
  color: $text;
  font-family: 'Inter', sans-serif;
  overflow-x: hidden;
}

// ── navbar ──
.navbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 200;
  background: rgba(7, 19, 43, 0.85);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid $card-border;
}
.nav-inner {
  max-width: 1200px;
  margin: 0 auto;
  padding: 14px 24px;
  display: flex;
  align-items: center;
  gap: 32px;
}
.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 1.25rem;
  font-weight: 700;
  white-space: nowrap;
}
.brand-icon {
  color: $primary-light;
  font-size: 1.4rem;
}
.brand-highlight {
  color: $primary-light;
}
.nav-links {
  display: flex;
  gap: 28px;
  margin-left: auto;
  a {
    color: $muted;
    text-decoration: none;
    font-size: 0.95rem;
    font-weight: 500;
    transition: color 0.25s;
    &:hover {
      color: white;
    }
  }
}
.btn-outline {
  margin-left: 16px;
  padding: 8px 20px;
  border: 1.5px solid $primary-light;
  border-radius: 50px;
  background: transparent;
  color: $primary-light;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  white-space: nowrap;
  transition: all 0.25s;
  i {
    margin-right: 6px;
  }
  &:hover {
    background: $primary-light;
    color: $darker;
  }
}

// ── hero ──
.hero {
  min-height: 100vh;
  display: flex;
  align-items: center;
  padding: 120px 24px 80px;
  position: relative;
  max-width: 1200px;
  margin: 0 auto;
  gap: 60px;
}
.hero-bg {
  position: fixed;
  inset: 0;
  z-index: 0;
  pointer-events: none;
  overflow: hidden;
}
.blob {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.25;
}
.blob-1 {
  width: 600px;
  height: 600px;
  background: $primary;
  top: -150px;
  right: -100px;
  animation: drift 20s infinite alternate ease-in-out;
}
.blob-2 {
  width: 400px;
  height: 400px;
  background: $primary-light;
  bottom: 10%;
  left: -80px;
  animation: drift 25s infinite alternate-reverse ease-in-out;
}
.blob-3 {
  width: 300px;
  height: 300px;
  background: #7c3aed;
  top: 50%;
  left: 40%;
  animation: drift 18s infinite alternate ease-in-out;
}
.grid-overlay {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(rgba(255, 255, 255, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.03) 1px, transparent 1px);
  background-size: 60px 60px;
}
.hero-content {
  position: relative;
  z-index: 10;
  flex: 1;
  max-width: 620px;
}
.badge-row {
  margin-bottom: 20px;
}
.badge {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  background: rgba(79, 195, 247, 0.15);
  border: 1px solid rgba(79, 195, 247, 0.35);
  color: $primary-light;
  padding: 6px 16px;
  border-radius: 50px;
  font-size: 0.85rem;
  font-weight: 500;
  animation: fadeInDown 0.7s ease-out both;
}
.hero-title {
  font-size: 3.8rem;
  font-weight: 800;
  line-height: 1.12;
  margin-bottom: 24px;
  animation: fadeInLeft 0.7s ease-out 0.15s both;
  .highlight {
    color: $primary-light;
  }
}
.hero-sub {
  font-size: 1.1rem;
  line-height: 1.8;
  color: $muted;
  margin-bottom: 40px;
  animation: fadeInLeft 0.7s ease-out 0.3s both;
  strong {
    color: white;
    font-weight: 600;
  }
}
.hero-actions {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
  animation: fadeInUp 0.7s ease-out 0.5s both;
}
.btn-primary {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  background: linear-gradient(135deg, $primary-light 0%, $primary 100%);
  color: white;
  border: none;
  padding: 16px 36px;
  font-size: 1.05rem;
  font-weight: 600;
  border-radius: 50px;
  cursor: pointer;
  transition: all 0.3s;
  box-shadow: 0 8px 24px rgba(26, 109, 187, 0.4);
  &:hover {
    transform: translateY(-3px);
    box-shadow: 0 14px 32px rgba(26, 109, 187, 0.55);
  }
}
.btn-ghost {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  color: $muted;
  text-decoration: none;
  padding: 16px 28px;
  font-size: 1rem;
  font-weight: 500;
  border-radius: 50px;
  border: 1px solid $card-border;
  transition: all 0.25s;
  &:hover {
    color: white;
    border-color: rgba(255, 255, 255, 0.3);
    background: $card-bg;
  }
}

// ── dna ring visual ──
.hero-visual {
  position: relative;
  z-index: 10;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 340px;
  height: 340px;
}
.dna-ring {
  position: relative;
  width: 280px;
  height: 280px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.ring {
  position: absolute;
  border-radius: 50%;
  border-style: solid;
  border-color: transparent;
}
.r1 {
  width: 280px;
  height: 280px;
  border-width: 2px;
  border-top-color: rgba(79, 195, 247, 0.6);
  border-right-color: rgba(79, 195, 247, 0.2);
  animation: spin 8s linear infinite;
}
.r2 {
  width: 210px;
  height: 210px;
  border-width: 2px;
  border-bottom-color: rgba(26, 109, 187, 0.7);
  border-left-color: rgba(26, 109, 187, 0.2);
  animation: spin 12s linear infinite reverse;
}
.r3 {
  width: 145px;
  height: 145px;
  border-width: 2px;
  border-top-color: rgba(124, 58, 237, 0.6);
  border-right-color: rgba(124, 58, 237, 0.2);
  animation: spin 6s linear infinite;
}
.center-icon {
  font-size: 3rem;
  color: $primary-light;
  text-shadow: 0 0 30px rgba(79, 195, 247, 0.6);
}

// ── shared section helpers ──
.section-inner {
  max-width: 1200px;
  margin: 0 auto;
  padding: 100px 24px;
  &.center {
    text-align: center;
  }
}
.section-tag {
  display: inline-block;
  color: $primary-light;
  font-size: 0.8rem;
  font-weight: 700;
  letter-spacing: 2px;
  text-transform: uppercase;
  margin-bottom: 14px;
  background: rgba(79, 195, 247, 0.12);
  border: 1px solid rgba(79, 195, 247, 0.3);
  padding: 4px 14px;
  border-radius: 50px;
}
.section-title {
  font-size: 2.4rem;
  font-weight: 800;
  margin-bottom: 20px;
  line-height: 1.2;
}
.section-desc {
  color: $muted;
  font-size: 1.05rem;
  margin-bottom: 56px;
}

// ── about ──
.about {
  position: relative;
  z-index: 10;
  .section-inner {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 60px;
    align-items: start;
    padding-top: 100px;
    padding-bottom: 100px;
  }
}
.about-text {
  p {
    color: $muted;
    line-height: 1.8;
    margin-bottom: 16px;
    font-size: 1rem;
    strong, em {
      color: white;
    }
  }
}
.about-card {
  background: $card-bg;
  border: 1px solid $card-border;
  border-radius: $radius;
  margin-top: 45px;
  padding: 32px;
  backdrop-filter: blur(8px);
}
.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  color: $primary-light;
  font-weight: 700;
  font-size: 0.9rem;
  letter-spacing: 1px;
  text-transform: uppercase;
  margin-bottom: 24px;
}
.card-icon {
  font-size: 1.1rem;
}
.project-badge {
  display: inline-block;
  background: linear-gradient(135deg, $primary-light, $primary);
  color: white;
  font-size: 0.75rem;
  font-weight: 700;
  padding: 3px 12px;
  border-radius: 50px;
  margin-bottom: 14px;
  letter-spacing: 0.5px;
}
.featured-project h3 {
  font-size: 1.25rem;
  font-weight: 700;
  margin-bottom: 12px;
}
.featured-project p {
  color: $muted;
  line-height: 1.7;
  font-size: 0.95rem;
  margin-bottom: 20px;
}
.project-meta {
  display: flex;
  flex-direction: column;
  gap: 8px;
  span {
    display: flex;
    align-items: center;
    gap: 10px;
    color: rgba(255, 255, 255, 0.5);
    font-size: 0.85rem;
    i {
      color: $primary-light;
      width: 14px;
    }
  }
}

// ── capabilities ──
.capabilities {
  position: relative;
  z-index: 10;
  background: rgba(255, 255, 255, 0.02);
  border-top: 1px solid $card-border;
  border-bottom: 1px solid $card-border;
}
.cap-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 24px;
  text-align: left;
}
.cap-card {
  background: $card-bg;
  border: 1px solid $card-border;
  border-radius: $radius;
  padding: 32px 24px;
  transition: all 0.3s;
  &:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(79, 195, 247, 0.35);
    transform: translateY(-4px);
  }
  h3 {
    font-size: 1.05rem;
    font-weight: 700;
    margin-bottom: 10px;
  }
  p {
    color: $muted;
    font-size: 0.9rem;
    line-height: 1.7;
  }
}
.cap-icon-wrap {
  width: 52px;
  height: 52px;
  border-radius: 14px;
  background: linear-gradient(135deg, rgba(79, 195, 247, 0.2), rgba(26, 109, 187, 0.2));
  border: 1px solid rgba(79, 195, 247, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 20px;
}
.cap-icon {
  font-size: 1.3rem;
  color: $primary-light;
}

// ── achievements ──
.achievements {
  position: relative;
  z-index: 10;
}
.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 24px;
}
.stat-card {
  background: $card-bg;
  border: 1px solid $card-border;
  border-radius: $radius;
  padding: 40px 24px;
  transition: all 0.3s;
  &:hover {
    background: rgba(255, 255, 255, 0.09);
    border-color: rgba(79, 195, 247, 0.35);
    transform: translateY(-4px);
  }
}
.stat-icon {
  font-size: 1.6rem;
  color: $primary-light;
  margin-bottom: 16px;
}
.stat-number {
  font-size: 3rem;
  font-weight: 800;
  color: white;
  line-height: 1;
  margin-bottom: 8px;
}
.stat-label {
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 6px;
}
.stat-sub {
  font-size: 0.8rem;
  color: $muted;
}

// ── contact ──
.contact {
  position: relative;
  z-index: 10;
  background: rgba(255, 255, 255, 0.02);
  border-top: 1px solid $card-border;
}
.contact-card {
  background: $card-bg;
  border: 1px solid $card-border;
  border-radius: $radius;
  padding: 48px;
  max-width: 860px;
  margin: 0 auto;
}
.contact-row {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 28px;
}
.contact-item {
  display: flex;
  align-items: flex-start;
  gap: 16px;
}
.contact-icon {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  background: rgba(79, 195, 247, 0.15);
  border: 1px solid rgba(79, 195, 247, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  color: $primary-light;
  flex-shrink: 0;
}
.contact-label {
  display: flex;
  justify-content: left;
  font-size: 0.8rem;
  color: $muted;
  margin-bottom: 4px;
}
.contact-value {
  font-size: 0.95rem;
  font-weight: 600;
  color: white;
}
.contact-actions {
  margin-top: 40px;
  display: flex;
  justify-content: center;
}
.btn-sample {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  background: linear-gradient(135deg, #34d399 0%, #059669 100%);
  color: white;
  border: none;
  padding: 16px 40px;
  font-size: 1.05rem;
  font-weight: 600;
  border-radius: 50px;
  cursor: pointer;
  transition: all 0.3s;
  box-shadow: 0 8px 24px rgba(5, 150, 105, 0.35);
  &:hover {
    transform: translateY(-3px);
    box-shadow: 0 14px 32px rgba(5, 150, 105, 0.5);
  }
}

// ── footer ──
.footer {
  position: relative;
  z-index: 10;
  border-top: 1px solid $card-border;
  padding: 28px 24px;
}
.footer-inner {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}
.footer-brand {
  display: flex;
  align-items: center;
  gap: 10px;
  font-weight: 700;
  color: $primary-light;
  i {
    font-size: 1.1rem;
  }
}
.footer-copy {
  font-size: 0.82rem;
  color: $muted;
}

// ── animations ──
@keyframes fadeInDown {
  from { opacity: 0; transform: translateY(-20px); }
  to   { opacity: 1; transform: translateY(0); }
}
@keyframes fadeInLeft {
  from { opacity: 0; transform: translateX(-28px); }
  to   { opacity: 1; transform: translateX(0); }
}
@keyframes fadeInUp {
  from { opacity: 0; transform: translateY(24px); }
  to   { opacity: 1; transform: translateY(0); }
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
@keyframes drift {
  from { transform: translate(0, 0) scale(1); }
  to   { transform: translate(40px, 30px) scale(1.08); }
}

// ── responsive ──
@media (max-width: 1024px) {
  .cap-grid,
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  .about .section-inner {
    grid-template-columns: 1fr;
  }
  .hero-visual {
    display: none;
  }
}
@media (max-width: 768px) {
  .nav-links {
    display: none;
  }
  .hero-title {
    font-size: 2.8rem;
  }
  .section-title {
    font-size: 1.9rem;
  }
  .contact-row {
    grid-template-columns: 1fr;
  }
  .contact-card {
    padding: 28px 20px;
  }
}
@media (max-width: 480px) {
  .hero-title {
    font-size: 2.2rem;
  }
  .cap-grid,
  .stats-grid {
    grid-template-columns: 1fr;
  }
  .hero-actions {
    flex-direction: column;
  }
}
</style>
