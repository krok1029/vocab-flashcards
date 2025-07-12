<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { toast } from 'svelte-sonner';
  import Button from '$lib/components/ui/button/button.svelte';
  import type { DictionaryEntry } from '$lib/domain/dictionary';
  import { searchWord } from '$lib/usecases/searchDictionary';
  import * as Alert from '$lib/components/ui/alert/index.js';

  let query = '';
  let entry: DictionaryEntry | null = null;
  let error = '';
  let loading = false;
  let audioPlayer: HTMLAudioElement | null = null;
  let existsInCard = false; // 用來控制 UI 顯示「已存在」

  function mapWordCardToDictionaryEntry(card: WordCard): DictionaryEntry {
    return {
      word: card.word,
      phonetic: parsePronunciation(card.pronunciation)?.phonetic ?? '',
      audio: parsePronunciation(card.pronunciation)?.audio ?? '',
      meanings: card.pos
        ? [
            {
              partOfSpeech: card.pos,
              definitions: [
                {
                  definition: card.definition ?? '',
                  example: undefined, // DB 中沒有儲存 example
                },
              ],
            },
          ]
        : [],
    };
  }

  // 輔助 function：從 JSON 字串中取出 pronunciation 音標與音檔
  function parsePronunciation(json: string | null | undefined): {
    phonetic?: string;
    audio?: string;
  } | null {
    try {
      return json ? JSON.parse(json) : null;
    } catch {
      return null;
    }
  }

  async function search() {
    if (!query.trim()) return;
    loading = true;
    error = '';
    entry = null;
    existsInCard = false;

    const word = query.trim();

    try {
      const card = await invoke<WordCard | null>('get_word_card_by_word', {
        wordQuery: word,
      });

      if (card) {
        entry = mapWordCardToDictionaryEntry(card);
        existsInCard = true;
      } else {
        entry = await searchWord(word);
      }
    } catch (e) {
      toast.error(`Search fail: ${e}`);
    } finally {
      loading = false;
    }
  }

  function prepareCardPayload(entry: DictionaryEntry) {
    return {
      word: entry.word,
      pos: JSON.stringify(entry.meanings.map((m) => m.partOfSpeech)),
      definition: entry.meanings[0]?.definitions?.[0]?.definition || '',
      pronunciation: JSON.stringify({
        phonetic: entry.phonetic || '',
        audio: entry.audio || '',
      }),
      verbs: JSON.stringify({}), // 未來若有可擴充
      familiarity: 0,
      seen_count: 1,
    };
  }

  async function saveWordCard() {
    if (!entry) return;
    const card = prepareCardPayload(entry);

    try {
      await invoke('save_word_card', { card });
      toast.success(`已儲存：${card.word}`);
    } catch (e) {
      toast.error(`儲存失敗: ${e}`);
    }
  }

  function playAudio() {
    if (audioPlayer) {
      audioPlayer.currentTime = 0;
      audioPlayer.play();
    }
  }
</script>

<main class="p-4 space-y-4">
  <h1 class="text-2xl font-bold">📖 字典查詢</h1>

  <div class="my-4 text-lg text-gray-500">
    <a href="/import" class="hover:underline text-blue-600">Go to import page</a
    >
  </div>

  {#if existsInCard}
    <Alert.Root>
      <Alert.Title>此單字已存在於單字卡中</Alert.Title>
    </Alert.Root>
  {/if}
  <div class="flex gap-2">
    <input
      class="border rounded px-2 py-1 flex-1"
      placeholder="輸入單字..."
      bind:value={query}
      on:keydown={(e) => e.key === 'Enter' && search()}
    />
    <Button class="bg-blue-500 text-white px-4 py-1 rounded" onclick={search}>
      查詢
    </Button>
    <Button
      class="px-4 py-1 rounded bg-green-600 text-white"
      onclick={saveWordCard}
      disabled={!entry}
    >
      加入單字卡
    </Button>
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
            <audio bind:this={audioPlayer} src={entry.audio} preload="auto"
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
