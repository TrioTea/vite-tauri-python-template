<template>
  <v-container class="fill-height" max-width="1400">
    <div class="page-container">
      <!-- Status Info -->
      <v-expand-transition>
        <v-card
          v-if="status.connected"
          class="status-card mb-6"
          color="success"
          elevation="8"
          variant="elevated"
        >
          <v-card-title class="d-flex align-center">
            <v-icon class="mr-2" size="24">mdi-check-circle</v-icon>
            连接状态：已连接
          </v-card-title>
          <v-card-text>
            <v-sheet class="info-sheet pa-4" rounded>
              <pre class="info-text">{{ status.info }}</pre>
            </v-sheet>
          </v-card-text>
        </v-card>
      </v-expand-transition>

      <!-- Logs Display -->
      <v-card class="logs-card mb-6" elevation="4" variant="elevated">
        <v-card-title class="d-flex align-center">
          <v-icon class="mr-2" color="primary">mdi-console</v-icon>
          服务器日志
          <v-spacer />
          <v-btn
            icon
            size="small"
            title="清空日志"
            variant="text"
            @click="clearLogs"
          >
            <v-icon>mdi-delete-sweep</v-icon>
          </v-btn>
        </v-card-title>
        <v-card-text class="pa-0">
          <v-sheet class="logs-container" color="grey-lighten-5">
            <div class="logs-content">
              {{ logs }}
            </div>
          </v-sheet>
        </v-card-text>
      </v-card>

      <!-- Action Buttons -->
      <div class="actions-section">
        <h2 class="text-h5 font-weight-bold mb-4 text-center">操作面板</h2>
        <v-row>
          <v-col
            v-for="(action, index) in actions"
            :key="index"
            cols="12"
            md="3"
            sm="6"
          >
            <v-hover v-slot="{ isHovering, props }">
              <v-card
                v-bind="props"
                class="action-card h-100"
                :class="{ 'card-hover': isHovering }"
                :color="action.color"
                :disabled="action.disabled"
                :elevation="isHovering ? 12 : 4"
                variant="elevated"
                @click="action.onClick"
              >
                <v-card-text class="pa-6 text-center">
                  <v-avatar
                    class="mb-4"
                    :color="action.disabled ? 'grey' : action.iconColor"
                    size="64"
                  >
                    <v-icon color="white" size="32">{{ action.icon }}</v-icon>
                  </v-avatar>
                  <h3 class="text-h6 font-weight-bold mb-2">
                    {{ action.title }}
                  </h3>
                  <p class="text-body-2 mb-0">{{ action.description }}</p>
                  <v-progress-linear
                    v-if="action.loading"
                    class="mt-3"
                    color="white"
                    indeterminate
                  />
                </v-card-text>
              </v-card>
            </v-hover>
          </v-col>
        </v-row>
      </div>

      <!-- Loading Overlay -->
      <v-overlay
        v-model="isLoading"
        class="align-center justify-center"
        persistent
      >
        <v-card class="pa-6 text-center" min-width="300">
          <v-progress-circular
            class="mb-4"
            color="primary"
            indeterminate
            size="64"
          />
          <div class="text-h6">{{ loadingMessage }}</div>
        </v-card>
      </v-overlay>
    </div>
  </v-container>
</template>

<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { listen } from '@tauri-apps/api/event'
  import { computed, onMounted, onUnmounted, ref } from 'vue'

  // 常量定义
  const docsUrl
    = 'https://github.com/dieharders/example-tauri-v2-python-server-sidecar'
  const DOMAIN = 'localhost'
  const PORT = '8008'

  // 响应式状态
  const status = ref({
    connected: false,
    info: '',
  })
  const logs = ref('[ui] 监听 sidecar 和网络日志中...')
  const isLoading = ref(false)
  const loadingMessage = ref('')

  // 清理函数引用
  let unlistenStdout: (() => void) | null = null
  let unlistenStderr: (() => void) | null = null
  let keydownListener: ((event: KeyboardEvent) => void) | null = null

  // 操作按钮配置
  const actions = computed(() => [
    {
      title: status.value.connected ? '已连接' : '连接主机',
      description: status.value.connected
        ? status.value.info
        : '建立与 API 服务器的连接',
      icon: status.value.connected ? 'mdi-check-circle' : 'mdi-server-network',
      iconColor: status.value.connected ? 'success' : 'primary',
      color: status.value.connected ? 'success' : 'primary',
      disabled: status.value.connected,
      loading: false,
      onClick: connectServerAction,
    },
    {
      title: '模拟 API',
      description: '从模拟端点获取 API 服务器响应示例',
      icon: 'mdi-api',
      iconColor: 'info',
      color: 'info',
      disabled: false,
      loading: false,
      onClick: mockAPIAction,
    },
    {
      title: '启动 Sidecar',
      description: '初始化一个新的 sidecar 进程',
      icon: 'mdi-play-circle',
      iconColor: 'success',
      color: 'success',
      disabled: false,
      loading: false,
      onClick: startSidecarAction,
    },
    {
      title: '停止 Sidecar',
      description: '强制关闭 sidecar 进程',
      icon: 'mdi-stop-circle',
      iconColor: 'error',
      color: 'error',
      disabled: false,
      loading: false,
      onClick: shutdownSidecarAction,
    },
  ])

  // 清空日志
  const clearLogs = () => {
    logs.value = '[ui] 日志已清空...'
  }

  // 显示加载状态
  const showLoading = (message: string) => {
    loadingMessage.value = message
    isLoading.value = true
  }

  // 隐藏加载状态
  const hideLoading = () => {
    isLoading.value = false
  }

  // 初始化 Sidecar 监听器
  const initSidecarListeners = async () => {
    try {
      console.log('[ui] 正在初始化 sidecar 监听器...')

      // 监听 stdout
      unlistenStdout = await listen('sidecar-stdout', (event: any) => {
        console.log('Sidecar stdout:', event.payload)
        if (
          event.payload
          && event.payload.length > 0
          && event.payload !== '\r\n'
        ) {
          logs.value += `\n${event.payload}`
        }
      })

      // 监听 stderr
      unlistenStderr = await listen('sidecar-stderr', (event: any) => {
        console.error('Sidecar stderr:', event.payload)
        if (
          event.payload
          && event.payload.length > 0
          && event.payload !== '\r\n'
        ) {
          logs.value += `\n${event.payload}`
        }
      })

      console.log('[ui] Sidecar 监听器初始化完成')
    } catch (error) {
      console.error('[ui] 初始化 sidecar 监听器失败:', error)
      logs.value += `\n[ui] 初始化 sidecar 监听器失败: ${error}`
    }
  }

  // API 调用函数
  const apiAction = async (endpoint: string, method = 'GET', payload?: any) => {
    const url = `http://${DOMAIN}:${PORT}/${endpoint}`
    try {
      const body = payload ? JSON.stringify(payload) : null
      const headers = {
        'Content-Type': 'application/json',
      }

      const res = await fetch(url, { method, headers, body })
      if (!res.ok) {
        throw new Error(`Response status: ${res.status}`)
      }
      const json = await res.json()
      console.log(json)

      // 成功响应
      if (json?.message) {
        logs.value += `\n[server-response] ${json.message}`
      }
      return json
    } catch (error) {
      console.error(`[server-response] ${error}`)
      logs.value += `\n[server-response] ${error}`
      throw error
    }
  }

  // 连接服务器
  const connectServerAction = async () => {
    try {
      showLoading('正在连接服务器...')
      console.log('[ui] 尝试连接服务器...')
      logs.value += '\n[ui] 尝试连接服务器...'
      const result = await apiAction('v1/connect')
      if (result) {
        status.value = {
          connected: true,
          info: `Host: ${result.data.host}\nProcess id: ${result.data.pid}\nDocs: ${result.data.host}/docs`,
        }
        logs.value += '\n[ui] 服务器连接成功'
      }
    } catch (error) {
      console.error(`[ui] Failed to connect to api server. ${error}`)
      logs.value += `\n[ui] 连接服务器失败: ${error}`
    } finally {
      hideLoading()
    }
  }

  // 关闭 Sidecar
  const shutdownSidecarAction = async () => {
    try {
      showLoading('正在关闭 Sidecar...')
      console.log('[ui] 尝试关闭 sidecar...')
      logs.value += '\n[ui] 尝试关闭 sidecar...'
      const result = await invoke('shutdown_sidecar')
      console.log('[ui] Sidecar 关闭命令结果:', result)
      logs.value += `\n[ui] Sidecar 关闭命令结果: ${result}`
      if (result) {
        status.value = {
          connected: false,
          info: '',
        }
      }
    } catch (error) {
      console.error(`[ui] Failed to shutdown sidecar. ${error}`)
      logs.value += `\n[ui] 关闭 sidecar 失败: ${error}`
    } finally {
      hideLoading()
    }
  }

  // 启动 Sidecar
  const startSidecarAction = async () => {
    try {
      showLoading('正在启动 Sidecar...')
      console.log('[ui] 尝试启动 sidecar...')
      logs.value += '\n[ui] 尝试启动 sidecar...'
      const result = await invoke('start_sidecar')
      console.log('[ui] Sidecar 启动命令结果:', result)
      logs.value += `\n[ui] Sidecar 启动命令结果: ${result}`
    } catch (error) {
      console.error(`[ui] Failed to start sidecar. ${error}`)
      logs.value += `\n[ui] 启动 sidecar 失败: ${error}`
    } finally {
      hideLoading()
    }
  }

  // 模拟 API 调用
  const mockAPIAction = async () => {
    try {
      showLoading('正在调用模拟 API...')
      console.log('[ui] 尝试调用模拟 API...')
      logs.value += '\n[ui] 尝试调用模拟 API...'
      await apiAction('v1/completions', 'POST', { prompt: 'An example query.' })
    } catch (error) {
      console.error(`[ui] Failed to get llm completion. ${error}`)
      logs.value += `\n[ui] 模拟 API 调用失败: ${error}`
    } finally {
      hideLoading()
    }
  }

  // 组件挂载时的初始化
  onMounted(async () => {
    console.log('[ui] 组件挂载，开始初始化...')

    // 初始化 Sidecar 监听器
    await initSidecarListeners()

    // 监听 F11 键切换全屏
    keydownListener = (event: KeyboardEvent) => {
      if (event.key === 'F11') {
        event.preventDefault()
        console.log('[ui] F11 pressed, toggling fullscreen')
        invoke('toggle_fullscreen').catch(error => {
          console.error('[ui] 切换全屏失败:', error)
        })
      }
    }
    window.addEventListener('keydown', keydownListener)

    console.log('[ui] 组件初始化完成')
  })

  // 组件卸载时的清理
  onUnmounted(() => {
    console.log('[ui] 组件卸载，清理资源...')

    // 清理监听器
    if (unlistenStdout) unlistenStdout()
    if (unlistenStderr) unlistenStderr()
    if (keydownListener) {
      window.removeEventListener('keydown', keydownListener)
    }

    console.log('[ui] 资源清理完成')
  })
</script>

<style scoped>
.page-container {
  padding: 20px 0;
}

.header-section {
  position: relative;
  overflow: hidden;
}

.header-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%) !important;
  color: white;
  border-radius: 16px !important;
}

.gradient-text {
  background: linear-gradient(45deg, #ffffff, #e3f2fd);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.status-card {
  border-radius: 12px !important;
  backdrop-filter: blur(10px);
  animation: slideInDown 0.5s ease-out;
}

.info-sheet {
  background: rgba(255, 255, 255, 0.1) !important;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.info-text {
  color: white;
  font-family: "Roboto Mono", monospace;
  font-size: 0.875rem;
  line-height: 1.5;
  margin: 0;
}

.logs-card {
  border-radius: 12px !important;
  overflow: hidden;
}

.logs-container {
  height: 320px;
  overflow-y: auto;
  border-radius: 0 0 12px 12px;
}

.logs-content {
  padding: 16px;
  font-family: "Roboto Mono", monospace;
  font-size: 0.875rem;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-word;
  min-height: 100%;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.actions-section {
  margin-top: 24px;
}

.action-card {
  border-radius: 16px !important;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  cursor: pointer;
  position: relative;
  overflow: hidden;
}

.action-card:not(.v-card--disabled):hover {
  transform: translateY(-4px);
}

.card-hover {
  transform: translateY(-4px) !important;
}

.action-card:before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.2),
    transparent
  );
  transition: left 0.5s;
}

.action-card:hover:before {
  left: 100%;
}

.v-card--disabled {
  opacity: 0.7 !important;
  cursor: not-allowed !important;
}

/* 滚动条样式 */
.logs-container::-webkit-scrollbar {
  width: 8px;
}

.logs-container::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

.logs-container::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

.logs-container::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

/* 动画效果 */
@keyframes slideInDown {
  from {
    opacity: 0;
    transform: translate3d(0, -30px, 0);
  }
  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translate3d(0, 30px, 0);
  }
  to {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
}

.action-card {
  animation: fadeInUp 0.6s ease-out;
}

.action-card:nth-child(1) {
  animation-delay: 0.1s;
}
.action-card:nth-child(2) {
  animation-delay: 0.2s;
}
.action-card:nth-child(3) {
  animation-delay: 0.3s;
}
.action-card:nth-child(4) {
  animation-delay: 0.4s;
}

/* 响应式设计 */
@media (max-width: 600px) {
  .page-container {
    padding: 10px 0;
  }

  .header-card .v-card-text {
    padding: 24px 16px !important;
  }

  .text-h3 {
    font-size: 1.75rem !important;
  }

  .action-card .v-card-text {
    padding: 24px 16px !important;
  }

  .logs-container {
    height: 250px;
  }
}

/* 深色主题适配 */
.v-theme--dark .logs-content {
  background: linear-gradient(135deg, #1a1a1a 0%, #2d2d30 100%);
  color: #e0e0e0;
}

.v-theme--dark .info-sheet {
  background: rgba(255, 255, 255, 0.05) !important;
}

.v-theme--dark .logs-container::-webkit-scrollbar-track {
  background: #2d2d30;
}

.v-theme--dark .logs-container::-webkit-scrollbar-thumb {
  background: #555;
}

.v-theme--dark .logs-container::-webkit-scrollbar-thumb:hover {
  background: #777;
}
</style>
