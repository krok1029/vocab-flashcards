<script lang="ts">
  export let title = '單字卡工具';
  export let description = '';
</script>

<svelte:head>
  <title>{title}</title>
  {#if description}
    <meta name="description" content={description} />
  {/if}
</svelte:head>

<div class="min-h-screen bg-gray-50">
  <!-- Navigation Header -->
  <header class="bg-white shadow-sm border-b">
    <div class="container mx-auto px-4 py-3">
      <nav class="flex items-center justify-between">
        <div class="flex items-center space-x-6">
          <h1 class="text-xl font-bold text-gray-800">
            <a href="/" class="hover:text-blue-600 transition-colors">
              📘 單字卡工具
            </a>
          </h1>
          <div class="flex space-x-4">
            <a 
              href="/" 
              class="text-gray-600 hover:text-blue-600 transition-colors"
            >
              字典查詢
            </a>
            <a 
              href="/import" 
              class="text-gray-600 hover:text-blue-600 transition-colors"
            >
              批次匯入
            </a>
            <a 
              href="/cards" 
              class="text-gray-600 hover:text-blue-600 transition-colors"
            >
              單字卡
            </a>
          </div>
        </div>
      </nav>
    </div>
  </header>

  <!-- Main Content -->
  <main class="container mx-auto px-4 py-6">
    <slot />
  </main>

  <!-- Footer -->
  <footer class="bg-white border-t mt-auto">
    <div class="container mx-auto px-4 py-4 text-center text-gray-500 text-sm">
      <p>© 2024 單字卡查詢與複習工具 - Built with Tauri & SvelteKit</p>
    </div>
  </footer>
</div>

<style>
  .container {
    max-width: 1200px;
  }
</style>
