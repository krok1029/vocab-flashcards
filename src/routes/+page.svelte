<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { DictionaryEntry } from '$lib/domain/dictionary';
  import { searchWord } from '$lib/usecases/searchDictionary';

  let query = '';
  let entry: DictionaryEntry | null = null;
  let error = '';
  let loading = false;
  let audioRef: HTMLAudioElement | null = null;

  async function search() {
    if (!query) return;
    loading = true;
    error = '';
    entry = null;

    try {
      entry = await searchWord(query);
    } catch (e) {
      error = (e as Error).message;
    } finally {
      loading = false;
    }
  }

async function saveWordCard() {
  if (!entry) return;

  const posList = entry.meanings.map((m) => m.partOfSpeech);
  const firstDefinition =
    entry.meanings[0]?.definitions?.[0]?.definition || '';
  const pronunciation = {
    phonetic: entry.phonetic || '',
    audio: entry.audio || '',
  };

  try {
    await invoke('save_word_card', {
      card: {
        word: entry.word,
        pos: JSON.stringify(posList),
        definition: firstDefinition,
        pronunciation: JSON.stringify(pronunciation),
        verbs: JSON.stringify({}), // 暫時無動詞變化資料
        familiarity: 0,
        seen_count: 1,
      },
    });
    alert(`儲存成功：${entry.word}`);
  } catch (e) {
    console.error(e);
    alert('儲存失敗');
  }
}

  function playAudio() {
    if (audioRef) {
      audioRef.currentTime = 0;
      audioRef.play();
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
    <button
      class="px-4 py-2 rounded bg-blue-600 text-white"
      on:click={saveWordCard}
      disabled={!entry}
    >
      加入單字卡
    </button>
  </div>

  {#if loading}
    <p>載入中...</p>
  {:else if error}
    <p class="text-red-500">{error}</p>
  {:else if entry}
    <div class="space-y-6">
      <div class="bg-gray-100 p-4 rounded shadow">
        <p class="text-xl font-bold flex items-center gap-2">
          {entry.word}
          {#if entry.audio}
            <button
              on:click={playAudio}
              class="text-sm text-blue-600 underline hover:text-blue-800"
            >
              🔊 播放發音
            </button>
            <audio bind:this={audioRef} src={entry.audio} preload="auto"
            ></audio>
          {/if}
        </p>

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
    </div>
  {/if}
</main>
