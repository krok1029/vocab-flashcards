<script lang="ts">
  export let message = '載入中...';
</script>

<div class="flex items-center justify-center p-4">
  <div class="flex items-center space-x-2">
    <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500"></div>
    <span class="text-gray-600">{message}</span>
  </div>
</div>
