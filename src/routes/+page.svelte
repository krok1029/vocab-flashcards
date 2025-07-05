<script lang="ts">
  let query = '';
  let result: any = null;
  let loading = false;

  async function search() {
    if (!query) return;
    loading = true;

    const res = await fetch(`https://dictionary-api.eliaschen.dev/api/dictionary/en/${query}`);
    result = await res.json();

    loading = false;
  }
</script>

<main class="p-4 space-y-4">
  <h1 class="text-2xl font-bold">字典查詢</h1>
  <div> <a href="/import">go to import page</a></div>

  <div class="flex gap-2">
    <input
      class="border rounded px-2 py-1 flex-1"
      placeholder="輸入單字..."
      bind:value={query}
      on:keydown={(e) => e.key === 'Enter' && search()}
    />
    <button class="bg-blue-500 text-white px-4 py-1 rounded" on:click={search}>查詢</button>
  </div>

  {#if loading}
    <p>載入中...</p>
  {:else if result}
    <div class="bg-gray-100 p-4 rounded">
      <p class="font-semibold text-lg">{result.word}</p>
      <p>詞性：{result.pos?.join(', ')}</p>
      <p>定義：</p>
      <ul class="list-disc list-inside">
        {#each result.definition as def}
          <li>{def.definition} <br /><small class="text-gray-500">{def.example}</small></li>
        {/each}
      </ul>
    </div>
  {/if}
</main>
