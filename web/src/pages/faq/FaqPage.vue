<template>
  <section id="report">
    <h3 class="text-3xl font-bold text-heading mt-4 mb-6">质控报告解读指南</h3>

    <div className="flex mb-6">
      <div className="bg-white p-1 rounded-full border border-slate-200 shadow-sm flex">
        <button :class="isDNABtnActive ? clzBTNActive : clzBTNInactive" @click="isDNABtnActive = !isDNABtnActive">
          🧬 基因组 (DNA)
        </button>
        <button :class="!isDNABtnActive ? clzBTNActive : clzBTNInactive" @click="isDNABtnActive = !isDNABtnActive">
          📊 定量组 (转录组(RNA) / 蛋白组 / 代谢组)
        </button>
      </div>
    </div>

    <div v-if="isDNABtnActive" class="flex flex-col gap-4">
      <div class="bg-blue-50 p-4 rounded-lg border border-blue-100 mb-2">
        <p class="text-blue-800 text-sm">
          <strong>场景假设：</strong> 我们去池塘里捞“变异”（Mutation），池塘里其实只有 **10条真鱼**（真实突变）。
        </p>
      </div>
      <div class="flex flex-col md:flex-row gap-4">
        <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200 w-full md:w-1/2">
          <div class="flex items-center gap-2 mb-4">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              class="w-6 h-6 text-indigo-600"
            >
              <circle cx="12" cy="12" r="10" />
              <circle cx="12" cy="12" r="6" />
              <circle cx="12" cy="12" r="2" />
            </svg>
            <h3 class="text-lg font-bold text-slate-800">1. 精确度 (Precision) - 网里有多少垃圾？</h3>
          </div>
          <div class="h-48 bg-indigo-50 rounded-lg flex flex-col items-center justify-center relative overflow-hidden">
            <div
              class="w-32 h-32 border-4 border-indigo-300 rounded-full flex flex-wrap items-center justify-center p-2 gap-1 bg-white"
            >
              <div v-for="i in 9" :key="{ i }" class="w-4 h-4 bg-green-500 rounded-full" title="真鱼"></div>
              <div class="w-4 h-4 bg-red-500 rotate-45" title="垃圾 (假阳性)"></div>
            </div>
            <div class="absolute bottom-2 right-2 text-xs text-indigo-400">网里的情况</div>
          </div>
          <div class="mt-4 text-slate-700 text-sm">
            <p class="font-bold text-indigo-700">定义：只要是我捞上来的，多少是真的？</p>
            <p class="mt-2">
              如果我捞了10个东西，9个是鱼，1个是垃圾。<br />
              <strong>精确度 = 90%</strong>
            </p>
            <p class="mt-2 text-xs text-red-500">❌ 低精确度 = 假警报太多，医生会被误导。</p>
          </div>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200 w-full md:w-1/2">
          <div class="flex items-center gap-2 mb-4">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              class="w-6 h-6 text-teal-600"
            >
              <circle cx="11" cy="11" r="8" />
              <line x1="21" x2="16.65" y1="21" y2="16.65" />
              <line x1="11" x2="11" y1="8" y2="14" />
              <line x1="8" x2="14" y1="11" y2="11" />
            </svg>
            <h3 class="text-lg font-bold text-slate-800">2. 召回率 (Recall) - 漏掉了多少鱼？</h3>
          </div>
          <div class="h-48 bg-teal-50 rounded-lg flex flex-col items-center justify-center relative overflow-hidden">
            <div class="flex gap-8">
              <div class="flex flex-col items-center">
                <span class="text-xs mb-1 text-slate-500">网里捞到的 (8条)</span>
                <div
                  class="w-20 h-20 border-2 border-teal-500 rounded-full bg-white flex flex-wrap p-1 justify-center content-center gap-1"
                >
                  <div v-for="i in 8" :key="{ i }" class="w-3 h-3 bg-green-500 rounded-full"></div>
                </div>
              </div>
              <div class="flex flex-col items-center">
                <span class="text-xs mb-1 text-slate-500">还在水里的 (2条)</span>
                <div
                  class="w-20 h-20 border-2 border-dashed border-slate-400 rounded-full bg-transparent flex flex-wrap p-1 justify-center content-center gap-1"
                >
                  <div class="w-3 h-3 bg-green-500 rounded-full opacity-50"></div>
                  <div class="w-3 h-3 bg-green-500 rounded-full opacity-50"></div>
                </div>
              </div>
            </div>
          </div>
          <div class="mt-4 text-slate-700 text-sm">
            <p class="font-bold text-teal-700">定义：池塘里所有的真鱼，我捞到了多少？</p>
            <p class="mt-2">
              池塘一共10条鱼，我只捞到了8条，漏了2条。<br />
              <strong>召回率 = 80%</strong>
            </p>
            <p class="mt-2 text-xs text-red-500">❌ 低召回率 = 漏诊，把病人放走了。</p>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="flex flex-col gap-4">
      <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
        <div class="flex items-center gap-2 mb-4">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            class="w-6 h-6 text-blue-600"
          >
            <path d="M22 12h-4l-3 9L9 3l-3 9H2" />
          </svg>
          <h3 class="text-xl font-bold text-slate-800">1. 信噪比 (SNR) - 组学检测的“视力”好不好？</h3>
        </div>
        <p class="text-slate-600 mb-6">
          <strong>怎么看图：</strong> 报告中的 PCA 图（散点聚类图）。<br />
          <strong>核心逻辑：</strong> 同一组样本（相同颜色）应该“抱团”，不同组样本应该“分开”。
        </p>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div class="flex flex-col items-center">
            <div class="relative w-64 h-64 bg-slate-50 border border-slate-200 rounded-lg p-4">
              <div class="absolute top-2 left-2 text-xs font-bold text-green-600 bg-green-100 px-2 py-1 rounded">
                ✅ Pass: 视力清晰 (SNR &ge; 10)
              </div>

              <div class="absolute top-1/4 left-1/4 w-12 h-12 bg-red-200 rounded-full opacity-50 blur-sm"></div>
              <div class="absolute top-[28%] left-[28%] w-2 h-2 bg-red-500 rounded-full"></div>
              <div class="absolute top-[22%] left-[25%] w-2 h-2 bg-red-500 rounded-full"></div>
              <div class="absolute top-[26%] left-[22%] w-2 h-2 bg-red-500 rounded-full"></div>

              <div class="absolute bottom-1/4 right-1/4 w-12 h-12 bg-blue-200 rounded-full opacity-50 blur-sm"></div>
              <div class="absolute bottom-[28%] right-[28%] w-2 h-2 bg-blue-500 rounded-full"></div>
              <div class="absolute bottom-[22%] right-[25%] w-2 h-2 bg-blue-500 rounded-full"></div>
              <div class="absolute bottom-[26%] right-[22%] w-2 h-2 bg-blue-500 rounded-full"></div>

              <div class="absolute top-1/4 right-1/4 w-12 h-12 bg-green-200 rounded-full opacity-50 blur-sm"></div>
              <div class="absolute top-[28%] right-[28%] w-2 h-2 bg-green-500 rounded-full"></div>
              <div class="absolute top-[22%] right-[25%] w-2 h-2 bg-green-500 rounded-full"></div>
              <div class="absolute top-[26%] right-[22%] w-2 h-2 bg-green-500 rounded-full"></div>
            </div>
            <p class="mt-3 text-sm text-center font-medium text-slate-700">
              <span class="text-green-600 font-bold">理想状态：</span><br />
              不同颜色的点分得很开，<br />同颜色的点聚得很紧。
            </p>
          </div>

          <div class="flex flex-col items-center">
            <div class="relative w-64 h-64 bg-slate-50 border border-slate-200 rounded-lg p-4">
              <div class="absolute top-2 left-2 text-xs font-bold text-red-600 bg-red-100 px-2 py-1 rounded">
                ❌ Fail: 视力模糊 (SNR &lt; 5)
              </div>
              <div class="absolute top-1/3 left-1/3 w-32 h-32 bg-purple-100 rounded-full opacity-30 blur-xl"></div>

              <div class="absolute top-[40%] left-[30%] w-2 h-2 bg-red-500 rounded-full"></div>
              <div class="absolute top-[45%] left-[50%] w-2 h-2 bg-blue-500 rounded-full"></div>
              <div class="absolute top-[35%] left-[45%] w-2 h-2 bg-green-500 rounded-full"></div>
              <div class="absolute top-[55%] left-[40%] w-2 h-2 bg-red-500 rounded-full"></div>
              <div class="absolute top-[60%] left-[60%] w-2 h-2 bg-blue-500 rounded-full"></div>
              <div class="absolute top-[50%] left-[55%] w-2 h-2 bg-green-500 rounded-full"></div>
            </div>
            <p class="mt-3 text-sm text-center font-medium text-slate-700">
              <span class="text-red-600 font-bold">糟糕状态：</span><br />
              大家混在一起，<br />分不清谁是谁（批次效应大）。
            </p>
          </div>
        </div>
      </div>

      <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
        <div class="flex items-center gap-2 mb-4">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            class="w-6 h-6 text-purple-600"
          >
            <path
              d="m12.83 2.44 9.8 5.5c.3.17.47.5.47.86 0 .37-.16.7-.47.87L12.83 15a1.8 1.8 0 0 1-1.66 0L1.37 9.67a1.002 1.002 0 0 1 0-1.73l9.8-5.5c.51-.29 1.15-.29 1.66 0Z"
            />
            <path d="m22 14.1-9.17 5.29a1.8 1.8 0 0 1-1.66 0L2 14.1" />
            <path d="m22 17.93-9.17 5.29a1.8 1.8 0 0 1-1.66 0L2 17.93" />
          </svg>
          <h3 class="text-xl font-bold text-slate-800">2. 相对相关性 (RC) - 测得准不准？</h3>
        </div>
        <p class="text-slate-600 mb-6">
          <strong>怎么看图：</strong> 报告中的 Scatter Plot（对角线散点图）。<br />
          <strong>核心逻辑：</strong> 所有的点应该乖乖地趴在 45° 对角虚线上。
        </p>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div class="flex flex-col items-center">
            <div
              class="relative w-64 h-64 bg-slate-50 border border-slate-200 rounded-lg p-8 flex items-end justify-start"
            >
              <div class="absolute top-2 left-2 text-xs font-bold text-green-600 bg-green-100 px-2 py-1 rounded">
                ✅ Pass: 准确 (RC &ge; 0.8)
              </div>
              <div
                class="absolute bottom-4 left-4 w-[220px] h-[220px] border-t border-r border-transparent border-dashed border-slate-300 transform -rotate-45 origin-bottom-left z-0"
              ></div>
              <div
                class="absolute bottom-4 left-4 w-[90%] h-[1px] bg-slate-300 transform -rotate-45 origin-bottom-left"
              ></div>

              <div class="absolute bottom-[20%] left-[21%] w-1.5 h-1.5 bg-purple-600 rounded-full"></div>
              <div class="absolute bottom-[40%] left-[42%] w-1.5 h-1.5 bg-purple-600 rounded-full"></div>
              <div class="absolute bottom-[60%] left-[59%] w-1.5 h-1.5 bg-purple-600 rounded-full"></div>
              <div class="absolute bottom-[80%] left-[79%] w-1.5 h-1.5 bg-purple-600 rounded-full"></div>
              <div class="absolute bottom-[30%] left-[32%] w-1.5 h-1.5 bg-purple-600 rounded-full"></div>
            </div>
            <p class="mt-3 text-sm text-center font-medium text-slate-700">
              <span class="text-green-600 font-bold">瘦长的椭圆：</span><br />
              实测值（Y轴）和标准值（X轴）<br />高度一致。
            </p>
          </div>

          <div class="flex flex-col items-center">
            <div
              class="relative w-64 h-64 bg-slate-50 border border-slate-200 rounded-lg p-8 flex items-end justify-start"
            >
              <div class="absolute top-2 left-2 text-xs font-bold text-red-600 bg-red-100 px-2 py-1 rounded">
                ❌ Fail: 偏差大 (RC &lt; 0.6)
              </div>
              <div
                class="absolute bottom-4 left-4 w-[90%] h-[1px] bg-slate-300 transform -rotate-45 origin-bottom-left opacity-30"
              ></div>

              <div class="absolute bottom-[20%] left-[50%] w-1.5 h-1.5 bg-purple-400 rounded-full"></div>
              <div class="absolute bottom-[70%] left-[20%] w-1.5 h-1.5 bg-purple-400 rounded-full"></div>
              <div class="absolute bottom-[50%] left-[60%] w-1.5 h-1.5 bg-purple-400 rounded-full"></div>
              <div class="absolute bottom-[30%] left-[80%] w-1.5 h-1.5 bg-purple-400 rounded-full"></div>
              <div class="absolute bottom-[60%] left-[30%] w-1.5 h-1.5 bg-purple-400 rounded-full"></div>
            </div>
            <p class="mt-3 text-sm text-center font-medium text-slate-700">
              <span class="text-red-600 font-bold">胖胖的圆球：</span><br />
              测出来的数据忽高忽低，<br />不靠谱。
            </p>
          </div>
        </div>
      </div>
    </div>
  </section>

  <section id="quartet-rna">
    <h3 class="text-3xl font-bold text-heading mt-8 mb-6">Quartet转录组数据文件示例</h3>
    <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
      <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
        <p class="mb-2">运行 QC 流程需要准备三个输入文件：</p>

        <!-- 表达矩阵部分 -->
        <div class="flex items-center mb-2">
          <div>
            <h2 class="text-2xl font-bold text-gray-800">1. 表达矩阵 (Expression Metrix)</h2>
          </div>
        </div>

        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-list-alt text-blue-500 mr-2"></i>
              结构要求
            </h3>
            <ul class="space-y-3 pl-2">
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">第一列必须是Ensembl ID</span>
                  <p class="text-gray-600 text-sm mt-1">
                    列名为 <span class="font-mono bg-gray-100 px-2 py-1 rounded">GENE_ID</span>,
                  </p>
                </div>
              </li>
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">其余列名中的样本名称需要与元数据文件中的library列一致</span>
                  <p class="text-gray-600 text-sm mt-1">经过fpkm转化的RNA表达矩阵</p>
                </div>
              </li>
            </ul>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-table text-blue-500 mr-2"></i>
              示例格式
            </h3>
            <div class="overflow-x-auto rounded-xl border border-gray-200">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-blue-50">
                  <tr>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      GENE_ID
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0001_20260224_test_d5
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0001_20260224_test_d6
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0002_20260224_test_d5
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0002_20260224_test_d6
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      ...
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">ENSG00000000457</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">2.19</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">2.14</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">2.07</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">2.12</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">ENSG00000001629</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">19.07</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">20.47</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">13.16</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">15.26</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- 表达量部分 -->
        <div class="flex items-center mt-2 mb-2">
          <div>
            <h2 class="text-2xl font-bold text-gray-800">2. 表达量 (Expression Count)</h2>
          </div>
        </div>

        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-list-alt text-blue-500 mr-2"></i>
              结构要求
            </h3>
            <ul class="space-y-3 pl-2">
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">第一列必须是Ensembl ID</span>
                  <p class="text-gray-600 text-sm mt-1">
                    列名为 <span class="font-mono bg-gray-100 px-2 py-1 rounded">GENE_ID</span>
                  </p>
                </div>
              </li>
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">其余列名中的样本名称需要与元数据文件中的library列一致</span>
                  <p class="text-gray-600 text-sm mt-1">为原始的counts数</p>
                </div>
              </li>
            </ul>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-table text-blue-500 mr-2"></i>
              示例格式
            </h3>
            <div class="overflow-x-auto rounded-xl border border-gray-200">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-blue-50">
                  <tr>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      GENE_ID
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0001_20260224_test_d5
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0001_20260224_test_d6
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0002_20260224_test_d5
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0002_20260224_test_d6
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      ...
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">ENSG00000000457</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">7</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">12</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">16</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">13</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">ENSG00000001629</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">68</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">67</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">76</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">72</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- 元数据部分 -->
        <div class="flex items-center mt-2 mb-2">
          <div>
            <h2 class="text-2xl font-bold text-gray-800">3. 元数据 (Metadata)</h2>
          </div>
        </div>

        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-list-alt text-green-500 mr-2"></i>
              结构要求（列名不区分大小写）
            </h3>
            <ul class="space-y-3 pl-2">
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">第一列是 group</span>
                  <p class="text-gray-600 text-sm mt-1">为技术重复编号</p>
                </div>
              </li>
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">第二列是 library</span>
                  <p class="text-gray-600 text-sm mt-1">
                    必须与表达矩阵和表达量的列名（样本ID）<span class="font-bold">完全一致</span
                    >，命名规范为：【批次号_测序日期_其他描述性信息】，例如：BATCH0001_20260224_xxxx
                  </p>
                </div>
              </li>
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">第三列是 sample</span>
                  <p class="text-gray-600 text-sm mt-1">为每个文件对应的样本编号,（D5, D6, F7, M8之一）</p>
                </div>
              </li>
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium"
                    >必须要<span class="font-bold">≥2个</span>技术重复，并且包含标准品<span class="font-bold"
                      >D6</span
                    ></span
                  >
                </div>
              </li>
            </ul>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-table text-green-500 mr-2"></i>
              示例格式
            </h3>
            <div class="overflow-x-auto rounded-xl border border-gray-200">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-green-50">
                  <tr>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                      group
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                      library
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                      sample
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">D5_1</td>
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                      BATCH0001_20260224_test_d5
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span
                        class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                        >D5</span
                      >
                    </td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">D6_1</td>
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                      BATCH0001_20260224_test_d6
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span
                        class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                        >D6</span
                      >
                    </td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">D5_2</td>
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                      BATCH0002_20260224_test_d5
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span
                        class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                        >D5</span
                      >
                    </td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">D6_2</td>
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                      BATCH0002_20260224_test_d6
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span
                        class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                        >D6</span
                      >
                    </td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">...</td>
                    <td class="px-6 py-4 whitespace-nowrap">...</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>

  <section id="quartet-protein">
    <h3 class="text-3xl font-bold text-heading mt-8 mb-6">Quartet蛋白质组数据文件示例</h3>
    <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
      <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
        <p class="mb-2">运行 QC 流程需要准备两个输入文件：</p>

        <!-- 表达矩阵部分 -->
        <div class="flex items-center mb-2">
          <div>
            <h2 class="text-2xl font-bold text-gray-800">1. 表达矩阵 (Expression Matrix)</h2>
          </div>
        </div>

        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-list-alt text-blue-500 mr-2"></i>
              结构要求
            </h3>
            <ul class="space-y-3 pl-2">
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">第一列必须是特征类型</span>
                  <p class="text-gray-600 text-sm mt-1">
                    列名为 <span class="font-mono bg-gray-100 px-2 py-1 rounded">Type</span>, 值为
                    <span class="font-mono bg-gray-100 px-2 py-1 rounded">Gene Symbol</span> 或
                    <span class="font-mono bg-gray-100 px-2 py-1 rounded">Peptide Sequence</span>
                  </p>
                </div>
              </li>
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">第二列必须是特征的具体名称</span>
                  <p class="text-gray-600 text-sm mt-1">
                    列名为 <span class="font-mono bg-gray-100 px-2 py-1 rounded">Feature</span>， 如Gene
                    Symbol对应AAAS等基因名称， Peptide Sequence对应NIQVDE等氨基酸序列，<span
                      class="font-mono bg-gray-100 px-2 py-1 rounded"
                      >请注意基因名称不能重复，也必须同步提供Peptide层面的数据</span
                    >
                  </p>
                </div>
              </li>
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">其余列名中的样本名称需要与元数据文件中的library列一致</span>
                  <p class="text-gray-600 text-sm mt-1">建议为 Log2 转换后的值</p>
                </div>
              </li>
            </ul>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-table text-blue-500 mr-2"></i>
              示例格式
            </h3>
            <div class="overflow-x-auto rounded-xl border border-gray-200">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-blue-50">
                  <tr>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      Type
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      Feature
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0001_20260224_test_d5
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0001_20260224_test_d6
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0002_20260224_test_d5
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      BATCH0002_20260224_test_d6
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                      ...
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">Gene Symbol</td>
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">AAAS</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">2.19</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">2.14</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">2.07</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">2.09</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">Peptide Sequence</td>
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">INDFFLR</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">8910.07</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">7226.47</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">7832.16</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">7654.28</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- 元数据部分 -->

        <div class="flex items-center mt-2 mb-2">
          <div>
            <h2 class="text-2xl font-bold text-gray-800">2. 元数据 (Metadata)</h2>
          </div>
        </div>

        <div class="space-y-6">
          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-list-alt text-green-500 mr-2"></i>
              结构要求（列名不区分大小写）
            </h3>
            <ul class="space-y-3 pl-2">
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">第一列是 library</span>
                  <p class="text-gray-600 text-sm mt-1">
                    必须与表达矩阵的列名（样本ID）<span class="font-bold">完全一致</span>，
                    命名规范为：【批次号_测序日期_其他描述性信息】，例如：BATCH0001_20260224_xxxx
                  </p>
                </div>
              </li>
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium">第二列是 sample</span>
                  <p class="text-gray-600 text-sm mt-1">为每个文件对应的样本编号,（D5, D6, F7, M8之一）</p>
                </div>
              </li>
              <li class="flex items-start">
                <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
                <div>
                  <span class="font-medium"
                    >必须要<span class="font-bold">≥2个</span>技术重复，并且包含标准品<span class="font-bold"
                      >D6</span
                    ></span
                  >
                </div>
              </li>
            </ul>
          </div>

          <div>
            <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
              <i class="fas fa-table text-green-500 mr-2"></i>
              示例格式
            </h3>
            <div class="overflow-x-auto rounded-xl border border-gray-200">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-green-50">
                  <tr>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                      library
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                      sample
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                      BATCH0001_20260224_test_d5
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span
                        class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                        >D5</span
                      >
                    </td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                      BATCH0001_20260224_test_d6
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span
                        class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                        >D6</span
                      >
                    </td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                      BATCH0002_20260224_test_d5
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span
                        class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                        >D5</span
                      >
                    </td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                      BATCH0002_20260224_test_d6
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <span
                        class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                        >D6</span
                      >
                    </td>
                  </tr>
                  <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">...</td>
                    <td class="px-6 py-4 whitespace-nowrap">...</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>

  <section id="quartet-metabolism">
    <h3 class="text-3xl font-bold text-heading mt-8 mb-6">Quartet代谢组数据文件示例</h3>
    <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
      <p class="mb-2">运行 QC 流程需要准备两个输入文件：</p>

      <!-- 表达矩阵部分 -->
      <div class="flex items-center mb-2">
        <div>
          <h2 class="text-2xl font-bold text-gray-800">1. 表达矩阵 (Expression Matrix)</h2>
        </div>
      </div>

      <div class="space-y-6">
        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-list-alt text-blue-500 mr-2"></i>
            结构要求
          </h3>
          <ul class="space-y-3 pl-2">
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第一列必须是代谢物名称</span>
                <p class="text-gray-600 text-sm mt-1">
                  列名为 <span class="font-mono bg-gray-100 px-2 py-1 rounded">metabolites</span>
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第二列必须是 HMDBID</span>
                <p class="text-gray-600 text-sm mt-1">
                  列名为 <span class="font-mono bg-gray-100 px-2 py-1 rounded">HMDBID</span>
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">其余列名中的样本名称需要与元数据文件中的col_names列一致</span>
                <p class="text-gray-600 text-sm mt-1">建议为 Log2 转换后的值</p>
              </div>
            </li>
          </ul>
        </div>

        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-table text-blue-500 mr-2"></i>
            示例格式
          </h3>
          <div class="overflow-x-auto rounded-xl border border-gray-200">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-blue-50">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    metabolites
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    HMDBID
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0001_20260224_test_d5
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0001_20260224_test_d6
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0002_20260224_test_d5
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0002_20260224_test_d6
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">...</th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">Palmitic acid</td>
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">HMDB0000220</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">32408251.42</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">15088675.58</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">13452134.8</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">23567834.8</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">Stearic acid</td>
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">HMDB0000827</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">18071143.07</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10455852.47</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10157017.16</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">12357534.16</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- 元数据部分 -->

      <div class="flex items-center mt-2 mb-2">
        <div>
          <h2 class="text-2xl font-bold text-gray-800">2. 元数据 (Metadata)</h2>
        </div>
      </div>

      <div class="space-y-6">
        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-list-alt text-green-500 mr-2"></i>
            结构要求（列名不区分大小写）
          </h3>
          <ul class="space-y-3 pl-2">
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第一列是 col_names</span>
                <p class="text-gray-600 text-sm mt-1">
                  必须与表达矩阵的列名（样本ID）<span class="font-bold">完全一致</span>，
                  命名规范为：【批次号_测序日期_其他描述性信息】，例如：BATCH0001_20260224_xxxx
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第二列是 sample</span>
                <p class="text-gray-600 text-sm mt-1">为每个文件对应的样本编号,（D5, D6, F7, M8之一）</p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium"
                  >必须要<span class="font-bold">≥2个</span>技术重复，并且包含标准品<span class="font-bold"
                    >D6</span
                  ></span
                >
              </div>
            </li>
          </ul>
        </div>

        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-table text-green-500 mr-2"></i>
            示例格式
          </h3>
          <div class="overflow-x-auto rounded-xl border border-gray-200">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-green-50">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                    col_names
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                    sample
                  </th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                    BATCH0001_20260224_test_d5
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                      >D5</span
                    >
                  </td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                    BATCH0001_20260224_test_d6
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                      >D6</span
                    >
                  </td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                    BATCH0002_20260224_test_d5
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                      >D5</span
                    >
                  </td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">
                    BATCH0002_20260224_test_d6
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                      >D6</span
                    >
                  </td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">...</td>
                  <td class="px-6 py-4 whitespace-nowrap">...</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </section>

  <section id="plasmix-protein">
    <h3 class="text-3xl font-bold text-heading mt-8 mb-6">Plasmix蛋白质组数据文件示例</h3>
    <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
      <p class="mb-2">运行 QC 流程需要准备两个输入文件：</p>

      <!-- 表达矩阵部分 -->
      <div class="flex items-center mb-2">
        <div>
          <h2 class="text-2xl font-bold text-gray-800">1. 表达矩阵 (Expression Matrix)</h2>
        </div>
      </div>

      <div class="space-y-6">
        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-list-alt text-blue-500 mr-2"></i>
            结构要求
          </h3>
          <ul class="space-y-3 pl-2">
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第一列必须是 特征 ID</span>
                <p class="text-gray-600 text-sm mt-1">
                  列名为：<span class="font-bold">Feature</span>， 值为<span
                    class="font-mono bg-gray-100 px-2 py-1 rounded"
                    >UniProt ID</span
                  >
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">其余列名中的样本名称需要与元数据文件中的library列一致</span>
                <p class="text-gray-600 text-sm mt-1">
                  <span class="font-bold">数值：</span>建议输入仪器导出的
                  <span class="font-bold">Log2 转换后的信号值</span>（程序内部会自动处理还原计算）
                </p>
              </div>
            </li>
          </ul>
        </div>

        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-table text-blue-500 mr-2"></i>
            示例格式
          </h3>
          <div class="overflow-x-auto rounded-xl border border-gray-200">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-blue-50">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    Feature
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0001_20260224_test_f
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0001_20260224_test_p
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0002_20260224_test_f
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0002_20260224_test_p
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">...</th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">P00740</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">14.2</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">13.8</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">14.1</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">15.4</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">P00915</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10.5</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10.1</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10.9</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10.2</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- 元数据部分 -->

      <div class="flex items-center mt-2 mb-2">
        <div>
          <h2 class="text-2xl font-bold text-gray-800">2. 元数据 (Metadata)</h2>
        </div>
      </div>

      <div class="space-y-6">
        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-list-alt text-green-500 mr-2"></i>
            结构要求（列名不区分大小写）
          </h3>
          <ul class="space-y-3 pl-2">
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第一列是 library</span>
                <p class="text-gray-600 text-sm mt-1">
                  必须与表达矩阵的列名（样本ID）<span class="font-bold">完全一致</span>，
                  命名规范为：【批次号_测序日期_其他描述性信息】，例如：BATCH0001_20260224_xxxx
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第二列是 sample</span>
                <p class="text-gray-600 text-sm mt-1">
                  样本类型。必须包含 P（作为参考样本），以及 M, Y, X, F 等测试样本
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第三列 platform</span>
                <p class="text-gray-600 text-sm mt-1">
                  检测平台名称，必须准确填写为 <span class="font-bold">SomaScan</span>,
                  <span class="font-bold">Olink</span> 或
                  <span class="font-bold">DIA</span>，这将决定参考数据集的选择和计算逻辑
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium"
                  >必须要<span class="font-bold">≥2个</span>技术重复，并且包含标准品<span class="font-bold"
                    >P</span
                  ></span
                >
              </div>
            </li>
          </ul>
        </div>

        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-table text-green-500 mr-2"></i>
            示例格式
          </h3>
          <div class="overflow-x-auto rounded-xl border border-gray-200">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-green-50">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                    library
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                    sample
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                    platform
                  </th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">BATCH0001_20260224_test_f</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                      >F</span
                    >
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">DIA</td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">BATCH0001_20260224_test_p</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                      >P</span
                    >
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">DIA</td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">BATCH0002_20260224_test_f</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                      >F</span
                    >
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">DIA</td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">BATCH0002_20260224_test_p</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                      >P</span
                    >
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">DIA</td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">...</td>
                  <td class="px-6 py-4 whitespace-nowrap">...</td>
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">...</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </section>

  <section id="plasmix-metabolism">
    <h3 class="text-3xl font-bold text-heading mt-8 mb-6">Plasmix代谢组数据文件示例</h3>
    <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
      <p class="mb-2">运行 QC 流程需要准备两个输入文件：</p>

      <!-- 表达矩阵部分 -->
      <div class="flex items-center mb-2">
        <div>
          <h2 class="text-2xl font-bold text-gray-800">1. 表达矩阵 (Expression Matrix)</h2>
        </div>
      </div>

      <div class="space-y-6">
        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-list-alt text-blue-500 mr-2"></i>
            结构要求
          </h3>
          <ul class="space-y-3 pl-2">
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第一列必须是 Feature ID</span>
                <p class="text-gray-600 text-sm mt-1">
                  列名为 <span class="font-mono bg-gray-100 px-2 py-1 rounded">Feature</span>，值为
                  <span class="font-mono bg-gray-100 px-2 py-1 rounded">HMDBID</span>
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">其余列名中的样本名称需要与元数据文件中的library列一致</span>
                <p class="text-gray-600 text-sm mt-1">建议为 Log2 转换后的值</p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-exclamation-triangle text-yellow-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">重复处理</span>
                <p class="text-gray-600 text-sm mt-1">
                  如果存在重复的 Feature ID，程序会自动添加后缀（如 <span class="font-mono">.1</span>,
                  <span class="font-mono">.2</span>）进行区分
                </p>
              </div>
            </li>
          </ul>
        </div>

        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-table text-blue-500 mr-2"></i>
            示例格式
          </h3>
          <div class="overflow-x-auto rounded-xl border border-gray-200">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-blue-50">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    Feature
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0001_20260224_test_m
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0001_20260224_test_p
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0002_20260224_test_m
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">
                    BATCH0002_20260224_test_p
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-blue-800 uppercase tracking-wider">...</th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">HMDB00001</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">14.2</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">13.8</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">14.1</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">14.6</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">HMDB00002</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10.5</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10.1</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10.9</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">10.3</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">...</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- 元数据部分 -->

      <div class="flex items-center mt-2 mb-2">
        <div>
          <h2 class="text-2xl font-bold text-gray-800">2. 元数据 (Metadata)</h2>
        </div>
      </div>

      <div class="space-y-6">
        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-list-alt text-green-500 mr-2"></i>
            结构要求（列名不区分大小写）
          </h3>
          <ul class="space-y-3 pl-2">
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第一列是 library</span>
                <p class="text-gray-600 text-sm mt-1">
                  必须与表达矩阵的列名（样本ID）<span class="font-bold">完全一致</span>，
                  命名规范为：【批次号_测序日期_其他描述性信息】，例如：BATCH0001_20260224_xxxx
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第二列是 sample</span>
                <p class="text-gray-600 text-sm mt-1">
                  样本组别名称。必须包含 P 样（作为参考分母），以及 M, Y, X, F 中的任意组合
                </p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-exclamation-triangle text-yellow-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium">第三列 platform</span>
                <p class="text-gray-600 text-sm mt-1">不再强制要求</p>
              </div>
            </li>
            <li class="flex items-start">
              <i class="fas fa-check-circle text-green-500 mt-1 mr-3"></i>
              <div>
                <span class="font-medium"
                  >必须要<span class="font-bold">≥2个</span>技术重复，并且包含标准品<span class="font-bold"
                    >P</span
                  ></span
                >
              </div>
            </li>
          </ul>
        </div>

        <div>
          <h3 class="text-lg font-semibold text-gray-700 mb-3 flex items-center">
            <i class="fas fa-table text-green-500 mr-2"></i>
            示例格式
          </h3>
          <div class="overflow-x-auto rounded-xl border border-gray-200">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-green-50">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                    library
                  </th>
                  <th class="px-6 py-3 text-left text-xs font-semibold text-green-800 uppercase tracking-wider">
                    sample
                  </th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">BATCH0001_20260224_test_m</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                      >M</span
                    >
                  </td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">BATCH0001_20260224_test_p</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                      >P</span
                    >
                  </td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">BATCH0002_20260224_test_m</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-purple-100 text-purple-800"
                      >M</span
                    >
                  </td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">BATCH0002_20260224_test_p</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-3 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800"
                      >P</span
                    >
                  </td>
                </tr>
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap font-mono text-sm text-gray-900">...</td>
                  <td class="px-6 py-4 whitespace-nowrap">...</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup>
import { ref } from 'vue'

const isDNABtnActive = ref(true)
const clzBTNActive = 'px-6 py-2 rounded-full text-sm font-medium transition-all bg-blue-600 text-white shadow-md'
const clzBTNInactive = 'px-6 py-2 rounded-full text-sm font-medium transition-all text-slate-500 hover:text-slate-700'
</script>
