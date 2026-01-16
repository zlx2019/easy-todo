<script setup>
import { invoke } from '@tauri-apps/api/core';
import { listen, once } from '@tauri-apps/api/event';
import { ref, computed, onMounted } from 'vue'

// 状态定义
const todos = ref([])
const newTitle = ref('')
const newContent = ref('')
const filterStatus = ref('all')
const isEditing = ref(false)
const editingTodo = ref(null)

// 过滤后的待办列表
const filteredTodos = computed(() => {
  switch (filterStatus.value) {
    case 'pending':
      return todos.value.filter(t => t.status === 0)
    case 'completed':
      return todos.value.filter(t => t.status === 1)
    default:
      return todos.value
  }
})

// 统计数据
const stats = computed(() => ({
  total: todos.value.length,
  pending: todos.value.filter(t => t.status === 0).length,
  completed: todos.value.filter(t => t.status === 1).length
}))

// 格式化时间
const formatDate = (dateString) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 获取所有待办
const get_todos = async () => { 
  todos.value = await invoke("todo_list");
  console.log(`reload todos`, todos.value);
}


// 添加新待办
const addTodo = async () => {
  if (!newTitle.value.trim()) return
  if (!newContent.value.trim()) return
  await invoke("add_todo", { "req": { "title": newTitle.value, "content": newContent.value } });
  await get_todos()
  newTitle.value = ''
  newContent.value = ''
}

// 切换状态
const toggleStatus = (todo) => {
  todo.status = todo.status === 0 ? 1 : 0
}

// 删除待办
const deleteTodo = (id) => {
  todos.value = todos.value.filter(t => t.id !== id)
}

// 开始编辑
const startEdit = (todo) => {
  isEditing.value = true
  editingTodo.value = { ...todo }
}

// 取消编辑
const cancelEdit = () => {
  isEditing.value = false
  editingTodo.value = null
}

// 保存编辑
const saveEdit = () => {
  const index = todos.value.findIndex(t => t.id === editingTodo.value.id)
  if (index !== -1) {
    todos.value[index] = { ...editingTodo.value }
  }
  cancelEdit()
}

// 清除已完成的
const clearCompleted = () => {
  todos.value = todos.value.filter(t => t.status === 0)
}

const incr_counter = () => { 
  invoke("incr_counter")
    .then((data) => { 
      console.log(`counter: ${data}`);
    })
  invoke("example").finally(() => { 
    console.log(`call example`);
    
  });
}

// 调用一些发布事件的命令
const publish_event = async () => { 
  await invoke("publish_global_event")
}


// 模拟初始化数据
onMounted(() => {
  // 加载待办
  get_todos();
  // 注册 Tauri 事件监听器
  listen('download-started', (event) => {
    console.log(`[download-started-listen] ${event.id} payload: ${event.payload}`);
  });
  listen('download-progress', (event) => {
    console.log(`[download-progress-listen] ${event.id} payload: ${event.payload}`);
  });
  listen('download-finished', (event) => {
    console.log(`[download-finished-listen] ${event.id} payload: ${event.payload}`);
  });

  

})
</script>

<template>
  <div class="todo-app">
    <!-- 头部 -->
    <header class="app-header">
      <h1 class="app-title">
        <span class="icon">📝</span> Easy Todo
      </h1>
      <p class="app-subtitle">简单高效的待办事项管理</p>
    </header>

    <!-- 统计卡片 -->
    <div class="stats-container">
      <div class="stat-card stat-total">
        <div class="stat-icon">📊</div>
        <div class="stat-info">
          <div class="stat-value">{{ stats.total }}</div>
          <div class="stat-label">总计</div>
        </div>
      </div>
      <div class="stat-card stat-pending">
        <div class="stat-icon">⏳</div>
        <div class="stat-info">
          <div class="stat-value">{{ stats.pending }}</div>
          <div class="stat-label">待完成</div>
        </div>
      </div>
      <div class="stat-card stat-completed">
        <div class="stat-icon">✅</div>
        <div class="stat-info">
          <div class="stat-value">{{ stats.completed }}</div>
          <div class="stat-label">已完成</div>
        </div>
      </div>
    </div>

    <!-- 添加待办表单 -->
    <div class="add-todo-section">
      <div class="input-group">
        <input
          v-model="newTitle"
          type="text"
          placeholder="输入待办标题..."
          class="todo-input title-input"
          @keyup.enter="addTodo"
        />
        <input
          v-model="newContent"
          type="text"
          placeholder="添加描述（可选）..."
          class="todo-input content-input"
          @keyup.enter="addTodo"
        />
        <button @click="addTodo" class="btn btn-primary">
          <span class="btn-icon">+</span> 添加
        </button>
      </div>
    </div>

    <!-- 过滤器 -->
    <div class="filter-section">
      <button
        v-for="f in ['all', 'pending', 'completed']"
        :key="f"
        @click="filterStatus = f"
        :class="['filter-btn', { active: filterStatus === f }]"
      >
        {{ { all: '全部', pending: '待完成', completed: '已完成' }[f] }}
      </button>
      <button
        v-if="stats.completed > 0"
        @click="clearCompleted"
        class="filter-btn clear-btn"
      >
        清除已完成
      </button>
    </div>

    <!-- 待办列表 -->
    <div class="todo-list">
      <div v-if="filteredTodos.length === 0" class="empty-state">
        <div class="empty-icon">📭</div>
        <p>暂无待办事项</p>
        <p class="empty-hint">添加一个新的待办开始吧！</p>
      </div>

      <!-- 待办项 -->
      <div
        v-for="todo in filteredTodos"
        :key="todo.id"
        :class="['todo-item', { completed: todo.status === 1 }]"
      >
        <div class="todo-checkbox" @click="toggleStatus(todo)">
          <div class="checkbox-inner" :class="{ checked: todo.status === 1 }">
            <svg v-if="todo.status === 1" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
          </div>
        </div>

        <div class="todo-content">
          <h3 class="todo-title">{{ todo.title }}</h3>
          <p v-if="todo.content" class="todo-description">{{ todo.content }}</p>
          <div class="todo-meta">
            <span class="todo-date">{{ formatDate(todo.created_at) }}</span>
            <span :class="['todo-status', todo.status === 1 ? 'status-completed' : 'status-pending']">
              {{ todo.status === 1 ? '已完成' : '待完成' }}
            </span>
          </div>
        </div>

        <div class="todo-actions">
          <button @click="startEdit(todo)" class="action-btn edit-btn" title="编辑">
            ✏️
          </button>
          <button @click="deleteTodo(todo.id)" class="action-btn delete-btn" title="删除">
            🗑️
          </button>
        </div>
      </div>



    </div>

    <!-- 编辑模态框 -->
    <div v-if="isEditing" class="modal-overlay" @click.self="cancelEdit">
      <div class="modal">
        <div class="modal-header">
          <h2>编辑待办</h2>
          <button @click="cancelEdit" class="modal-close">✕</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>标题</label>
            <input v-model="editingTodo.title" type="text" class="modal-input" />
          </div>
          <div class="form-group">
            <label>描述</label>
            <textarea v-model="editingTodo.content" class="modal-textarea" rows="3"></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="cancelEdit" class="btn btn-secondary">取消</button>
          <button @click="saveEdit" class="btn btn-primary">保存</button>
        </div>
      </div>
    </div>
  </div>

  <button @click="incr_counter">自增</button>
  <button @click="publish_event" >发布事件</button>
</template>

<style scoped>

</style>
