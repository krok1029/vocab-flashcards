<script lang="ts">
  import type { DictionaryEntry } from "$lib/domain/dictionary";
  import { searchWord } from "$lib/usecases/searchDictionary";

  let query = '';
  let entries: DictionaryEntry[] = [];
  let error = '';
  let loading = false;

  async function search() {
    if (!query) return;
    loading = true;
    entries = [];
    error = '';

    try {
      entries = await searchWord(query);
    } catch (e) {
      error = (e as Error).message;
    } finally {
      loading = false;
    }
  }
</script>

<main class="p-4 space-y-4">
  <h1 class="text-2xl font-bold">📖 字典查詢</h1>
  <div class="my-4 text-lg text-gray-500">
    <a href="/import">Go to import page</a>
  </div>

  <div class="flex gap-2">
    <input
      class="border rounded px-2 py-1 flex-1"
      placeholder="輸入單字..."
      bind:value={query}
      on:keydown={(e) => e.key === 'Enter' && search()}
    />
    <button class="bg-blue-500 text-white px-4 py-1 rounded" on:click={search}>
      查詢
    </button>
  </div>

  {#if loading}
    <p>載入中...</p>
  {:else if error}
    <p class="text-red-500">{error}</p>
  {:else if entries.length > 0}
    <div class="space-y-6">
      {#each entries as entry}
        <div class="bg-gray-100 p-4 rounded shadow">
          <p class="text-xl font-bold">{entry.word}</p>

          {#if entry.phonetic}
            <p class="text-gray-600">音標：{entry.phonetic}</p>
          {/if}

          {#each entry.meanings as meaning}
            <div class="mt-3">
              <p class="font-semibold text-blue-600">{meaning.partOfSpeech}</p>
              <ul class="list-disc list-inside ml-4">
                {#each meaning.definitions as def}
                  <li>
                    {def.definition}
                    {#if def.example}
                      <br />
                      <small class="text-gray-500">例句：{def.example}</small>
                    {/if}
                  </li>
                {/each}
              </ul>
            </div>
          {/each}
        </div>
      {/each}
    </div>
  {/if}
</main>
