<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-300">
    <header class="bg-white dark:bg-gray-800 shadow">
      <div class="max-w-7xl mx-auto py-6 px-4">
        <div class="flex justify-between items-center">
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">工作休息平衡计时器</h1>
          <button
            @click="toggleTheme"
            class="p-2 rounded-lg bg-gray-200 dark:bg-gray-700"
          >
            {{ isDark ? '🌞' : '🌙' }}
          </button>
        </div>
      </div>
    </header>

    <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
      <ForwardTimer />
    </main>

    <footer class="bg-white dark:bg-gray-800 shadow mt-8">
      <div class="max-w-7xl mx-auto py-4 px-4 text-center text-gray-600 dark:text-gray-400">
        保持工作与休息的平衡 - 3:1黄金比例
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import ForwardTimer from './components/ForwardTimer.vue'

const isDark = ref(false)

function toggleTheme() {
  isDark.value = !isDark.value
  document.documentElement.classList.toggle('dark')
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
}

onMounted(() => {
  // 检查用户之前的主题偏好
  const theme = localStorage.getItem('theme')
  isDark.value = theme === 'dark'
  if (isDark.value) {
    document.documentElement.classList.add('dark')
  }
})
</script>
