<script lang="ts" setup>
import { useBreakpoint } from 'vuestic-ui'
import { useAuthStore } from '../../stores/auth'
import { formatDate } from '../omics/composables/useOmics'

const authStore = useAuthStore()
const breakpoints = useBreakpoint()
</script>

<template>
  <section class="content">
    <header class="header-class">
      <div class="header-content">
        <div class="logo">
          <div class="logo-icon">
            <i class="fas fa-chart-line"></i>
          </div>
          <div class="logo-text">
            <h1>申康QDP组学质控报告Dashboard</h1>
            <p>上海申康临床队列及组学标准品质控平台</p>
          </div>
        </div>
        <div v-if="!breakpoints.smDown" class="user-info">
          <div class="user-avatar">
            <i class="fas fa-user-md"></i>
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
    <div class="flex flex-col gap-4">
      <div class="flex flex-col md:flex-row gap-4">
        <div class="card standard-intro w-full md:w-2/5">
          <div class="card-header">
            <h2 class="card-title">标准品介绍</h2>
            <i class="fas fa-vial card-icon"></i>
          </div>
          <div class="standard-item">
            <div class="standard-name">中华家系1号（Quartet）标准品</div>
            <div class="standard-desc">
              基于中华家系（D5, D6, F7, M8）构建，适用于组织、细胞等样本的基因组、转录组、蛋白组及代谢组的质量控制。
            </div>
          </div>
          <div class="standard-item">
            <div class="standard-name">Plasmix 血浆标准品</div>
            <div class="standard-desc">专门针对血浆、血清等无细胞体液样本设计，用于蛋白组和代谢组的质量控制。</div>
          </div>
          <div class="standard-item">
            <div class="standard-name">组学标准品的价值</div>
            <div class="standard-desc">
              <ul style="padding-left: 20px; margin-top: 8px">
                <li>评估性能指标（Precision, Recall, SNR, RC等）</li>
                <li>监控批次效应</li>
                <li>数据质量透明化</li>
              </ul>
            </div>
          </div>
        </div>

        <div class="card qc-standards w-full md:w-3/5">
          <div class="card-header">
            <h2 class="card-title">组学质控标准</h2>
            <i class="fas fa-tasks card-icon"></i>
          </div>
          <div class="qc-table-container">
            <table class="qc-table">
              <thead>
                <tr>
                  <th>标准品</th>
                  <th>组学类型</th>
                  <th>文件格式</th>
                  <th>核心质控指标</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <td><span class="standard-badge quartet-badge">Quartet</span></td>
                  <td>基因组</td>
                  <td>VCF变异检测文件</td>
                  <td>SNV Precision ≥ 0.99；SNV Recall ≥ 0.99</td>
                </tr>
                <tr>
                  <td><span class="standard-badge quartet-badge">Quartet</span></td>
                  <td>转录组</td>
                  <td>基因表达矩阵</td>
                  <td>SNR ≥ 10；RC ≥ 0.80</td>
                </tr>
                <tr>
                  <td><span class="standard-badge quartet-badge">Quartet</span></td>
                  <td>蛋白组</td>
                  <td>肽段表达矩阵</td>
                  <td>标称特性Recall ≥ 0.90；SNR ≥ 10；RC ≥ 0.80</td>
                </tr>
                <tr>
                  <td><span class="standard-badge quartet-badge">Quartet</span></td>
                  <td>代谢组</td>
                  <td>代谢物表达矩阵</td>
                  <td>SNR ≥ 10；RC ≥ 0.80</td>
                </tr>
                <tr>
                  <td><span class="standard-badge plasmix-badge">Plasmix</span></td>
                  <td>蛋白组</td>
                  <td>蛋白质表达矩阵</td>
                  <td>蛋白鉴定数 ≥ 1,000；标称特性Recall ≥ 0.90；SNR ≥ 5；RC ≥ 0.80</td>
                </tr>
                <tr>
                  <td><span class="standard-badge plasmix-badge">Plasmix</span></td>
                  <td>代谢组</td>
                  <td>代谢物表达矩阵</td>
                  <td>SNR ≥ 5；RC ≥ 0.80</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
    <footer style="padding-top: 20px; margin-top: 25px; border-top: 1px solid #ddd; color: #777; font-size: 0.9rem">
      <p>
        1. 遵循《上海临床队列组学检测工作指引（征求意见稿）》, 2025/11/26.
        原则：科学先进、合规可靠、标准统一、质量可控、过程可溯
      </p>
      <p style="margin-top: 8px">
        2. 参考文献：Zheng, Y. et al. Multi-omics data integration using ratio-based quantitative profiling with Quartet
        reference materials. Nature Biotechnology 1-17 (2023).
      </p>
    </footer>
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

.card {
  background-color: white;
  border-radius: 12px;
  padding: 25px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  transition:
    transform 0.3s ease,
    box-shadow 0.3s ease;
}

.card:hover {
  transform: translateY(-5px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.08);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  border-bottom: 1px solid #eee;
  padding-bottom: 15px;
}

.card-title {
  font-size: 1.3rem;
  color: var(--va-primary);
  font-weight: 600;
}

.card-icon {
  color: var(--va-primary);
  font-size: 1.5rem;
}

.standard-intro {
  grid-column: span 4;
}

.standard-item {
  margin-bottom: 25px;
  padding-bottom: 20px;
  border-bottom: 1px solid #eee;
}

.standard-item:last-child {
  margin-bottom: 0;
  padding-bottom: 0;
  border-bottom: none;
}

.standard-name {
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 8px;
  color: #333;
}

.standard-desc {
  font-size: 0.95rem;
  color: #666;
  line-height: 1.5;
}

.standard-badge {
  display: inline-block;
  padding: 5px 12px;
  border-radius: 20px;
  font-size: 0.85rem;
  font-weight: 600;
}

.qc-standards {
  grid-column: span 9;
}

.qc-table-container {
  overflow-x: auto;
}

.qc-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 10px;
}

.qc-table th {
  background-color: #f8f9fa;
  padding: 15px;
  text-align: left;
  font-weight: 600;
  color: #444;
  border-bottom: 2px solid #eaeaea;
}

.qc-table td {
  padding: 15px;
  border-bottom: 1px solid #eee;
}

.qc-table tr:hover {
  background-color: #f9fafb;
}
.quartet-badge {
  background-color: #e8f4fd;
  color: var(--va-primary);
}

.plasmix-badge {
  background-color: #f0f7f0;
  color: var(--va-success);
}
</style>
