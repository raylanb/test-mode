<template>
  <div class="timer-container">
    <div class="timer-card">
      <div class="mb-6">
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          目标时间（分钟）
        </label>
        <input
          type="number"
          v-model="targetDuration"
          min="30"
          max="120"
          step="5"
          class="block w-full px-3 py-2 text-gray-900 dark:text-white bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400"
        >
      </div>

      <div class="grid grid-cols-2 gap-6 mb-8">
        <div class="time-display work-time" :class="{ 'active': isWorkMode }">
          <div class="time-label text-gray-700 dark:text-gray-300">工作时间</div>
          <div class="time-value text-gray-900 dark:text-white">{{ formatTime(currentWorkPeriodTime) }}</div>
          <div class="time-total text-sm text-gray-600 dark:text-gray-400 mt-2">
            总计: {{ formatTime(elapsedTime) }}
          </div>
        </div>
        <div class="time-display rest-time" :class="{ 'active': !isWorkMode }">
          <div class="time-label text-gray-700 dark:text-gray-300">休息时间</div>
          <div class="time-value text-gray-900 dark:text-white">{{ formatTime(totalRestTime) }}</div>
        </div>
      </div>

      <div class="mb-8">
        <div class="relative pt-1">
          <div class="flex mb-2 items-center justify-between">
            <div class="text-right">
              <span class="text-sm font-semibold inline-block text-gray-700 dark:text-gray-300">
                {{ progressPercentage.toFixed(1) }}%
              </span>
            </div>
          </div>
          <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-gray-200 dark:bg-gray-700">
            <div
              :style="{ width: progressPercentage + '%' }"
              class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-blue-500 dark:bg-blue-400"
            ></div>
          </div>
        </div>
      </div>

      <div class="flex justify-center space-x-4 mb-8">
        <button
          @click="toggleTimer"
          class="px-6 py-3 text-base font-medium rounded-lg shadow-sm text-white bg-blue-500 dark:bg-blue-400 hover:bg-blue-600 dark:hover:bg-blue-500 focus:outline-none focus:ring-0 active:bg-blue-600 dark:active:bg-blue-500 transition-colors duration-200"
        >
          {{ buttonText }}
        </button>
        <button
          @click="resetTimer"
          class="px-6 py-3 text-base font-medium rounded-lg shadow-sm text-white bg-gray-500 dark:bg-gray-400 hover:bg-gray-600 dark:hover:bg-gray-500 focus:outline-none focus:ring-0 active:bg-gray-600 dark:active:bg-gray-500 transition-colors duration-200"
        >
          重置
        </button>
      </div>

      <div class="stats-card">
        <h3 class="text-lg font-semibold mb-4 text-gray-800 dark:text-white">统计信息</h3>
        <div class="grid grid-cols-2 gap-6">
          <div class="stat-item">
            <div class="stat-label text-gray-700 dark:text-gray-300">{{ isWorkMode ? '剩余工作' : '剩余休息' }}</div>
            <div class="stat-value text-gray-900 dark:text-white">{{ formatTime(remainingTime) }}</div>
          </div>
          <div class="stat-item">
            <div class="stat-label text-gray-700 dark:text-gray-300">目标达成率</div>
            <div class="stat-value text-gray-900 dark:text-white">{{ progressPercentage.toFixed(1) }}%</div>
          </div>
          <div class="stat-item">
            <div class="stat-label text-gray-700 dark:text-gray-300">开始时间</div>
            <div class="stat-value text-gray-900 dark:text-white">{{ startTimeFormatted }}</div>
          </div>
          <div class="stat-item">
            <div class="stat-label text-gray-700 dark:text-gray-300">预计完成</div>
            <div class="stat-value text-gray-900 dark:text-white">{{ estimatedEndTimeFormatted }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onUnmounted } from 'vue'
import confetti from 'canvas-confetti'

const targetDuration = ref(60) // 默认60分钟
const elapsedTime = ref(0) // 已经过时间（秒）
const restTime = ref(0) // 休息时间（秒）
const workTimeAtLastRest = ref(0) // 上次休息时的工作时间
const isRunning = ref(false)
const isWorkMode = ref(true)
const startTime = ref(null) // 总开始时间
const currentPeriodStartTime = ref(null) // 当前工作时段的开始时间
const accumulatedRestTime = ref(0) // 累积的休息时间
let timer = null

// 计算属性
const progressPercentage = computed(() => {
  if (!targetDuration.value) return 0
  // 使用总工作时间计算进度
  return Math.min((elapsedTime.value / (targetDuration.value * 60)) * 100, 100)
})

const remainingTime = computed(() => {
  if (isWorkMode.value) {
    // 使用总工作时间计算剩余时间
    return Math.max(targetDuration.value * 60 - elapsedTime.value, 0)
  } else {
    return Math.max(restTime.value, 0)
  }
})

const startTimeFormatted = computed(() => {
  if (!currentPeriodStartTime.value || !isWorkMode.value) return '未开始'
  const date = new Date(currentPeriodStartTime.value)
  return `${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
})

const estimatedEndTimeFormatted = computed(() => {
  if (!currentPeriodStartTime.value || !isRunning.value || !isWorkMode.value) return '未开始'
  // 计算还需要工作的总时间（秒）
  const remainingWorkTime = targetDuration.value * 60 - elapsedTime.value
  // 基于当前工作时段的开始时间计算预计完成时间
  const endTime = new Date(currentPeriodStartTime.value + remainingWorkTime * 1000)
  return `${String(endTime.getHours()).padStart(2, '0')}:${String(endTime.getMinutes()).padStart(2, '0')}`
})

const buttonText = computed(() => {
  if (!isRunning.value) return '开始'
  return isWorkMode.value ? '休息' : '继续工作'
})

const currentWorkPeriodTime = computed(() => {
  return elapsedTime.value - workTimeAtLastRest.value
})

const earnedRestTime = computed(() => {
  // 当前工作时段应得的休息时间
  return Math.floor(currentWorkPeriodTime.value / 3)
})

const totalRestTime = computed(() => {
  if (!isWorkMode.value) {
    // 休息模式下显示剩余休息时间
    return restTime.value
  }
  // 工作模式下显示可用的总休息时间
  return earnedRestTime.value + accumulatedRestTime.value
})

// 方法
function formatTime(seconds) {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const remainingSeconds = Math.floor(seconds % 60)
  
  if (hours > 0) {
    return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(remainingSeconds).padStart(2, '0')}`
  }
  return `${String(minutes).padStart(2, '0')}:${String(remainingSeconds).padStart(2, '0')}`
}

function toggleTimer() {
  if (!isRunning.value) {
    startTimer()
  } else {
    if (isWorkMode.value) {
      // 从工作切换到休息
      isWorkMode.value = false
      // 设置休息时间为当前工作时段应得的休息时间加上累积的休息时间
      const newRestTime = earnedRestTime.value + accumulatedRestTime.value
      if (newRestTime > 0) {
        restTime.value = newRestTime
        // 记录当前的工作时间点
        workTimeAtLastRest.value = elapsedTime.value
        // 重置累积的休息时间（因为已经开始使用）
        accumulatedRestTime.value = 0
      } else {
        // 如果没有可用的休息时间，恢复工作模式
        isWorkMode.value = true
      }
    } else {
      // 从休息切换到工作
      isWorkMode.value = true
      // 保存剩余的休息时间
      if (restTime.value > 0) {
        accumulatedRestTime.value = restTime.value
      }
      // 更新当前工作时段的开始时间
      currentPeriodStartTime.value = Date.now()
    }
  }
}

function startTimer() {
  const now = Date.now()
  if (!startTime.value) {
    startTime.value = now
    currentPeriodStartTime.value = now
  } else if (isWorkMode.value) {
    // 只在工作模式下更新当前时段开始时间
    currentPeriodStartTime.value = now
  }
  isRunning.value = true
  timer = setInterval(() => {
    if (isWorkMode.value) {
      elapsedTime.value++
      if (elapsedTime.value >= targetDuration.value * 60) {
        celebrate()
        stopTimer()
      }
    } else {
      restTime.value--
      if (restTime.value <= 0) {
        // 休息时间结束
        isWorkMode.value = true
        isRunning.value = false
        clearInterval(timer)
        // 更新当前工作时段的开始时间
        currentPeriodStartTime.value = Date.now()
      }
    }
  }, 1000)
}

function stopTimer() {
  isRunning.value = false
  clearInterval(timer)
}

function resetTimer() {
  stopTimer()
  elapsedTime.value = 0
  restTime.value = 0
  workTimeAtLastRest.value = 0
  accumulatedRestTime.value = 0
  startTime.value = null
  currentPeriodStartTime.value = null
  isWorkMode.value = true
}

function celebrate() {
  const duration = 3 * 1000
  const animationEnd = Date.now() + duration
  const defaults = { startVelocity: 30, spread: 360, ticks: 60, zIndex: 0 }

  function randomInRange(min, max) {
    return Math.random() * (max - min) + min
  }

  const interval = setInterval(() => {
    const timeLeft = animationEnd - Date.now()

    if (timeLeft <= 0) {
      return clearInterval(interval)
    }

    const particleCount = 50 * (timeLeft / duration)
    confetti({
      ...defaults,
      particleCount,
      origin: { x: randomInRange(0.1, 0.3), y: Math.random() - 0.2 }
    })
    confetti({
      ...defaults,
      particleCount,
      origin: { x: randomInRange(0.7, 0.9), y: Math.random() - 0.2 }
    })
  }, 250)
}

// 组件卸载时清理定时器
onUnmounted(() => {
  if (timer) {
    clearInterval(timer)
  }
})
</script>

<style scoped>
.timer-container {
  @apply min-h-screen flex items-center justify-center p-4;
}

.timer-card {
  @apply w-full max-w-2xl p-6 rounded-2xl shadow-xl;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
}

.dark .timer-card {
  background: rgba(31, 41, 55, 0.95);
}

.time-display {
  @apply p-4 rounded-lg text-center transition-all duration-200;
  background: rgba(243, 244, 246, 0.8);  
  border: 1px solid rgba(229, 231, 235, 0.5);
  backdrop-filter: blur(5px);
}

.dark .time-display {
  background: rgba(55, 65, 81, 0.8);
}

.time-display.active {
  @apply ring-2 ring-blue-500 dark:ring-blue-400;
  transform: translateY(-2px);
  background: rgba(243, 244, 246, 0.9);  
}

.dark .time-display.active {
  background: rgba(55, 65, 81, 0.9);
}

.stats-card {
  margin-top: 2rem;
  padding: 1.5rem;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(229, 231, 235, 0.5);
}

.dark .stats-card {
  background: rgba(31, 41, 55, 0.8);
}

.stat-item {
  padding: 1rem;
  border-radius: 12px;
  background: rgba(243, 244, 246, 0.8);
  transition: transform 0.2s ease;
  border: 1px solid rgba(229, 231, 235, 0.5);
  backdrop-filter: blur(5px);
}

.dark .stat-item {
  background: rgba(55, 65, 81, 0.7);
}

.stat-item:hover {
  transform: translateY(-2px);
  background: rgba(243, 244, 246, 0.9);
}

.dark .stat-item:hover {
  background: rgba(55, 65, 81, 0.8);
}
</style>
